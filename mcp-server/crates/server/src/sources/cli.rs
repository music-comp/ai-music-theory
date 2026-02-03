//! CLI handlers for source management commands.
//!
//! This module implements the handlers for:
//! - `sources scan` - Scan concept cards for source references
//! - `sources validate` - Validate sources across cards, config, filesystem
//! - `sources alias` - Manage source title aliases

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::error::{Error, Result};
use crate::sources::scanner::scan_concept_cards_with_stats;
use crate::sources::validator::{validate_sources, ValidationMode};
use crate::util::paths::config_dir;

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
                    obj["suggestions"] = serde_json::json!(
                        m.suggestions.iter().map(|s| {
                            serde_json::json!({
                                "config_id": s.config_id,
                                "title": s.title,
                                "similarity": s.similarity,
                            })
                        }).collect::<Vec<_>>()
                    );
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
        println!("  Sources resolved:       {}", report.stats.sources_resolved);
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
                println!(
                    "  \"{}\" ({} cards)",
                    missing.title, missing.card_count
                );
                if !missing.sample_card_ids.is_empty() {
                    println!(
                        "    Sample cards: {}",
                        missing.sample_card_ids.join(", ")
                    );
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
        AliasSubcommand::Add { source_id, alias } => {
            handle_alias_add(&source_id, &alias).await
        }
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
        .ok_or_else(|| Error::config(format!("Missing [sources.{}] section in config", category)))?;

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
        .ok_or_else(|| Error::config(format!("Missing [sources.{}] section in config", category)))?;

    let aliases_table = match category_table.get_mut("aliases").and_then(|a| a.as_table_mut()) {
        Some(t) => t,
        None => {
            println!("No aliases configured for category '{}'", category);
            return Ok(());
        }
    };

    let alias_array = match aliases_table.get_mut(file_id).and_then(|a| a.as_array_mut()) {
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
    config_dir()
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
    fn test_validation_mode_parsing() {
        // Test that mode strings are parsed correctly
        let modes = [
            ("all", ValidationMode::All),
            ("cards-config", ValidationMode::CardsConfig),
            ("cards-fs", ValidationMode::CardsFilesystem),
            ("config-fs", ValidationMode::ConfigFilesystem),
        ];

        for (mode_str, expected) in modes {
            let mode = match mode_str {
                "all" => ValidationMode::All,
                "cards-config" => ValidationMode::CardsConfig,
                "cards-fs" => ValidationMode::CardsFilesystem,
                "config-fs" => ValidationMode::ConfigFilesystem,
                _ => ValidationMode::All,
            };
            assert_eq!(mode, expected);
        }
    }

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
}
