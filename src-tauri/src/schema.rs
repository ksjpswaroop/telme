//! SQLite schema + migrations.
//!
//! Run [`run_migrations`] on every app start before opening the DB to other code.
//!
//! Schema (matches ARCHITECTURE.md §2.3):
//!
//! - `files`: registered file paths + status
//! - `chunks`: text chunks with ordinals (FK files.id ON DELETE CASCADE)
//! - `chunks_fts`: FTS5 mirror for BM25 keyword search
//! - `chunk_vectors`: vector index (added in Phase 3 once sqlite-vec is wired)
//! - `config`: app-level key/value config
//! - `schema_version`: tracks applied migrations

use rusqlite::Connection;

use crate::error::AppResult;

/// Current schema version. Bump when adding migrations.
pub const SCHEMA_VERSION: i32 = 1;

pub fn run_migrations(conn: &Connection) -> AppResult<()> {
    // Pragmas first — idempotent, safe to set on every startup.
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS files (
            id          INTEGER PRIMARY KEY,
            path        TEXT UNIQUE NOT NULL,
            mtime       INTEGER NOT NULL,
            size        INTEGER NOT NULL,
            content_hash TEXT,
            indexed_at  INTEGER,
            status      TEXT NOT NULL DEFAULT 'pending'
        );
        CREATE INDEX IF NOT EXISTS idx_files_path   ON files(path);
        CREATE INDEX IF NOT EXISTS idx_files_status ON files(status);

        CREATE TABLE IF NOT EXISTS chunks (
            id          INTEGER PRIMARY KEY,
            file_id     INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
            ordinal     INTEGER NOT NULL,
            text        TEXT NOT NULL,
            token_count INTEGER
        );
        CREATE INDEX IF NOT EXISTS idx_chunks_file ON chunks(file_id);

        CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
            text,
            content='chunks',
            content_rowid='id',
            tokenize='porter unicode61'
        );

        CREATE TABLE IF NOT EXISTS config (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#,
    )?;

    // Triggers to keep FTS in sync with chunks. Created once.
    conn.execute_batch(
        r#"
        CREATE TRIGGER IF NOT EXISTS chunks_ai AFTER INSERT ON chunks BEGIN
            INSERT INTO chunks_fts(rowid, text) VALUES (new.id, new.text);
        END;
        CREATE TRIGGER IF NOT EXISTS chunks_ad AFTER DELETE ON chunks BEGIN
            INSERT INTO chunks_fts(chunks_fts, rowid, text) VALUES ('delete', old.id, old.text);
        END;
        CREATE TRIGGER IF NOT EXISTS chunks_au AFTER UPDATE ON chunks BEGIN
            INSERT INTO chunks_fts(chunks_fts, rowid, text) VALUES ('delete', old.id, old.text);
            INSERT INTO chunks_fts(rowid, text) VALUES (new.id, new.text);
        END;
        "#,
    )?;

    // Record current version if absent.
    let current: Option<i32> = conn
        .query_row(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .ok();
    if current.is_none() {
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?1)",
            [SCHEMA_VERSION],
        )?;
        tracing::info!(version = SCHEMA_VERSION, "schema initialized");
    } else if current != Some(SCHEMA_VERSION) {
        // Future migrations would branch here.
        tracing::warn!(
            current = current.unwrap_or(0),
            target = SCHEMA_VERSION,
            "schema version mismatch; migrations not yet implemented"
        );
    }

    Ok(())
}
