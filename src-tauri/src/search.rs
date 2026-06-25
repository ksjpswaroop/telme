//! Search pipeline — hybrid semantic + BM25 keyword.
//!
//! Flow:
//! 1. Embed query via Ollama (768-dim). If Ollama is unreachable, fall back
//!    to BM25-only and surface a `degraded` flag.
//! 2. KNN over `chunk_vectors` (sqlite-vec) — top 50.
//! 3. FTS5 over `chunks_fts` — top 50.
//! 4. Hybrid fusion: `score = w * semantic + (1-w) * keyword`.
//! 5. Group by file, keep best chunk per file. Return top-K.

use std::collections::HashMap;

use serde::Serialize;

use crate::db::Db;
use crate::embedder::Embedder;
use crate::error::AppResult;

/// One search hit returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub chunk_id: i64,
    pub file_id: i64,
    pub path: String,
    pub filename: String,
    pub snippet: String,
    pub score: f32,
    pub kind: HitKind,
    pub file_type: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HitKind {
    Semantic,
    Keyword,
    Hybrid,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResults {
    pub hits: Vec<SearchHit>,
    pub total_candidates: usize,
    pub latency_ms: u64,
    pub degraded: bool,
}

pub async fn search(
    db: &Db,
    embedder: &dyn Embedder,
    query: &str,
    top_k: usize,
    semantic_weight: f32,
) -> AppResult<SearchResults> {
    let start = std::time::Instant::now();
    let q = query.trim();
    if q.is_empty() {
        return Ok(SearchResults {
            hits: Vec::new(),
            total_candidates: 0,
            latency_ms: 0,
            degraded: false,
        });
    }

    // 1. Try semantic KNN; degrade gracefully on error.
    let (semantic_hits, degraded) = match embedder.embed(q).await {
        Ok(vec) => match knn_search(db, &vec, q, top_k * 5) {
            Ok(hits) => (hits, false),
            Err(e) => {
                tracing::warn!(error = %e, "semantic search failed; falling back to BM25");
                (Vec::new(), true)
            }
        },
        Err(e) => {
            tracing::warn!(error = %e, "embed query failed; falling back to BM25");
            (Vec::new(), true)
        }
    };

    // 2. BM25
    let keyword_hits = fts_search(db, q, top_k * 5)?;

    // 3. Fuse.
    let fused = fuse(semantic_hits, keyword_hits, semantic_weight);

    // 4. Group by file; keep best chunk.
    let mut by_file: HashMap<i64, SearchHit> = HashMap::new();
    for hit in fused {
        let entry = by_file.entry(hit.file_id).or_insert_with(|| hit.clone());
        if hit.score > entry.score {
            *entry = hit;
        }
    }

    let mut hits: Vec<SearchHit> = by_file.into_values().collect();
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(top_k);

    let total_candidates = hits.len();
    let latency_ms = start.elapsed().as_millis() as u64;
    Ok(SearchResults {
        hits,
        total_candidates,
        latency_ms,
        degraded,
    })
}

// ---- KNN ----

#[derive(Debug, Clone)]
struct ScoredChunk {
    chunk_id: i64,
    file_id: i64,
    path: String,
    filename: String,
    snippet: String,
    raw_score: f32,
    normalized: f32,
    file_type: String,
}

fn knn_search(db: &Db, query_vec: &[f32], query_text: &str, limit: usize) -> AppResult<Vec<ScoredChunk>> {
    use rusqlite::params;
    let json_vec = serde_json::to_string(query_vec)?;
    let rows: Vec<(i64, f64)> = db.with_conn(|conn| -> AppResult<Vec<(i64, f64)>> {
        let mut stmt = match conn.prepare(
            "SELECT v.rowid, v.distance
             FROM chunk_vectors v
             WHERE v.embedding MATCH ?1
             ORDER BY v.distance
             LIMIT ?2",
        ) {
            Ok(s) => s,
            Err(e) => {
                // sqlite-vec disabled — return empty semantic hits.
                tracing::debug!(error = %e, "knn_search skipped (sqlite-vec disabled?)");
                return Ok(Vec::new());
            }
        };
        let rows = stmt
            .query_map(params![json_vec, limit as i64], |r| {
                Ok((r.get::<_, i64>(0)?, r.get::<_, f64>(1)?))
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(rows)
    })?;

    if rows.is_empty() {
        return Ok(Vec::new());
    }

    // Convert L2 distance to a 0..1 similarity.
    let max_d = rows.iter().map(|(_, d)| *d).fold(0.0_f64, f64::max).max(1e-6);

    // Hydrate with chunk metadata.
    let chunk_ids: Vec<i64> = rows.iter().map(|(id, _)| *id).collect();
    let placeholders: String = chunk_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT c.id, c.file_id, c.text, f.path FROM chunks c
         JOIN files f ON f.id = c.file_id
         WHERE c.id IN ({})",
        placeholders
    );
    let meta: HashMap<i64, (i64, String, String)> = db.with_conn(|conn| -> AppResult<HashMap<i64, (i64, String, String)>> {
        let mut hydrate = conn.prepare(&sql)?;
        let params_vec: Vec<&dyn rusqlite::ToSql> = chunk_ids
            .iter()
            .map(|i| i as &dyn rusqlite::ToSql)
            .collect();
        let m = hydrate
            .query_map(params_vec.as_slice(), |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, i64>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                ))
            })?
            .filter_map(|r| r.ok())
            .map(|r| (r.0, (r.1, r.2, r.3)))
            .collect();
        Ok(m)
    })?;

    let mut out = Vec::with_capacity(rows.len());
    for (chunk_id, distance) in rows {
        let (file_id, text, path) = match meta.get(&chunk_id) {
            Some(t) => (t.0, t.1.clone(), t.2.clone()),
            None => continue,
        };
        let similarity = 1.0 - (distance / max_d).min(1.0);
        let snippet = make_snippet(&text, query_text);
        let filename = path.rsplit('/').next().unwrap_or(&path).to_string();
        let file_type = classify_type(&path);
        out.push(ScoredChunk {
            chunk_id,
            file_id,
            path,
            filename,
            snippet,
            raw_score: similarity as f32,
            normalized: similarity as f32,
            file_type,
        });
    }
    Ok(out)
}

