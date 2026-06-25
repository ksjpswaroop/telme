//! Telme backend (Phase 3).
//!
//! - Phase 1: global hotkey + title bar show/hide.
//! - Phase 2: SQLite + folder management + walker + chunker + plain-text extractor.
//! - Phase 3: Ollama embeddings + sqlite-vec KNN + hybrid search.
//!
//! Phase 5 will add the FS watcher; Phase 6 polish + Windows.

use std::sync::Arc;

use tauri::{Manager, PhysicalPosition};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub mod chunker;
pub mod config;
pub mod db;
pub mod embedder;
pub mod error;
pub mod extractor;
pub mod folders;
pub mod indexer;
pub mod schema;
pub mod search;
pub mod walker;
pub mod watcher;

use tauri_plugin_opener::OpenerExt;

use embedder::OllamaEmbedder;
pub use error::{AppError, AppResult, TelmeError};

/// Tracks whether the title bar is currently visible.
static TITLEBAR_VISIBLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn primary_hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space)
}
fn fallback_hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::ALT), Code::Space)
}

fn toggle_titlebar(app: &tauri::AppHandle) -> tauri::Result<()> {
    let window = app
        .get_webview_window("titlebar")
        .ok_or_else(|| tauri::Error::WebviewNotFound)?;

    if TITLEBAR_VISIBLE.swap(true, std::sync::atomic::Ordering::SeqCst) {
        window.set_focus()?;
        return Ok(());
    }

    if let Ok(Some(monitor)) = window.current_monitor() {
        let scale = monitor.scale_factor();
        let mon_pos = monitor.position();
        let mon_size = monitor.size();
        let win_w = (700.0_f64 * scale).round() as i32;
        let x = mon_pos.x + ((mon_size.width as i32 - win_w) / 2).max(0);
        let y = mon_pos.y + (80.0_f64 * scale).round() as i32;
        window.set_position(PhysicalPosition::new(x, y))?;
    }

    window.show()?;
    window.set_focus()?;
    Ok(())
}

fn hide_titlebar(app: &tauri::AppHandle) -> tauri::Result<()> {
    let window = app
        .get_webview_window("titlebar")
        .ok_or_else(|| tauri::Error::WebviewNotFound)?;
    TITLEBAR_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
    window.hide()?;
    Ok(())
}

// ---------- Phase 1 commands ----------

