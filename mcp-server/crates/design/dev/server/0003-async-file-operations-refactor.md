# Async File Operations Refactor

## Overview

Refactor the MCP server's file operations to:

1. Use `async-walkdir` for async-safe directory traversal
2. Consolidate duplicated file-finding patterns into shared utilities
3. Use `tokio::fs` for all file I/O
4. Create a clean `util::files` module that all tool implementations share

## Dependencies to Add

```toml
[dependencies]
async-walkdir = "1"
tokio = { version = "1", features = ["fs", "sync"] }
futures = "0.3"  # For StreamExt
```

You can likely remove `walkdir` after refactoring (unless used elsewhere ... but if it _is_ used elsewhere after this refactor, examine to see if it _should_ or if that too should be refactored along the lines given below).

## Current Anti-Patterns to Fix

### 1. Duplicated File-Finding Logic

These functions are nearly identical:

- `find_concept_file(base_path, id)`
- `find_guide_file(base_path, id)`
- `find_chapter_file(source_dir, chapter)`

Those are just examples. Perforn an exhuasive search of the codebase to identify other potential duplications of this type.

All follow the pattern:

1. Try exact match with patterns
2. Fall back to recursive search by file stem

**Refactor to:** A single generic `find_file_by_id()` function.

### 2. Duplicated Directory Scanning

These functions share scanning logic:

- `scan_concept_cards(base_path)`
- `scan_converted_sources(base_path)`

Both walk directories and collect metadata.

Those are just examples. Perforn an exhuasive search of the codebase to identify other potential duplications of this type.

**Refactor to:** A generic `scan_directory()` with a closure/trait for item extraction.

### 2.5. Heads-up on Recent Markdown refactor

If the recent Markdown refactor has changed the way files are read/parsed, make sure the `read_file()` utility (below) plays nicely with the new markdown parsing. They should compose cleanly:

```rust
let content = files::read_file(&path).await?;
let parsed = markdown::parse_with_frontmatter(&content)?;
```

### 3. Sync Operations in Async Context

All current `WalkDir` usage is synchronous. In an rmcp async server, this can block the executor.

**Refactor to:** Use `async-walkdir` throughout.

## Proposed Module Structure

```
src/
├── util/
│   ├── mod.rs
│   ├── files.rs      # File discovery and reading utilities
│   └── markdown.rs   # Markdown parsing (your other refactor)
├── tools/
│   ├── concepts.rs   # Uses util::files
│   ├── guides.rs     # Uses util::files
│   ├── sources.rs    # Uses util::files
│   └── ...
```

## New `util/files.rs` Implementation

```rust
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
pub async fn find_file_by_id(
    base_path: &Path,
    id: &str,
    options: FindOptions,
) -> Result<PathBuf> {
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
        let entry = entry_result.map_err(|e| Error::io(e.into()))?;
        let path = entry.path();

        // Check depth limit
        if let Some(max_depth) = options.max_depth {
            let depth = path.strip_prefix(base_path)
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
            if stem == id || stem.starts_with(&format!("{}-", id)) || stem.starts_with(&format!("{}_", id)) {
                return Ok(path.to_path_buf());
            }
        }
    }

    Err(Error::not_found(format!(
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
pub async fn find_all_files(
    base_path: &Path,
    options: FindOptions,
) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();
    let mut walker = WalkDir::new(base_path);

    while let Some(entry_result) = walker.next().await {
        let entry = entry_result.map_err(|e| Error::io(e.into()))?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Check depth limit
        if let Some(max_depth) = options.max_depth {
            let depth = path.strip_prefix(base_path)
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

        let relative_path = path
            .strip_prefix(base_path)
            .unwrap_or(path)
            .to_path_buf();

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

        let found = find_file_by_id(
            temp.path(),
            "pitch-class",
            FindOptions::markdown(),
        )
        .await
        .unwrap();

        assert_eq!(found, file_path);
    }

    #[tokio::test]
    async fn test_find_file_by_id_prefix_match() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("01-16-intervals.md");
        fs::write(&file_path, "# Intervals").await.unwrap();

        let found = find_file_by_id(
            temp.path(),
            "01-16",
            FindOptions::markdown(),
        )
        .await
        .unwrap();

        assert_eq!(found, file_path);
    }

    #[tokio::test]
    async fn test_find_all_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("one.md"), "# One").await.unwrap();
        fs::write(temp.path().join("two.md"), "# Two").await.unwrap();
        fs::write(temp.path().join("skip.txt"), "skip").await.unwrap();

        let files = find_all_files(temp.path(), FindOptions::markdown())
            .await
            .unwrap();

        assert_eq!(files.len(), 2);
    }
}
```

