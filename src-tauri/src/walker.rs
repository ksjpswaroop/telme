//! Recursive directory walker with skip-pattern filtering.
//!
//! Phase 2 walks indexed folders and yields `Candidate { path, mtime, size }`
//! for every file we *might* extract. The actual extraction happens downstream
//! in `extractor::extract`.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use ignore::WalkBuilder;

use crate::error::AppResult;

const MAX_DEPTH: usize = 32;

/// A single file candidate emitted by the walker.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub path: PathBuf,
    pub mtime: i64,
    pub size: u64,
}

/// Walk `root` recursively and yield every file that passes our filters.
///
/// Filters:
/// - Skip hidden files and dirs (`.git`, `.DS_Store`, anything starting with `.`)
/// - Skip common heavy/binary dirs (`node_modules`, `target`, `dist`, `build`,
///   `.venv`, `__pycache__`, `vendor`, `.gradle`)
/// - Skip symlinks (avoid cycles)
/// - Skip files larger than [`crate::extractor::MAX_FILE_BYTES`]
pub fn walk(root: &Path) -> AppResult<Vec<Candidate>> {
    let mut out = Vec::new();

    let walker = WalkBuilder::new(root)
        .max_depth(Some(MAX_DEPTH))
        .follow_links(false)
        .require_git(false)
        .hidden(true) // skip dotfiles / dotdirs
        .filter_entry(|e| !is_ignored_dir(e.depth(), e.path(), e.file_type()))
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = meta.len();
        if size > crate::extractor::MAX_FILE_BYTES {
            continue;
        }

        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        out.push(Candidate {
            path: path.to_path_buf(),
            mtime,
            size,
        });
    }

    Ok(out)
}

/// Names of directories we never want to descend into.
const IGNORED_DIRS: &[&str] = &[
    "node_modules",
    "target",
    "dist",
    "build",
    ".venv",
    "venv",
    "__pycache__",
    "vendor",
    ".gradle",
    ".idea",
    ".vscode",
    ".next",
    ".nuxt",
    "DerivedData",
];

fn is_ignored_dir(depth: usize, path: &Path, ft: Option<std::fs::FileType>) -> bool {
    if depth == 0 {
        return false;
    }
    if !matches!(ft, Some(t) if t.is_dir()) {
        return false;
    }
    let name = match path.file_name().and_then(|s| s.to_str()) {
        Some(n) => n,
        None => return false,
    };
    if name.starts_with('.') {
        return true;
    }
    IGNORED_DIRS.iter().any(|d| *d == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn walks_simple_tree() {
        let dir = std::env::temp_dir().join("telme_test_walker");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("a.txt"), "a").unwrap();
        fs::write(dir.join("sub/b.md"), "b").unwrap();

        let files = walk(&dir).unwrap();
        let names: Vec<String> = files
            .iter()
            .map(|c| c.path.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert!(names.contains(&"a.txt".to_string()));
        assert!(names.contains(&"b.md".to_string()));
        assert_eq!(files.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn skips_node_modules_and_hidden() {
        let dir = std::env::temp_dir().join("telme_test_walker_skip");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("node_modules/pkg")).unwrap();
        fs::create_dir_all(dir.join(".git/objects")).unwrap();
        fs::write(dir.join("node_modules/pkg/i.js"), "i").unwrap();
        fs::write(dir.join("keep.txt"), "k").unwrap();

        let files = walk(&dir).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].path.file_name().unwrap().to_string_lossy(),
            "keep.txt"
        );

        let _ = fs::remove_dir_all(&dir);
    }
}
