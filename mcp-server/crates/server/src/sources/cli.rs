//! CLI handlers for source management commands.
//!
//! This module implements the handlers for:
//! - `sources scan` - Scan concept cards for source references
//! - `sources validate` - Validate sources across cards, config, filesystem
//! - `sources alias` - Manage source title aliases

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::path_resolver;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::sources::scanner::scan_concept_cards_with_stats;
use crate::sources::validator::{validate_sources, ValidationMode};

/// Source management commands.
#[derive(Parser, Debug)]
pub struct SourcesCommands {
    #[command(subcommand)]
    pub command: SourcesSubcommand,
}

/// Source management subcommands.
#[derive(Subcommand, Debug)]
pub enum SourcesSubcommand {
    /// Scan concept cards to find all referenced sources
    Scan {
        /// Output format: table or json
        #[arg(long, default_value = "table")]
        output: String,

        /// Show list of cards referencing each source
        #[arg(long)]
        show_cards: bool,
    },

    /// Validate sources against config and filesystem
    Validate {
        /// Validation mode: all, cards-config, cards-fs, config-fs
        #[arg(long, default_value = "all")]
        mode: String,

        /// Show fuzzy match suggestions for missing sources
        #[arg(long)]
        suggest_matches: bool,

        /// Similarity threshold for suggestions (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        threshold: f32,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage source title aliases
    Alias(AliasCommands),
}

/// Alias subcommands.
#[derive(Parser, Debug)]
pub struct AliasCommands {
    #[command(subcommand)]
    pub command: AliasSubcommand,
}

/// Alias management subcommands.
#[derive(Subcommand, Debug)]
pub enum AliasSubcommand {
    /// List all configured aliases
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Add an alias for a source
    Add {
        /// Source ID (e.g., general-open-music-theory)
        source_id: String,

        /// Alias title to add
        alias: String,
    },

