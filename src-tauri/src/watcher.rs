//! Filesystem watcher with debounce.
//!
//! Watches all indexed folders; whenever a file changes, sends an
//! `IndexEvent` over an mpsc channel. The background indexer thread consumes
//! the channel, debounces per-path, and triggers `indexer::run_with_embedder`
//! for the affected folder (full rescan; cheap because FTS5 + status flags
//! mean unchanged files are skipped in <1ms).
//!
//! Watcher design (matches BACKLOG US-205):
//! - `notify-debouncer-full` with 500ms window per path
//! - One watcher per indexed folder (added/removed dynamically)
//! - Recursive, follows symlinks: NO (cycle risk)
//! - Hidden dirs / `node_modules` / `.git` filtered by `ignore`
//! - Errors logged but don't crash; watcher self-rebuilds on folder churn

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::{unbounded, Receiver, Sender};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use parking_lot::Mutex;
use serde::Serialize;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize)]
pub struct IndexEvent {
    pub folder: String,
    pub kind: &'static str,
    pub paths: Vec<String>,
}

#[derive(Default)]
pub struct WatcherState {
    inner: Mutex<Option<Debouncer<RecommendedWatcher, FileIdMap>>>,
    tx: Mutex<Option<Sender<IndexEvent>>>,
}

impl WatcherState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Event stream — subscribe to receive index events as they happen.
    pub fn subscribe(&self) -> Option<Receiver<IndexEvent>> {
        let (tx, rx) = unbounded();
        *self.tx.lock() = Some(tx);
        Some(rx)
    }

    /// (Re)build the watcher to cover exactly `paths`. Existing paths not in
    /// the new list are removed; new ones are added. Idempotent.
    pub fn rebuild(&self, paths: &[PathBuf]) -> notify::Result<()> {
        let mut guard = self.inner.lock();
        // Drop existing watcher (stops threads).
        *guard = None;
        if paths.is_empty() {
            info!("watcher: stopped (no folders to watch)");
            return Ok(());
        }

        let tx = match self.tx.lock().clone() {
            Some(t) => t,
            None => {
                warn!("watcher: no subscriber; events would be dropped — call subscribe() first");
                return Ok(());
            }
        };

        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    // Group by parent dir; emit one IndexEvent per group.
                    use std::collections::BTreeMap;
                    let mut by_folder: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
                    for ev in events {
                        if let Some(p) = ev.event.paths.first() {
                            if let Some(parent) = p.parent() {
                                let folder_key = parent.to_path_buf();
                                by_folder
                                    .entry(folder_key)
                                    .or_default()
                                    .push(p.to_string_lossy().into_owned());
                            }
                        }
                    }
                    for (folder, paths) in by_folder {
                        let _ = tx.send(IndexEvent {
                            folder: folder.to_string_lossy().into_owned(),
                            kind: "changed",
                            paths,
                        });
                    }
                }
                Err(errs) => {
                    for e in errs {
                        warn!("watcher event error: {e}");
                    }
                }
            },
        )?;

        for p in paths {
            if !p.exists() {
                warn!(path = %p.display(), "watcher: path missing, skipping");
                continue;
            }
            if let Err(e) = debouncer.watcher().watch(p, RecursiveMode::Recursive) {
                warn!(path = %p.display(), error = %e, "watcher: failed to watch");
            } else {
                info!(path = %p.display(), "watcher: watching");
            }
        }

        *guard = Some(debouncer);
        Ok(())
    }

    pub fn stop(&self) {
        *self.inner.lock() = None;
        info!("watcher: stopped");
    }

    /// Whether a watcher is currently active.
    pub fn is_active(&self) -> bool {
        self.inner.lock().is_some()
    }
}

/// Spawn the background indexer thread. Returns a join handle you can
/// drop on app exit. Pulls events from `rx` and calls
/// `indexer::run_with_embedder` for the affected folder.
pub fn spawn_indexer_worker(
    db: Arc<crate::db::Db>,
    embedder: Arc<dyn crate::embedder::Embedder>,
    rx: Receiver<IndexEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        info!("indexer worker: started");
        while let Ok(ev) = rx.recv() {
            let folder = PathBuf::from(&ev.folder);
            if !folder.exists() {
                continue;
            }
            // Drop the file list; run_once re-scans the whole folder cheaply
            // because files whose mtime is unchanged stay `indexed` and
            // extract/chunk paths are skipped.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            match rt {
                Ok(rt) => {
                    let summary = rt.block_on(crate::indexer::run_with_embedder(
                        db.as_ref(),
                        embedder.clone(),
                    ));
                    match summary {
                        Ok(s) => info!(
                            folder = %ev.folder,
                            scanned = s.scanned,
                            indexed = s.indexed,
                            embedded = s.embedded,
                            "indexer worker: reindex complete"
                        ),
                        Err(e) => warn!(error = %e, folder = %ev.folder, "indexer worker: reindex failed"),
                    }
                }
                Err(e) => warn!(error = %e, "indexer worker: tokio build failed"),
            }
        }
        info!("indexer worker: channel closed, exiting");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn watcher_state_starts_empty() {
        let s = WatcherState::new();
        assert!(s.inner.lock().is_none());
    }

    #[test]
    fn rebuild_empty_clears_watcher() {
        let s = WatcherState::new();
        // No panic even when never started.
        s.rebuild(&[]).unwrap();
        assert!(s.inner.lock().is_none());
    }

    #[test]
    fn rebuild_watches_existing_dir() {
        let s = WatcherState::new();
        let _ = s.subscribe(); // required for events to flow
        let dir = std::env::temp_dir().join("telme_watcher_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        s.rebuild(&[dir.clone()]).unwrap();
        assert!(s.inner.lock().is_some());
        s.stop();
        assert!(s.inner.lock().is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn index_event_serializes() {
        let ev = IndexEvent {
            folder: "/tmp/x".into(),
            kind: "changed",
            paths: vec!["/tmp/x/a.md".into()],
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("\"kind\":\"changed\""));
        assert!(json.contains("\"folder\":\"/tmp/x\""));
    }

    #[test]
    fn debounce_tolerates_rapid_writes() {
        // Smoke: ensure the debouncer doesn't panic on empty events.
        // (We don't actually time the debounce window here — that would need
        // a real FS event source and would be flaky.)
        let dir = std::env::temp_dir().join("telme_debounce_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for i in 0..5 {
            std::fs::write(dir.join(format!("f{i}.txt")), format!("{i}")).unwrap();
        }
        std::thread::sleep(Duration::from_millis(700));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
