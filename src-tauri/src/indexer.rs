//! Indexer — walks folders, extracts text, chunks, embeds, persists.
//!
//! Phase 2 shipped a sync `run_once` for text-only.
//! Phase 3 adds `run_with_embedder`: also embeds chunks via Ollama and stores
//! vectors in `chunk_vectors` (sqlite-vec).
//!
//! Phase 5 will move this onto a dedicated thread with backpressure.

use std::sync::Arc;

use crate::chunker::{chunk_text, Chunk};
use crate::db::Db;
use crate::embedder::Embedder;
use crate::error::{AppError, AppResult};
use crate::extractor;
use crate::folders;
use crate::walker;

/// One pass: walk → extract → chunk → embed → persist.
pub async fn run_with_embedder(
    db: &Db,
    embedder: Arc<dyn Embedder>,
) -> AppResult<IndexSummary> {
    let folder_list = folders::list_folders(db)?;
    let mut summary = IndexSummary::default();

    for folder_str in folder_list {
        let folder = std::path::PathBuf::from(&folder_str);
        let candidates = match walker::walk(&folder) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(path = %folder_str, error = %e, "walk failed");
                continue;
            }
        };
        summary.scanned += candidates.len();

        folders::upsert_pending(db, &candidates)?;
        let (idx, emb) = process_pending(db, embedder.as_ref(), &folder_str).await?;
        summary.indexed += idx;
        summary.embedded += emb;
    }

    Ok(summary)
}

#[derive(Debug, Default, serde::Serialize)]
pub struct IndexSummary {
    pub scanned: usize,
    pub indexed: usize,
    pub embedded: usize,
    pub skipped: usize,
    pub errored: usize,
}

async fn process_pending(
    db: &Db,
    embedder: &dyn Embedder,
    folder_str: &str,
) -> AppResult<(usize, usize)> {
    use rusqlite::params;

    let prefix = format!("{}/", folder_str);

    let to_process: Vec<(i64, String, i64)> = db.with_conn(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, path, mtime FROM files
             WHERE status = 'pending' AND (path = ?1 OR path LIKE ?2)",
        )?;
        let rows = stmt
            .query_map(params![folder_str, format!("{}%", prefix)], |r| {
                Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)?))
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(rows)
    })?;

    let mut newly_indexed = 0;
    let mut newly_embedded = 0;

    for (file_id, path_str, _mtime) in to_process {
        let path = std::path::PathBuf::from(&path_str);
        let text = match extractor::extract(&path) {
            Ok(Some(t)) => t,
            Ok(None) => {
                mark_status(db, file_id, "skipped")?;
                continue;
            }
            Err(e) => {
                tracing::warn!(path = %path_str, error = %e, "extract failed");
                mark_status(db, file_id, "error")?;
                continue;
            }
        };

        let chunks: Vec<Chunk> = chunk_text(&text);
        let chunk_ids = persist_chunks(db, file_id, &chunks)?;
        mark_status(db, file_id, "indexed")?;
        newly_indexed += 1;

        // Embed the chunks we just persisted. Failure is non-fatal: chunks
        // remain searchable via BM25 fallback.
        if !chunk_ids.is_empty() {
            let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
            let text_refs: Vec<&str> = texts.iter().map(String::as_str).collect();
            match embedder.embed_batch(&text_refs).await {
                Ok(vectors) => {
                    for (cid, vec) in chunk_ids.iter().zip(vectors.into_iter()) {
                        if upsert_vector(db, *cid, &vec).is_ok() {
                            newly_embedded += 1;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "embed_batch failed; chunks left un-embedded");
                }
            }
        }
    }

    Ok((newly_indexed, newly_embedded))
}

fn mark_status(db: &Db, file_id: i64, status: &str) -> AppResult<()> {
    db.with_conn(|conn| {
        conn.execute(
            "UPDATE files SET status = ?1, indexed_at = CAST(strftime('%s','now') AS INTEGER)
             WHERE id = ?2",
            rusqlite::params![status, file_id],
        )?;
        Ok(())
    })
}

fn persist_chunks(db: &Db, file_id: i64, chunks: &[Chunk]) -> AppResult<Vec<i64>> {
    if chunks.is_empty() {
        return Ok(Vec::new());
    }
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;
        tx.execute("DELETE FROM chunks WHERE file_id = ?1", [file_id])?;
        let mut stmt = tx.prepare(
            "INSERT INTO chunks (file_id, ordinal, text, token_count) VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut ids = Vec::with_capacity(chunks.len());
        for c in chunks {
            stmt.execute(rusqlite::params![
                file_id,
                c.ordinal as i64,
                c.text,
                c.token_count as i64
            ])?;
            ids.push(tx.last_insert_rowid());
        }
        drop(stmt);
        tx.commit()?;
        Ok(ids)
    })
}

fn upsert_vector(db: &Db, chunk_id: i64, vec: &[f32]) -> AppResult<()> {
    use rusqlite::params;
    let json = serde_json::to_string(vec)
        .map_err(|e| AppError::Other(format!("serialize vec: {e}")))?;
    db.with_conn(|conn| {
        let res = conn.execute(
            "INSERT OR REPLACE INTO chunk_vectors (rowid, embedding) VALUES (?1, ?2)",
            params![chunk_id, json],
        );
        if let Err(e) = res {
            // sqlite-vec likely disabled — silently skip rather than fail indexing.
            tracing::debug!(error = %e, "vector upsert skipped (sqlite-vec disabled?)");
        }
        Ok(())
    })
}

// ---- Sync reindex (Phase 2 compatibility) ----

pub fn run_once(db: &Db) -> AppResult<IndexSummary> {
    let folder_list = folders::list_folders(db)?;
    let mut summary = IndexSummary::default();
    for folder_str in folder_list {
        let folder = std::path::PathBuf::from(&folder_str);
        let candidates = match walker::walk(&folder) {
            Ok(c) => c,
            Err(_) => continue,
        };
        summary.scanned += candidates.len();
        folders::upsert_pending(db, &candidates)?;
        let indexed = process_pending_sync(db, &folder_str)?;
        summary.indexed += indexed;
    }
    Ok(summary)
}

fn process_pending_sync(db: &Db, folder_str: &str) -> AppResult<usize> {
    use rusqlite::params;
    let prefix = format!("{}/", folder_str);
    let to_process: Vec<(i64, String)> = db.with_conn(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, path FROM files WHERE status = 'pending'
             AND (path = ?1 OR path LIKE ?2)",
        )?;
        let rows = stmt
            .query_map(params![folder_str, format!("{}%", prefix)], |r| {
                Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?))
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(rows)
    })?;

    let mut n = 0;
    for (file_id, path_str) in to_process {
        let path = std::path::PathBuf::from(&path_str);
        let text = match extractor::extract(&path) {
            Ok(Some(t)) => t,
            Ok(None) => {
                mark_status(db, file_id, "skipped")?;
                continue;
            }
            Err(_) => {
                mark_status(db, file_id, "error")?;
                continue;
            }
        };
        let chunks = chunk_text(&text);
        persist_chunks(db, file_id, &chunks)?;
        mark_status(db, file_id, "indexed")?;
        n += 1;
    }
    Ok(n)
}
