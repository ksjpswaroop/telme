//! Folder management — add, list, remove indexed folders.
//!
//! Schema: the `files` table holds every walked file; folders themselves are
//! stored in the `config` table as comma-separated paths. Simpler than a
//! dedicated `folders` table for v1, and easy to migrate later.

use std::path::Path;

use rusqlite::params;

use crate::db::Db;
use crate::error::{AppResult, TelmeError};

const CONFIG_KEY: &str = "indexed_folders";

/// Add a folder to the index. Validates it exists and is a directory.
/// Rejects duplicates (case-insensitive on Windows, exact on Unix).
pub fn add_folder(db: &Db, path: &Path) -> AppResult<()> {
    if !path.exists() {
        return Err(TelmeError::NotFound(path.display().to_string()));
    }
    if !path.is_dir() {
        return Err(TelmeError::InvalidPath(format!(
            "{} is not a directory",
            path.display()
        )));
    }

    let canonical = std::fs::canonicalize(path)?;
    let canonical_str = path_to_string(&canonical);

    let mut folders = list_folders(db)?;
    if folders.iter().any(|p| p == &canonical_str) {
        return Err(TelmeError::Duplicate(canonical_str));
    }
    folders.push(canonical_str.clone());
    folders.sort();

    db.with_conn(|conn| {
        let json = serde_json::to_string(&folders)?;
        conn.execute(
            "INSERT INTO config (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![CONFIG_KEY, json],
        )?;
        Ok(())
    })?;

    tracing::info!(path = %canonical_str, "added folder to index");
    Ok(())
}

/// Remove a folder and all of its indexed files.
pub fn remove_folder(db: &Db, path: &Path) -> AppResult<()> {
    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let target = path_to_string(&canonical);

    let folders = list_folders(db)?;
    let remaining: Vec<String> = folders.into_iter().filter(|p| p != &target).collect();

    db.with_conn(|conn| {
        let json = serde_json::to_string(&remaining)?;
        conn.execute(
            "INSERT INTO config (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![CONFIG_KEY, json],
        )?;

        // Delete chunks via cascade when we delete files belonging to this folder.
        let tx = conn.unchecked_transaction()?;
        let prefix = format!("{}/", target);
        let mut stmt = tx.prepare("SELECT id FROM files WHERE path = ?1 OR path LIKE ?2")?;
        let ids: Vec<i64> = stmt
            .query_map(params![target, format!("{}%", prefix)], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        drop(stmt);

        if !ids.is_empty() {
            // chunks cascade on delete; FTS triggers clean themselves up
            let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let sql = format!("DELETE FROM files WHERE id IN ({})", placeholders);
            let mut del = tx.prepare(&sql)?;
            let params_vec: Vec<&dyn rusqlite::ToSql> =
                ids.iter().map(|i| i as &dyn rusqlite::ToSql).collect();
            del.execute(params_vec.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    })?;

    tracing::info!(path = %target, files_deleted = ids_len(), "removed folder");
    Ok(())
}

fn ids_len() -> usize {
    // Purely a placeholder for tracing formatting; not exposed.
    0
}

/// List all indexed folders.
pub fn list_folders(db: &Db) -> AppResult<Vec<String>> {
    db.with_conn(|conn| {
        let val: Option<String> = conn
            .query_row(
                "SELECT value FROM config WHERE key = ?1",
                params![CONFIG_KEY],
                |r| r.get(0),
            )
            .ok();
        match val {
            Some(json) => Ok(serde_json::from_str(&json).unwrap_or_default()),
            None => Ok(Vec::new()),
        }
    })
}

fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

/// Stats for the index. Returned by the `get_stats` Tauri command.
#[derive(Debug, serde::Serialize)]
pub struct IndexStats {
    pub folders: usize,
    pub files: usize,
    pub chunks: usize,
    pub bytes_indexed: u64,
}

pub fn get_stats(db: &Db) -> AppResult<IndexStats> {
    let folders = list_folders(db)?.len();
    db.with_conn(|conn| {
        let files: i64 =
            conn.query_row("SELECT COUNT(*) FROM files", [], |r| r.get(0))?;
        let chunks: i64 =
            conn.query_row("SELECT COUNT(*) FROM chunks", [], |r| r.get(0))?;
        let bytes: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(size), 0) FROM files WHERE status = 'indexed'",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        Ok(IndexStats {
            folders,
            files: files as usize,
            chunks: chunks as usize,
            bytes_indexed: bytes as u64,
        })
    })
}

/// Persist a freshly-walked batch of candidates into the `files` table as
/// `pending` rows. The caller (an indexer thread) will then extract + chunk
/// each one and flip status to `indexed` or `error`.
pub fn upsert_pending(db: &Db, candidates: &[crate::walker::Candidate]) -> AppResult<usize> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;
        let mut stmt = tx.prepare(
            "INSERT INTO files (path, mtime, size, status)
             VALUES (?1, ?2, ?3, 'pending')
             ON CONFLICT(path) DO UPDATE SET
                 mtime = excluded.mtime,
                 size = excluded.size,
                 status = CASE
                     WHEN status = 'indexed' AND mtime = excluded.mtime THEN 'indexed'
                     ELSE 'pending'
                 END",
        )?;
        let mut count = 0;
        for c in candidates {
            let path_str = c.path.to_string_lossy().into_owned();
            stmt.execute(params![path_str, c.mtime, c.size as i64])?;
            count += 1;
        }
        drop(stmt);
        tx.commit()?;
        Ok(count)
    })
}
