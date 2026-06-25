//! End-to-end verification of the Telme search pipeline.
//!
//! Walks a test corpus, extracts text, chunks, embeds via Ollama, persists
//! to SQLite, then issues a real search and prints hits.
//!
//! Run with:
//!     cargo run --example verify_e2e -- <corpus_path> <db_path>

use std::path::PathBuf;
use std::sync::Arc;

use telme_lib::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let corpus = PathBuf::from(args.get(1).cloned().unwrap_or_else(|| "/tmp/telme_verify_corpus".into()));
    let db_path = PathBuf::from(args.get(2).cloned().unwrap_or_else(|| "/tmp/telme_verify.db".into()));

    // Force a fresh DB.
    let _ = std::fs::remove_file(&db_path);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    println!("=== Telme end-to-end verification ===");
    println!("corpus: {}", corpus.display());
    println!("db:     {}", db_path.display());
    println!();

    let db = db::Db::open(&db_path)?;
    folders::add_folder(&db, &corpus)?;

    let cfg = config::AppConfig::default();
    let embedder: Arc<dyn embedder::Embedder> = Arc::new(embedder::OllamaEmbedder::new(cfg.clone())?);

    println!("1. Ping Ollama via direct HTTP test...");
    let ping_url = format!("{}/api/tags", cfg.ollama_url);
    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    let reachable = http.get(&ping_url).send().await.map(|r| r.status().is_success()).unwrap_or(false);
    println!("   ollama_reachable: {reachable}");
    if !reachable {
        eprintln!("   ⚠ Ollama not reachable; results will degrade to BM25-only");
    }

    println!();
    println!("2. Indexing...");
    let summary = indexer::run_with_embedder(&db, embedder.clone()).await?;
    println!("   scanned:  {}", summary.scanned);
    println!("   indexed:  {}", summary.indexed);
    println!("   embedded: {}", summary.embedded);
    println!("   skipped:  {}", summary.skipped);
    println!("   errored:  {}", summary.errored);

    println!();
    println!("3. Stats...");
    let stats = folders::get_stats(&db)?;
    println!("   folders:  {}", stats.folders);
    println!("   files:    {}", stats.files);
    println!("   chunks:   {}", stats.chunks);
    println!("   bytes:    {}", stats.bytes_indexed);

    println!();
    println!("4. Search queries...");
    let queries = [
        ("semantic-ir",    "how do search engines rank documents?"),
        ("semantic-rust",  "Rust performance optimization for production"),
        ("keyword-bm25",   "sqlite-vec"),
        ("mixed",          "tauri global shortcut"),
        ("nonsense",       "quantum entanglement in garden gnomes"),
    ];
    for (label, q) in queries {
        let resp = search::search(&db, embedder.as_ref(), q, 5, 0.7).await?;
        println!(
            "   [{label:>15}] q={q:?} latency={}ms degraded={} hits={}",
            resp.latency_ms, resp.degraded, resp.hits.len()
        );
        for (i, h) in resp.hits.iter().enumerate() {
            let fname = std::path::Path::new(&h.path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&h.path);
            let snip = h.snippet.replace('\n', " ");
            let snip = if snip.len() > 80 { format!("{}…", &snip[..80]) } else { snip };
            println!(
                "     #{i} ({kind:?}) score={score:.3} {fname} — {snip}",
                i = i + 1,
                kind = h.kind,
                score = h.score,
                fname = fname,
                snip = snip
            );
        }
    }

    println!();
    println!("5. Verify schema + FTS triggers...");
    let conn = rusqlite::Connection::open(&db_path)?;
    let ver: i32 = conn.query_row("SELECT version FROM schema_version ORDER BY version DESC LIMIT 1", [], |r| r.get(0))?;
    println!("   schema_version: {ver}");
    let fts_count: i64 = conn.query_row("SELECT COUNT(*) FROM chunks_fts", [], |r| r.get(0))?;
    let chunk_count: i64 = conn.query_row("SELECT COUNT(*) FROM chunks", [], |r| r.get(0))?;
    println!("   chunks in FTS: {fts_count} | chunks table: {chunk_count}");
    assert_eq!(fts_count, chunk_count, "FTS out of sync with chunks");

    println!();
    println!("6. Remove folder...");
    folders::remove_folder(&db, &corpus)?;
    let after = folders::get_stats(&db)?;
    println!("   files after remove:  {}", after.files);
    println!("   chunks after remove: {}", after.chunks);
    assert_eq!(after.files, 0, "files not cleaned up");
    assert_eq!(after.chunks, 0, "chunks not cleaned up");

    println!();
    println!("7. Reindex again WITHOUT removing; check embeddings this time...");
    folders::add_folder(&db, &corpus)?;
    let summary2 = indexer::run_with_embedder(&db, embedder.clone()).await?;
    println!("   re-index scanned:  {}", summary2.scanned);
    println!("   re-index indexed:  {}", summary2.indexed);
    println!("   re-index embedded: {}", summary2.embedded);

    // Check vectors table now.
    let conn2 = rusqlite::Connection::open(&db_path)?;
    let has_vec_table: i64 = conn2
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='chunk_vectors'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    println!("   chunk_vectors table exists: {}", has_vec_table > 0);
    if has_vec_table > 0 {
        let n: i64 = conn2.query_row("SELECT COUNT(*) FROM chunk_vectors", [], |r| r.get(0))?;
        println!("   vectors in chunk_vectors: {}", n);
    }

    println!();
    println!("✅ All checks passed.");
    Ok(())
}
