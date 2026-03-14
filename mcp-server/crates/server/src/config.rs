use confyg::{conf, Confygery};
use fabryk::core::{ConfigManager, ConfigProvider};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Main configuration structure for the MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub sources: SourcesConfig,
    // Allow unused - will be used when dynamic log level configuration is implemented
    #[allow(dead_code)]
    pub logging: twyg::Opts,
    // Allow unused - will be used when search backends are implemented (Phase 2+)
    #[allow(dead_code)]
    pub search: SearchConfig,
}

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
}

/// Paths configuration with variable expansion.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

    /// Get the concepts unified directory as an absolute PathBuf (v0.3.0).
    #[cfg(feature = "fts")]
    pub fn concepts_unified_path(&self) -> Result<PathBuf> {
        expand_path(&self.concepts_unified)
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourcesConfig {
    #[serde(default)]
    pub oxford: SourceCategory,
    #[serde(default)]
    pub general: SourceCategory,
    #[serde(default)]
    pub papers: SourceCategory,
}

/// A category of source files.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceCategory {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub files: HashMap<String, String>,
    /// Aliases for source titles (file_id -> alternate titles).
    /// Used to match concept card references that differ from extracted filenames.
    #[serde(default)]
    pub aliases: HashMap<String, Vec<String>>,
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

/// Query matching mode for multi-word queries.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMode {
    /// Match ANY term (OR logic) - original behavior
    Or,
    /// Match ALL terms (AND logic) - strict matching
    And,
    /// Match at least N% of terms - flexible matching
    MinimumMatch(f32),
    /// Smart tiered: 2 words = AND, 3+ = OR with 60% minimum
    Smart,
}

/// Search configuration.
/// Fields will be used when search backends are implemented (Phase 2+).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

    /// Query mode: how to match multi-word queries (smart, and, or, minimum_match)
    #[serde(default = "default_query_mode")]
    pub query_mode: QueryMode,

    /// Minimum match percentage for OR queries with 3+ terms (0.0-1.0)
    #[serde(default = "default_minimum_match")]
    pub minimum_match_percent: f32,

    /// Enable stopword filtering for natural language queries
    #[serde(default = "default_enable_stopwords")]
    pub enable_stopwords: bool,

    /// Custom stopwords (in addition to English defaults)
    #[serde(default)]
    pub custom_stopwords: Vec<String>,

    /// Domain-specific terms to preserve (not filtered as stopwords)
    #[serde(default = "default_stopword_allowlist")]
    pub stopword_allowlist: Vec<String>,

    // Field-specific relevance boosting (v0.3.0)
    /// Title field boost multiplier
    #[serde(default = "default_field_boost_title")]
    pub field_boost_title: f32,

    /// Description field boost multiplier
    #[serde(default = "default_field_boost_description")]
    pub field_boost_description: f32,

    /// Content field boost multiplier
    #[serde(default = "default_field_boost_content")]
    pub field_boost_content: f32,
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

fn default_query_mode() -> QueryMode {
    QueryMode::Smart
}

fn default_minimum_match() -> f32 {
    0.6 // 60% of terms must match for OR queries with 3+ terms
}

fn default_enable_stopwords() -> bool {
    true
}

fn default_stopword_allowlist() -> Vec<String> {
    vec![
        // Music theory Roman numerals and solfège syllables
        "I", "V", "ii", "IV", "vi", "vii", "i", "v", "iv", "do", "re", "mi", "fa", "sol", "la",
        "ti",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn default_field_boost_title() -> f32 {
    3.0
}

fn default_field_boost_description() -> f32 {
    2.0
}

fn default_field_boost_content() -> f32 {
    1.0
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            index_path: default_index_path(),
            rebuild_on_startup: false,
            snippet_size: default_snippet_size(),
            fuzzy_search: false,
            fuzzy_distance: default_fuzzy_distance(),
            query_mode: default_query_mode(),
            minimum_match_percent: default_minimum_match(),
            enable_stopwords: default_enable_stopwords(),
            custom_stopwords: vec![],
            stopword_allowlist: default_stopword_allowlist(),
            field_boost_title: default_field_boost_title(),
            field_boost_description: default_field_boost_description(),
            field_boost_content: default_field_boost_content(),
        }
    }
}

impl SearchConfig {
    /// Get the index path as an absolute PathBuf.
    /// Will be used when search backends are implemented (Phase 2+).
    #[allow(dead_code)]
    pub fn index_path(&self) -> Result<PathBuf> {
        expand_path(&self.index_path)
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

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            paths: PathsConfig::default(),
            sources: SourcesConfig::default(),
            logging: twyg::Opts::default(),
            search: SearchConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "music-theory-skill".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            base: ".".to_string(),
            sources_md: "sources-md".to_string(),
            concept_cards: "concept-cards".to_string(),
            concepts_unified: "concepts-unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: ".".to_string(),
        }
    }
}

// ============================================================================
// Fabryk ConfigProvider implementation
// ============================================================================

impl ConfigProvider for Config {
    fn project_name(&self) -> &str {
        &self.server.name
    }

