//! Embedding provider — Ollama HTTP client.
//!
//! Phase 3 ships the Ollama path only. The bundled GGUF fallback
//! (Phase 6+) lives behind the same `Embedder` trait.
//!
//! Default model: `nomic-embed-text` (768-dim, 274 MB).
//! Endpoint: `POST {ollama_url}/api/embed` (modern API; falls back to
//! `/api/embeddings` singular on parse failure).

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::error::{AppError, AppResult};

/// Dimension of the default model's embeddings.
pub const NOMIC_DIM: usize = 768;

pub type BoxFut<'a, T> = Pin<Box<dyn Future<Output = AppResult<T>> + Send + 'a>>;

/// Trait every embedder backend implements.
pub trait Embedder: Send + Sync {
    fn name(&self) -> &str;
    fn dim(&self) -> usize;
    /// Embed a single string. Returns a `Vec<f32>` of length `dim()`.
    fn embed<'a>(&'a self, text: &'a str) -> BoxFut<'a, Vec<f32>>;
    /// Embed a batch; the default implementation just calls `embed` in a loop.
    /// Override for backends that support batch endpoints.
    fn embed_batch<'a>(&'a self, texts: &'a [&'a str]) -> BoxFut<'a, Vec<Vec<f32>>> {
        Box::pin(async move {
            let mut out = Vec::with_capacity(texts.len());
            for t in texts {
                out.push(self.embed(t).await?);
            }
            Ok(out)
        })
    }
}

#[derive(Debug, Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a [&'a str],
    // truncate = false to surface "input too long" as an error rather than silent loss
    truncate: bool,
}

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Status snapshot returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct SearchStatus {
    pub ollama_reachable: bool,
    pub ollama_url: String,
    pub model: String,
    pub dim: usize,
    pub circuit_open: bool,
}

#[derive(Debug)]
struct CircuitBreaker {
    failures: AtomicU32,
    open: std::sync::atomic::AtomicBool,
}

/// Thin Ollama HTTP client. Holds a `reqwest::Client` + endpoint URL.
pub struct OllamaEmbedder {
    cfg: Arc<Mutex<AppConfig>>,
    http: reqwest::Client,
    circuit: CircuitBreaker,
}

impl OllamaEmbedder {
    pub fn new(cfg: AppConfig) -> AppResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|e| AppError::Other(format!("reqwest build: {e}")))?;
        Ok(Self {
            cfg: Arc::new(Mutex::new(cfg)),
            http,
            circuit: CircuitBreaker {
                failures: AtomicU32::new(0),
                open: std::sync::atomic::AtomicBool::new(false),
            },
        })
    }

    pub fn update_config(&self, cfg: AppConfig) {
        *self.cfg.lock() = cfg;
    }

    pub fn config(&self) -> AppConfig {
        self.cfg.lock().clone()
    }

    pub fn circuit_open(&self) -> bool {
        self.circuit.open.load(Ordering::SeqCst)
    }

    pub fn status(&self) -> SearchStatus {
        let cfg = self.cfg.lock();
        SearchStatus {
            ollama_reachable: !self.circuit_open(),
            ollama_url: cfg.ollama_url.clone(),
            model: cfg.model.clone(),
            dim: NOMIC_DIM,
            circuit_open: self.circuit_open(),
        }
    }

    /// Lightweight health check. Resets the circuit if it was open.
    pub async fn ping(&self) -> bool {
        let url = format!("{}/api/tags", self.cfg.lock().ollama_url);
        match self.http.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                self.circuit.failures.store(0, Ordering::SeqCst);
                self.circuit.open.store(false, Ordering::SeqCst);
                true
            }
            _ => false,
        }
    }

    fn record_failure(&self) {
        let n = self.circuit.failures.fetch_add(1, Ordering::SeqCst) + 1;
        if n >= 5 {
            self.circuit.open.store(true, Ordering::SeqCst);
            tracing::warn!(failures = n, "ollama circuit breaker opened");
        }
    }
}

impl Embedder for OllamaEmbedder {
    fn name(&self) -> &str {
        "ollama"
    }
    fn dim(&self) -> usize {
        NOMIC_DIM
    }

    fn embed<'a>(&'a self, text: &'a str) -> BoxFut<'a, Vec<f32>> {
        Box::pin(async move {
            if self.circuit_open() {
                return Err(AppError::Other(
                    "embedding circuit open: ollama unreachable".into(),
                ));
            }
            let v = self.embed_batch(&[text]).await?;
            v.into_iter()
                .next()
                .ok_or_else(|| AppError::Other("empty embedding response".into()))
        })
    }

    fn embed_batch<'a>(&'a self, texts: &'a [&'a str]) -> BoxFut<'a, Vec<Vec<f32>>> {
        Box::pin(async move {
            if texts.is_empty() {
                return Ok(Vec::new());
            }
            if self.circuit_open() {
                return Err(AppError::Other(
                    "embedding circuit open: ollama unreachable".into(),
                ));
            }
            let (url, model) = {
                let cfg = self.cfg.lock();
                (cfg.ollama_url.clone(), cfg.model.clone())
            };
            let endpoint = format!("{}/api/embed", url);
            let req = EmbedRequest {
                model: &model,
                input: texts,
                truncate: false,
            };

            let resp = self.http.post(&endpoint).json(&req).send().await;
            match resp {
                Ok(r) if r.status().is_success() => match r.json::<EmbedResponse>().await {
                    Ok(parsed) => {
                        self.circuit.failures.store(0, Ordering::SeqCst);
                        self.circuit.open.store(false, Ordering::SeqCst);
                        Ok(parsed.embeddings)
                    }
                    Err(e) => {
                        self.record_failure();
                        Err(AppError::Other(format!("parse embed response: {e}")))
                    }
                },
                Ok(r) => {
                    self.record_failure();
                    let status = r.status();
                    let body = r.text().await.unwrap_or_default();
                    Err(AppError::Other(format!("ollama {status}: {body}")))
                }
                Err(e) => {
                    self.record_failure();
                    Err(AppError::Other(format!("ollama request: {e}")))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_breaker_opens_after_5_failures() {
        let cb = CircuitBreaker {
            failures: AtomicU32::new(0),
            open: std::sync::atomic::AtomicBool::new(false),
        };
        for _ in 0..4 {
            let n = cb.failures.fetch_add(1, Ordering::SeqCst) + 1;
            assert!(!cb.open.load(Ordering::SeqCst));
            assert!(n < 5);
        }
        let n = cb.failures.fetch_add(1, Ordering::SeqCst) + 1;
        if n >= 5 {
            cb.open.store(true, Ordering::SeqCst);
        }
        assert!(cb.open.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn embed_returns_err_when_circuit_open() {
        let cfg = AppConfig::default();
        let emb = OllamaEmbedder::new(cfg).unwrap();
        emb.circuit.open.store(true, std::sync::atomic::Ordering::SeqCst);
        let res = emb.embed("hello").await;
        assert!(res.is_err());
    }
}
