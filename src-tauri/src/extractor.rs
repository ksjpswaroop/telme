//! Plain-text extractor + dispatcher.
//!
//! Phase 2 ships plain-text + code files only. PDF / DOCX / HTML extractors
//! arrive in later phases.

use std::path::Path;

use crate::error::AppResult;

/// Maximum file size we will even attempt to extract (50 MB).
pub const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;

/// File types we know how to extract in Phase 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Text,
    Code,
    Markdown,
}

impl FileKind {
    pub fn classify(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_string_lossy().to_lowercase();
        Some(match ext.as_str() {
            "md" | "markdown" => FileKind::Markdown,
            "txt" | "rst" | "log" => FileKind::Text,
            // Code + config + data — treat as plain text
            "rs" | "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs"
            | "py" | "rb" | "go" | "java" | "kt" | "swift"
            | "c" | "h" | "cpp" | "hpp" | "cc" | "cs" | "m" | "mm"
            | "json" | "yaml" | "yml" | "toml" | "xml" | "ini" | "env"
            | "sh" | "bash" | "zsh" | "fish"
            | "html" | "css" | "scss" | "sass" | "less"
            | "sql" | "graphql" | "proto" => FileKind::Code,
            _ => return None,
        })
    }

    pub fn is_text_like(&self) -> bool {
        matches!(self, FileKind::Text | FileKind::Code | FileKind::Markdown)
    }
}

/// Extract text content from a file at `path`.
///
/// Returns `Ok(None)` if the file is a type we don't handle, the file is too
/// large, or the file no longer exists. Errors only for unexpected I/O failures.
pub fn extract(path: &Path) -> AppResult<Option<String>> {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };

    if meta.len() > MAX_FILE_BYTES {
        return Ok(None);
    }

    let kind = match FileKind::classify(path) {
        Some(k) => k,
        None => return Ok(None),
    };

    if !kind.is_text_like() {
        return Ok(None);
    }

    let bytes = std::fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    Ok(Some(text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn classifies_known_extensions() {
        assert_eq!(FileKind::classify(Path::new("a.md")), Some(FileKind::Markdown));
        assert_eq!(FileKind::classify(Path::new("a.txt")), Some(FileKind::Text));
        assert_eq!(FileKind::classify(Path::new("a.rs")), Some(FileKind::Code));
        assert_eq!(FileKind::classify(Path::new("a.py")), Some(FileKind::Code));
    }

    #[test]
    fn unknown_extension_returns_none() {
        assert_eq!(FileKind::classify(Path::new("a.pdf")), None);
        assert_eq!(FileKind::classify(Path::new("a.docx")), None);
        assert_eq!(FileKind::classify(Path::new("a.png")), None);
    }

    #[test]
    fn extracts_small_text_file() {
        let dir = std::env::temp_dir().join("telme_test_extract");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("sample.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "hello world").unwrap();
        drop(f);

        let got = extract(&path).unwrap();
        assert_eq!(got.as_deref(), Some("hello world\n"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn extract_returns_none_for_missing() {
        let got = extract(Path::new("/no/such/file/abc.txt")).unwrap();
        assert_eq!(got, None);
    }

    #[test]
    fn extract_returns_none_for_pdf() {
        let dir = std::env::temp_dir().join("telme_test_pdf");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.4 fake").unwrap();
        assert_eq!(extract(&path).unwrap(), None);
        std::fs::remove_dir_all(&dir).ok();
    }
}
