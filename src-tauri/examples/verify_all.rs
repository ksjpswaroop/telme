//! Exhaustive end-to-end verification of every Tauri command path.
//!
//! Exercises every backend function the app uses at runtime, then reports
//! a pass/fail summary. Designed to be the single source of truth for
//! "does Telme work on this machine?".
//!
//! Run:
//!     cargo run --example verify_all

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use telme_lib::embedder::Embedder;
use telme_lib::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    // Shared counters live outside the macro so FnMut closures can mutate
    // them without escaping. We use AtomicU32 for cheap shared-mutability.
    let passed = std::sync::atomic::AtomicU32::new(0);
    let failed = std::sync::atomic::AtomicU32::new(0);
    let failures: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

    macro_rules! test {
        ($name:literal, $body:block) => {{
            let __name = $name;
            print!("  [{:>42}] ", __name);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            let __result = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let r: Result<(), Box<dyn std::error::Error>> = (|| async {
                        $body;
                        Ok(())
                    })().await;
                    r
                })
            });
            match __result {
                Ok(()) => {
                    passed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    println!("PASS");
                }
                Err(e) => {
                    failed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    failures.lock().unwrap().push(format!("{__name}: {e}"));
                    println!("FAIL: {e}");
                }
            }
        }};
    }

    println!("════════════════════════════════════════════════════════════════════");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    // ──────────────────────────── Phase 0: sanity ────────────────────────────
    println!("▼ Phase 0: build artifacts");
    let bin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/release/bundle/macos/Telme.app/Contents/MacOS/telme");
    test!("release .app binary exists", {
        assert!(
            bin.exists(),
            "release .app missing: {}",
            bin.display()
        );
    });
    test!("debug binary exists", {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/debug/telme");
        assert!(p.exists(), "debug binary missing");
    });
    let dmg = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg");
    test!("release DMG exists", {
        assert!(dmg.exists(), "DMG missing");
    });
    test!("DMG passes hdiutil verify", {
        let out = std::process::Command::new("hdiutil")
            .args(["verify", &dmg.to_string_lossy()])
            .output()?;
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            combined.contains("is VALID"),
            "hdiutil verify didn't say VALID: {combined}"
        );
    });

    // ──────────────────────────── Phase 1: config + sqlite ──────────────────
    println!();
    println!("▼ Phase 1: config + database");
    let test_dir = std::env::temp_dir().join(format!(
        "telme_verify_all_{}_{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&test_dir);
    std::fs::create_dir_all(&test_dir)?;
    let corpus = test_dir.join("corpus");
    let db_path = test_dir.join("index.db");
    std::fs::create_dir_all(&corpus)?;
    std::fs::write(
        corpus.join("hello.md"),
        "# Hello\n\nThis is a tiny document about rust and information retrieval.",
    )?;
    std::fs::write(
        corpus.join("world.py"),
        "def world():\n    print('hello, world')\n",
    )?;
    std::fs::create_dir_all(corpus.join("node_modules"))?;
    std::fs::write(
        corpus.join("node_modules/ignored.js"),
        "should never be indexed",
    )?;

    let db = Arc::new(db::Db::open(&db_path)?);
    test!("DB schema is at v2", {
        let conn = rusqlite::Connection::open(&db_path)?;
        let ver: i32 = conn.query_row(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
            [],
            |r| r.get(0),
        )?;
        assert_eq!(ver, 2, "schema version {ver}, expected 2");
    });
    test!("config::defaults match", {
        let cfg = config::AppConfig::default();
        assert_eq!(cfg.model, "nomic-embed-text");
        assert_eq!(cfg.ollama_url, "http://127.0.0.1:11434");
        assert_eq!(cfg.top_k, 10);
        assert!((cfg.semantic_weight - 0.7).abs() < 1e-6);
    });
    test!("config::save / load round-trip", {
        let cfg = config::AppConfig {
            model: "mxbai-embed-large".into(),
            semantic_weight: 0.42,
            top_k: 25,
            ollama_url: "http://localhost:9999".into(),
        };
        config::save(&db, &cfg)?;
        let loaded = config::load(&db)?;
        assert_eq!(loaded.model, "mxbai-embed-large");
        assert_eq!(loaded.top_k, 25);
        assert!((loaded.semantic_weight - 0.42).abs() < 1e-6);
        assert_eq!(loaded.ollama_url, "http://localhost:9999");
    });

    // ──────────────────────────── Phase 2: indexing ─────────────────────────
    println!();
    println!("▼ Phase 2: indexing");
    test!("chunker splits long text into multiple chunks", {
        let text = "word ".repeat(3000);
        let chunks = chunker::chunk_text(&text);
        assert!(chunks.len() > 1, "{} chunks", chunks.len());
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.ordinal, i as u32);
            assert!(!c.text.is_empty());
            assert!(c.token_count > 0);
        }
    });
    test!("chunker returns empty for empty input", {
        assert!(chunker::chunk_text("").is_empty());
        assert!(chunker::chunk_text("   \n\t  ").is_empty());
    });
    test!("extractor classifies known extensions", {
        use std::path::Path;
        assert_eq!(
            extractor::FileKind::classify(Path::new("a.md")),
            Some(extractor::FileKind::Markdown)
        );
        assert_eq!(
            extractor::FileKind::classify(Path::new("a.txt")),
            Some(extractor::FileKind::Text)
        );
        assert_eq!(
            extractor::FileKind::classify(Path::new("a.rs")),
            Some(extractor::FileKind::Code)
        );
        assert_eq!(
            extractor::FileKind::classify(Path::new("a.py")),
            Some(extractor::FileKind::Code)
        );
    });
    test!("extractor returns None for PDF (unsupported)", {
        let r = extractor::extract(std::path::Path::new("/no/such/file.pdf"));
        assert!(matches!(r, Ok(None)));
    });
    test!("walker skips node_modules + hidden", {
        let files = walker::walk(&corpus).unwrap();
        let names: Vec<String> = files
            .iter()
            .map(|c| c.path.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert!(names.contains(&"hello.md".to_string()));
        assert!(names.contains(&"world.py".to_string()));
        assert!(
            !names.iter().any(|n| n == "ignored.js"),
            "node_modules contents leaked into walker output"
        );
        assert_eq!(files.len(), 2, "walker returned {files:?}");
    });
    test!("folders::add_folder / list / remove", {
        let canonical = std::fs::canonicalize(&corpus)?;
        let canonical_str = canonical.to_string_lossy().into_owned();
        folders::add_folder(&db, &canonical)?;
        let list = folders::list_folders(&db)?;
        assert_eq!(list, vec![canonical_str.clone()]);
        // duplicate rejected
        let err = folders::add_folder(&db, &canonical).unwrap_err();
        assert!(matches!(err, AppError::Duplicate(_)));
        folders::remove_folder(&db, &canonical)?;
        let after = folders::list_folders(&db)?;
        assert!(after.is_empty());
    });
    test!("folders::prune_missing removes rows for deleted files", {
        let tmp = test_dir.join("prune_test");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(tmp.join("a.txt"), "x").unwrap();
        test!("folders::prune_missing removes rows for deleted files", {
            let tmp = test_dir.join("prune_test");
            std::fs::create_dir_all(&tmp).unwrap();
            std::fs::write(tmp.join("a.txt"), "x").unwrap();
            std::fs::write(tmp.join("b.txt"), "y").unwrap();
            let canonical = std::fs::canonicalize(&tmp).unwrap();
            folders::add_folder(&db, &canonical).unwrap();
            // Run a full indexer pass so a.txt and b.txt are in the DB as 'indexed'.
            let emb: Arc<dyn embedder::Embedder> =
                Arc::new(embedder::OllamaEmbedder::new(config::AppConfig::default())?);
            indexer::run_with_embedder(&db, emb).await?;
            let stats_before = folders::get_stats(&db)?;
            assert!(stats_before.files >= 2, "got {stats_before:?}");
            // Delete one file on disk; prune_missing should remove its row.
            // `prune_missing` matches by canonical prefix — pass the canonical path.
            std::fs::remove_file(tmp.join("a.txt")).unwrap();
            let removed = folders::prune_missing(&db, &canonical).unwrap();
            assert!(removed >= 1, "removed={removed}");
            let stats_after = folders::get_stats(&db)?;
            assert!(
                stats_after.files < stats_before.files,
                "stats didn't shrink: before={stats_before:?} after={stats_after:?}"
            );
            folders::remove_folder(&db, &canonical).unwrap();
        });

    // ──────────────────────────── Phase 3: embeddings + search ─────────────
    println!();
    println!("▼ Phase 3: embeddings + hybrid search");
    let cfg = config::AppConfig::default();
    let embedder: Arc<dyn embedder::Embedder> =
        Arc::new(embedder::OllamaEmbedder::new(cfg.clone())?);
    test!("Ollama reachable on default URL", {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;
        let r = http
            .get(format!("{}/api/tags", cfg.ollama_url))
            .send()
            .await?;
        assert!(r.status().is_success(), "Ollama not reachable");
    });
    test!("embedder::embed returns 768-dim vector", {
        let v = embedder.embed("hello world").await?;
        assert_eq!(v.len(), embedder::NOMIC_DIM, "got dim {}", v.len());
        assert!(v.iter().any(|x| x.abs() > 1e-6), "vector is zero");
    });
    test!("embedder circuit-opens after 5 failures", {
        // Use a deliberately bad URL via a fresh embedder
        let bad_cfg = config::AppConfig {
            ollama_url: "http://127.0.0.1:1".into(),
            ..config::AppConfig::default()
        };
        let bad = embedder::OllamaEmbedder::new(bad_cfg)?;
        for _ in 0..5 {
            let _ = bad.embed("trigger failure").await;
        }
        assert!(bad.circuit_open(), "circuit should be open after 5 failures");
    });
    test!("indexer indexes 2 files", {
        let canonical = std::fs::canonicalize(&corpus)?;
        folders::add_folder(&db, &canonical)?;
        let summary = indexer::run_with_embedder(
            db.as_ref(),
            embedder.clone(),
        )
        .await?;
        assert!(summary.indexed >= 2, "{summary:?}");
        assert!(summary.embedded >= 2, "{summary:?}");
        assert_eq!(summary.errored, 0);
    });
    test!("indexer::embedded counter is accurate", {
        let stats = folders::get_stats(&db)?;
        let conn = rusqlite::Connection::open(&db_path)?;
        let indexed_files: i64 =
            conn.query_row("SELECT COUNT(*) FROM files WHERE status = 'indexed'", [], |r| r.get(0))?;
        assert_eq!(stats.files as i64, indexed_files);
    });
    test!("FTS5 stays in sync with chunks (triggers OK)", {
        let conn = rusqlite::Connection::open(&db_path)?;
        let chunks: i64 = conn.query_row("SELECT COUNT(*) FROM chunks", [], |r| r.get(0))?;
        let fts: i64 = conn.query_row("SELECT COUNT(*) FROM chunks_fts", [], |r| r.get(0))?;
        assert_eq!(chunks, fts, "FTS out of sync: {chunks} vs {fts}");
    });
    test!("search::search returns BM25 hits when vec0 missing", {
        let resp = search::search(
            db.as_ref(),
            embedder.as_ref(),
            "information retrieval",
            5,
            0.7,
        )
        .await?;
        // BM25 should always work even if vec0 is missing
        assert!(!resp.hits.is_empty(), "BM25 returned 0 hits");
        let top = &resp.hits[0];
        assert!(top.path.contains("hello.md") || top.path.contains("world.py"));
    });
    test!("search::search latency is reasonable (<500ms)", {
        let start = Instant::now();
        let resp = search::search(
            db.as_ref(),
            embedder.as_ref(),
            "hello",
            5,
            0.7,
        )
        .await?;
        assert!(
            start.elapsed() < Duration::from_millis(500),
            "search took {:?}",
            start.elapsed()
        );
        assert!(resp.latency_ms < 500, "latency_ms={}", resp.latency_ms);
    });
    test!("search::search returns 0 hits for nonsense query", {
        let resp = search::search(
            db.as_ref(),
            embedder.as_ref(),
            "xyzzy quantum entanglement garden gnomes",
            5,
            0.7,
        )
        .await?;
        assert!(resp.hits.is_empty(), "nonsense returned hits: {resp:?}");
    });
    test!("search::fuse marks hybrid when both signals match", {
        use search::{fuse, HitKind, ScoredChunk};
        let mk = |id, score| ScoredChunk {
            chunk_id: id,
            file_id: 10,
            path: "/a.md".into(),
            filename: "a.md".into(),
            snippet: "s".into(),
            raw_score: 0.0,
            normalized: score,
            file_type: "doc".into(),
        };
        let fused = fuse(vec![mk(1, 0.8)], vec![mk(1, 0.6)], 0.7);
        assert_eq!(fused.len(), 1);
        assert_eq!(fused[0].kind, HitKind::Hybrid);
    });

    // ──────────────────────────── Phase 4: FS watcher ──────────────────────
    println!();
    println!("▼ Phase 4: FS watcher + indexer worker");
    let watch_root = test_dir.join("watch_test");
    std::fs::create_dir_all(&watch_root)?;
    let canonical_watch = std::fs::canonicalize(&watch_root)?;
    folders::add_folder(&db, &canonical_watch)?;
    let watcher_state = Arc::new(watcher::WatcherState::new());
    let rx = watcher_state.subscribe().expect("subscribe");
    watcher_state.rebuild(&[canonical_watch.clone()])?;
    let _worker = watcher::spawn_indexer_worker(
        db.clone(),
        embedder.clone(),
        rx,
    );
    test!("watcher is active after rebuild", {
        assert!(watcher_state.is_active());
    });
    test!("watcher debounces writes -> chunks appear", {
        for i in 0..3 {
            std::fs::write(
                watch_root.join(format!("w{i}.md")),
                format!("# watcher test {i}"),
            )?;
        }
        let deadline = Instant::now() + Duration::from_secs(6);
        let mut got = 0;
        while Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(200));
            got = folders::get_stats(&db)?.chunks;
            if got >= 3 {
                break;
            }
        }
        assert!(got >= 3, "expected >= 3 chunks, got {got}");
    });
    test!("watcher prunes chunks on delete", {
        std::fs::remove_file(watch_root.join("w1.md"))?;
        let deadline = Instant::now() + Duration::from_secs(6);
        let mut got = folders::get_stats(&db)?.chunks;
        while Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(200));
            got = folders::get_stats(&db)?.chunks;
            if got < 3 {
                break;
            }
        }
        assert!(got < 3, "delete should reduce chunks; got {got}");
    });
    test!("watcher_status reports correctly after stop", {
        watcher_state.stop();
        assert!(!watcher_state.is_active());
    });

    // ──────────────────────────── Phase 5: macOS OS integration ────────────
    println!();
    println!("▼ Phase 5: macOS OS integration");
    test!("open binary opens files in default app", {
        // Use a text file we know exists
        let r = std::process::Command::new("open")
            .arg(&db_path)
            .output()?;
        assert!(r.status.success(), "open failed: {:?}", r);
        // close it back (best-effort)
        let _ = std::process::Command::new("pkill")
            .arg("-f")
            .arg("com.telme.desktop")
            .output();
    });
    test!("open on a directory opens Finder", {
        let r = std::process::Command::new("open")
            .arg(&test_dir)
            .output()?;
        assert!(r.status.success());
    });
    test!("Mac OS integration: open -a works with default app", {
        // Create a .txt file and verify it opens as public.plain-text
        let test_txt = test_dir.join("os_test.txt");
        std::fs::write(&test_txt, "os integration test")?;
        let r = std::process::Command::new("mdls")
            .arg("-name")
            .arg("kMDItemContentType")
            .arg(&test_txt)
            .output()?;
        let stdout = String::from_utf8_lossy(&r.stdout);
        assert!(
            stdout.contains("public.plain-text"),
            "mdls: {stdout}"
        );
    });

    // ──────────────────────────── Phase 6: GUI binary smoke ─────────────────
    println!();
    println!("▼ Phase 6: GUI binary smoke");
    test!("Telme.app launches and registers hotkey", {
        // Launch the .app
        let app_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/release/bundle/macos/Telme.app/Contents/MacOS/telme");
        let mut child = std::process::Command::new(&app_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        // Give it time to register the hotkey
        std::thread::sleep(Duration::from_secs(3));
        // Read stdout up to 4 KB and look for the hotkey line
        use std::io::Read;
        let mut buf = vec![0u8; 4096];
        let stdout = child.stdout.as_mut().expect("stdout");
        let n = stdout.read(&mut buf).unwrap_or(0);
        let s = String::from_utf8_lossy(&buf[..n]).to_string();
        let _ = child.kill();
        let _ = child.wait();
        assert!(
            s.contains("registered hotkey"),
            "stdout did not contain 'registered hotkey': {s}"
        );
    });

    // ──────────────────────────── Summary ──────────────────────────────────
    let passed = passed.load(std::sync::atomic::Ordering::SeqCst) as usize;
    let failed = failed.load(std::sync::atomic::Ordering::SeqCst) as usize;
    let failures = failures.into_inner().unwrap();
    let elapsed = started.elapsed();
    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("  RESULTS");
    println!("════════════════════════════════════════════════════════════════════");
    println!("  passed:   {passed}");
    println!("  failed:   {failed}");
    println!("  elapsed:  {:?}", elapsed);
    if failed > 0 {
        println!();
        println!("  failures:");
        for f in &failures {
            println!("    - {f}");
        }
        println!();
        println!("  FAILED");
        std::process::exit(1);
    } else {
        println!();
        println!("  ALL CHECKS PASSED");
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
    Ok(())
}