// ---- FTS ----

fn fts_search(db: &Db, query: &str, limit: usize) -> AppResult<Vec<ScoredChunk>> {
    use rusqlite::params;
    let escaped = query.replace('"', " ").trim().to_string();
    if escaped.is_empty() {
        return Ok(Vec::new());
    }
    let fts_query = format!("\"{}\"*", escaped);
    let sql = "
        SELECT c.id, c.file_id, c.text, f.path, bm25(chunks_fts) AS score
        FROM chunks_fts
        JOIN chunks c ON c.id = chunks_fts.rowid
        JOIN files f ON f.id = c.file_id
        WHERE chunks_fts MATCH ?1
        ORDER BY score ASC
        LIMIT ?2";
    let rows: Vec<(i64, i64, String, String, f64)> = db.with_conn(|conn| {
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt
            .query_map(params![fts_query, limit as i64], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, i64>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, f64>(4)?,
                ))
            })?
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();
        Ok(rows)
    })?;

    // bm25 returns negative scores; lower (more negative) = better match.
    // Normalize to 0..1 where 1.0 = best.
    let min_score = rows.iter().map(|(_, _, _, _, s)| *s).fold(0.0_f64, f64::min);
    let max_score = rows.iter().map(|(_, _, _, _, s)| *s).fold(0.0_f64, f64::max);
    let span = (max_score - min_score).abs().max(1e-6);

    let mut out = Vec::with_capacity(rows.len());
    for (chunk_id, file_id, text, path, score) in rows {
        let normalized = ((max_score - score) / span).clamp(0.0, 1.0);
        let snippet = make_snippet(&text, query);
        let filename = path.rsplit('/').next().unwrap_or(&path).to_string();
        let file_type = classify_type(&path);
        out.push(ScoredChunk {
            chunk_id,
            file_id,
            path,
            filename,
            snippet,
            raw_score: score as f32,
            normalized: normalized as f32,
            file_type,
        });
    }
    Ok(out)
}

// ---- Fusion ----

