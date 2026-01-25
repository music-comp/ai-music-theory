use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Main configuration structure for the MCP server.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub sources: SourcesConfig,
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
    pub base: String,
    pub sources_md: String,
    pub concept_cards: String,
    pub concepts_unified: String,
    pub guides: String,
    pub skill_docs: String,
}

impl PathsConfig {
    /// Get the base path expanded to an absolute PathBuf.
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

impl Config {
    /// Load configuration from the default location.
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from("config/default.toml");
        Self::load_from_path(&config_path)
    }

    /// Load configuration from a specific path.
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)
            .map_err(|e| Error::parse_error(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
}

/// Expand shell variables and tildes in paths.
fn expand_path(path_str: &str) -> Result<PathBuf> {
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
}