#[tauri::command]
fn show_titlebar(app: tauri::AppHandle) -> Result<(), String> {
    toggle_titlebar(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn hide_titlebar_cmd(app: tauri::AppHandle) -> Result<(), String> {
    hide_titlebar(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn close_titlebar(app: tauri::AppHandle) -> Result<(), String> {
    hide_titlebar(&app).map_err(|e| e.to_string())
}

// ---------- Phase 2 commands ----------

#[tauri::command]
fn add_folder(
    db: tauri::State<'_, Arc<db::Db>>,
    watcher: tauri::State<'_, Arc<watcher::WatcherState>>,
    path: String,
) -> Result<(), String> {
    folders::add_folder(db.inner().as_ref(), std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;
    // Re-key the watcher to cover all folders.
    let folders = folders::list_folders(db.inner().as_ref()).map_err(|e| e.to_string())?;
    let paths: Vec<std::path::PathBuf> =
        folders.into_iter().map(std::path::PathBuf::from).collect();
    watcher.rebuild(&paths).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn remove_folder(
    db: tauri::State<'_, Arc<db::Db>>,
    watcher: tauri::State<'_, Arc<watcher::WatcherState>>,
    path: String,
) -> Result<(), String> {
    folders::remove_folder(db.inner().as_ref(), std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;
    let folders = folders::list_folders(db.inner().as_ref()).map_err(|e| e.to_string())?;
    let paths: Vec<std::path::PathBuf> =
        folders.into_iter().map(std::path::PathBuf::from).collect();
    watcher.rebuild(&paths).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn list_folders(db: tauri::State<'_, Arc<db::Db>>) -> Result<Vec<String>, String> {
    folders::list_folders(&db).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_stats(db: tauri::State<'_, Arc<db::Db>>) -> Result<folders::IndexStats, String> {
    folders::get_stats(&db).map_err(|e| e.to_string())
}

#[tauri::command]
fn reindex(db: tauri::State<'_, Arc<db::Db>>) -> Result<indexer::IndexSummary, String> {
    indexer::run_once(&db).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_path(db: tauri::State<'_, Arc<db::Db>>) -> String {
    db.path().to_string_lossy().into_owned()
}

// ---------- Phase 3 commands ----------

#[tauri::command]
fn get_search_status(
    embedder: tauri::State<'_, Arc<OllamaEmbedder>>,
) -> embedder::SearchStatus {
    embedder.status()
}

#[tauri::command]
async fn search(
    db: tauri::State<'_, Arc<db::Db>>,
    embedder: tauri::State<'_, Arc<OllamaEmbedder>>,
    cfg: tauri::State<'_, Arc<parking_lot::Mutex<config::AppConfig>>>,
    query: String,
) -> Result<search::SearchResults, String> {
    let cfg_guard = cfg.lock().clone();
    let db_ref = db.inner().clone();
    let emb_ref: Arc<dyn embedder::Embedder> = embedder.inner().clone();
    search::search(&db_ref, emb_ref.as_ref(), &query, cfg_guard.top_k, cfg_guard.semantic_weight)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn reindex_with_embeddings(
    db: tauri::State<'_, Arc<db::Db>>,
    embedder: tauri::State<'_, Arc<OllamaEmbedder>>,
) -> Result<indexer::IndexSummary, String> {
    let db_ref = db.inner().clone();
    let emb_ref: Arc<dyn embedder::Embedder> = embedder.inner().clone();
    indexer::run_with_embedder(&db_ref, emb_ref)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_file(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn reveal_file(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use std::path::PathBuf;
    let p = PathBuf::from(&path);
    let parent = p.parent().unwrap_or(&p);
    app.opener()
        .open_path(parent.to_string_lossy().into_owned(), None::<&str>)
        .map_err(|e| e.to_string())
}

// ---------- Phase 2 Sprint 3: FS watcher + indexer worker ----------

/// Trigger a manual reindex of a single folder. Returns IndexSummary.
#[tauri::command]
async fn reindex_folder(
    db: tauri::State<'_, Arc<db::Db>>,
    embedder: tauri::State<'_, Arc<OllamaEmbedder>>,
    path: String,
) -> Result<indexer::IndexSummary, String> {
    let db_ref = db.inner().clone();
    let emb_ref: Arc<dyn embedder::Embedder> = embedder.inner().clone();
    let folder = std::path::PathBuf::from(&path);
    // Ensure folder is registered
    let _ = folders::add_folder(&db_ref, &folder);
    let summary = indexer::run_with_embedder(&db_ref, emb_ref)
        .await
        .map_err(|e| e.to_string())?;
    Ok(summary)
}

/// Returns whether the watcher is currently active.
#[tauri::command]
fn watcher_status(watcher: tauri::State<'_, Arc<watcher::WatcherState>>) -> bool {
    watcher.is_active()
}

/// Re-key the watcher to watch exactly the current set of indexed folders.
/// Safe to call repeatedly; cheap when nothing changed.
#[tauri::command]
fn resync_watcher(
    db: tauri::State<'_, Arc<db::Db>>,
    watcher: tauri::State<'_, Arc<watcher::WatcherState>>,
) -> Result<usize, String> {
    let folders = folders::list_folders(db.inner().as_ref()).map_err(|e| e.to_string())?;
    let paths: Vec<std::path::PathBuf> = folders.into_iter().map(std::path::PathBuf::from).collect();
    let n = paths.len();
    watcher
        .rebuild(&paths)
        .map_err(|e| format!("watcher rebuild: {e}"))?;
    Ok(n)
}

// ---------- App entry ----------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,telme_lib=debug")),
        )
        .with_target(false)
        .compact()
        .init();

    let primary = primary_hotkey();
    let fallback = fallback_hotkey();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    if *shortcut == primary || *shortcut == fallback {
                        if let Err(err) = toggle_titlebar(app) {
                            tracing::warn!(?err, "failed to toggle titlebar from hotkey");
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            show_titlebar,
            hide_titlebar_cmd,
            close_titlebar,
            add_folder,
            remove_folder,
            list_folders,
            get_stats,
            reindex,
            db_path,
            get_search_status,
            search,
            reindex_with_embeddings,
            open_file,
            reveal_file,
            reindex_folder,
            watcher_status,
            resync_watcher,
        ])
        .setup(move |app| {
            // ---- Phase 1: hotkey + window ----
            let gs = app.global_shortcut();
            match gs.register(primary) {
                Ok(_) => tracing::info!("registered hotkey: \u{2318}\u{21E7}Space"),
                Err(primary_err) => {
                    tracing::warn!(?primary_err, "primary hotkey unavailable, trying \u{2318}\u{2325}Space");
                    gs.register(fallback)?;
                }
            }
            if let Some(window) = app.get_webview_window("titlebar") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        TITLEBAR_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
                        let _ = win.hide();
                    }
                });
            }

            // ---- Phase 2: open the index DB ----
            let db_path = db::default_db_path()?;
            tracing::info!(path = ?db_path, "opening index database");
            let db = db::Db::open(&db_path)?;
            let db_arc = Arc::new(db);
            app.manage(db_arc.clone());

            // ---- Phase 3: load config + spin up embedder ----
            let cfg = config::load(&db_arc).unwrap_or_default();
            tracing::info!(
                model = %cfg.model,
                semantic_weight = cfg.semantic_weight,
                top_k = cfg.top_k,
                ollama = %cfg.ollama_url,
                "loaded app config"
            );
            app.manage(Arc::new(parking_lot::Mutex::new(cfg.clone())));

            let embedder = OllamaEmbedder::new(cfg)
                .expect("failed to construct Ollama embedder");
            app.manage(Arc::new(embedder));

            // ---- Phase 2 Sprint 3: FS watcher + indexer worker ----
            let watcher_state = Arc::new(watcher::WatcherState::new());
            // Subscribe to events BEFORE rebuilding so events have a target.
            let rx = watcher_state
                .subscribe()
                .expect("watcher subscribe should always succeed");
            // Build the watcher for the current set of indexed folders.
            if let Ok(folder_list) = folders::list_folders(&db_arc) {
                let paths: Vec<std::path::PathBuf> =
                    folder_list.into_iter().map(std::path::PathBuf::from).collect();
                if let Err(e) = watcher_state.rebuild(&paths) {
                    tracing::warn!(error = %e, "initial watcher rebuild failed");
                }
            }
            app.manage(watcher_state);

            // Spawn the indexer worker thread that consumes events and re-scans.
            let db_for_worker = db_arc.clone();
            let embedder_state: tauri::State<'_, Arc<OllamaEmbedder>> =
                app.state::<Arc<OllamaEmbedder>>();
            let emb_for_worker: Arc<dyn embedder::Embedder> = embedder_state.inner().clone();
            let _worker_handle = watcher::spawn_indexer_worker(db_for_worker, emb_for_worker, rx);
            // Intentionally leaked: thread lives for app lifetime.

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db(tag: &str) -> db::Db {
        let dir = std::env::temp_dir().join(format!(
            "telme_test_{}_{}_{:?}",
            tag,
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.db");
        db::Db::open(&path).expect("open db")
    }

    #[test]
    fn add_and_list_folders() {
        let db = temp_db("folders");
        let dir = std::env::temp_dir();
        let canonical = dir.canonicalize().unwrap();
        let path_str = canonical.to_string_lossy().into_owned();

        folders::add_folder(&db, &canonical).unwrap();
        let list = folders::list_folders(&db).unwrap();
        assert_eq!(list, vec![path_str.clone()]);

        // Duplicate should error
        let err = folders::add_folder(&db, &canonical).unwrap_err();
        assert!(matches!(err, TelmeError::Duplicate(_)));

        // Remove and verify gone
        folders::remove_folder(&db, &canonical).unwrap();
        let list = folders::list_folders(&db).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn empty_stats_for_empty_db() {
        let db = temp_db("stats");
        let s = folders::get_stats(&db).unwrap();
        assert_eq!(s.folders, 0);
        assert_eq!(s.files, 0);
        assert_eq!(s.chunks, 0);
    }

    #[test]
    fn indexer_runs_on_populated_folder() {
        let db = temp_db("indexer");
        let root = std::env::temp_dir().join(format!("telme_test_idx_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("a.md"), "# hello world\nthis is a test file").unwrap();
        std::fs::write(root.join("b.rs"), "fn main() { println!(\"hi\"); }").unwrap();
        std::fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        std::fs::write(root.join("node_modules/pkg/junk.js"), "ignored").unwrap();

        folders::add_folder(&db, &root).unwrap();
        let summary = indexer::run_once(&db).unwrap();
        assert!(summary.scanned >= 2, "scanned={}", summary.scanned);
        assert!(summary.indexed >= 2, "indexed={}", summary.indexed);

        let stats = folders::get_stats(&db).unwrap();
        assert!(stats.files >= 2);
        assert!(stats.chunks >= 2);
        assert_eq!(stats.folders, 1);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn chunker_produces_ordinals() {
        let text = "the quick brown fox. ".repeat(500);
        let chunks = chunker::chunk_text(&text);
        assert!(chunks.len() > 1);
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.ordinal, i as u32);
            assert!(!c.text.is_empty());
        }
    }

    #[test]
    fn walker_skips_node_modules_and_hidden() {
        let root = std::env::temp_dir().join(format!("telme_test_walk_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("node_modules/x")).unwrap();
        std::fs::create_dir_all(root.join(".git/objects")).unwrap();
        std::fs::write(root.join("node_modules/x/j.js"), "j").unwrap();
        std::fs::write(root.join(".git/objects/abc"), "g").unwrap();
        std::fs::write(root.join("keep.md"), "k").unwrap();

        let files = walker::walk(&root).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path.file_name().unwrap(), "keep.md");

        let _ = std::fs::remove_dir_all(&root);
    }
}