fn fuse(
    semantic: Vec<ScoredChunk>,
    keyword: Vec<ScoredChunk>,
    w: f32,
) -> Vec<SearchHit> {
    let kw_w = 1.0 - w;
    let mut by_chunk: HashMap<i64, (ScoredChunk, f32, HitKind)> = HashMap::new();

    for c in semantic {
        let entry = by_chunk.entry(c.chunk_id);
        let score = c.normalized;
        match entry {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let e = o.get_mut();
                e.1 = w * score + kw_w * e.1;
                e.2 = HitKind::Hybrid;
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert((c, w * score, HitKind::Semantic));
            }
        }
    }
    for c in keyword {
        let entry = by_chunk.entry(c.chunk_id);
        let score = c.normalized;
        match entry {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let e = o.get_mut();
                e.1 = w * e.1 + kw_w * score;
                e.2 = HitKind::Hybrid;
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert((c, kw_w * score, HitKind::Keyword));
            }
        }
    }

    by_chunk
        .into_values()
        .map(|(c, score, kind)| SearchHit {
            chunk_id: c.chunk_id,
            file_id: c.file_id,
            path: c.path,
            filename: c.filename,
            snippet: c.snippet,
            score,
            kind,
            file_type: c.file_type,
        })
        .collect()
}

// ---- Helpers ----

fn make_snippet(text: &str, hint: &str) -> String {
    const WINDOW: usize = 200;
    if text.len() <= WINDOW {
        return text.to_string();
    }
    let lower = text.to_lowercase();
    let needle = hint.to_lowercase();
    let pos = lower.find(&needle).unwrap_or(0);
    let start = pos.saturating_sub(WINDOW / 2);
    let end = (start + WINDOW).min(text.len());
    let mut s = text[start..end].to_string();
    if start > 0 {
        s = format!("…{s}");
    }
    if end < text.len() {
        s.push('…');
    }
    s
}

fn classify_type(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "md" | "markdown" | "txt" | "rst" => "doc".into(),
        "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "rb" | "go" | "java" | "kt" | "swift"
        | "c" | "cpp" | "h" | "hpp" | "cs" => "code".into(),
        "pdf" => "pdf".into(),
        _ => "other".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuse_semantic_only() {
        let c = ScoredChunk {
            chunk_id: 1,
            file_id: 10,
            path: "/a.md".into(),
            filename: "a.md".into(),
            snippet: "s".into(),
            raw_score: 0.0,
            normalized: 0.9,
            file_type: "doc".into(),
        };
        let fused = fuse(vec![c], Vec::new(), 0.7);
        assert_eq!(fused.len(), 1);
        assert_eq!(fused[0].kind, HitKind::Semantic);
    }

    #[test]
    fn fuse_keyword_only() {
        let c = ScoredChunk {
            chunk_id: 1,
            file_id: 10,
            path: "/a.md".into(),
            filename: "a.md".into(),
            snippet: "s".into(),
            raw_score: 0.0,
            normalized: 0.8,
            file_type: "doc".into(),
        };
        let fused = fuse(Vec::new(), vec![c], 0.7);
        assert_eq!(fused.len(), 1);
        assert_eq!(fused[0].kind, HitKind::Keyword);
    }

    #[test]
    fn fuse_marks_hybrid_when_both_match() {
        let sa = ScoredChunk {
            chunk_id: 1,
            file_id: 10,
            path: "/a.md".into(),
            filename: "a.md".into(),
            snippet: "s".into(),
            raw_score: 0.0,
            normalized: 0.8,
            file_type: "doc".into(),
        };
        let kb = ScoredChunk {
            chunk_id: 1,
            file_id: 10,
            path: "/a.md".into(),
            filename: "a.md".into(),
            snippet: "s".into(),
            raw_score: 0.0,
            normalized: 0.6,
            file_type: "doc".into(),
        };
        let fused = fuse(vec![sa], vec![kb], 0.7);
        assert_eq!(fused[0].kind, HitKind::Hybrid);
    }

    #[test]
    fn snippet_uses_hint_position() {
        let text = "a".repeat(150) + "NEEDLE" + &"b".repeat(150);
        let s = make_snippet(&text, "NEEDLE");
        assert!(s.contains("NEEDLE"));
        assert!(s.starts_with('…') || s.len() <= 220);
    }
}
