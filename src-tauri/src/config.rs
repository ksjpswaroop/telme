//! App-level config (settings + dynamic state).
//!
//! Persisted in the `config` table as JSON values under string keys.

use rusqlite::params;

use crate::db::Db;
use crate::error::AppResult;

const KEY_EMBEDDING_MODEL: &str = "embedding_model";
const KEY_SEMANTIC_WEIGHT: &str = "semantic_weight";
const KEY_TOP_K: &str = "top_k";
const KEY_OLLAMA_URL: &str = "ollama_url";

/// Defaults; overridden on first run if absent.
pub const DEFAULT_MODEL: &str = "nomic-embed-text";
pub const DEFAULT_SEMANTIC_WEIGHT: f32 = 0.7;
pub const DEFAULT_TOP_K: usize = 10;
pub const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub model: String,
    pub semantic_weight: f32,
    pub top_k: usize,
    pub ollama_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: DEFAULT_MODEL.to_string(),
            semantic_weight: DEFAULT_SEMANTIC_WEIGHT,
            top_k: DEFAULT_TOP_K,
            ollama_url: DEFAULT_OLLAMA_URL.to_string(),
        }
    }
}

pub fn load(db: &Db) -> AppResult<AppConfig> {
    db.with_conn(|conn| {
        let mut cfg = AppConfig::default();

        if let Some(v) = read_string(conn, KEY_EMBEDDING_MODEL)? {
            cfg.model = v;
        }
        if let Some(v) = read_string(conn, KEY_SEMANTIC_WEIGHT)? {
            if let Ok(f) = v.parse::<f32>() {
                cfg.semantic_weight = f;
            }
        }
        if let Some(v) = read_string(conn, KEY_TOP_K)? {
            if let Ok(n) = v.parse::<usize>() {
                cfg.top_k = n;
            }
        }
        if let Some(v) = read_string(conn, KEY_OLLAMA_URL)? {
            cfg.ollama_url = v;
        }

        Ok(cfg)
    })
}

pub fn save(db: &Db, cfg: &AppConfig) -> AppResult<()> {
    db.with_conn(|conn| {
        write_string(conn, KEY_EMBEDDING_MODEL, &cfg.model)?;
        write_string(
            conn,
            KEY_SEMANTIC_WEIGHT,
            &cfg.semantic_weight.to_string(),
        )?;
        write_string(conn, KEY_TOP_K, &cfg.top_k.to_string())?;
        write_string(conn, KEY_OLLAMA_URL, &cfg.ollama_url)?;
        Ok(())
    })
}

fn read_string(conn: &rusqlite::Connection, key: &str) -> AppResult<Option<String>> {
    let v: Option<String> = conn
        .query_row(
            "SELECT value FROM config WHERE key = ?1",
            params![key],
            |r| r.get(0),
        )
        .ok();
    Ok(v)
}

fn write_string(conn: &rusqlite::Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db(tag: &str) -> Db {
        let dir = std::env::temp_dir().join(format!(
            "telme_test_cfg_{}_{:?}",
            tag,
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Db::open(&dir.join("test.db")).unwrap()
    }

    #[test]
    fn defaults_when_empty() {
        let db = temp_db("defaults");
        let cfg = load(&db).unwrap();
        assert_eq!(cfg.model, DEFAULT_MODEL);
        assert_eq!(cfg.top_k, DEFAULT_TOP_K);
        assert!((cfg.semantic_weight - DEFAULT_SEMANTIC_WEIGHT).abs() < 1e-6);
        assert_eq!(cfg.ollama_url, DEFAULT_OLLAMA_URL);
    }

    #[test]
    fn round_trip() {
        let db = temp_db("round");
        let cfg = AppConfig {
            model: "mxbai-embed-large".into(),
            semantic_weight: 0.5,
            top_k: 25,
            ollama_url: "http://localhost:11434".into(),
        };
        save(&db, &cfg).unwrap();
        let loaded = load(&db).unwrap();
        assert_eq!(loaded.model, "mxbai-embed-large");
        assert_eq!(loaded.top_k, 25);
    }
}
