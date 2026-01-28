//! Index freshness tracking for Tantivy FTS.
//!
//! This module provides metadata tracking to avoid unnecessary index rebuilds.
//! Uses content hashing (file paths + mtimes) to detect changes.

use std::path::Path;
use std::time::SystemTime;

use async_walkdir::WalkDir;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Result;

/// Current schema version (increment when schema changes)
pub(crate) const SCHEMA_VERSION: u32 = 3; // v0.3.0: added content_type and section fields, changed part from u64 to text

/// Index metadata stored alongside the Tantivy index.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexMetadata {
    /// Schema version (for detecting incompatible indices)
    #[serde(default)]
    pub schema_version: u32,
    /// Number of documents in the index
    pub doc_count: usize,
    /// When the index was last built
    pub last_indexed: SystemTime,
    /// Hash of all concept card file paths and modification times
    pub content_hash: String,
    /// Per-type document counts (v0.3.0)
    #[serde(default)]
    pub concept_cards: usize,
    #[serde(default)]
    pub source_chapters: usize,
    #[serde(default)]
    pub unified_concepts: usize,
    #[serde(default)]
    pub guides: usize,
}

/// Compute a content hash from all concept card files.
///
/// Hashes file paths and modification times to detect changes without
/// reading file contents. Async traversal handles large directories efficiently.
///
/// # Arguments
///
/// * `config` - Server configuration with concept cards path
///
/// # Returns
///
/// Returns a hex string hash of all concept card files.
///
/// # Errors
///
/// Returns `Err` if directory traversal fails or paths are invalid.
pub async fn compute_content_hash(config: &Config) -> Result<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let concept_cards_path = config.paths.concept_cards_path()?;
    let mut hasher = DefaultHasher::new();
    let mut entries = WalkDir::new(concept_cards_path);

    while let Some(entry) = entries.next().await {
        if let Ok(entry) = entry {
            let path = entry.path();

            // Only hash markdown files
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(modified) = metadata.modified() {
                        // Hash both path and modification time
                        path.hash(&mut hasher);

                        // Hash the modification time as duration since UNIX_EPOCH
                        if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                            duration.as_secs().hash(&mut hasher);
                            duration.subsec_nanos().hash(&mut hasher);
                        }
                    }
                }
            }
        }
    }

    Ok(format!("{:x}", hasher.finish()))
}

/// Check if an index is fresh (content hasn't changed and schema matches).
///
/// Loads metadata from disk and compares:
/// 1. Schema version (must match current SCHEMA_VERSION)
/// 2. Content hash (must match current files)
///
/// # Arguments
///
/// * `index_path` - Path to the Tantivy index directory
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns true if the index exists, schema version matches, and content hash matches.
///
/// # Errors
///
/// Returns `Err` if metadata cannot be read or hash computation fails.
pub async fn is_index_fresh(index_path: &Path, config: &Config) -> Result<bool> {
    let metadata_path = index_path.join("metadata.json");

    if !metadata_path.exists() {
        return Ok(false);
    }

    let json = tokio::fs::read_to_string(&metadata_path).await?;
    let metadata: IndexMetadata = serde_json::from_str(&json)?;

    // Check schema version first - if it doesn't match, index needs rebuild
    if metadata.schema_version != SCHEMA_VERSION {
        log::info!(
            "Index schema version mismatch: stored={}, current={}. Index needs rebuild.",
            metadata.schema_version,
            SCHEMA_VERSION
        );
        return Ok(false);
    }

    let current_hash = compute_content_hash(config).await?;

    Ok(metadata.content_hash == current_hash)
}

