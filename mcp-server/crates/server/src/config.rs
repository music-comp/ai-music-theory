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
    // Allow unused - will be used when search backends are implemented (Phase 2+)
    #[allow(dead_code)]
    pub search: SearchConfig,
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
#[derive(Debug, Clone, Deserialize, Default)]
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

/// Search configuration.
/// Fields will be used when search backends are implemented (Phase 2+).
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct SearchConfig {
    /// Backend selection: "simple" or "tantivy"
    #[serde(default = "default_backend")]
    pub backend: String,

    /// Tantivy index directory (relative to skill root or absolute)
    #[serde(default = "default_index_path")]
    pub index_path: String,

    /// Rebuild index on startup (useful for development)
    #[serde(default)]
    pub rebuild_on_startup: bool,

    /// Snippet size in characters for search results
    #[serde(default = "default_snippet_size")]
    pub snippet_size: usize,

    /// Enable fuzzy search (typo tolerance)
    #[serde(default)]
    pub fuzzy_search: bool,

    /// Maximum edit distance for fuzzy matching (1-2)
    #[serde(default = "default_fuzzy_distance")]
    pub fuzzy_distance: u8,
}

fn default_backend() -> String {
    "simple".to_string()
}

fn default_index_path() -> String {
    ".tantivy-index".to_string()
}

fn default_snippet_size() -> usize {
    200
}

fn default_fuzzy_distance() -> u8 {
    2
}

