//! Source reference scanner for concept cards.
//!
//! This module scans concept cards to extract source references,
//! building a de-duplicated map of source titles to card IDs.

use std::collections::HashMap;

use crate::config::Config;
use crate::error::Result;
use crate::metadata::extract_concept_metadata;
use crate::sources::types::SourceReference;
use crate::util::files::{find_all_files, FindOptions};

/// Scan all concept cards and extract source references.
///
/// Returns a map of source titles to their references (including card IDs).
/// Sources without a title in the frontmatter are excluded.
///
/// # Arguments
///
/// * `config` - Application configuration with paths
///
/// # Returns
///
/// A HashMap where:
/// - Key: Source title (e.g., "Open Music Theory")
/// - Value: SourceReference containing the title and list of card IDs
///
/// # Errors
///
/// Returns `Err` if:
/// - The concept cards path cannot be resolved
/// - File scanning fails
pub async fn scan_concept_cards(config: &Config) -> Result<HashMap<String, SourceReference>> {
    let cards_path = config.paths.concept_cards_path()?;
    let mut sources: HashMap<String, SourceReference> = HashMap::new();

    // Find all markdown files in concept cards directory
    let files = find_all_files(&cards_path, FindOptions::markdown()).await?;

    for file_info in files {
        // Extract metadata from each concept card
        if let Ok(metadata) = extract_concept_metadata(&cards_path, &file_info.path).await {
            // Only process cards with a source field
            if let Some(source_title) = metadata.source {
                let source_title = source_title.trim().to_string();
                if !source_title.is_empty() {
                    sources
                        .entry(source_title.clone())
                        .or_insert_with(|| SourceReference::new(&source_title))
                        .add_card(&metadata.id);
                }
            }
        }
    }

    Ok(sources)
}

/// Scan statistics from a scan operation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanStats {
    /// Total number of concept cards scanned
    pub total_cards: usize,
    /// Number of unique sources found
    pub unique_sources: usize,
    /// Number of cards with source references
    pub cards_with_sources: usize,
}

impl ScanStats {
    /// Create new scan statistics.
    pub fn new(total_cards: usize, unique_sources: usize, cards_with_sources: usize) -> Self {
        Self {
            total_cards,
            unique_sources,
            cards_with_sources,
        }
    }
}

