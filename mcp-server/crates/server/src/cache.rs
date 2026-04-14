//! Pre-built cache distribution for the music-theory MCP server.
//!
//! Delegates to `fabryk_cli::cache` for all generic cache management, providing
//! project-specific constants (release URL, project prefix, file paths).

use std::path::PathBuf;

use crate::config::Config;
use crate::error::{Error, Result};

// Re-export fabryk cache types for use by other modules.
pub use fabryk_cli::cache::{
    BackendPaths, BackendStatus, CacheBackend, CacheEntry, CacheManifest, CacheProject,
    CacheStatusReport, PackagePaths, archive_name, checksum_url, download_cache, extract_archive,
    load_manifest, package_cache, parse_backend_arg, release_url, save_manifest, shell_download,
    verify_checksum,
};

/// GitHub Release base URL for this project's cache archives.
pub const RELEASE_BASE_URL: &str =
    "https://github.com/oxur/ai-music-theory/releases/download";

/// Project prefix for cache archive names.
pub const PROJECT_PREFIX: &str = "music-theory";

/// Build a [`CacheProject`] for this application.
pub fn project() -> CacheProject {
    CacheProject {
        prefix: PROJECT_PREFIX.to_string(),
        release_base_url: RELEASE_BASE_URL.to_string(),
    }
}

/// Build [`BackendPaths`] from the server configuration.
pub fn backend_paths(config: &Config) -> Result<BackendPaths> {
    let base = config.paths.base_path()?;
    Ok(BackendPaths {
        graph: base.join("data/graphs/concept_graph.json"),
        fts: base.join(".tantivy-index"),
        vector: base.join(".cache/vector/vector-cache.json"),
    })
}

/// Build [`PackagePaths`] for this project's cache layout.
pub fn package_paths() -> PackagePaths {
    PackagePaths {
        graph: vec!["data/graphs/concept_graph.json".to_string()],
        fts: vec![".tantivy-index".to_string()],
        fts_excludes: vec!["*.lock".to_string(), ".tmp*".to_string()],
        vector: vec![".cache/vector/vector-cache.json".to_string()],
    }
}

/// Check the installation status of all three cache backends.
pub fn cache_status(config: &Config) -> Result<CacheStatusReport> {
    let base = config.paths.base_path()?;
    let paths = backend_paths(config)?;
    fabryk_cli::cache::cache_status(&base, &paths).map_err(|e| Error::operation(e.to_string()))
}

/// Download and install a pre-built cache for the given backend.
pub fn download_project_cache(
    backend: &CacheBackend,
    config: &Config,
    force: bool,
) -> Result<()> {
    let base = config.paths.base_path()?;
    let version = env!("CARGO_PKG_VERSION");
    let proj = project();
    fabryk_cli::cache::download_cache(backend, &base, version, &proj, force)
        .map_err(|e| Error::operation(e.to_string()))
}

/// Create a distributable archive for a cache backend.
pub fn package_project_cache(
    backend: &CacheBackend,
    config: &Config,
    output_dir: &std::path::Path,
) -> Result<PathBuf> {
    let base = config.paths.base_path()?;
    let version = env!("CARGO_PKG_VERSION");
    let proj = project();
    let paths = package_paths();
    fabryk_cli::cache::package_cache(backend, &base, output_dir, version, &proj, &paths)
        .map_err(|e| Error::operation(e.to_string()))
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Project-specific constant tests

    #[test]
    fn test_project_constants() {
        assert_eq!(PROJECT_PREFIX, "music-theory");
        assert!(RELEASE_BASE_URL.contains("ai-music-theory"));
    }

    #[test]
    fn test_project_archive_name() {
        let proj = project();
        let name = archive_name(&CacheBackend::Graph, "1.2.3", &proj);
        assert_eq!(name, "music-theory-cache-graph-v1.2.3.tar.gz");
    }

    #[test]
    fn test_project_release_url() {
        let proj = project();
        let url = release_url(&CacheBackend::Graph, "1.0.0", &proj);
        assert!(url.starts_with(RELEASE_BASE_URL));
        assert!(url.contains("music-theory-cache-graph"));
    }

    #[test]
    fn test_backend_paths_from_config() {
        let mut config = Config::default();
        config.paths.base = ".".to_string();
        let paths = backend_paths(&config).unwrap();
        assert!(paths.graph.to_string_lossy().contains("concept_graph.json"));
        assert!(paths.fts.to_string_lossy().contains("tantivy-index"));
        assert!(paths.vector.to_string_lossy().contains("vector-cache.json"));
    }

    #[test]
    fn test_package_paths() {
        let paths = package_paths();
        assert_eq!(paths.graph.len(), 1);
        assert_eq!(paths.fts.len(), 1);
        assert_eq!(paths.vector.len(), 1);
        assert!(!paths.fts_excludes.is_empty());
    }

    #[test]
    fn test_cache_status_empty() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = Config::default();
        config.paths.base = dir.path().to_string_lossy().to_string();

        let report = cache_status(&config).unwrap();

        assert!(!report.graph.files_present);
        assert!(!report.fts.files_present);
        assert!(!report.vector.files_present);

        let display = format!("{report}");
        assert!(display.contains("not installed"));
    }
}
