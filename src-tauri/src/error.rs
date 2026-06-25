//! Unified error type for Telme backend.

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum TelmeError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid path: {0}")]
    InvalidPath(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("duplicate: {0}")]
    Duplicate(String),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

impl Serialize for TelmeError {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = std::result::Result<T, TelmeError>;