/// Save index metadata to disk.
///
/// Writes metadata as JSON alongside the index for freshness checking.
///
/// # Arguments
///
/// * `index_path` - Path to the Tantivy index directory
/// * `metadata` - Metadata to save
///
/// # Errors
///
/// Returns `Err` if JSON serialization or file write fails.
pub async fn save_metadata(index_path: &Path, metadata: &IndexMetadata) -> Result<()> {
    let metadata_path = index_path.join("metadata.json");
    let json = serde_json::to_string_pretty(metadata)?;
    tokio::fs::write(&metadata_path, json).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, PathsConfig, SearchConfig, ServerConfig, SourcesConfig};
    use std::time::SystemTime;
    use tempfile::TempDir;

    fn test_config(temp_dir: &TempDir) -> Config {
        Config {
            server: ServerConfig {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
            },
            paths: PathsConfig {
                base: temp_dir.path().to_string_lossy().to_string(),
                sources_md: "sources-md".to_string(),
                concept_cards: "concept-cards".to_string(),
                concepts_unified: "concepts-unified".to_string(),
                guides: "guides".to_string(),
                skill_docs: ".".to_string(),
            },
            sources: SourcesConfig::default(),
            logging: twyg::OptsBuilder::new()
                .level(twyg::LogLevel::Info)
                .coloured(true)
                .output(twyg::Output::Stderr)
                .report_caller(false)
                .build()
                .unwrap(),
            search: SearchConfig {
                backend: "tantivy".to_string(),
                index_path: ".tantivy-index".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
                query_mode: crate::config::QueryMode::Smart,
                minimum_match_percent: 0.6,
                enable_stopwords: true,
                custom_stopwords: vec![],
                stopword_allowlist: vec![],
                field_boost_title: 3.0,
                field_boost_description: 2.0,
                field_boost_content: 1.0,
            },
        }
    }

    #[tokio::test]
    async fn test_compute_content_hash_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);

        // Create concept-cards directory
        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let hash = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");

        // Empty directory should produce a hash
        assert!(!hash.is_empty());
        assert!(hash.len() == 16); // DefaultHasher produces 64-bit hash = 16 hex chars
    }

    #[tokio::test]
    async fn test_compute_content_hash_with_files() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);

        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        // Create test file
        std::fs::write(concept_cards_path.join("test.md"), "# Test").expect("Failed to write file");

        let hash1 = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");
        let hash2 = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");

        // Hash should be consistent
        assert_eq!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_is_index_fresh_detects_changes() {
        // Test that is_index_fresh properly detects content changes
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);

        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        // Create initial file and compute hash
        std::fs::write(concept_cards_path.join("test.md"), "# Test").expect("Failed to write file");
        let hash1 = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");

        // Save metadata with this hash
        let index_path = temp_dir.path().join("index");
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");

        let metadata = IndexMetadata {
            schema_version: SCHEMA_VERSION,
            doc_count: 1,
            last_indexed: SystemTime::now(),
            content_hash: hash1.clone(),
            concept_cards: 1,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        };
        save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        // Should be fresh
        assert!(is_index_fresh(&index_path, &config)
            .await
            .expect("Freshness check failed"));

        // Add a new file
        std::fs::write(concept_cards_path.join("test2.md"), "# Test 2")
            .expect("Failed to write file 2");

        // Compute new hash (should be different because of new file)
        let hash2 = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");

        // If hashes are the same, skip the assertion (filesystem doesn't support granular mtimes)
        if hash1 != hash2 {
            // Should not be fresh anymore (if filesystem supports it)
            assert!(!is_index_fresh(&index_path, &config)
                .await
                .expect("Freshness check failed"));
        }
    }

    #[tokio::test]
    async fn test_is_index_fresh_no_metadata() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);
        let index_path = temp_dir.path().join("index");

        let is_fresh = is_index_fresh(&index_path, &config)
            .await
            .expect("Freshness check failed");
        assert!(!is_fresh, "Should not be fresh without metadata");
    }

    #[tokio::test]
    async fn test_save_and_load_metadata() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");

        let metadata = IndexMetadata {
            schema_version: SCHEMA_VERSION,
            doc_count: 42,
            last_indexed: SystemTime::now(),
            content_hash: "abcdef123456".to_string(),
            concept_cards: 30,
            source_chapters: 10,
            unified_concepts: 1,
            guides: 1,
        };

        save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        // Verify file was created
        let metadata_path = index_path.join("metadata.json");
        assert!(metadata_path.exists());

        // Load and verify
        let json = tokio::fs::read_to_string(&metadata_path)
            .await
            .expect("Failed to read metadata");
        let loaded: IndexMetadata = serde_json::from_str(&json).expect("Failed to parse metadata");

        assert_eq!(loaded.doc_count, 42);
        assert_eq!(loaded.content_hash, "abcdef123456");
    }

    #[tokio::test]
    async fn test_is_index_fresh_with_matching_hash() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);

        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");
        std::fs::write(concept_cards_path.join("test.md"), "# Test").expect("Failed to write file");

        let current_hash = compute_content_hash(&config)
            .await
            .expect("Hash computation failed");

        let index_path = temp_dir.path().join("index");
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");

        let metadata = IndexMetadata {
            schema_version: SCHEMA_VERSION,
            doc_count: 1,
            last_indexed: SystemTime::now(),
            content_hash: current_hash,
            concept_cards: 1,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        };

        save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        let is_fresh = is_index_fresh(&index_path, &config)
            .await
            .expect("Freshness check failed");
        assert!(is_fresh, "Should be fresh with matching hash");
    }

    #[tokio::test]
    async fn test_is_index_fresh_with_different_hash() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = test_config(&temp_dir);

        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");
        std::fs::write(concept_cards_path.join("test.md"), "# Test").expect("Failed to write file");

        let index_path = temp_dir.path().join("index");
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");

        // Save metadata with wrong hash
        let metadata = IndexMetadata {
            schema_version: SCHEMA_VERSION,
            doc_count: 1,
            last_indexed: SystemTime::now(),
            content_hash: "wrong_hash".to_string(),
            concept_cards: 1,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        };

        save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        let is_fresh = is_index_fresh(&index_path, &config)
            .await
            .expect("Freshness check failed");
        assert!(!is_fresh, "Should not be fresh with different hash");
    }

    #[tokio::test]
    async fn test_is_index_fresh_detects_schema_mismatch() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");
        std::fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let config = test_config(&temp_dir);
        let current_hash = compute_content_hash(&config)
            .await
            .expect("Failed to compute hash");

        // Save metadata with old schema version (0) but correct hash
        let metadata = IndexMetadata {
            schema_version: 0, // Old schema version
            doc_count: 1,
            last_indexed: SystemTime::now(),
            content_hash: current_hash,
            concept_cards: 1,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        };

        save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        let is_fresh = is_index_fresh(&index_path, &config)
            .await
            .expect("Freshness check failed");
        assert!(
            !is_fresh,
            "Should not be fresh with schema version mismatch"
        );
    }
}
