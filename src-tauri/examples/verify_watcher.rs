//! End-to-end test of the FS watcher → debounce → indexer worker path.
//!
//! Creates a temp dir, adds it as an indexed folder, registers the watcher
//! on it, then writes/modifies/deletes files. Polls `get_stats` to confirm
//! the worker thread picks up the events and reindexes.
//!
//! Run with:
//!     cargo run --example verify_watcher -- <watch_dir> <db_path>

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use telme_lib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let watch_dir = PathBuf::from(
        args.get(1).cloned().unwrap_or_else(|| "/tmp/telme_watcher_e2e".into()),
    );
    let db_path = PathBuf::from(
        args.get(2).cloned().unwrap_or_else(|| "/tmp/telme_watcher_e2e.db".into()),
    );

    // Fresh DB
    let _ = std::fs::remove_file(&db_path);
    let _ = std::fs::remove_dir_all(&watch_dir);
    std::fs::create_dir_all(&watch_dir)?;

    println!("=== Telme watcher e2e ===");
    println!("watch_dir: {}", watch_dir.display());
    println!("db:        {}", db_path.display());
    println!();

    let db = Arc::new(db::Db::open(&db_path)?);
    folders::add_folder(&db, &watch_dir)?;

    let cfg = config::AppConfig::default();
    let embedder: Arc<dyn embedder::Embedder> =
        Arc::new(embedder::OllamaEmbedder::new(cfg.clone())?);

    // Set up watcher + worker
    let watcher_state = Arc::new(watcher::WatcherState::new());
    let rx = watcher_state.subscribe().expect("subscribe");
    watcher_state.rebuild(&[watch_dir.clone()])?;
    assert!(watcher_state.is_active(), "watcher should be active after rebuild");
    let _worker = watcher::spawn_indexer_worker(db.clone(), embedder.clone(), rx);

    // Initial empty state
    println!("0. Initial state (empty watch_dir)...");
    let s0 = folders::get_stats(&db)?;
    println!("   chunks: {}", s0.chunks);
    assert_eq!(s0.chunks, 0, "no chunks before writes");

    // Write 3 files in quick succession — debouncer should collapse into one event
    println!();
    println!("1. Writing 3 files (debouncer should fire once)...");
    for i in 0..3 {
        std::fs::write(
            watch_dir.join(format!("file_{i}.md")),
            format!("# File {i}\n\nContent {i}"),
        )?;
    }
    println!("   wrote files. Waiting up to 5s for worker...");

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut chunks = 0;
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(200));
        chunks = folders::get_stats(&db)?.chunks;
        if chunks >= 3 {
            break;
        }
    }
    println!("   chunks after write: {}", chunks);
    assert!(chunks >= 3, "expected >= 3 chunks after write; got {chunks}");

    // Modify a file — should re-extract (chunks stay at 3, mtime updates)
    println!();
    println!("2. Modifying file_0.md...");
    std::fs::write(
        watch_dir.join("file_0.md"),
        "# File 0 — UPDATED\n\nMore content here to ensure a fresh chunk.",
    )?;
    std::thread::sleep(Duration::from_millis(200));
    let chunks_after_modify = folders::get_stats(&db)?.chunks;
    println!("   chunks after modify: {}", chunks_after_modify);
    // chunks count is per-file and we modified only file_0 — total should stay >= 3

    // Delete a file
    println!();
    println!("3. Deleting file_1.md...");
    std::fs::remove_file(watch_dir.join("file_1.md"))?;
    let mut chunks_after_delete = chunks_after_modify;
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(200));
        chunks_after_delete = folders::get_stats(&db)?.chunks;
        if chunks_after_delete < chunks_after_modify {
            break;
        }
    }
    println!("   chunks after delete: {}", chunks_after_delete);
    assert!(
        chunks_after_delete < chunks_after_modify,
        "delete should reduce chunk count; before={chunks_after_modify} after={chunks_after_delete}"
    );

    // Drop the watch — verify graceful stop
    println!();
    println!("4. Stopping watcher...");
    watcher_state.stop();
    assert!(!watcher_state.is_active(), "watcher should be stopped");
    println!("   stopped cleanly");

    println!();
    println!("✅ Watcher e2e passed.");
    Ok(())
}
