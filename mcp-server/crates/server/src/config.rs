use confyg::{conf, Confygery};
use fabryk::core::{ConfigManager, ConfigProvider};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Main configuration structure for the MCP server.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub sources: SourcesConfig,
    pub logging: twyg::Opts,
    pub search: SearchConfig,
    pub lancedb: LanceDbConfig,
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
    pub base: String,
    pub sources_md: String,
    pub concept_cards: String,
    pub concepts_unified: String,
    pub guides: String,
    pub skill_docs: String,
}

impl PathsConfig {
    /// Get the base path expanded to an absolute PathBuf.
    /// Used by skill_docs_path() and available for future path resolution.
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
    pub fn index_path(&self) -> Result<PathBuf> {
        expand_path(&self.index_path)
    }
}

/// LanceDB vector search configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LanceDbConfig {
    /// Embedding model name for vector search.
    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,

    /// Path to fastembed model cache directory.
    /// If None, fastembed uses its default (`.fastembed_cache` relative to CWD).
    /// Set to an absolute path for reliable operation when CWD varies.
    #[serde(default)]
    pub embedding_cache_dir: Option<String>,

    /// Enable vector search.
    /// Set to false to disable even when content directories exist.
    #[serde(default = "default_lancedb_enabled")]
    pub enabled: bool,
}

fn default_embedding_model() -> String {
    "bge-small-en-v1.5".to_string()
}

fn default_lancedb_enabled() -> bool {
    true
}

impl Default for LanceDbConfig {
    fn default() -> Self {
        Self {
            embedding_model: default_embedding_model(),
            embedding_cache_dir: None,
            enabled: default_lancedb_enabled(),
        }
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
        expand_path(&self.paths.base)
    }

    fn content_path(&self, content_type: &str) -> fabryk::core::Result<PathBuf> {
        match content_type {
            "concept_cards" => expand_path(&self.paths.concept_cards),
            "sources_md" => expand_path(&self.paths.sources_md),
            "concepts_unified" => expand_path(&self.paths.concepts_unified),
            "guides" => expand_path(&self.paths.guides),
            "skill_docs" => expand_path(&self.paths.skill_docs),
            _ => Err(Error::config(format!(
                "Unknown content type: {}",
                content_type
            ))),
        }
    }

