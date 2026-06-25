//! Text chunking — split text into overlapping windows for embedding/indexing.
//!
//! Phase 2 default: 512 tokens per chunk, 50-token overlap. Matches ARCHITECTURE.md §6.1.

use text_splitter::{ChunkConfig, TextSplitter};

const CHUNK_TOKENS: usize = 512;
const OVERLAP_TOKENS: usize = 50;

/// One chunk ready to be embedded and stored.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub ordinal: u32,
    pub text: String,
    pub token_count: usize,
}

/// Split a UTF-8 text blob into overlapping chunks.
///
/// Returns an empty Vec for empty/whitespace-only input.
///
/// Tokenization is model-agnostic — `text-splitter` uses the `tokenizers`-style
/// heuristic (`text_splitter::ChunkCapacity::Tokens`) which is close enough to
/// most embedding model tokenizers (e.g. nomic, mxbai, all-MiniLM) for chunking.
pub fn chunk_text(text: &str) -> Vec<Chunk> {
    if text.trim().is_empty() {
        return Vec::new();
    }

    let cfg = ChunkConfig::new(CHUNK_TOKENS).with_overlap(OVERLAP_TOKENS).expect("valid config");
    let splitter = TextSplitter::new(cfg);

    splitter
        .chunks(text)
        .enumerate()
        .map(|(i, s)| Chunk {
            ordinal: i as u32,
            text: s.to_string(),
            token_count: approx_token_count(s),
        })
        .collect()
}

/// Rough token count — used only for stats; doesn't need to be precise.
fn approx_token_count(s: &str) -> usize {
    // 1 token ≈ 4 chars of English text. Good enough for chunk stats.
    (s.chars().count() + 3) / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_yields_no_chunks() {
        assert!(chunk_text("").is_empty());
        assert!(chunk_text("   \n\t  ").is_empty());
    }

    #[test]
    fn short_text_yields_one_chunk() {
        let chunks = chunk_text("hello world this is short text");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].ordinal, 0);
    }

    #[test]
    fn long_text_yields_multiple_chunks() {
        let long = "lorem ipsum ".repeat(2_000);
        let chunks = chunk_text(&long);
        assert!(chunks.len() > 1, "expected multiple chunks, got {}", chunks.len());
        for c in &chunks {
            assert!(!c.text.is_empty());
            assert!(c.token_count > 0);
        }
    }

    #[test]
    fn ordinals_are_sequential() {
        let long = "the quick brown fox jumps over the lazy dog. ".repeat(1_000);
        let chunks = chunk_text(&long);
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.ordinal, i as u32);
        }
    }
}
