//! Async file utilities for the MCP server.
//!
//! Provides unified file discovery and reading operations used across
//! all tool implementations.

use async_walkdir::WalkDir;
use futures::StreamExt;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::error::{Error, Result};

/// Options for finding files by ID.
#[derive(Debug, Clone, Default)]
pub struct FindOptions {
    /// File extension to match (without dot), e.g., "md"
    pub extension: Option<&'static str>,
    /// Maximum directory depth to search (None = unlimited)
    pub max_depth: Option<usize>,
    /// Additional filename patterns to try before recursive search
    /// e.g., ["{id}.md", "{id}/README.md", "{id}/index.md"]
    pub patterns: Vec<String>,
}

impl FindOptions {
    /// Create options for finding markdown files.
    pub fn markdown() -> Self {
        Self {
            extension: Some("md"),
            max_depth: None,
            patterns: vec![],
        }
    }

    /// Add patterns to try before recursive search.
    /// Use `{id}` as placeholder for the file ID.
    pub fn with_patterns(mut self, patterns: Vec<&str>) -> Self {
        self.patterns = patterns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set maximum search depth.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

/// Find a file by ID within a base directory.
///
/// Search strategy:
/// 1. Try each pattern in `options.patterns` (with `{id}` replaced)
/// 2. Fall back to recursive search matching file stem
///
/// # Example
///
/// ```rust
/// let path = find_file_by_id(
///     &concepts_dir,
///     "pitch-class",
///     FindOptions::markdown()
///         .with_patterns(vec!["{id}.md", "{id}/README.md"])
/// ).await?;
/// ```
pub async fn find_file_by_id(base_path: &Path, id: &str, options: FindOptions) -> Result<PathBuf> {
    // Phase 1: Try explicit patterns
    for pattern in &options.patterns {
        let relative = pattern.replace("{id}", id);
        let path = base_path.join(&relative);
        if fs::try_exists(&path).await.unwrap_or(false) {
            return Ok(path);
        }
    }

    // Phase 2: Try simple {id}.md if extension specified
    if let Some(ext) = options.extension {
        let simple_path = base_path.join(format!("{}.{}", id, ext));
        if fs::try_exists(&simple_path).await.unwrap_or(false) {
            return Ok(simple_path);
        }
    }

    // Phase 3: Recursive search by file stem
    let mut walker = WalkDir::new(base_path);

    while let Some(entry_result) = walker.next().await {
        let entry = entry_result.map_err(Error::io)?;
        let path = entry.path();

        // Check depth limit
        if let Some(max_depth) = options.max_depth {
            let depth = path
                .strip_prefix(base_path)
                .map(|p| p.components().count())
                .unwrap_or(0);
            if depth > max_depth {
                continue;
            }
        }

        // Check extension if specified
        if let Some(ext) = options.extension {
            if path.extension().and_then(|e| e.to_str()) != Some(ext) {
                continue;
            }
        }

        // Match by file stem
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            if stem == id
                || stem.starts_with(&format!("{}-", id))
                || stem.starts_with(&format!("{}_", id))
            {
                return Ok(path.to_path_buf());
            }
        }
    }

    Err(Error::not_found_msg(format!(
        "File with id '{}' not found in {}",
        id,
        base_path.display()
    )))
}

/// Information about a discovered file.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub stem: String,
    #[allow(dead_code)]
    pub relative_path: PathBuf,
}

/// Find all files matching criteria in a directory.
///
/// # Example
///
/// ```rust
/// let files = find_all_files(
///     &sources_dir,
///     FindOptions::markdown().with_max_depth(2)
/// ).await?;
/// ```
pub async fn find_all_files(base_path: &Path, options: FindOptions) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();
    let mut walker = WalkDir::new(base_path);

    while let Some(entry_result) = walker.next().await {
        let entry = entry_result.map_err(Error::io)?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Check depth limit
        if let Some(max_depth) = options.max_depth {
            let depth = path
                .strip_prefix(base_path)
                .map(|p| p.components().count())
                .unwrap_or(0);
            if depth > max_depth {
                continue;
            }
        }

        // Check extension if specified
        if let Some(ext) = options.extension {
            if path.extension().and_then(|e| e.to_str()) != Some(ext) {
                continue;
            }
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let relative_path = path.strip_prefix(base_path).unwrap_or(&path).to_path_buf();

        files.push(FileInfo {
            path: path.to_path_buf(),
            stem,
            relative_path,
        });
    }

    Ok(files)
}

/// List immediate subdirectories of a path.
pub async fn list_subdirectories(base_path: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    let mut entries = fs::read_dir(base_path).await.map_err(Error::io)?;

    while let Some(entry) = entries.next_entry().await.map_err(Error::io)? {
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

/// Count files matching criteria in a directory.
pub async fn count_files(base_path: &Path, options: FindOptions) -> Result<usize> {
    let files = find_all_files(base_path, options).await?;
    Ok(files.len())
}

/// Read a file's contents as a string.
pub async fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .await
        .map_err(|e| Error::io_with_path(e, path))
}

/// Check if a path exists.
pub async fn exists(path: &Path) -> bool {
    fs::try_exists(path).await.unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_find_file_by_id_exact_match() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("pitch-class.md");
        fs::write(&file_path, "# Pitch Class").await.unwrap();

        let found = find_file_by_id(temp.path(), "pitch-class", FindOptions::markdown())
            .await
            .unwrap();

        assert_eq!(found, file_path);
    }

    #[tokio::test]
    async fn test_find_file_by_id_prefix_match() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("01-16-intervals.md");
        fs::write(&file_path, "# Intervals").await.unwrap();

        let found = find_file_by_id(temp.path(), "01-16", FindOptions::markdown())
            .await
            .unwrap();

        assert_eq!(found, file_path);
    }

    #[tokio::test]
    async fn test_find_all_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("one.md"), "# One")
            .await
            .unwrap();
        fs::write(temp.path().join("two.md"), "# Two")
            .await
            .unwrap();
        fs::write(temp.path().join("skip.txt"), "skip")
            .await
            .unwrap();

        let files = find_all_files(temp.path(), FindOptions::markdown())
            .await
            .unwrap();

        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_list_subdirectories() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join("dir1")).await.unwrap();
        fs::create_dir(temp.path().join("dir2")).await.unwrap();
        fs::write(temp.path().join("file.txt"), "content")
            .await
            .unwrap();

        let dirs = list_subdirectories(temp.path()).await.unwrap();

        assert_eq!(dirs.len(), 2);
    }

    #[tokio::test]
    async fn test_count_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("one.md"), "# One")
            .await
            .unwrap();
        fs::write(temp.path().join("two.md"), "# Two")
            .await
            .unwrap();

        let count = count_files(temp.path(), FindOptions::markdown())
            .await
            .unwrap();

        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_read_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.md");
        let content = "# Test Content";
        fs::write(&file_path, content).await.unwrap();

        let read_content = read_file(&file_path).await.unwrap();

        assert_eq!(read_content, content);
    }

    #[tokio::test]
    async fn test_exists() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("exists.md");
        fs::write(&file_path, "content").await.unwrap();

        assert!(exists(&file_path).await);
        assert!(!exists(&temp.path().join("nonexistent.md")).await);
    }
}
