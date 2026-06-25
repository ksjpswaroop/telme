//! Indexer thread — walks folders, extracts text, chunks, persists.
//!
//! Phase 2 ships a synchronous `run_once` that walks all indexed folders and
//! processes every file. The async/threaded version with backpressure lands in
//! a later phase alongside the file watcher.

use crate::chunker::{chunk_text, Chunk};
use crate::db::Db;
use crate::error::AppResult;
use crate::extractor;
use crate::folders;
use crate::walker;

/// One pass: walk → extract → chunk → persist (text only; embeddings in Phase 3).
pub fn run_once(db: &Db) -> AppResult<IndexSummary> {
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

        // Register all candidates as pending; mtime-unchanged rows stay 'indexed'.
        folders::upsert_pending(db, &candidates)?;

        // Process any pending rows for this folder.
        summary.indexed += process_pending(db, &folder_str)?;
    }

    Ok(summary)
}

#[derive(Debug, Default, serde::Serialize)]
pub struct IndexSummary {
    pub scanned: usize,
    pub indexed: usize,
}

/// Returns count of files newly indexed this pass.
fn process_pending(db: &Db, folder_str: &str) -> AppResult<usize> {
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
        persist_chunks(db, file_id, &chunks)?;
        mark_status(db, file_id, "indexed")?;
        newly_indexed += 1;
    }

    Ok(newly_indexed)
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

fn persist_chunks(db: &Db, file_id: i64, chunks: &[Chunk]) -> AppResult<()> {
    if chunks.is_empty() {
        return Ok(());
    }
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;
        // Wipe any prior chunks (re-index case). FTS triggers clean up.
        tx.execute("DELETE FROM chunks WHERE file_id = ?1", [file_id])?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO chunks (file_id, ordinal, text, token_count) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for c in chunks {
                stmt.execute(rusqlite::params![
                    file_id,
                    c.ordinal as i64,
                    c.text,
                    c.token_count as i64
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    })
}