impl SearchConfig {
    /// Get the index path as an absolute PathBuf.
    /// Will be used when search backends are implemented (Phase 2+).
    #[allow(dead_code)]
    pub fn index_path(&self) -> Result<PathBuf> {
        expand_path(&self.index_path)
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

        let config_dir = paths::config_dir().ok_or_else(|| {
            Error::config(format!(
                "Could not locate config directory.\n{}",
                paths::debug_paths()
            ))
        })?;

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
/// Relative paths are resolved against the skill root directory.
fn expand_path(path_str: &str) -> Result<PathBuf> {
    use crate::util::paths;

    // First expand environment variables and tilde via shellexpand
    let expanded = shellexpand::full(path_str)
        .map_err(|e| Error::invalid_path(PathBuf::from(path_str), e.to_string()))?;

    let path = PathBuf::from(expanded.as_ref());

    // If the path is relative, resolve it against skill_root()
    if path.is_relative() {
        let skill_root = paths::skill_root().ok_or_else(|| {
            Error::config(format!(
                "Cannot resolve relative path '{}': skill root not found.\n{}",
                path_str,
                paths::debug_paths()
            ))
        })?;
        Ok(skill_root.join(path))
    } else {
        Ok(path)
    }
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

    #[test]
    fn test_expand_path_relative() {
        // Relative paths should be resolved against skill_root
        let result = expand_path("test/path");
        // Should either succeed with an absolute path or fail with skill root error
        match result {
            Ok(path) => assert!(
                path.is_absolute(),
                "Relative path should be resolved to absolute"
            ),
            Err(e) => assert!(
                e.to_string().contains("skill root") || e.to_string().contains("Cannot resolve")
            ),
        }
    }

    #[test]
    fn test_expand_path_dot() {
        // "." should be resolved against skill_root
        let result = expand_path(".");
        match result {
            Ok(path) => assert!(
                path.is_absolute(),
                "Dot path should be resolved to absolute"
            ),
            Err(e) => assert!(
                e.to_string().contains("skill root") || e.to_string().contains("Cannot resolve")
            ),
        }
    }

    #[test]
    fn test_source_category_file_path_success() {
        let mut files = HashMap::new();
        files.insert("test-file".to_string(), "test.pdf".to_string());

        let category = SourceCategory {
            path: "/test/path".to_string(),
            files,
        };

        let result = category.file_path("test-file");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("test.pdf"));
    }

    #[test]
    fn test_source_category_file_path_not_found() {
        let category = SourceCategory {
            path: "/test/path".to_string(),
            files: HashMap::new(),
        };

        let result = category.file_path("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("File ID 'nonexistent' not found"));
    }

    #[test]
    fn test_source_category_default() {
        let category = SourceCategory::default();
        assert_eq!(category.path, "");
        assert!(category.files.is_empty());
    }

    #[test]
    fn test_paths_config_base_path() {
        let config = PathsConfig {
            base: "/test/base".to_string(),
            sources_md: "sources".to_string(),
            concept_cards: "concepts".to_string(),
            concepts_unified: "unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: "docs".to_string(),
        };

        let result = config.base_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/test/base"));
    }

    #[test]
    fn test_paths_config_sources_md_path() {
        let config = PathsConfig {
            base: ".".to_string(),
            sources_md: "/absolute/sources".to_string(),
            concept_cards: "concepts".to_string(),
            concepts_unified: "unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: "docs".to_string(),
        };

        let result = config.sources_md_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/sources"));
    }

    #[test]
    fn test_paths_config_concept_cards_path() {
        let config = PathsConfig {
            base: ".".to_string(),
            sources_md: "sources".to_string(),
            concept_cards: "/absolute/concepts".to_string(),
            concepts_unified: "unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: "docs".to_string(),
        };

        let result = config.concept_cards_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/concepts"));
    }

    #[test]
    fn test_paths_config_guides_path() {
        let config = PathsConfig {
            base: ".".to_string(),
            sources_md: "sources".to_string(),
            concept_cards: "concepts".to_string(),
            concepts_unified: "unified".to_string(),
            guides: "/absolute/guides".to_string(),
            skill_docs: "docs".to_string(),
        };

        let result = config.guides_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/guides"));
    }

    #[test]
    fn test_paths_config_skill_docs_path() {
        let config = PathsConfig {
            base: ".".to_string(),
            sources_md: "sources".to_string(),
            concept_cards: "concepts".to_string(),
            concepts_unified: "unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: "/absolute/docs".to_string(),
        };

        let result = config.skill_docs_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/docs"));
    }

    #[test]
    fn test_default_backend() {
        assert_eq!(default_backend(), "simple");
    }

    #[test]
    fn test_default_index_path() {
        assert_eq!(default_index_path(), ".tantivy-index");
    }

    #[test]
    fn test_default_snippet_size() {
        assert_eq!(default_snippet_size(), 200);
    }

    #[test]
    fn test_default_fuzzy_distance() {
        assert_eq!(default_fuzzy_distance(), 2);
    }

    #[test]
    fn test_search_config_defaults() {
        let json = r#"{}"#;
        let config: SearchConfig = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(config.backend, "simple");
        assert_eq!(config.index_path, ".tantivy-index");
        assert!(!config.rebuild_on_startup);
        assert_eq!(config.snippet_size, 200);
        assert!(!config.fuzzy_search);
        assert_eq!(config.fuzzy_distance, 2);
    }

    #[test]
    fn test_search_config_custom_values() {
        let json = r#"{
            "backend": "tantivy",
            "index_path": "/custom/path",
            "rebuild_on_startup": true,
            "snippet_size": 150,
            "fuzzy_search": true,
            "fuzzy_distance": 1
        }"#;
        let config: SearchConfig = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(config.backend, "tantivy");
        assert_eq!(config.index_path, "/custom/path");
        assert!(config.rebuild_on_startup);
        assert_eq!(config.snippet_size, 150);
        assert!(config.fuzzy_search);
        assert_eq!(config.fuzzy_distance, 1);
    }

    #[test]
    fn test_search_config_index_path_absolute() {
        let config = SearchConfig {
            backend: "tantivy".to_string(),
            index_path: "/absolute/index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: false,
            fuzzy_distance: 2,
        };

        let result = config.index_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/index"));
    }

    #[test]
    fn test_search_config_index_path_relative() {
        let config = SearchConfig {
            backend: "tantivy".to_string(),
            index_path: "relative/index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: false,
            fuzzy_distance: 2,
        };

        let result = config.index_path();
        // Should either resolve to absolute or error with skill root message
        match result {
            Ok(path) => assert!(
                path.is_absolute(),
                "Relative path should resolve to absolute"
            ),
            Err(e) => assert!(
                e.to_string().contains("skill root") || e.to_string().contains("Cannot resolve")
            ),
        }
    }
}
