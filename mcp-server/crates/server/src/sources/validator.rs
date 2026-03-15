//! Source validation logic.
//!
//! This module validates source references across:
//! - Concept cards vs configuration
//! - Configuration vs filesystem

use std::path::PathBuf;

use crate::config::Config;
use crate::error::Result;
use crate::sources::resolver::SourceResolver;
use crate::sources::scanner::scan_concept_cards_with_stats;
use crate::sources::types::{MissingFile, MissingSource, ValidationReport};

/// Validation mode for the validate command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationMode {
    /// Validate everything
    #[default]
    All,
    /// Only validate cards against config
    CardsConfig,
    /// Only validate cards against filesystem (via config)
    CardsFilesystem,
    /// Only validate config against filesystem
    ConfigFilesystem,
}

/// Validate sources according to the specified mode.
///
/// This function performs cross-validation between:
/// - Source references in concept cards
/// - Source configuration (titles extracted from filenames, aliases)
/// - Actual source files on the filesystem
///
/// # Arguments
///
/// * `config` - Application configuration
/// * `mode` - What to validate (All, CardsConfig, CardsFilesystem, ConfigFilesystem)
/// * `suggest_matches` - Whether to compute fuzzy match suggestions for missing sources
/// * `threshold` - Similarity threshold for fuzzy suggestions (0.0-1.0)
///
/// # Returns
///
/// A `ValidationReport` containing:
/// - Sources referenced in cards but not found in config
/// - Sources in config but not found on filesystem
/// - Summary statistics
pub async fn validate_sources(
    config: &Config,
    mode: ValidationMode,
    suggest_matches: bool,
    threshold: f32,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Scan concept cards for source references
    let (sources_in_cards, scan_stats) = scan_concept_cards_with_stats(config).await?;

    // Create resolver from config
    let resolver = SourceResolver::from_config(&config.sources);

    // Initialize stats
    report.stats.total_cards_scanned = scan_stats.total_cards;
    report.stats.unique_sources_found = sources_in_cards.len();
    report.stats.sources_in_config = resolver.known_ids().len();

    // Validate cards against config (unless mode is ConfigFilesystem only)
    if mode == ValidationMode::All
        || mode == ValidationMode::CardsConfig
        || mode == ValidationMode::CardsFilesystem
    {
        let mut resolved_count = 0;

        for (title, reference) in &sources_in_cards {
            let (resolved_id, _method) = resolver.resolve(title);

            if resolved_id.is_some() {
                resolved_count += 1;
            } else {
                // Source not found in config
                let mut missing = MissingSource::new(title, &reference.card_ids);

                // Add fuzzy suggestions if requested
                if suggest_matches {
                    let suggestions = resolver.suggest_matches(title, threshold);
                    for suggestion in suggestions {
                        missing.add_suggestion(suggestion);
                    }
                }

                report.missing_from_config.push(missing);
            }
        }

        report.stats.sources_resolved = resolved_count;
        report.stats.missing_from_config = report.missing_from_config.len();
    }

    // Validate config against filesystem (unless mode is CardsConfig only)
    if mode == ValidationMode::All
        || mode == ValidationMode::ConfigFilesystem
        || mode == ValidationMode::CardsFilesystem
    {
        let missing_files = check_filesystem(config).await?;
        report.stats.sources_on_disk = report.stats.sources_in_config - missing_files.len();
        report.stats.missing_from_disk = missing_files.len();
        report.missing_from_filesystem = missing_files;
    }

    Ok(report)
}