    fn cache_path(&self, cache_type: &str) -> fabryk::core::Result<PathBuf> {
        let base = self.base_path()?;
        match cache_type {
            "fts" => expand_path(&self.search.index_path),
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
                .map_err(|e| Error::config(format!("config init: {e}")))?
                .with_opts(opts)
                .map_err(|e| Error::config(format!("config opts: {e}")))?
                .add_file("default.toml")
                .map_err(|e| Error::config(format!("config file: {e}")))?
                .build()
                .map_err(|e| Error::config(format!("config build: {e}")))?;

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
        toml_edit::ser::to_string_pretty(self).map_err(|e| Error::config(e.to_string()))
    }

    fn to_env_vars(&self) -> fabryk::core::Result<Vec<(String, String)>> {
        let value: toml_edit::DocumentMut = self
            .to_toml_string()?
            .parse()
            .map_err(|e: toml_edit::TomlError| Error::config(e.to_string()))?;
        let mut vars = Vec::new();
        flatten_toml_table(value.as_table(), "MUSIC_THEORY", &mut vars);
        Ok(vars)
    }
}

/// Recursively flatten a TOML table into `KEY=value` pairs.
fn flatten_toml_table(table: &toml_edit::Table, prefix: &str, out: &mut Vec<(String, String)>) {
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

    // ========================================================================
    // Default impl tests
    // ========================================================================

    #[test]
    fn test_config_default_has_expected_sub_configs() {
        let config = Config::default();
        assert_eq!(config.server.name, "music-theory-skill");
        assert!(!config.server.version.is_empty());
        assert_eq!(config.paths.base, ".");
        assert!(config.sources.oxford.path.is_empty());
        assert!(config.sources.general.path.is_empty());
        assert!(config.sources.papers.path.is_empty());
    }

    #[test]
    fn test_server_config_default_values() {
        let config = ServerConfig::default();
        assert_eq!(config.name, "music-theory-skill");
        assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_paths_config_default_values() {
        let config = PathsConfig::default();
        assert_eq!(config.base, ".");
        assert_eq!(config.sources_md, "sources-md");
        assert_eq!(config.concept_cards, "concept-cards");
        assert_eq!(config.concepts_unified, "concepts-unified");
        assert_eq!(config.guides, "guides");
        assert_eq!(config.skill_docs, ".");
    }

    #[test]
    fn test_sources_config_default_empty() {
        let config = SourcesConfig::default();
        assert!(config.oxford.path.is_empty());
        assert!(config.oxford.files.is_empty());
        assert!(config.general.path.is_empty());
        assert!(config.general.files.is_empty());
        assert!(config.papers.path.is_empty());
        assert!(config.papers.files.is_empty());
    }

    #[test]
    fn test_search_config_default_impl_values() {
        let config = SearchConfig::default();
        assert_eq!(config.backend, "simple");
        assert_eq!(config.index_path, ".tantivy-index");
        assert!(!config.rebuild_on_startup);
        assert_eq!(config.snippet_size, 200);
        assert!(!config.fuzzy_search);
        assert_eq!(config.fuzzy_distance, 2);
        assert_eq!(config.query_mode, QueryMode::Smart);
        assert!((config.minimum_match_percent - 0.6).abs() < f32::EPSILON);
        assert!(config.enable_stopwords);
        assert!(config.custom_stopwords.is_empty());
        assert!(!config.stopword_allowlist.is_empty());
        assert!((config.field_boost_title - 3.0).abs() < f32::EPSILON);
        assert!((config.field_boost_description - 2.0).abs() < f32::EPSILON);
        assert!((config.field_boost_content - 1.0).abs() < f32::EPSILON);
    }

    // ========================================================================
    // Default function tests (remaining coverage)
    // ========================================================================

    #[test]
    fn test_default_query_mode_is_smart() {
        assert_eq!(default_query_mode(), QueryMode::Smart);
    }

    #[test]
    fn test_default_minimum_match_is_sixty_percent() {
        assert!((default_minimum_match() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn test_default_enable_stopwords_is_true() {
        assert!(default_enable_stopwords());
    }

    #[test]
    fn test_default_stopword_allowlist_contains_music_terms() {
        let list = default_stopword_allowlist();
        assert!(list.contains(&"I".to_string()));
        assert!(list.contains(&"V".to_string()));
        assert!(list.contains(&"do".to_string()));
        assert!(list.contains(&"re".to_string()));
        assert!(list.contains(&"mi".to_string()));
        assert!(list.contains(&"fa".to_string()));
        assert!(list.contains(&"sol".to_string()));
        assert!(list.contains(&"la".to_string()));
        assert!(list.contains(&"ti".to_string()));
        assert!(list.contains(&"ii".to_string()));
        assert!(list.contains(&"IV".to_string()));
        assert!(list.contains(&"vi".to_string()));
        assert!(list.contains(&"vii".to_string()));
        assert!(list.contains(&"i".to_string()));
        assert!(list.contains(&"v".to_string()));
        assert!(list.contains(&"iv".to_string()));
        assert_eq!(list.len(), 16);
    }

    #[test]
    fn test_default_field_boost_title() {
        assert!((default_field_boost_title() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_default_field_boost_description() {
        assert!((default_field_boost_description() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_default_field_boost_content() {
        assert!((default_field_boost_content() - 1.0).abs() < f32::EPSILON);
    }

    // ========================================================================
    // QueryMode serde tests
    // ========================================================================

    #[test]
    fn test_query_mode_serde_or() {
        let json = r#""or""#;
        let mode: QueryMode = serde_json::from_str(json).expect("Should deserialize Or");
        assert_eq!(mode, QueryMode::Or);
        let serialized = serde_json::to_string(&mode).unwrap();
        assert_eq!(serialized, r#""or""#);
    }

    #[test]
    fn test_query_mode_serde_and() {
        let json = r#""and""#;
        let mode: QueryMode = serde_json::from_str(json).expect("Should deserialize And");
        assert_eq!(mode, QueryMode::And);
    }

    #[test]
    fn test_query_mode_serde_smart() {
        let json = r#""smart""#;
        let mode: QueryMode = serde_json::from_str(json).expect("Should deserialize Smart");
        assert_eq!(mode, QueryMode::Smart);
    }

    #[test]
    fn test_query_mode_serde_minimum_match() {
        let json = r#"{"minimum_match":0.75}"#;
        let mode: QueryMode = serde_json::from_str(json).expect("Should deserialize MinimumMatch");
        assert_eq!(mode, QueryMode::MinimumMatch(0.75));
    }

    #[test]
    fn test_query_mode_debug() {
        assert!(format!("{:?}", QueryMode::Or).contains("Or"));
        assert!(format!("{:?}", QueryMode::And).contains("And"));
        assert!(format!("{:?}", QueryMode::Smart).contains("Smart"));
        assert!(format!("{:?}", QueryMode::MinimumMatch(0.5)).contains("MinimumMatch"));
    }

    #[test]
    fn test_query_mode_clone() {
        let mode = QueryMode::MinimumMatch(0.8);
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    // ========================================================================
    // SearchConfig serde - query mode and stopword fields
    // ========================================================================

    #[test]
    fn test_search_config_serde_with_query_mode_and_stopwords() {
        let json = r#"{
            "query_mode": "and",
            "minimum_match_percent": 0.8,
            "enable_stopwords": false,
            "custom_stopwords": ["the", "a"],
            "stopword_allowlist": ["I", "V"],
            "field_boost_title": 5.0,
            "field_boost_description": 3.0,
            "field_boost_content": 2.0
        }"#;
        let config: SearchConfig = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(config.query_mode, QueryMode::And);
        assert!((config.minimum_match_percent - 0.8).abs() < f32::EPSILON);
        assert!(!config.enable_stopwords);
        assert_eq!(config.custom_stopwords, vec!["the", "a"]);
        assert_eq!(config.stopword_allowlist, vec!["I", "V"]);
        assert!((config.field_boost_title - 5.0).abs() < f32::EPSILON);
        assert!((config.field_boost_description - 3.0).abs() < f32::EPSILON);
        assert!((config.field_boost_content - 2.0).abs() < f32::EPSILON);
    }

    // ========================================================================
    // expand_path error path
    // ========================================================================

    #[test]
    fn test_expand_path_with_invalid_env_var_returns_error() {
        // An undefined env var in shellexpand causes an error
        let result = expand_path("$NONEXISTENT_VAR_THAT_SHOULD_NOT_EXIST_12345/path");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("NONEXISTENT_VAR_THAT_SHOULD_NOT_EXIST_12345") || msg.contains("path"),
            "Error should reference the invalid env var or path, got: {msg}"
        );
    }

    // ========================================================================
    // PathsConfig - concepts_unified_path (fts feature)
    // ========================================================================

    #[cfg(feature = "fts")]
    #[test]
    fn test_paths_config_concepts_unified_path_absolute() {
        let config = PathsConfig {
            base: ".".to_string(),
            sources_md: "sources".to_string(),
            concept_cards: "concepts".to_string(),
            concepts_unified: "/absolute/unified".to_string(),
            guides: "guides".to_string(),
            skill_docs: "docs".to_string(),
        };

        let result = config.concepts_unified_path();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/unified"));
    }

    // ========================================================================
    // ConfigProvider trait tests
    // ========================================================================

    #[test]
    fn test_config_provider_project_name() {
        let config = Config::default();
        let name = ConfigProvider::project_name(&config);
        assert_eq!(name, "music-theory-skill");
    }

    #[test]
    fn test_config_provider_base_path_absolute() {
        let mut config = Config::default();
        config.paths.base = "/test/base".to_string();
        let result = ConfigProvider::base_path(&config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/test/base"));
    }

    #[test]
    fn test_config_provider_content_path_concept_cards() {
        let mut config = Config::default();
        config.paths.concept_cards = "/abs/concepts".to_string();
        let result = config.content_path("concept_cards");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/concepts"));
    }

    #[test]
    fn test_config_provider_content_path_sources_md() {
        let mut config = Config::default();
        config.paths.sources_md = "/abs/sources".to_string();
        let result = config.content_path("sources_md");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/sources"));
    }

    #[test]
    fn test_config_provider_content_path_concepts_unified() {
        let mut config = Config::default();
        config.paths.concepts_unified = "/abs/unified".to_string();
        let result = config.content_path("concepts_unified");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/unified"));
    }

    #[test]
    fn test_config_provider_content_path_guides() {
        let mut config = Config::default();
        config.paths.guides = "/abs/guides".to_string();
        let result = config.content_path("guides");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/guides"));
    }

    #[test]
    fn test_config_provider_content_path_skill_docs() {
        let mut config = Config::default();
        config.paths.skill_docs = "/abs/docs".to_string();
        let result = config.content_path("skill_docs");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/docs"));
    }

    #[test]
    fn test_config_provider_content_path_unknown_type_returns_error() {
        let config = Config::default();
        let result = config.content_path("nonexistent_type");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains("Unknown content type: nonexistent_type"));
    }

    #[test]
    fn test_config_provider_cache_path_fts() {
        let mut config = Config::default();
        config.search.index_path = "/abs/index".to_string();
        let result = config.cache_path("fts");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/index"));
    }

    #[test]
    fn test_config_provider_cache_path_graph() {
        let mut config = Config::default();
        config.paths.base = "/abs/base".to_string();
        let result = config.cache_path("graph");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/base/data/graphs"));
    }

    #[test]
    fn test_config_provider_cache_path_vector() {
        let mut config = Config::default();
        config.paths.base = "/abs/base".to_string();
        let result = config.cache_path("vector");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/abs/base/.cache/vector"));
    }

    #[test]
    fn test_config_provider_cache_path_unknown_type() {
        let mut config = Config::default();
        config.paths.base = "/abs/base".to_string();
        let result = config.cache_path("custom_cache");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            PathBuf::from("/abs/base/.cache/custom_cache")
        );
    }

    // ========================================================================
    // ConfigManager trait tests
    // ========================================================================

    #[test]
    fn test_config_manager_project_name_static() {
        assert_eq!(
            <Config as ConfigManager>::project_name(),
            "music-theory-skill"
        );
    }

    #[test]
    fn test_config_manager_default_config_path_is_some() {
        // dirs::config_dir() returns Some on most systems
        let result = Config::default_config_path();
        if let Some(path) = result {
            assert!(path.to_string_lossy().contains("music-theory-skill"));
            assert!(path.to_string_lossy().contains("config.toml"));
        }
        // If dirs::config_dir() returns None (unlikely on macOS), that is also valid
    }

    #[test]
    fn test_config_manager_resolve_config_path_with_explicit() {
        let result = Config::resolve_config_path(Some("/explicit/path"));
        assert_eq!(result, Some(PathBuf::from("/explicit/path")));
    }

    #[test]
    fn test_config_manager_to_toml_string_roundtrip() {
        let config = Config::default();
        let toml_str = config.to_toml_string();
        assert!(toml_str.is_ok());
        let toml_str = toml_str.unwrap();
        assert!(toml_str.contains("[server]"));
        assert!(toml_str.contains("name"));
        assert!(toml_str.contains("music-theory-skill"));
    }

    #[test]
    fn test_config_manager_to_env_vars_produces_prefixed_keys() {
        let config = Config::default();
        let result = config.to_env_vars();
        assert!(result.is_ok());
        let vars = result.unwrap();
        assert!(!vars.is_empty(), "Should produce at least one env var");
        for (key, _) in &vars {
            assert!(
                key.starts_with("MUSIC_THEORY_"),
                "All keys should be prefixed with MUSIC_THEORY_, got: {key}"
            );
        }
    }

    #[test]
    fn test_config_manager_to_env_vars_contains_server_name() {
        let config = Config::default();
        let vars = config.to_env_vars().unwrap();
        let server_name = vars
            .iter()
            .find(|(k, _)| k.contains("SERVER") && k.contains("NAME"));
        assert!(
            server_name.is_some(),
            "Should contain a server name env var"
        );
        assert_eq!(server_name.unwrap().1, "music-theory-skill");
    }

    #[test]
    fn test_config_manager_load_with_none_path_returns_default_or_resolved() {
        // When no config path is given and resolve_config_path returns None,
        // load should return the default config. In practice it may find a
        // config dir on the system, so we just verify it doesn't panic.
        let result = <Config as ConfigManager>::load(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_manager_load_with_nonexistent_path_returns_error() {
        let result = <Config as ConfigManager>::load(Some("/nonexistent/path/that/does/not/exist"));
        // This should error because default.toml won't be found at that path
        assert!(result.is_err());
    }

    use serial_test::serial;
    use tempfile::TempDir;

    #[test]
    #[serial(config_env)]
    fn test_config_manager_load_with_valid_toml_file() {
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let config_content = r#"
[server]
name = "test-server"
version = "0.0.1"

[paths]
base = "/tmp/test"

[search]
backend = "tantivy"
"#;
        std::fs::write(tmp.path().join("default.toml"), config_content)
            .expect("Failed to write config");

        let path_str = tmp.path().to_string_lossy().to_string();
        let result = <Config as ConfigManager>::load(Some(&path_str));
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.server.name, "test-server");
        assert_eq!(config.server.version, "0.0.1");
        assert_eq!(config.paths.base, "/tmp/test");
        assert_eq!(config.search.backend, "tantivy");
    }

    #[test]
    #[serial(config_env)]
    fn test_config_manager_resolve_config_path_with_env_var() {
        let tmp = TempDir::new().expect("Failed to create temp dir");
        std::fs::write(tmp.path().join("default.toml"), "[server]\nname = \"test\"")
            .expect("Failed to write config");

        std::env::set_var(
            "MUSIC_THEORY_CONFIG_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );

        let result = Config::resolve_config_path(None);
        assert!(result.is_some());
        // The env var path should be returned since it contains default.toml
        let resolved = result.unwrap();
        assert_eq!(resolved, tmp.path());

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");
    }

    #[test]
    #[serial(config_env)]
    fn test_config_manager_resolve_config_path_env_var_without_toml_falls_through() {
        let tmp = TempDir::new().expect("Failed to create temp dir");
        // Do NOT write default.toml - the env var path should be skipped

        std::env::set_var(
            "MUSIC_THEORY_CONFIG_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );

        let result = Config::resolve_config_path(None);
        // Should fall through to util::paths::config_dir() since no default.toml
        // The result depends on the environment, but it should NOT be the tmp path
        if let Some(ref path) = result {
            assert_ne!(path, &tmp.path().to_path_buf());
        }

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");
    }

    // ========================================================================
    // flatten_toml_table tests
    // ========================================================================

    #[test]
    fn test_flatten_toml_table_simple_values() {
        let toml_str = r#"
key1 = "value1"
key2 = 42
key3 = true
key4 = 3.14
"#;
        let doc: toml_edit::DocumentMut = toml_str.parse().unwrap();
        let mut vars = Vec::new();
        flatten_toml_table(doc.as_table(), "PREFIX", &mut vars);

        let map: HashMap<String, String> = vars.into_iter().collect();
        assert_eq!(map.get("PREFIX_KEY1").unwrap().trim(), "value1");
        assert_eq!(map.get("PREFIX_KEY2").unwrap().trim(), "42");
        assert_eq!(map.get("PREFIX_KEY3").unwrap().trim(), "true");
        assert!(map.get("PREFIX_KEY4").unwrap().trim().starts_with("3.14"));
    }

    #[test]
    fn test_flatten_toml_table_nested_tables() {
        let toml_str = r#"
[outer]
inner_key = "inner_value"

[outer.nested]
deep_key = "deep_value"
"#;
        let doc: toml_edit::DocumentMut = toml_str.parse().unwrap();
        let mut vars = Vec::new();
        flatten_toml_table(doc.as_table(), "APP", &mut vars);

        let map: HashMap<String, String> = vars.into_iter().collect();
        assert_eq!(map.get("APP_OUTER_INNER_KEY").unwrap(), "inner_value");
        assert_eq!(map.get("APP_OUTER_NESTED_DEEP_KEY").unwrap(), "deep_value");
    }

    #[test]
    fn test_flatten_toml_table_array_of_tables() {
        let toml_str = r#"
[[items]]
name = "first"

[[items]]
name = "second"
"#;
        let doc: toml_edit::DocumentMut = toml_str.parse().unwrap();
        let mut vars = Vec::new();
        flatten_toml_table(doc.as_table(), "ROOT", &mut vars);

        // ArrayOfTables should produce a single entry with the formatted representation
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].0, "ROOT_ITEMS");
        assert!(vars[0].1.contains("name"));
    }

    #[test]
    fn test_flatten_toml_table_empty() {
        let toml_str = "";
        let doc: toml_edit::DocumentMut = toml_str.parse().unwrap();
        let mut vars = Vec::new();
        flatten_toml_table(doc.as_table(), "EMPTY", &mut vars);
        assert!(vars.is_empty());
    }

    // ========================================================================
    // Clone and Debug trait coverage
    // ========================================================================

    #[test]
    fn test_config_clone() {
        let config = Config::default();
        let cloned = config.clone();
        assert_eq!(cloned.server.name, config.server.name);
        assert_eq!(cloned.paths.base, config.paths.base);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("Config"));
        assert!(debug.contains("ServerConfig"));
        assert!(debug.contains("PathsConfig"));
    }

    #[test]
    fn test_server_config_clone_and_debug() {
        let config = ServerConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.name, config.name);
        let debug = format!("{:?}", config);
        assert!(debug.contains("ServerConfig"));
    }

    #[test]
    fn test_paths_config_clone_and_debug() {
        let config = PathsConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.base, config.base);
        let debug = format!("{:?}", config);
        assert!(debug.contains("PathsConfig"));
    }

    #[test]
    fn test_sources_config_clone_and_debug() {
        let config = SourcesConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.oxford.path, config.oxford.path);
        let debug = format!("{:?}", config);
        assert!(debug.contains("SourcesConfig"));
    }

    #[test]
    fn test_source_category_clone_and_debug() {
        let mut cat = SourceCategory::default();
        cat.path = "/test".to_string();
        cat.files.insert("key".to_string(), "val.pdf".to_string());
        cat.aliases
            .insert("key".to_string(), vec!["alias1".to_string()]);
        let cloned = cat.clone();
        assert_eq!(cloned.path, cat.path);
        assert_eq!(cloned.files.len(), 1);
        assert_eq!(cloned.aliases.len(), 1);
        let debug = format!("{:?}", cat);
        assert!(debug.contains("SourceCategory"));
    }

    #[test]
    fn test_search_config_clone_and_debug() {
        let config = SearchConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.backend, config.backend);
        let debug = format!("{:?}", config);
        assert!(debug.contains("SearchConfig"));
    }

    // ========================================================================
    // Serde round-trip for top-level Config
    // ========================================================================

    #[test]
    fn test_config_serde_roundtrip_via_toml() {
        let config = Config::default();
        let toml_str = toml_edit::ser::to_string_pretty(&config).expect("Should serialize to TOML");
        let deserialized: Config =
            toml_edit::de::from_str(&toml_str).expect("Should deserialize from TOML");
        assert_eq!(deserialized.server.name, config.server.name);
        assert_eq!(deserialized.paths.base, config.paths.base);
        assert_eq!(deserialized.search.backend, config.search.backend);
    }

    // ========================================================================
    // SourceCategory with aliases populated
    // ========================================================================

    #[test]
    fn test_source_category_file_path_with_aliases() {
        let mut files = HashMap::new();
        files.insert("harmony".to_string(), "harmony.pdf".to_string());
        let mut aliases = HashMap::new();
        aliases.insert(
            "harmony".to_string(),
            vec!["Harmony Basics".to_string(), "Intro to Harmony".to_string()],
        );

        let category = SourceCategory {
            path: "/books".to_string(),
            files,
            aliases,
        };

        // file_path should still work - aliases don't affect file lookup
        let result = category.file_path("harmony");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/books/harmony.pdf"));
    }

    // ========================================================================
    // SourceCategory serde deserialization
    // ========================================================================

    #[test]
    fn test_source_category_serde_from_json() {
        let json = r#"{
            "path": "/sources",
            "files": {"book1": "book1.pdf"},
            "aliases": {"book1": ["Book One", "First Book"]}
        }"#;
        let cat: SourceCategory = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(cat.path, "/sources");
        assert_eq!(cat.files.get("book1").unwrap(), "book1.pdf");
        let aliases = cat.aliases.get("book1").unwrap();
        assert_eq!(aliases.len(), 2);
        assert!(aliases.contains(&"Book One".to_string()));
    }

    #[test]
    fn test_sources_config_serde_from_json() {
        let json = r#"{
            "oxford": {"path": "/oxford", "files": {}, "aliases": {}},
            "general": {"path": "/general", "files": {}, "aliases": {}},
            "papers": {"path": "/papers", "files": {}, "aliases": {}}
        }"#;
        let config: SourcesConfig = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(config.oxford.path, "/oxford");
        assert_eq!(config.general.path, "/general");
        assert_eq!(config.papers.path, "/papers");
    }

    // ========================================================================
    // expand_path with environment variable
    // ========================================================================

    #[test]
    fn test_expand_path_with_home_env_var() {
        let result = expand_path("$HOME/test");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.is_absolute());
        assert!(path.to_string_lossy().ends_with("/test"));
    }
}