    fn base_path(&self) -> fabryk::core::Result<PathBuf> {
        expand_path(&self.paths.base).map_err(|e| fabryk::core::Error::config(e.to_string()))
    }

    fn content_path(&self, content_type: &str) -> fabryk::core::Result<PathBuf> {
        let result = match content_type {
            "concept_cards" => expand_path(&self.paths.concept_cards),
            "sources_md" => expand_path(&self.paths.sources_md),
            "concepts_unified" => expand_path(&self.paths.concepts_unified),
            "guides" => expand_path(&self.paths.guides),
            "skill_docs" => expand_path(&self.paths.skill_docs),
            _ => Err(Error::config(format!(
                "Unknown content type: {}",
                content_type
            ))),
        };
        result.map_err(|e| fabryk::core::Error::config(e.to_string()))
    }

    fn cache_path(&self, cache_type: &str) -> fabryk::core::Result<PathBuf> {
        let base = self.base_path()?;
        match cache_type {
            "fts" => {
                // Use configured index path if set, otherwise default
                expand_path(&self.search.index_path)
                    .map_err(|e| fabryk::core::Error::config(e.to_string()))
            }
            "graph" => Ok(base.join("data").join("graphs")),
            "vector" => Ok(base.join(".cache").join("vector")),
            _ => Ok(base.join(".cache").join(cache_type)),
        }
    }
}

// ============================================================================
// Fabryk ConfigManager implementation
// ============================================================================

impl ConfigManager for Config {
    fn load(config_path: Option<&str>) -> fabryk::core::Result<Self> {
        let resolved = config_path
            .map(PathBuf::from)
            .or_else(|| Self::resolve_config_path(None));

        if let Some(path) = resolved {
            let path_str = path.to_string_lossy().to_string();
            let mut opts = conf::Options::default();
            opts.add_path(&path_str);

            let config: Config = Confygery::new()
                .map_err(|e| fabryk::core::Error::config(format!("config init: {e}")))?
                .with_opts(opts)
                .map_err(|e| fabryk::core::Error::config(format!("config opts: {e}")))?
                .add_file("default.toml")
                .map_err(|e| fabryk::core::Error::config(format!("config file: {e}")))?
                .build()
                .map_err(|e| fabryk::core::Error::config(format!("config build: {e}")))?;

            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    fn resolve_config_path(explicit: Option<&str>) -> Option<PathBuf> {
        if let Some(path) = explicit {
            return Some(PathBuf::from(path));
        }

        // Check env var
        if let Ok(path) = std::env::var("MUSIC_THEORY_CONFIG_DIR") {
            let expanded = crate::util::paths::expand_tilde(&path);
            if expanded.join("default.toml").exists() {
                return Some(expanded);
            }
        }

        // Fall back to util::paths resolution
        crate::util::paths::config_dir()
    }

    fn default_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("music-theory-skill").join("config.toml"))
    }

    fn project_name() -> &'static str {
        "music-theory-skill"
    }

    fn to_toml_string(&self) -> fabryk::core::Result<String> {
        toml_edit::ser::to_string_pretty(self)
            .map_err(|e| fabryk::core::Error::config(e.to_string()))
    }

    fn to_env_vars(&self) -> fabryk::core::Result<Vec<(String, String)>> {
        let value: toml_edit::DocumentMut = self
            .to_toml_string()?
            .parse()
            .map_err(|e: toml_edit::TomlError| fabryk::core::Error::config(e.to_string()))?;
        let mut vars = Vec::new();
        flatten_toml_table(value.as_table(), "MUSIC_THEORY", &mut vars);
        Ok(vars)
    }
}

/// Recursively flatten a TOML table into `KEY=value` pairs.
fn flatten_toml_table(
    table: &toml_edit::Table,
    prefix: &str,
    out: &mut Vec<(String, String)>,
) {
    for (key, item) in table.iter() {
        let env_key = format!("{}_{}", prefix, key.to_uppercase());
        match item {
            toml_edit::Item::Table(t) => flatten_toml_table(t, &env_key, out),
            toml_edit::Item::Value(v) => {
                let s = match v {
                    toml_edit::Value::String(s) => s.value().to_string(),
                    other => other.to_string(),
                };
                out.push((env_key, s));
            }
            toml_edit::Item::ArrayOfTables(arr) => {
                out.push((env_key, format!("{arr}")));
            }
            toml_edit::Item::None => {}
        }
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
            aliases: HashMap::new(),
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
            aliases: HashMap::new(),
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
        assert!(category.aliases.is_empty());
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
            query_mode: QueryMode::Smart,
            minimum_match_percent: 0.6,
            enable_stopwords: true,
            custom_stopwords: vec![],
            stopword_allowlist: vec![],
            field_boost_title: 3.0,
            field_boost_description: 2.0,
            field_boost_content: 1.0,
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
            query_mode: QueryMode::Smart,
            minimum_match_percent: 0.6,
            enable_stopwords: true,
            custom_stopwords: vec![],
            stopword_allowlist: vec![],
            field_boost_title: 3.0,
            field_boost_description: 2.0,
            field_boost_content: 1.0,
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