/// Check which configured sources are missing from the filesystem.
async fn check_filesystem(config: &Config) -> Result<Vec<MissingFile>> {
    let mut missing = Vec::new();

    // Check each category
    for (category_name, category) in [
        ("oxford", &config.sources.oxford),
        ("general", &config.sources.general),
        ("papers", &config.sources.papers),
    ] {
        // Skip if no base path configured
        if category.path.is_empty() {
            continue;
        }

        let base_path = PathBuf::from(&category.path);

        for (file_id, filename) in &category.files {
            let file_path = base_path.join(filename);
            let source_id = format!("{}-{}", category_name, file_id);

            // Check if file exists
            if !tokio::fs::try_exists(&file_path).await.unwrap_or(false) {
                missing.push(MissingFile::new(&source_id, &file_path, category_name));
            }
        }
    }

    Ok(missing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::collections::HashMap;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_config_with_sources(
        temp_dir: &TempDir,
        sources: HashMap<String, String>,
    ) -> Config {
        let concept_cards_path = temp_dir.path().join("concept-cards");
        let sources_path = temp_dir.path().join("sources");
        fs::create_dir_all(&concept_cards_path).await.unwrap();
        fs::create_dir_all(&sources_path).await.unwrap();

        // Build sources config section
        let mut sources_config = String::new();
        sources_config.push_str("[sources.general]\n");
        sources_config.push_str(&format!("path = \"{}\"\n", sources_path.display()));
        sources_config.push_str("[sources.general.files]\n");
        for (id, filename) in &sources {
            sources_config.push_str(&format!("{} = \"{}\"\n", id, filename));
        }
        sources_config.push_str("[sources.general.aliases]\n");
        sources_config.push_str("[sources.oxford]\n");
        sources_config.push_str("path = \"\"\n");
        sources_config.push_str("[sources.oxford.files]\n");
        sources_config.push_str("[sources.oxford.aliases]\n");
        sources_config.push_str("[sources.papers]\n");
        sources_config.push_str("path = \"\"\n");
        sources_config.push_str("[sources.papers.files]\n");
        sources_config.push_str("[sources.papers.aliases]\n");

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

{}

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
            sources_config,
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

    async fn create_concept_card(temp_dir: &TempDir, filename: &str, source: &str) {
        let dir = temp_dir.path().join("concept-cards").join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let content = format!(
            r#"---
title: "Test Concept"
category: "harmony"
source: "{}"
---
# Test

Content"#,
            source
        );
        fs::write(dir.join(filename), content).await.unwrap();
    }

    async fn create_source_file(temp_dir: &TempDir, filename: &str) {
        let dir = temp_dir.path().join("sources");
        fs::create_dir_all(&dir).await.unwrap();
        fs::write(dir.join(filename), "dummy content")
            .await
            .unwrap();
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_all_valid() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "open-music-theory".to_string(),
            "[2022] Gotham - Open Music Theory.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Create concept card referencing the source
        create_concept_card(&temp_dir, "test.md", "Open Music Theory").await;

        // Create the actual source file
        create_source_file(&temp_dir, "[2022] Gotham - Open Music Theory.pdf").await;

        let report = validate_sources(&config, ValidationMode::All, false, 0.7)
            .await
            .unwrap();

        assert!(report.is_valid());
        assert!(report.missing_from_config.is_empty());
        assert!(report.missing_from_filesystem.is_empty());
        assert_eq!(report.stats.sources_resolved, 1);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_missing_from_config() {
        let temp_dir = TempDir::new().unwrap();

        // No sources configured
        let config = create_test_config_with_sources(&temp_dir, HashMap::new()).await;

        // Create concept card referencing a non-existent source
        create_concept_card(&temp_dir, "test.md", "Unknown Source").await;

        let report = validate_sources(&config, ValidationMode::All, false, 0.7)
            .await
            .unwrap();

        assert!(!report.is_valid());
        assert_eq!(report.missing_from_config.len(), 1);
        assert_eq!(report.missing_from_config[0].title, "Unknown Source");
        assert_eq!(report.stats.missing_from_config, 1);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_missing_from_filesystem() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "missing-source".to_string(),
            "[2022] Author - Missing Source.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Don't create the source file - it's missing

        let report = validate_sources(&config, ValidationMode::All, false, 0.7)
            .await
            .unwrap();

        assert!(!report.is_valid());
        assert_eq!(report.missing_from_filesystem.len(), 1);
        assert_eq!(
            report.missing_from_filesystem[0].config_id,
            "general-missing-source"
        );
        assert_eq!(report.stats.missing_from_disk, 1);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_with_suggestions() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "open-music-theory".to_string(),
            "[2022] Gotham - Open Music Theory.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Create source file
        create_source_file(&temp_dir, "[2022] Gotham - Open Music Theory.pdf").await;

        // Create concept card with a typo in source name
        create_concept_card(&temp_dir, "test.md", "Open Music Theor").await; // Missing 'y'

        let report = validate_sources(&config, ValidationMode::All, true, 0.7)
            .await
            .unwrap();

        assert!(!report.is_valid());
        assert_eq!(report.missing_from_config.len(), 1);

        // Should have a suggestion
        let missing = &report.missing_from_config[0];
        assert!(!missing.suggestions.is_empty());
        assert!(missing.suggestions[0].similarity > 0.7);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_mode_cards_config_only() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "missing-file".to_string(),
            "[2022] Author - Missing.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Create concept card matching config
        create_concept_card(&temp_dir, "test.md", "Missing").await;

        // Don't create file - but with CardsConfig mode, we shouldn't check filesystem

        let report = validate_sources(&config, ValidationMode::CardsConfig, false, 0.7)
            .await
            .unwrap();

        // Should be valid because we're only checking cards vs config
        assert!(report.is_valid());
        assert!(report.missing_from_config.is_empty());
        assert!(report.missing_from_filesystem.is_empty());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_mode_config_filesystem_only() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "missing-file".to_string(),
            "[2022] Author - Missing.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Create concept card with unknown source - should be ignored in ConfigFilesystem mode
        create_concept_card(&temp_dir, "test.md", "Unknown Source").await;

        let report = validate_sources(&config, ValidationMode::ConfigFilesystem, false, 0.7)
            .await
            .unwrap();

        // Should have missing from filesystem but not from config (since we skip card validation)
        assert!(!report.is_valid());
        assert!(report.missing_from_config.is_empty());
        assert_eq!(report.missing_from_filesystem.len(), 1);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_validate_sources_stats() {
        let temp_dir = TempDir::new().unwrap();

        let mut sources = HashMap::new();
        sources.insert(
            "source-1".to_string(),
            "[2022] Author - Source One.pdf".to_string(),
        );
        sources.insert(
            "source-2".to_string(),
            "[2022] Author - Source Two.pdf".to_string(),
        );

        let config = create_test_config_with_sources(&temp_dir, sources).await;

        // Create source files
        create_source_file(&temp_dir, "[2022] Author - Source One.pdf").await;
        create_source_file(&temp_dir, "[2022] Author - Source Two.pdf").await;

        // Create concept cards
        create_concept_card(&temp_dir, "card-1.md", "Source One").await;
        create_concept_card(&temp_dir, "card-2.md", "Source One").await;
        create_concept_card(&temp_dir, "card-3.md", "Source Two").await;
        create_concept_card(&temp_dir, "card-4.md", "Unknown Source").await;

        let report = validate_sources(&config, ValidationMode::All, false, 0.7)
            .await
            .unwrap();

        assert_eq!(report.stats.total_cards_scanned, 4);
        assert_eq!(report.stats.unique_sources_found, 3); // One, Two, Unknown
        assert_eq!(report.stats.sources_in_config, 2);
        assert_eq!(report.stats.sources_resolved, 2); // One, Two
        assert_eq!(report.stats.missing_from_config, 1); // Unknown
        assert_eq!(report.stats.sources_on_disk, 2);
        assert_eq!(report.stats.missing_from_disk, 0);
    }

    #[test]
    fn test_validation_mode_default() {
        let mode = ValidationMode::default();
        assert_eq!(mode, ValidationMode::All);
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_check_filesystem_empty_path() {
        let temp_dir = TempDir::new().unwrap();

        // Config with empty paths - should skip
        let config = create_test_config_with_sources(&temp_dir, HashMap::new()).await;

        let missing = check_filesystem(&config).await.unwrap();
        assert!(missing.is_empty());
    }
}
