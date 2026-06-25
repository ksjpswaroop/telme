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
//!
//! Schema versions:
//! - v1: initial — files, chunks, chunks_fts, config
//! - v2: adds `chunk_vectors` (sqlite-vec) for Phase 3 embeddings

use rusqlite::Connection;

use crate::embedder::NOMIC_DIM;
use crate::error::AppResult;

/// Current schema version. Bump when adding migrations.
pub const SCHEMA_VERSION: i32 = 2;

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

    // sqlite-vec vector index. The `vec0` virtual table module ships in a
    // SQLite extension; loading it requires either `sqlite-vec`'s bundled
    // SQLite build OR a manual `sqlite3_vec_init` call against the extension
    // shared library. v0.1 of `sqlite-vec` doesn't ship a stable loader, so
    // we attempt creation and gracefully degrade to BM25-only if the extension
    // is missing.
    let vec_dim = NOMIC_DIM as i64;
    let vec_ok = conn
        .execute_batch(&format!(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS chunk_vectors USING vec0(
                embedding float[{vec_dim}]
            );
            "#,
        ))
        .is_ok();
    if !vec_ok {
        tracing::warn!(
            "sqlite-vec not available; chunk_vectors disabled (BM25-only search)"
        );
    }

    // Record current version if absent, else migrate forward.
    let current: Option<i32> = conn
        .query_row(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .ok();
    match current {
        None => {
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?1)",
                [SCHEMA_VERSION],
            )?;
            tracing::info!(version = SCHEMA_VERSION, "schema initialized");
        }
        Some(v) if v < SCHEMA_VERSION => {
            // v1 -> v2: chunk_vectors was added above (CREATE IF NOT EXISTS).
            // v2 is additive; nothing to copy.
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?1)",
                [SCHEMA_VERSION],
            )?;
            tracing::info!(from = v, to = SCHEMA_VERSION, "schema migrated");
        }
        Some(v) if v > SCHEMA_VERSION => {
            tracing::warn!(
                on_disk = v,
                app = SCHEMA_VERSION,
                "on-disk schema newer than app; refusing to downgrade"
            );
        }
        _ => {} // current
    }

    Ok(())
}
