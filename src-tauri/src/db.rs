//! SQLite connection wrapper + app data dir resolution.

use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use parking_lot::Mutex;
use rusqlite::Connection;

use crate::error::AppResult;
use crate::schema;

/// Thread-safe handle to the app's SQLite database.
///
/// A single `Connection` is held behind a mutex; rusqlite's `Connection`
/// is `!Sync` and not safe to share without synchronization. WAL mode
/// allows readers to run concurrently with the (single) writer.
pub struct Db {
    conn: Mutex<Connection>,
    path: PathBuf,
}

impl Db {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        schema::run_migrations(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            path: path.to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Run a closure with exclusive access to the connection.
    pub fn with_conn<R>(&self, f: impl FnOnce(&mut Connection) -> AppResult<R>) -> AppResult<R> {
        let mut guard = self.conn.lock();
        f(&mut guard)
    }
}

/// Resolve the app data directory.
///
/// On macOS: `~/Library/Application Support/com.telme.desktop/`
/// On Windows: `%APPDATA%/com.telme.desktop/`
/// On Linux: `~/.local/share/com.telme.desktop/`
pub fn app_data_dir() -> AppResult<PathBuf> {
    let dirs = ProjectDirs::from("com", "telme", "desktop")
        .ok_or_else(|| crate::error::TelmeError::Other("no home directory".into()))?;
    Ok(dirs.data_dir().to_path_buf())
}

/// Default database path: `<app_data_dir>/index.db`.
pub fn default_db_path() -> AppResult<PathBuf> {
    Ok(app_data_dir()?.join("index.db"))
}