    /// Remove an alias from a source
    Remove {
        /// Source ID (e.g., general-open-music-theory)
        source_id: String,

        /// Alias title to remove
        alias: String,
    },
}

/// Handle sources commands.
pub async fn handle_sources_command(
    sources_cmds: SourcesCommands,
    _log_level_override: Option<String>,
) -> Result<()> {
    let config = Config::load()?;

    match sources_cmds.command {
        SourcesSubcommand::Scan { output, show_cards } => {
            handle_scan(&config, &output, show_cards).await
        }
        SourcesSubcommand::Validate {
            mode,
            suggest_matches,
            threshold,
            json,
        } => handle_validate(&config, &mode, suggest_matches, threshold, json).await,
        SourcesSubcommand::Alias(alias_cmds) => handle_alias(&config, alias_cmds).await,
    }
}

/// Handle the scan command.
async fn handle_scan(config: &Config, output: &str, show_cards: bool) -> Result<()> {
    let (sources, stats) = scan_concept_cards_with_stats(config).await?;

    if output == "json" {
        // JSON output
        let mut json_sources: Vec<serde_json::Value> = sources
            .iter()
            .map(|(title, reference)| {
                let mut obj = serde_json::json!({
                    "title": title,
                    "card_count": reference.card_count(),
                });
                if show_cards {
                    obj["card_ids"] = serde_json::json!(reference.card_ids);
                }
                obj
            })
            .collect();

        // Sort by card count descending
        json_sources.sort_by(|a, b| {
            let count_a = a["card_count"].as_u64().unwrap_or(0);
            let count_b = b["card_count"].as_u64().unwrap_or(0);
            count_b.cmp(&count_a)
        });

        let output = serde_json::json!({
            "stats": {
                "total_cards": stats.total_cards,
                "unique_sources": stats.unique_sources,
                "cards_with_sources": stats.cards_with_sources,
            },
            "sources": json_sources,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        // Table output
        println!("Source Scan Results");
        println!("===================\n");

        println!("Statistics:");
        println!("  Total cards scanned:    {}", stats.total_cards);
        println!("  Cards with sources:     {}", stats.cards_with_sources);
        println!("  Unique sources found:   {}", stats.unique_sources);
        println!();

        if sources.is_empty() {
            println!("No sources found.");
            return Ok(());
        }

        // Sort by card count descending
        let mut sorted: Vec<_> = sources.iter().collect();
        sorted.sort_by(|a, b| b.1.card_count().cmp(&a.1.card_count()));

        println!("Sources (by card count):");
        println!("{:-<60}", "");

        for (title, reference) in sorted {
            println!(
                "  {:50} {:>4} cards",
                truncate_string(title, 50),
                reference.card_count()
            );
            if show_cards {
                for card_id in &reference.card_ids {
                    println!("    - {}", card_id);
                }
            }
        }
    }

    Ok(())
}

/// Handle the validate command.
async fn handle_validate(
    config: &Config,
    mode_str: &str,
    suggest_matches: bool,
    threshold: f32,
    json: bool,
) -> Result<()> {
    let mode = match mode_str {
        "all" => ValidationMode::All,
        "cards-config" => ValidationMode::CardsConfig,
        "cards-fs" => ValidationMode::CardsFilesystem,
        "config-fs" => ValidationMode::ConfigFilesystem,
        _ => {
            eprintln!("Unknown validation mode: {}. Using 'all'.", mode_str);
            ValidationMode::All
        }
    };

    let report = validate_sources(config, mode, suggest_matches, threshold).await?;

    if json {
        // JSON output
        let missing_config: Vec<serde_json::Value> = report
            .missing_from_config
            .iter()
            .map(|m| {
                let mut obj = serde_json::json!({
                    "title": m.title,
                    "card_count": m.card_count,
                    "sample_card_ids": m.sample_card_ids,
                });
                if !m.suggestions.is_empty() {
                    obj["suggestions"] = serde_json::json!(m
                        .suggestions
                        .iter()
                        .map(|s| {
                            serde_json::json!({
                                "config_id": s.config_id,
                                "title": s.title,
                                "similarity": s.similarity,
                            })
                        })
                        .collect::<Vec<_>>());
                }
                obj
            })
            .collect();

        let missing_fs: Vec<serde_json::Value> = report
            .missing_from_filesystem
            .iter()
            .map(|m| {
                serde_json::json!({
                    "config_id": m.config_id,
                    "expected_path": m.expected_path.display().to_string(),
                    "category": m.category,
                })
            })
            .collect();

        let output = serde_json::json!({
            "valid": report.is_valid(),
            "stats": {
                "total_cards_scanned": report.stats.total_cards_scanned,
                "unique_sources_found": report.stats.unique_sources_found,
                "sources_resolved": report.stats.sources_resolved,
                "sources_in_config": report.stats.sources_in_config,
                "sources_on_disk": report.stats.sources_on_disk,
                "missing_from_config": report.stats.missing_from_config,
                "missing_from_disk": report.stats.missing_from_disk,
            },
            "missing_from_config": missing_config,
            "missing_from_filesystem": missing_fs,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        // Human-readable output
        println!("Source Validation Report");
        println!("========================\n");

        // Statistics
        println!("Statistics:");
        println!(
            "  Total cards scanned:    {}",
            report.stats.total_cards_scanned
        );
        println!(
            "  Unique sources found:   {}",
            report.stats.unique_sources_found
        );
        println!(
            "  Sources resolved:       {}",
            report.stats.sources_resolved
        );
        println!(
            "  Sources in config:      {}",
            report.stats.sources_in_config
        );
        println!("  Sources on disk:        {}", report.stats.sources_on_disk);
        println!();

        // Missing from config
        if !report.missing_from_config.is_empty() {
            println!(
                "Missing from configuration ({}):",
                report.missing_from_config.len()
            );
            println!("{:-<60}", "");
            for missing in &report.missing_from_config {
                println!("  \"{}\" ({} cards)", missing.title, missing.card_count);
                if !missing.sample_card_ids.is_empty() {
                    println!("    Sample cards: {}", missing.sample_card_ids.join(", "));
                }
                if !missing.suggestions.is_empty() {
                    println!("    Suggestions:");
                    for suggestion in &missing.suggestions {
                        println!(
                            "      - {} ({}%, id: {})",
                            suggestion.title,
                            (suggestion.similarity * 100.0) as u32,
                            suggestion.config_id
                        );
                    }
                }
            }
            println!();
        }

        // Missing from filesystem
        if !report.missing_from_filesystem.is_empty() {
            println!(
                "Missing from filesystem ({}):",
                report.missing_from_filesystem.len()
            );
            println!("{:-<60}", "");
            for missing in &report.missing_from_filesystem {
                println!("  {} ({})", missing.config_id, missing.category);
                println!("    Expected: {}", missing.expected_path.display());
            }
            println!();
        }

        // Summary
        if report.is_valid() {
            println!("✓ All sources validated successfully!");
        } else {
            println!(
                "✗ Validation failed: {} issues found",
                report.missing_from_config.len() + report.missing_from_filesystem.len()
            );
        }
    }

    // Exit with appropriate code
    std::process::exit(report.exit_code());
}

/// Handle alias commands.
async fn handle_alias(config: &Config, alias_cmds: AliasCommands) -> Result<()> {
    match alias_cmds.command {
        AliasSubcommand::List { json } => handle_alias_list(config, json).await,
        AliasSubcommand::Add { source_id, alias } => handle_alias_add(&source_id, &alias).await,
        AliasSubcommand::Remove { source_id, alias } => {
            handle_alias_remove(&source_id, &alias).await
        }
    }
}

/// Handle the alias list command.
async fn handle_alias_list(config: &Config, json: bool) -> Result<()> {
    let mut all_aliases: Vec<(String, String, Vec<String>)> = Vec::new();

    // Collect aliases from all categories
    for (category_name, category) in [
        ("oxford", &config.sources.oxford),
        ("general", &config.sources.general),
        ("papers", &config.sources.papers),
    ] {
        for (file_id, aliases) in &category.aliases {
            if !aliases.is_empty() {
                let source_id = format!("{}-{}", category_name, file_id);
                all_aliases.push((source_id, category_name.to_string(), aliases.clone()));
            }
        }
    }

    if json {
        let output: Vec<serde_json::Value> = all_aliases
            .iter()
            .map(|(source_id, category, aliases)| {
                serde_json::json!({
                    "source_id": source_id,
                    "category": category,
                    "aliases": aliases,
                })
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("Configured Aliases");
        println!("==================\n");

        if all_aliases.is_empty() {
            println!("No aliases configured.");
            return Ok(());
        }

        for (source_id, _category, aliases) in &all_aliases {
            println!("{}:", source_id);
            for alias in aliases {
                println!("  - \"{}\"", alias);
            }
            println!();
        }

        println!("Total: {} sources with aliases", all_aliases.len());
    }

    Ok(())
}

/// Handle the alias add command.
async fn handle_alias_add(source_id: &str, alias: &str) -> Result<()> {
    // Parse source_id to get category and file_id
    let (category, file_id) = parse_source_id(source_id)?;

    // Get config file path
    let config_path = get_config_path()?;

    // Read and parse the config file
    let content = tokio::fs::read_to_string(&config_path).await?;
    let mut doc = content
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| Error::config(format!("Failed to parse config: {}", e)))?;

    // Navigate to sources.<category>.aliases.<file_id>
    let sources = doc
        .get_mut("sources")
        .and_then(|s| s.as_table_mut())
        .ok_or_else(|| Error::config("Missing [sources] section in config".to_string()))?;

    let category_table = sources
        .get_mut(category)
        .and_then(|c| c.as_table_mut())
        .ok_or_else(|| {
            Error::config(format!("Missing [sources.{}] section in config", category))
        })?;

    // Ensure aliases table exists
    if !category_table.contains_key("aliases") {
        category_table.insert("aliases", toml_edit::table());
    }

    let aliases_table = category_table
        .get_mut("aliases")
        .and_then(|a| a.as_table_mut())
        .ok_or_else(|| {
            Error::config(format!(
                "Failed to access [sources.{}.aliases] section",
                category
            ))
        })?;

    // Get or create the alias array for this file_id
    if !aliases_table.contains_key(file_id) {
        let mut arr = toml_edit::Array::new();
        arr.set_trailing_comma(false);
        aliases_table.insert(file_id, toml_edit::value(arr));
    }

    let alias_array = aliases_table
        .get_mut(file_id)
        .and_then(|item| item.as_value_mut())
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| {
            Error::config(format!(
                "Failed to access aliases for {} (expected array)",
                source_id
            ))
        })?;

    // Check if alias already exists
    let alias_exists = alias_array.iter().any(|v| v.as_str() == Some(alias));
    if alias_exists {
        println!("Alias \"{}\" already exists for {}", alias, source_id);
        return Ok(());
    }

    // Add the new alias
    alias_array.push(alias);

    // Write back to file
    tokio::fs::write(&config_path, doc.to_string()).await?;

    println!("✓ Added alias \"{}\" to {}", alias, source_id);
    println!("  Config updated: {}", config_path.display());

    Ok(())
}

/// Handle the alias remove command.
async fn handle_alias_remove(source_id: &str, alias: &str) -> Result<()> {
    // Parse source_id to get category and file_id
    let (category, file_id) = parse_source_id(source_id)?;

    // Get config file path
    let config_path = get_config_path()?;

    // Read and parse the config file
    let content = tokio::fs::read_to_string(&config_path).await?;
    let mut doc = content
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| Error::config(format!("Failed to parse config: {}", e)))?;

    // Navigate to sources.<category>.aliases.<file_id>
    let sources = doc
        .get_mut("sources")
        .and_then(|s| s.as_table_mut())
        .ok_or_else(|| Error::config("Missing [sources] section in config".to_string()))?;

    let category_table = sources
        .get_mut(category)
        .and_then(|c| c.as_table_mut())
        .ok_or_else(|| {
            Error::config(format!("Missing [sources.{}] section in config", category))
        })?;

    let aliases_table = match category_table
        .get_mut("aliases")
        .and_then(|a| a.as_table_mut())
    {
        Some(t) => t,
        None => {
            println!("No aliases configured for category '{}'", category);
            return Ok(());
        }
    };

    let alias_array = match aliases_table
        .get_mut(file_id)
        .and_then(|a| a.as_array_mut())
    {
        Some(a) => a,
        None => {
            println!("No aliases configured for {}", source_id);
            return Ok(());
        }
    };

    // Find and remove the alias
    let original_len = alias_array.len();
    alias_array.retain(|v| v.as_str() != Some(alias));

    if alias_array.len() == original_len {
        println!("Alias \"{}\" not found for {}", alias, source_id);
        return Ok(());
    }

    // If array is now empty, remove the key
    if alias_array.is_empty() {
        aliases_table.remove(file_id);
    }

    // If aliases table is now empty, remove it
    if aliases_table.is_empty() {
        category_table.remove("aliases");
    }

    // Write back to file
    tokio::fs::write(&config_path, doc.to_string()).await?;

    println!("✓ Removed alias \"{}\" from {}", alias, source_id);
    println!("  Config updated: {}", config_path.display());

    Ok(())
}

/// Parse a source ID into category and file_id.
///
/// Source IDs have the format: `<category>-<file_id>`
/// Example: `general-persichetti-20th-century` -> ("general", "persichetti-20th-century")
fn parse_source_id(source_id: &str) -> Result<(&str, &str)> {
    // Valid categories
    let categories = ["oxford", "general", "papers"];

    for cat in categories {
        if let Some(file_id) = source_id.strip_prefix(&format!("{}-", cat)) {
            if !file_id.is_empty() {
                return Ok((cat, file_id));
            }
        }
    }

    Err(Error::config(format!(
        "Invalid source ID '{}'. Expected format: <category>-<file_id> where category is one of: oxford, general, papers",
        source_id
    )))
}

/// Get the path to the config file.
fn get_config_path() -> Result<PathBuf> {
    path_resolver()
        .config_dir()
        .map(|d| d.join("default.toml"))
        .ok_or_else(|| Error::config("Could not locate config directory".to_string()))
}

/// Truncate a string to a maximum length, adding "..." if truncated.
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s[..max_len].to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;
    use tokio::fs;

    // ---------------------------------------------------------------
    // Helper: create a temp config directory with a TOML config file
    // and set the MUSIC_THEORY_CONFIG_DIR env var. Returns Config.
    // ---------------------------------------------------------------

    async fn create_test_config(temp_dir: &TempDir) -> Config {
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

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

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]

[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]

[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

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

    /// Create a test config with sources (general category) and aliases.
    async fn create_test_config_with_sources(
        temp_dir: &TempDir,
        files: &[(&str, &str)],
        aliases: &[(&str, &[&str])],
    ) -> Config {
        let concept_cards_path = temp_dir.path().join("concept-cards");
        let sources_path = temp_dir.path().join("sources");
        fs::create_dir_all(&concept_cards_path).await.unwrap();
        fs::create_dir_all(&sources_path).await.unwrap();

        let mut files_section = String::new();
        for (id, filename) in files {
            files_section.push_str(&format!("{} = \"{}\"\n", id, filename));
        }

        let mut aliases_section = String::new();
        for (id, alias_list) in aliases {
            let formatted: Vec<String> = alias_list.iter().map(|a| format!("\"{}\"", a)).collect();
            aliases_section.push_str(&format!("{} = [{}]\n", id, formatted.join(", ")));
        }

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]

[sources.general]
path = "{sources}"
[sources.general.files]
{files_section}
[sources.general.aliases]
{aliases_section}

[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            sources = sources_path.display(),
            files_section = files_section,
            aliases_section = aliases_section,
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);
        let config = Config::load().unwrap();
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        config
    }

    /// Create a concept card with optional source in the temp dir.
    async fn create_concept_card(
        temp_dir: &TempDir,
        category: &str,
        filename: &str,
        source: Option<&str>,
    ) {
        let dir = temp_dir.path().join("concept-cards").join(category);
        fs::create_dir_all(&dir).await.unwrap();

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

        fs::write(dir.join(filename), content).await.unwrap();
    }

    // ===================================================================
    // truncate_string tests
    // ===================================================================

    #[test]
    fn test_truncate_string_short() {
        assert_eq!(truncate_string("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_string_exact() {
        assert_eq!(truncate_string("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_string_long() {
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }

    #[test]
    fn test_truncate_string_very_short_max() {
        assert_eq!(truncate_string("hello", 3), "hel");
    }

    #[test]
    fn test_truncate_string_empty_input() {
        assert_eq!(truncate_string("", 10), "");
    }

    #[test]
    fn test_truncate_string_empty_input_zero_max() {
        assert_eq!(truncate_string("", 0), "");
    }

    #[test]
    fn test_truncate_string_max_len_zero() {
        // max_len = 0 <= 3, so uses s[..0]
        assert_eq!(truncate_string("hello", 0), "");
    }

    #[test]
    fn test_truncate_string_max_len_one() {
        assert_eq!(truncate_string("hello", 1), "h");
    }

    #[test]
    fn test_truncate_string_max_len_two() {
        assert_eq!(truncate_string("hello", 2), "he");
    }

    #[test]
    fn test_truncate_string_max_len_four_truncates() {
        // max_len = 4, s.len() = 5, max_len > 3, so format!("{}...", &s[..1]) = "h..."
        assert_eq!(truncate_string("hello", 4), "h...");
    }

    #[test]
    fn test_truncate_string_long_title() {
        let long = "A Very Long Title That Should Definitely Be Truncated For Display";
        let result = truncate_string(long, 20);
        assert_eq!(result.len(), 20);
        assert!(result.ends_with("..."));
        assert_eq!(result, "A Very Long Title...");
    }

    // ===================================================================
    // parse_source_id tests
    // ===================================================================

    #[test]
    fn test_parse_source_id_general() {
        let (cat, file_id) = parse_source_id("general-persichetti-20th-century").unwrap();
        assert_eq!(cat, "general");
        assert_eq!(file_id, "persichetti-20th-century");
    }

    #[test]
    fn test_parse_source_id_oxford() {
        let (cat, file_id) = parse_source_id("oxford-gollin-handbook").unwrap();
        assert_eq!(cat, "oxford");
        assert_eq!(file_id, "gollin-handbook");
    }

    #[test]
    fn test_parse_source_id_papers() {
        let (cat, file_id) = parse_source_id("papers-fiore").unwrap();
        assert_eq!(cat, "papers");
        assert_eq!(file_id, "fiore");
    }

    #[test]
    fn test_parse_source_id_invalid_category() {
        let result = parse_source_id("unknown-source");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_source_id_empty_file_id() {
        let result = parse_source_id("general-");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_source_id_no_dash() {
        let result = parse_source_id("generalsource");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_source_id_empty_string() {
        let result = parse_source_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_source_id_just_category() {
        let result = parse_source_id("general");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_source_id_error_message_contains_input() {
        let result = parse_source_id("invalid-id");
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("invalid-id"),
            "Error should contain the input: {}",
            msg
        );
    }

    #[test]
    fn test_parse_source_id_error_message_lists_categories() {
        let result = parse_source_id("badcat-source");
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("oxford"), "Error should list oxford: {}", msg);
        assert!(
            msg.contains("general"),
            "Error should list general: {}",
            msg
        );
        assert!(msg.contains("papers"), "Error should list papers: {}", msg);
    }

    #[test]
    fn test_parse_source_id_oxford_with_multiple_dashes() {
        let (cat, file_id) = parse_source_id("oxford-gollin-neo-riemannian-handbook").unwrap();
        assert_eq!(cat, "oxford");
        assert_eq!(file_id, "gollin-neo-riemannian-handbook");
    }

    #[test]
    fn test_parse_source_id_papers_single_word() {
        let (cat, file_id) = parse_source_id("papers-lewin").unwrap();
        assert_eq!(cat, "papers");
        assert_eq!(file_id, "lewin");
    }

    // ===================================================================
    // get_config_path tests
    // ===================================================================

    #[test]
    #[serial(config_env)]
    fn test_get_config_path_with_valid_env() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();
        // config_dir() requires default.toml to exist
        std::fs::write(config_dir.join("default.toml"), "[server]").unwrap();
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", config_dir);

        let result = get_config_path();
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("default.toml"));
    }

    #[test]
    #[serial(config_env)]
    fn test_get_config_path_returns_default_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();
        std::fs::write(config_dir.join("default.toml"), "[server]").unwrap();
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", config_dir);

        let path = get_config_path().unwrap();
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert_eq!(path.file_name().unwrap(), "default.toml");
    }

    // ===================================================================
    // Validation mode parsing tests (covering the match in handle_validate)
    // ===================================================================

    #[test]
    fn test_validation_mode_parsing_all() {
        let mode = match "all" {
            "all" => ValidationMode::All,
            "cards-config" => ValidationMode::CardsConfig,
            "cards-fs" => ValidationMode::CardsFilesystem,
            "config-fs" => ValidationMode::ConfigFilesystem,
            _ => ValidationMode::All,
        };
        assert_eq!(mode, ValidationMode::All);
    }

    #[test]
    fn test_validation_mode_parsing_cards_config() {
        let mode = match "cards-config" {
            "all" => ValidationMode::All,
            "cards-config" => ValidationMode::CardsConfig,
            "cards-fs" => ValidationMode::CardsFilesystem,
            "config-fs" => ValidationMode::ConfigFilesystem,
            _ => ValidationMode::All,
        };
        assert_eq!(mode, ValidationMode::CardsConfig);
    }

    #[test]
    fn test_validation_mode_parsing_cards_fs() {
        let mode = match "cards-fs" {
            "all" => ValidationMode::All,
            "cards-config" => ValidationMode::CardsConfig,
            "cards-fs" => ValidationMode::CardsFilesystem,
            "config-fs" => ValidationMode::ConfigFilesystem,
            _ => ValidationMode::All,
        };
        assert_eq!(mode, ValidationMode::CardsFilesystem);
    }

    #[test]
    fn test_validation_mode_parsing_config_fs() {
        let mode = match "config-fs" {
            "all" => ValidationMode::All,
            "cards-config" => ValidationMode::CardsConfig,
            "cards-fs" => ValidationMode::CardsFilesystem,
            "config-fs" => ValidationMode::ConfigFilesystem,
            _ => ValidationMode::All,
        };
        assert_eq!(mode, ValidationMode::ConfigFilesystem);
    }

    #[test]
    fn test_validation_mode_parsing_unknown_defaults_to_all() {
        let mode = match "bogus-mode" {
            "all" => ValidationMode::All,
            "cards-config" => ValidationMode::CardsConfig,
            "cards-fs" => ValidationMode::CardsFilesystem,
            "config-fs" => ValidationMode::ConfigFilesystem,
            _ => ValidationMode::All,
        };
        assert_eq!(mode, ValidationMode::All);
    }

    // ===================================================================
    // Clap parsing tests for CLI structs
    // ===================================================================

    #[test]
    fn test_sources_subcommand_scan_defaults() {
        // Verify that Scan parses with default values
        let cmd = SourcesCommands::parse_from(["sources", "scan"]);
        match cmd.command {
            SourcesSubcommand::Scan { output, show_cards } => {
                assert_eq!(output, "table");
                assert!(!show_cards);
            }
            _ => panic!("Expected Scan subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_scan_json_output() {
        let cmd = SourcesCommands::parse_from(["sources", "scan", "--output", "json"]);
        match cmd.command {
            SourcesSubcommand::Scan { output, show_cards } => {
                assert_eq!(output, "json");
                assert!(!show_cards);
            }
            _ => panic!("Expected Scan subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_scan_show_cards() {
        let cmd = SourcesCommands::parse_from(["sources", "scan", "--show-cards"]);
        match cmd.command {
            SourcesSubcommand::Scan { output, show_cards } => {
                assert_eq!(output, "table");
                assert!(show_cards);
            }
            _ => panic!("Expected Scan subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_scan_json_with_show_cards() {
        let cmd =
            SourcesCommands::parse_from(["sources", "scan", "--output", "json", "--show-cards"]);
        match cmd.command {
            SourcesSubcommand::Scan { output, show_cards } => {
                assert_eq!(output, "json");
                assert!(show_cards);
            }
            _ => panic!("Expected Scan subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_validate_defaults() {
        let cmd = SourcesCommands::parse_from(["sources", "validate"]);
        match cmd.command {
            SourcesSubcommand::Validate {
                mode,
                suggest_matches,
                threshold,
                json,
            } => {
                assert_eq!(mode, "all");
                assert!(!suggest_matches);
                assert!((threshold - 0.7).abs() < f32::EPSILON);
                assert!(!json);
            }
            _ => panic!("Expected Validate subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_validate_custom_mode() {
        let cmd = SourcesCommands::parse_from(["sources", "validate", "--mode", "cards-config"]);
        match cmd.command {
            SourcesSubcommand::Validate { mode, .. } => {
                assert_eq!(mode, "cards-config");
            }
            _ => panic!("Expected Validate subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_validate_all_options() {
        let cmd = SourcesCommands::parse_from([
            "sources",
            "validate",
            "--mode",
            "config-fs",
            "--suggest-matches",
            "--threshold",
            "0.85",
            "--json",
        ]);
        match cmd.command {
            SourcesSubcommand::Validate {
                mode,
                suggest_matches,
                threshold,
                json,
            } => {
                assert_eq!(mode, "config-fs");
                assert!(suggest_matches);
                assert!((threshold - 0.85).abs() < f32::EPSILON);
                assert!(json);
            }
            _ => panic!("Expected Validate subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_alias_list_defaults() {
        let cmd = SourcesCommands::parse_from(["sources", "alias", "list"]);
        match cmd.command {
            SourcesSubcommand::Alias(alias_cmds) => match alias_cmds.command {
                AliasSubcommand::List { json } => {
                    assert!(!json);
                }
                _ => panic!("Expected List subcommand"),
            },
            _ => panic!("Expected Alias subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_alias_list_json() {
        let cmd = SourcesCommands::parse_from(["sources", "alias", "list", "--json"]);
        match cmd.command {
            SourcesSubcommand::Alias(alias_cmds) => match alias_cmds.command {
                AliasSubcommand::List { json } => {
                    assert!(json);
                }
                _ => panic!("Expected List subcommand"),
            },
            _ => panic!("Expected Alias subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_alias_add() {
        let cmd = SourcesCommands::parse_from([
            "sources",
            "alias",
            "add",
            "general-open-music-theory",
            "OMT",
        ]);
        match cmd.command {
            SourcesSubcommand::Alias(alias_cmds) => match alias_cmds.command {
                AliasSubcommand::Add { source_id, alias } => {
                    assert_eq!(source_id, "general-open-music-theory");
                    assert_eq!(alias, "OMT");
                }
                _ => panic!("Expected Add subcommand"),
            },
            _ => panic!("Expected Alias subcommand"),
        }
    }

    #[test]
    fn test_sources_subcommand_alias_remove() {
        let cmd = SourcesCommands::parse_from([
            "sources",
            "alias",
            "remove",
            "general-open-music-theory",
            "OMT",
        ]);
        match cmd.command {
            SourcesSubcommand::Alias(alias_cmds) => match alias_cmds.command {
                AliasSubcommand::Remove { source_id, alias } => {
                    assert_eq!(source_id, "general-open-music-theory");
                    assert_eq!(alias, "OMT");
                }
                _ => panic!("Expected Remove subcommand"),
            },
            _ => panic!("Expected Alias subcommand"),
        }
    }

    // ===================================================================
    // handle_scan tests
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_empty_sources() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // No concept cards => empty results, prints "No sources found."
        let result = handle_scan(&config, "table", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_empty_sources() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let result = handle_scan(&config, "json", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_with_sources() {
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

        let result = handle_scan(&config, "table", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_with_show_cards() {
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

        let result = handle_scan(&config, "table", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_with_sources() {
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

        let result = handle_scan(&config, "json", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_with_show_cards() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(
            &temp_dir,
            "harmony",
            "concept-1.md",
            Some("Open Music Theory"),
        )
        .await;

        let result = handle_scan(&config, "json", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_multiple_cards_same_source() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        for i in 0..5 {
            create_concept_card(
                &temp_dir,
                "harmony",
                &format!("concept-{}.md", i),
                Some("Open Music Theory"),
            )
            .await;
        }

        let result = handle_scan(&config, "json", true).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_list tests
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_list_table_no_aliases() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let result = handle_alias_list(&config, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_list_json_no_aliases() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let result = handle_alias_list(&config, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_list_table_with_aliases() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT", "Gotham OMT"])],
        )
        .await;

        let result = handle_alias_list(&config, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_list_json_with_aliases() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT", "Gotham OMT"])],
        )
        .await;

        let result = handle_alias_list(&config, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_list_table_multiple_categories() {
        // This tests the loop over multiple categories. We use general only
        // since test helpers create general sources, but it exercises the
        // iteration logic with multiple file IDs.
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config_with_sources(
            &temp_dir,
            &[
                ("source-a", "[2020] Author - Source A.pdf"),
                ("source-b", "[2021] Author - Source B.pdf"),
            ],
            &[
                ("source-a", &["Alias A1", "Alias A2"]),
                ("source-b", &["Alias B1"]),
            ],
        )
        .await;

        let result = handle_alias_list(&config, false).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias tests (dispatcher)
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_dispatches_list() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let alias_cmds = AliasCommands {
            command: AliasSubcommand::List { json: false },
        };
        let result = handle_alias(&config, alias_cmds).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_add tests
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_new_alias() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[],
        )
        .await;

        // Point config dir for get_config_path
        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-open-music-theory", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        // Verify the alias was written to the config file
        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(
            content.contains("OMT"),
            "Config should contain the added alias"
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_duplicate_alias() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Adding an alias that already exists should succeed (no-op)
        let result = handle_alias_add("general-open-music-theory", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_invalid_source_id() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config(&temp_dir).await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("invalid-source", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_creates_aliases_section() {
        // Test that adding an alias creates the aliases table if missing
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[], // No aliases configured initially
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-source-a", "New Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(content.contains("New Alias"));
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_second_alias_to_existing() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-open-music-theory", "Open MT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(content.contains("OMT"), "Original alias should remain");
        assert!(content.contains("Open MT"), "New alias should be added");
    }

    // ===================================================================
    // handle_alias_remove tests
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_existing() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT", "Open MT"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("general-open-music-theory", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(!content.contains("\"OMT\""), "Removed alias should be gone");
        assert!(content.contains("Open MT"), "Other alias should remain");
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_nonexistent_alias() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Removing an alias that doesn't exist should succeed (prints message)
        let result = handle_alias_remove("general-open-music-theory", "NonExistent").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_last_alias_removes_key() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("open-music-theory", "[2022] Gotham - Open Music Theory.pdf")],
            &[("open-music-theory", &["OMT"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("general-open-music-theory", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        // Verify the config was cleaned up
        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(!content.contains("\"OMT\""), "Removed alias should be gone");
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_invalid_source_id() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config(&temp_dir).await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("invalid-source", "OMT").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_no_aliases_for_category() {
        let temp_dir = TempDir::new().unwrap();
        // Create config without any aliases section populated
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[], // No aliases at all
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Trying to remove from a source with no aliases configured
        let result = handle_alias_remove("general-source-a", "SomeAlias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Should succeed gracefully (prints "No aliases configured")
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_sources_command tests (integration)
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_sources_command_scan() {
        let temp_dir = TempDir::new().unwrap();

        // Set up the config so Config::load() works
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]
[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]
[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

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
            temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cmds = SourcesCommands {
            command: SourcesSubcommand::Scan {
                output: "table".to_string(),
                show_cards: false,
            },
        };

        let result = handle_sources_command(cmds, None).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_sources_command_alias_list() {
        let temp_dir = TempDir::new().unwrap();

        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]
[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]
[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

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
            temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cmds = SourcesCommands {
            command: SourcesSubcommand::Alias(AliasCommands {
                command: AliasSubcommand::List { json: true },
            }),
        };

        let result = handle_sources_command(cmds, None).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias dispatcher tests for Add and Remove variants
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_dispatches_add() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let alias_cmds = AliasCommands {
            command: AliasSubcommand::Add {
                source_id: "general-source-a".to_string(),
                alias: "Test Alias".to_string(),
            },
        };
        let result = handle_alias(&config, alias_cmds).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_dispatches_remove() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[("source-a", &["Existing Alias"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let alias_cmds = AliasCommands {
            command: AliasSubcommand::Remove {
                source_id: "general-source-a".to_string(),
                alias: "Existing Alias".to_string(),
            },
        };
        let result = handle_alias(&config, alias_cmds).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_remove: missing aliases table in TOML
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_no_aliases_table_in_toml() {
        // Create a config where the general section has no aliases table at all
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Intentionally omit [sources.general.aliases] from the TOML
        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]

[sources.general]
path = ""
[sources.general.files]
source-a = "[2020] Author - Source A.pdf"

[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // This should hit the "No aliases configured for category" early return
        let result = handle_alias_remove("general-source-a", "SomeAlias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_add: missing sources section error path
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_missing_category_in_toml() {
        // Create a config where the TOML file has no [sources.oxford] section
        // but we try to add an alias for an oxford source
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Intentionally use a minimal config that only has general
        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Trying to add an alias for oxford which doesn't exist in TOML
        let result = handle_alias_add("oxford-some-source", "My Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("oxford"),
            "Error should mention the missing category: {}",
            err_msg
        );
    }

    // ===================================================================
    // handle_alias_remove: missing category in TOML
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_missing_category_in_toml() {
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("oxford-some-source", "My Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("oxford"),
            "Error should mention the missing category: {}",
            err_msg
        );
    }

    // ===================================================================
    // SourcesCommands Debug derive tests
    // ===================================================================

    #[test]
    fn test_sources_commands_debug() {
        let cmd = SourcesCommands::parse_from(["sources", "scan"]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Scan"));
    }

    #[test]
    fn test_alias_commands_debug() {
        let cmd = SourcesCommands::parse_from(["sources", "alias", "list"]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("List"));
    }

    #[test]
    fn test_alias_subcommand_add_debug() {
        let cmd = SourcesCommands::parse_from(["sources", "alias", "add", "general-x", "y"]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Add"));
    }

    #[test]
    fn test_alias_subcommand_remove_debug() {
        let cmd = SourcesCommands::parse_from(["sources", "alias", "remove", "general-x", "y"]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Remove"));
    }

    #[test]
    fn test_validate_subcommand_debug() {
        let cmd = SourcesCommands::parse_from(["sources", "validate"]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Validate"));
    }

    // Note: handle_validate cannot be tested directly because it calls
    // std::process::exit() which terminates the test process. The validation
    // logic itself is thoroughly tested in sources/validator.rs. The mode
    // parsing and JSON/table formatting are covered by the mode parsing tests
    // and by the fact that handle_validate delegates to validate_sources().

    // ===================================================================
    // handle_alias_add: missing [sources] section in TOML
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_missing_sources_section_in_toml() {
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Config with no [sources] section at all
        let config_content = r#"
[server]
name = "test"
version = "0.1.0"
"#;

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-some-source", "Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("[sources]"),
            "Error should mention missing [sources] section: {}",
            err_msg
        );
    }

    // ===================================================================
    // handle_alias_add: TOML parse error
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_invalid_toml_returns_error() {
        let temp_dir = TempDir::new().unwrap();

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        // Write invalid TOML content
        fs::write(
            config_dir.join("default.toml"),
            "this is not [[ valid toml {{",
        )
        .await
        .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-some-source", "Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
    }

    // ===================================================================
    // handle_alias_remove: missing [sources] section in TOML
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_missing_sources_section_in_toml() {
        let temp_dir = TempDir::new().unwrap();

        // Config with no [sources] section at all
        let config_content = r#"
[server]
name = "test"
version = "0.1.0"
"#;

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("general-some-source", "Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("[sources]"),
            "Error should mention missing [sources] section: {}",
            err_msg
        );
    }

    // ===================================================================
    // handle_alias_remove: TOML parse error
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_invalid_toml_returns_error() {
        let temp_dir = TempDir::new().unwrap();

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(
            config_dir.join("default.toml"),
            "this is not [[ valid toml {{",
        )
        .await
        .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_remove("general-some-source", "Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
    }

    // ===================================================================
    // handle_alias_remove: aliases table exists but no entry for file_id
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_no_aliases_for_specific_file_id() {
        let temp_dir = TempDir::new().unwrap();
        // Create config with aliases for a different source, not the one we try to remove from
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[
                ("source-a", "[2020] Author - Source A.pdf"),
                ("source-b", "[2021] Author - Source B.pdf"),
            ],
            &[("source-a", &["Alias A"])], // Only source-a has aliases
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Try to remove from source-b which has no aliases array
        let result = handle_alias_remove("general-source-b", "SomeAlias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Should succeed gracefully (prints "No aliases configured for ...")
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_remove: missing category in TOML (remove path)
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_remove_missing_category_section() {
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Config with [sources] but without [sources.papers]
        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.general]
path = ""
[sources.general.files]
[sources.general.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Try to remove from papers category which doesn't exist in TOML
        let result = handle_alias_remove("papers-some-source", "Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("papers"),
            "Error should mention missing category: {}",
            err_msg
        );
    }

    // ===================================================================
    // handle_scan: cards with no source field
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_with_no_source_cards() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // Create cards without source field - they should be scanned but
        // not contribute to sources count
        create_concept_card(&temp_dir, "harmony", "no-source.md", None).await;
        create_concept_card(
            &temp_dir,
            "harmony",
            "with-source.md",
            Some("Open Music Theory"),
        )
        .await;

        let result = handle_scan(&config, "table", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_with_no_source_cards() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(&temp_dir, "harmony", "no-source.md", None).await;

        let result = handle_scan(&config, "json", false).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_scan: long source title exercises truncate_string in context
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_long_source_title_truncated() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        let long_title =
            "A Very Long Source Title That Exceeds Fifty Characters And Should Be Truncated";
        create_concept_card(&temp_dir, "harmony", "long-title.md", Some(long_title)).await;

        let result = handle_scan(&config, "table", true).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_alias_add: category exists but no aliases table yet (ensures
    // the "create aliases table" branch at line 448 is hit)
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_alias_add_creates_aliases_table_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).await.unwrap();

        // Config with [sources.general] but NO [sources.general.aliases] table
        let config_content = format!(
            r#"
[server]
name = "test"
version = "0.1.0"

[paths]
base = "{base}"
sources_md = "sources-md"
concept_cards = "{cards}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = ""
[sources.oxford.files]
[sources.oxford.aliases]

[sources.general]
path = ""
[sources.general.files]
my-source = "[2020] Author - My Source.pdf"

[sources.papers]
path = ""
[sources.papers.files]
[sources.papers.aliases]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{index}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            base = temp_dir.path().display(),
            cards = concept_cards_path.display(),
            index = temp_dir.path().join("test-index").display(),
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).await.unwrap();
        fs::write(config_dir.join("default.toml"), &config_content)
            .await
            .unwrap();

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_alias_add("general-my-source", "New Alias").await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());

        let content = fs::read_to_string(config_dir.join("default.toml"))
            .await
            .unwrap();
        assert!(
            content.contains("New Alias"),
            "Config should contain the new alias"
        );
    }

    // ===================================================================
    // handle_scan: multiple sources with different card counts for sorting
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_table_sorting_by_card_count() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        // Create cards so sources have different counts
        create_concept_card(&temp_dir, "harmony", "c1.md", Some("Source A")).await;
        create_concept_card(&temp_dir, "harmony", "c2.md", Some("Source B")).await;
        create_concept_card(&temp_dir, "harmony", "c3.md", Some("Source B")).await;
        create_concept_card(&temp_dir, "harmony", "c4.md", Some("Source B")).await;

        // Table output sorts by card count descending
        let result = handle_scan(&config, "table", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_scan_json_sorting_by_card_count() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir).await;

        create_concept_card(&temp_dir, "harmony", "c1.md", Some("Source A")).await;
        create_concept_card(&temp_dir, "harmony", "c2.md", Some("Source B")).await;
        create_concept_card(&temp_dir, "harmony", "c3.md", Some("Source B")).await;

        // JSON output sorts by card count descending
        let result = handle_scan(&config, "json", false).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // handle_sources_command: Alias Add and Remove dispatch paths
    // ===================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_sources_command_alias_add() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cmds = SourcesCommands {
            command: SourcesSubcommand::Alias(AliasCommands {
                command: AliasSubcommand::Add {
                    source_id: "general-source-a".to_string(),
                    alias: "Test Alias".to_string(),
                },
            }),
        };

        let result = handle_sources_command(cmds, None).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_sources_command_alias_remove() {
        let temp_dir = TempDir::new().unwrap();
        let _config = create_test_config_with_sources(
            &temp_dir,
            &[("source-a", "[2020] Author - Source A.pdf")],
            &[("source-a", &["Test Alias"])],
        )
        .await;

        let config_dir = temp_dir.path().join("config");
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cmds = SourcesCommands {
            command: SourcesSubcommand::Alias(AliasCommands {
                command: AliasSubcommand::Remove {
                    source_id: "general-source-a".to_string(),
                    alias: "Test Alias".to_string(),
                },
            }),
        };

        let result = handle_sources_command(cmds, None).await;
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }
}