## Refactoring the Tool Implementations

### Before (concepts.rs)

```rust
fn find_concept_file(base_path: &Path, concept_id: &str) -> Result<PathBuf> {
    // ... 30+ lines of duplicated logic
}
```

### After (concepts.rs)

```rust
use crate::util::files::{find_file_by_id, FindOptions};

async fn find_concept_file(base_path: &Path, concept_id: &str) -> Result<PathBuf> {
    find_file_by_id(
        base_path,
        concept_id,
        FindOptions::markdown()
            .with_patterns(vec!["{id}.md", "{id}/README.md"]),
    )
    .await
}
```

### Before (sources.rs)

```rust
fn scan_converted_sources(base_path: &Path) -> Result<Vec<SourceInfo>> {
    // ... manual iteration with sync fs::read_dir
}
```

### After (sources.rs)

```rust
use crate::util::files::{list_subdirectories, count_files, FindOptions};

async fn scan_converted_sources(base_path: &Path) -> Result<Vec<SourceInfo>> {
    let mut sources = Vec::new();

    for dir in list_subdirectories(base_path).await? {
        let source_id = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let chapter_count = count_files(&dir, FindOptions::markdown()).await?;
        let title = humanize_source_id(&source_id);

        sources.push(SourceInfo {
            id: source_id,
            title,
            format: SourceFormat::Markdown,
            path: dir.to_string_lossy().to_string(),
            chapters: Some(chapter_count),
            status: SourceStatus::Converted,
        });
    }

    Ok(sources)
}
```

## Error Handling Integration

Make sure your `Error` type has these helpers (you may already have them from your error refactor):

```rust
impl Error {
    pub fn io(err: std::io::Error) -> Self {
        // ... convert io error
    }

    pub fn io_with_path(err: std::io::Error, path: &Path) -> Self {
        // ... convert with path context
    }

    pub fn not_found(context: impl Into<String>) -> Self {
        // ... create not found error
    }
}
```

## Checklist

- [ ] Add `async-walkdir` and `futures` to Cargo.toml
- [ ] Create `src/util/mod.rs` (if not exists)
- [ ] Create `src/util/files.rs` with shared utilities
- [ ] Refactor `tools/concepts.rs` to use `util::files`
- [ ] Refactor `tools/guides.rs` to use `util::files`
- [ ] Refactor `tools/sources.rs` to use `util::files`
- [ ] Refactor any other file operations in other tool modules
- [ ] Update function signatures from sync to async where needed
- [ ] Run tests: `cargo test`
- [ ] Run clippy: `cargo clippy`
- [ ] Remove `walkdir` from Cargo.toml if no longer used

## Testing

After refactoring, all existing tests should pass. The new `util::files` module should have its own unit tests (included in the implementation above).

```bash
cargo test
cargo clippy -- -D warnings
```

## Notes

- `async-walkdir` is async-runtime agnostic (works with tokio)
- `tokio::fs::try_exists` is the async equivalent of `Path::exists()`
- The `FindOptions` builder pattern makes the API flexible without lots of function variants
- `FileInfo` provides structured data that callers can use to build their domain types
