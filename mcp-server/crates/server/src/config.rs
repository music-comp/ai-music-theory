use confyg::{conf, Confygery};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Main configuration structure for the MCP server.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub sources: SourcesConfig,
    // Allow unused - will be used when dynamic log level configuration is implemented
    #[allow(dead_code)]
    pub logging: LoggingConfig,
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
}

/// Paths configuration with variable expansion.
#[derive(Debug, Clone, Deserialize)]
pub struct PathsConfig {
    // Used by serde deserialization and accessed via base_path()
    #[allow(dead_code)]
    pub base: String,
    pub sources_md: String,
    pub concept_cards: String,
    // Unified concepts directory - future feature for merged concept storage
    #[allow(dead_code)]
    pub concepts_unified: String,
    pub guides: String,
    pub skill_docs: String,
}

impl PathsConfig {
    /// Get the base path expanded to an absolute PathBuf.
    /// Used by skill_docs_path() and available for future path resolution
    #[allow(dead_code)]
    pub fn base_path(&self) -> Result<PathBuf> {
        expand_path(&self.base)
    }

    /// Get the sources markdown directory as an absolute PathBuf.
    pub fn sources_md_path(&self) -> Result<PathBuf> {
        expand_path(&self.sources_md)
    }

    /// Get the concept cards directory as an absolute PathBuf.
    pub fn concept_cards_path(&self) -> Result<PathBuf> {
        expand_path(&self.concept_cards)
    }

    /// Get the guides directory as an absolute PathBuf.
    pub fn guides_path(&self) -> Result<PathBuf> {
        expand_path(&self.guides)
    }

    /// Get the skill docs directory as an absolute PathBuf.
    pub fn skill_docs_path(&self) -> Result<PathBuf> {
        expand_path(&self.skill_docs)
    }
}

/// Source file locations configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    #[serde(default)]
    pub oxford: SourceCategory,
    #[serde(default)]
    pub general: SourceCategory,
    #[serde(default)]
    pub papers: SourceCategory,
}

/// A category of source files.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SourceCategory {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub files: HashMap<String, String>,
}

impl SourceCategory {
    /// Get the full path to a specific file in this category.
    pub fn file_path(&self, file_id: &str) -> Result<PathBuf> {
        let filename = self
            .files
            .get(file_id)
            .ok_or_else(|| Error::config(format!("File ID '{}' not found", file_id)))?;

        let base = expand_path(&self.path)?;
        Ok(base.join(filename))
    }
}

/// Logging configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl LoggingConfig {
    /// Convert logging configuration to twyg options.
    pub fn to_twyg(&self) -> Result<twyg::Opts> {
        use twyg::{Color, ColorAttribute, Colors, LogLevel, OptsBuilder, Output, TSFormat};

        // Parse level string into LogLevel enum
        let level: LogLevel = self
            .level
            .parse()
            .map_err(|_| Error::config(format!("Invalid log level: {}", self.level)))?;

        // Configure custom colors for twyg
        let colors = Colors {
            timestamp: Some(Color::fg(ColorAttribute::HiBlack)),
            ..Default::default()
        };

        // Build opts using OptsBuilder (MCP requires stderr output)
        OptsBuilder::new()
            .coloured(true)
            .output(Output::Stderr)
            .level(level)
            .timestamp_format(TSFormat::Simple)
            .colors(colors)
            .build()
            .map_err(|e| Error::config(format!("Failed to build twyg opts: {}", e)))
    }
}

impl Config {
    /// Load configuration from the default location.
    /// Searches for config directory using binary-relative path resolution.
    pub fn load() -> Result<Self> {
        use crate::util::paths;

        let config_dir = paths::config_dir()
            .ok_or_else(|| Error::config(
                format!("Could not locate config directory.\n{}", paths::debug_paths())
            ))?;

        let mut opts = conf::Options::default();
        opts.add_path(config_dir.to_string_lossy().as_ref());

        let config: Config = Confygery::new()
            .map_err(|e| Error::config(format!("Failed to initialize confyg: {}", e)))?
            .with_opts(opts)
            .map_err(|e| Error::config(format!("Failed to set options: {}", e)))?
            .add_file("default.toml")
            .map_err(|e| Error::config(format!("Failed to add config file: {}", e)))?
            .build()
            .map_err(|e| Error::config(format!("Failed to build config: {}", e)))?;

        Ok(config)
    }
}

/// Expand shell variables and tildes in paths.
fn expand_path(path_str: &str) -> Result<PathBuf> {
    // First expand environment variables via shellexpand
    let expanded = shellexpand::full(path_str)
        .map_err(|e| Error::invalid_path(PathBuf::from(path_str), e.to_string()))?;

    Ok(PathBuf::from(expanded.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_path_with_tilde() {
        let result = expand_path("~/test");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(!path.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_expand_path_absolute() {
        let result = expand_path("/absolute/path");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_logging_config_to_twyg_valid() {
        let config = LoggingConfig {
            level: "debug".to_string(),
        };
        let result = config.to_twyg();
        assert!(result.is_ok(), "Should parse valid log level 'debug'");
    }

    #[test]
    fn test_logging_config_to_twyg_invalid() {
        let config = LoggingConfig {
            level: "invalid".to_string(),
        };
        let result = config.to_twyg();
        assert!(result.is_err(), "Should reject invalid log level");
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid log level"));
    }

    #[test]
    fn test_logging_config_to_twyg_all_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error"];
        for level in levels {
            let config = LoggingConfig {
                level: level.to_string(),
            };
            let result = config.to_twyg();
            assert!(result.is_ok(), "Should parse valid log level '{}'", level);
        }
    }
}