/// Scan concept cards and return both references and statistics.
///
/// This is an extended version of `scan_concept_cards` that also
/// computes statistics about the scan.
pub async fn scan_concept_cards_with_stats(
    config: &Config,
) -> Result<(HashMap<String, SourceReference>, ScanStats)> {
    let cards_path = config.paths.concept_cards_path()?;
    let mut sources: HashMap<String, SourceReference> = HashMap::new();
    let mut total_cards = 0;
    let mut cards_with_sources = 0;

    // Find all markdown files in concept cards directory
    let files = find_all_files(&cards_path, FindOptions::markdown()).await?;

    for file_info in files {
        total_cards += 1;

        // Extract metadata from each concept card
        if let Ok(metadata) = extract_concept_metadata(&cards_path, &file_info.path).await {
            // Only process cards with a source field
            if let Some(source_title) = metadata.source {
                let source_title = source_title.trim().to_string();
                if !source_title.is_empty() {
                    cards_with_sources += 1;
                    sources
                        .entry(source_title.clone())
                        .or_insert_with(|| SourceReference::new(&source_title))
                        .add_card(&metadata.id);
                }
            }
        }
    }

    let stats = ScanStats::new(total_cards, sources.len(), cards_with_sources);

    Ok((sources, stats))
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_config(temp_dir: &TempDir) -> Config {
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Create a minimal config
        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            temp_dir.path().join("test-index").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);
        let config = Config::load().unwrap();
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        config
    }

    async fn create_concept_card(
        temp_dir: &TempDir,
        category: &str,
        filename: &str,
        source: Option<&str>,
    ) -> PathBuf {
        let dir = temp_dir.path().join("concept-cards").join(category);
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join(filename);

        let content = if let Some(src) = source {
            format!(
                r#"---
title: "Test Concept"
category: "{}"
source: "{}"
---
# Test

Content"#,
                category, src
            )
        } else {
            format!(
                r#"---
title: "Test Concept"
category: "{}"
---
# Test

Content"#,
                category
            )
        };

        fs::write(&file_path, content).await.unwrap();
        file_path
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_empty() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let sources = scan_concept_cards(&config).await.unwrap();
        assert!(sources.is_empty());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_single_source() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(
            &temp_dir,
            "harmony",
            "test-concept.md",
            Some("Open Music Theory"),
        )
        .await;

        let sources = scan_concept_cards(&config).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert!(sources.contains_key("Open Music Theory"));
        assert_eq!(sources["Open Music Theory"].card_count(), 1);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_multiple_cards_same_source() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-1.md",
            Some("Open Music Theory"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "fundamentals",
            "concept-2.md",
            Some("Open Music Theory"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "voice-leading",
            "concept-3.md",
            Some("Open Music Theory"),
        )
        .await;

        let sources = scan_concept_cards(&config).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources["Open Music Theory"].card_count(), 3);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_multiple_sources() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-1.md",
            Some("Open Music Theory"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-2.md",
            Some("A Geometry of Music"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-3.md",
            Some("Twentieth-Century Harmony"),
        )
        .await;

        let sources = scan_concept_cards(&config).await.unwrap();
        assert_eq!(sources.len(), 3);
        assert!(sources.contains_key("Open Music Theory"));
        assert!(sources.contains_key("A Geometry of Music"));
        assert!(sources.contains_key("Twentieth-Century Harmony"));
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_no_source_field() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // Card with source
        create_concept_card(
            &temp_dir,
            "harmony",
            "with-source.md",
            Some("Open Music Theory"),
        )
        .await;

        // Card without source
        create_concept_card(&temp_dir, "harmony", "without-source.md", None).await;

        let sources = scan_concept_cards(&config).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert!(sources.contains_key("Open Music Theory"));
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_with_stats() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // 3 cards with sources
        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-1.md",
            Some("Open Music Theory"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-2.md",
            Some("Open Music Theory"),
        )
        .await;
        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-3.md",
            Some("A Geometry of Music"),
        )
        .await;

        // 1 card without source
        create_concept_card(&temp_dir, "fundamentals", "no-source.md", None).await;

        let (sources, stats) = scan_concept_cards_with_stats(&config).await.unwrap();

        assert_eq!(sources.len(), 2);
        assert_eq!(stats.total_cards, 4);
        assert_eq!(stats.unique_sources, 2);
        assert_eq!(stats.cards_with_sources, 3);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_trims_whitespace() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // Create card with whitespace in source
        let dir = temp_dir.path().join("concept-cards").join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let content = r#"---
title: "Test"
source: "  Open Music Theory  "
---
# Test"#;
        fs::write(dir.join("test.md"), content).await.unwrap();

        let sources = scan_concept_cards(&config).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert!(sources.contains_key("Open Music Theory"));
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_scan_concept_cards_empty_source_ignored() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // Create card with empty source
        let dir = temp_dir.path().join("concept-cards").join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let content = r#"---
title: "Test"
source: ""
---
# Test"#;
        fs::write(dir.join("test.md"), content).await.unwrap();

        let sources = scan_concept_cards(&config).await.unwrap();
        assert!(sources.is_empty());
    }

    #[test]
    fn test_scan_stats_new() {
        let stats = ScanStats::new(100, 10, 80);
        assert_eq!(stats.total_cards, 100);
        assert_eq!(stats.unique_sources, 10);
        assert_eq!(stats.cards_with_sources, 80);
    }

    #[test]
    fn test_scan_stats_default() {
        let stats = ScanStats::default();
        assert_eq!(stats.total_cards, 0);
        assert_eq!(stats.unique_sources, 0);
        assert_eq!(stats.cards_with_sources, 0);
    }
}
