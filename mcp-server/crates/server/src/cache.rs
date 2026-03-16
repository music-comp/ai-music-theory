//! Pre-built cache distribution for the music-theory MCP server.
//!
//! Manages download, packaging, and status reporting for three backend caches
//! (graph, FTS, vector) distributed as GitHub Release assets. Each cache is
//! packaged as a `.tar.gz` archive with a `.sha256` sidecar for integrity
//! verification.
//!
//! # Workflow
//!
//! **Downloading** (end-user):
//! 1. Determine which backends are needed via [`cache_status`]
//! 2. Call [`download_cache`] for each backend (or use [`parse_backend_arg`] with `"all"`)
//! 3. Archives are fetched, verified, and extracted into the configured base directory
//!
//! **Packaging** (release process):
//! 1. Build caches locally (index, graph build, vector embed)
//! 2. Call [`package_cache`] for each backend to create distributable archives

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::{Error, Result};

/// Base URL for GitHub Release assets.
const RELEASE_BASE_URL: &str = "https://github.com/oxur/ai-music-theory/releases/download";

/// Filename for the cache manifest stored alongside downloaded caches.
const MANIFEST_FILENAME: &str = "cache-manifest.json";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Identifies one of the three cache backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheBackend {
    /// Knowledge graph (concept_graph.json).
    Graph,
    /// Full-text search (Tantivy index).
    Fts,
    /// Vector embeddings (LanceDB / fastembed cache).
    Vector,
}

impl fmt::Display for CacheBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Graph => write!(f, "graph"),
            Self::Fts => write!(f, "fts"),
            Self::Vector => write!(f, "vector"),
        }
    }
}

impl FromStr for CacheBackend {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "graph" => Ok(Self::Graph),
            "fts" => Ok(Self::Fts),
            "vector" => Ok(Self::Vector),
            other => Err(Error::operation(&format!(
                "Unknown cache backend: '{other}'. Expected one of: graph, fts, vector"
            ))),
        }
    }
}

/// Persistent record of which caches have been downloaded and their versions.
///
/// Stored as `cache-manifest.json` in the base data directory.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheManifest {
    /// Graph cache entry, if downloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph: Option<CacheEntry>,
    /// FTS cache entry, if downloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fts: Option<CacheEntry>,
    /// Vector cache entry, if downloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vector: Option<CacheEntry>,
}

impl CacheManifest {
    /// Return the entry for a given backend, if present.
    pub fn get(&self, backend: &CacheBackend) -> Option<&CacheEntry> {
        match backend {
            CacheBackend::Graph => self.graph.as_ref(),
            CacheBackend::Fts => self.fts.as_ref(),
            CacheBackend::Vector => self.vector.as_ref(),
        }
    }

    /// Set the entry for a given backend.
    pub fn set(&mut self, backend: &CacheBackend, entry: CacheEntry) {
        match backend {
            CacheBackend::Graph => self.graph = Some(entry),
            CacheBackend::Fts => self.fts = Some(entry),
            CacheBackend::Vector => self.vector = Some(entry),
        }
    }
}

/// Metadata for a single downloaded cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Server version the cache was built for.
    pub version: String,
    /// ISO 8601 timestamp of when the cache was downloaded.
    pub downloaded_at: String,
    /// SHA-256 checksum of the archive that was verified at download time.
    pub checksum: String,
}

/// Per-backend status information.
#[derive(Debug, Clone)]
pub struct BackendStatus {
    /// Which backend this status describes.
    pub backend: CacheBackend,
    /// Installed version from the manifest, if any.
    pub installed_version: Option<String>,
    /// Whether the expected files actually exist on disk.
    pub files_present: bool,
}

/// Aggregated status report across all three cache backends.
#[derive(Debug, Clone)]
pub struct CacheStatusReport {
    /// Status of the graph cache.
    pub graph: BackendStatus,
    /// Status of the FTS cache.
    pub fts: BackendStatus,
    /// Status of the vector cache.
    pub vector: BackendStatus,
}

impl fmt::Display for CacheStatusReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cache Status:")?;
        for status in [&self.graph, &self.fts, &self.vector] {
            let state = if status.files_present {
                match &status.installed_version {
                    Some(v) => format!("installed (v{v})"),
                    None => "files present (not tracked)".to_string(),
                }
            } else {
                match &status.installed_version {
                    Some(v) => format!("manifest says v{v}, but files missing"),
                    None => "not installed".to_string(),
                }
            };
            writeln!(f, "  {}: {}", status.backend, state)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// URL / naming helpers
// ---------------------------------------------------------------------------

/// Return the archive filename for a backend at a given version.
///
/// Format: `music-theory-cache-{backend}-v{version}.tar.gz`
pub fn archive_name(backend: &CacheBackend, version: &str) -> String {
    format!("music-theory-cache-{backend}-v{version}.tar.gz")
}

/// Return the full GitHub Release download URL for a cache archive.
pub fn release_url(backend: &CacheBackend, version: &str) -> String {
    let name = archive_name(backend, version);
    format!("{RELEASE_BASE_URL}/v{version}/{name}")
}

/// Return the URL for the `.sha256` sidecar checksum file.
pub fn checksum_url(backend: &CacheBackend, version: &str) -> String {
    format!("{}.sha256", release_url(backend, version))
}

// ---------------------------------------------------------------------------
// Manifest persistence
// ---------------------------------------------------------------------------

/// Load the cache manifest from `{base_path}/cache-manifest.json`.
///
/// Returns an empty default manifest if the file does not exist.
/// Returns an error if the file exists but cannot be parsed.
pub fn load_manifest(base_path: &Path) -> Result<CacheManifest> {
    let path = base_path.join(MANIFEST_FILENAME);
    if !path.exists() {
        return Ok(CacheManifest::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|e| Error::io_with_path(e, &path))?;
    serde_json::from_str(&contents).map_err(|e| {
        Error::operation(&format!(
            "Failed to parse cache manifest at {}: {e}",
            path.display()
        ))
    })
}

/// Write the cache manifest as pretty-printed JSON.
pub fn save_manifest(base_path: &Path, manifest: &CacheManifest) -> Result<()> {
    let path = base_path.join(MANIFEST_FILENAME);
    let json = serde_json::to_string_pretty(manifest).map_err(|e| {
        Error::operation(&format!("Failed to serialize cache manifest: {e}"))
    })?;
    std::fs::write(&path, json).map_err(|e| Error::io_with_path(e, &path))
}

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

/// Check the installation status of all three cache backends.
///
/// Loads the manifest and verifies that the expected files or directories
/// actually exist on disk.
pub fn cache_status(config: &Config) -> Result<CacheStatusReport> {
    let base = config.paths.base_path()?;
    let manifest = load_manifest(&base)?;

    let graph_present = base.join("data/graphs/concept_graph.json").exists();
    let fts_present = base.join(".tantivy-index").is_dir();
    let vector_present = base.join(".cache/vector/vector-cache.json").exists();

    Ok(CacheStatusReport {
        graph: BackendStatus {
            backend: CacheBackend::Graph,
            installed_version: manifest.graph.as_ref().map(|e| e.version.clone()),
            files_present: graph_present,
        },
        fts: BackendStatus {
            backend: CacheBackend::Fts,
            installed_version: manifest.fts.as_ref().map(|e| e.version.clone()),
            files_present: fts_present,
        },
        vector: BackendStatus {
            backend: CacheBackend::Vector,
            installed_version: manifest.vector.as_ref().map(|e| e.version.clone()),
            files_present: vector_present,
        },
    })
}

// ---------------------------------------------------------------------------
// Shell helpers
// ---------------------------------------------------------------------------

/// Download a URL to a local file using `curl`.
///
/// Runs `curl -fSL -o {dest} {url}` and maps non-zero exit codes to an error.
/// Progress output is written to stderr by curl.
pub fn shell_download(url: &str, dest: &Path) -> Result<()> {
    log::info!("Downloading {} ...", url);
    let output = Command::new("curl")
        .args(["-fSL", "-o"])
        .arg(dest)
        .arg(url)
        .output()
        .map_err(|e| {
            Error::operation(&format!("Failed to execute curl: {e}"))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::operation(&format!(
            "curl exited with status {}: {stderr}",
            output.status
        )));
    }
    Ok(())
}

/// Verify the SHA-256 checksum of a file.
///
/// Runs `shasum -a 256` and compares the output against `expected_hash`.
pub fn verify_checksum(archive: &Path, expected_hash: &str) -> Result<()> {
    log::debug!("Verifying checksum of {} ...", archive.display());
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(archive)
        .output()
        .map_err(|e| {
            Error::operation(&format!("Failed to execute shasum: {e}"))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::operation(&format!(
            "shasum exited with status {}: {stderr}",
            output.status
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let actual_hash = stdout
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim();

    if actual_hash != expected_hash.trim() {
        return Err(Error::operation(&format!(
            "Checksum mismatch for {}: expected {}, got {actual_hash}",
            archive.display(),
            expected_hash.trim()
        )));
    }
    log::debug!("Checksum verified: {actual_hash}");
    Ok(())
}

/// Extract a `.tar.gz` archive into a target directory.
pub fn extract_archive(archive: &Path, target_dir: &Path) -> Result<()> {
    log::info!("Extracting {} to {} ...", archive.display(), target_dir.display());
    let output = Command::new("tar")
        .args(["-xzf"])
        .arg(archive)
        .arg("-C")
        .arg(target_dir)
        .output()
        .map_err(|e| {
            Error::operation(&format!("Failed to execute tar: {e}"))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::operation(&format!(
            "tar exited with status {}: {stderr}",
            output.status
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Download orchestration
// ---------------------------------------------------------------------------

/// Download and install a pre-built cache for the given backend.
///
/// Skips the download if the manifest already records an installation at the
/// current server version, unless `force` is `true`.
///
/// Steps:
/// 1. Check manifest for existing installation
/// 2. Download the archive and its `.sha256` sidecar
/// 3. Verify the checksum
/// 4. Extract the archive into the base directory
/// 5. Update the manifest
/// 6. Clean up temporary files
pub fn download_cache(backend: &CacheBackend, config: &Config, force: bool) -> Result<()> {
    let base = config.paths.base_path()?;
    let version = env!("CARGO_PKG_VERSION");

    // Check manifest for existing installation.
    let mut manifest = load_manifest(&base)?;
    if !force {
        if let Some(entry) = manifest.get(backend) {
            if entry.version == version {
                log::info!(
                    "{} cache v{version} already installed, skipping (use --force to re-download)",
                    backend
                );
                return Ok(());
            }
        }
    }

    log::info!("Installing {backend} cache v{version} ...");

    let archive_file = base.join(format!(".cache-download-{backend}.tar.gz"));
    let checksum_file = base.join(format!(".cache-download-{backend}.sha256"));

    // Download archive and checksum sidecar.
    let archive_url = release_url(backend, version);
    let cs_url = checksum_url(backend, version);

    shell_download(&archive_url, &archive_file)?;
    shell_download(&cs_url, &checksum_file)?;

    // Read expected checksum from sidecar.
    let expected_hash = std::fs::read_to_string(&checksum_file)
        .map_err(|e| Error::io_with_path(e, &checksum_file))?;
    let expected_hash = expected_hash
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim();

    // Verify and extract.
    verify_checksum(&archive_file, expected_hash)?;
    extract_archive(&archive_file, &base)?;

    // Update manifest.
    manifest.set(
        backend,
        CacheEntry {
            version: version.to_string(),
            downloaded_at: iso8601_now(),
            checksum: expected_hash.to_string(),
        },
    );
    save_manifest(&base, &manifest)?;

    // Clean up temp files.
    let _ = std::fs::remove_file(&archive_file);
    let _ = std::fs::remove_file(&checksum_file);

    log::info!("{backend} cache v{version} installed successfully.");
    Ok(())
}

// ---------------------------------------------------------------------------
// Packaging
// ---------------------------------------------------------------------------

/// Create a distributable `.tar.gz` archive for a cache backend.
///
/// The archive is written to `{output_dir}/{archive_name}` and its path is
/// returned on success.
pub fn package_cache(
    backend: &CacheBackend,
    base_path: &Path,
    output_dir: &Path,
    version: &str,
) -> Result<PathBuf> {
    std::fs::create_dir_all(output_dir).map_err(|e| Error::io_with_path(e, output_dir))?;

    let name = archive_name(backend, version);
    let output_path = output_dir.join(&name);

    log::info!("Packaging {backend} cache as {name} ...");

    let mut cmd = Command::new("tar");
    cmd.arg("-czf").arg(&output_path).arg("-C").arg(base_path);

    match backend {
        CacheBackend::Graph => {
            cmd.arg("data/graphs/concept_graph.json");
        }
        CacheBackend::Fts => {
            cmd.args(["--exclude=*.lock", "--exclude=.tmp*"]);
            cmd.arg(".tantivy-index");
        }
        CacheBackend::Vector => {
            cmd.arg(".cache/vector/vector-cache.json");
        }
    }

    let output = cmd.output().map_err(|e| {
        Error::operation(&format!("Failed to execute tar: {e}"))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::operation(&format!(
            "tar exited with status {}: {stderr}",
            output.status
        )));
    }

    log::info!("Created {}", output_path.display());
    Ok(output_path)
}

// ---------------------------------------------------------------------------
// Argument parsing helper
// ---------------------------------------------------------------------------

/// Parse a backend argument string into one or more [`CacheBackend`] values.
///
/// Accepts `"all"` (case-insensitive) to return all three backends, or a
/// single backend name such as `"graph"`, `"fts"`, or `"vector"`.
pub fn parse_backend_arg(arg: &str) -> Result<Vec<CacheBackend>> {
    if arg.eq_ignore_ascii_case("all") {
        return Ok(vec![
            CacheBackend::Graph,
            CacheBackend::Fts,
            CacheBackend::Vector,
        ]);
    }
    let backend = CacheBackend::from_str(arg)?;
    Ok(vec![backend])
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Return the current time as an ISO 8601 string (UTC).
///
/// Uses `std::time::SystemTime` to avoid requiring chrono as a non-optional
/// dependency. The format is `YYYY-MM-DDTHH:MM:SSZ`.
fn iso8601_now() -> String {
    let now = SystemTime::now();
    let duration = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Decompose seconds since epoch into date/time components.
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Convert days since epoch to year/month/day.
    let (year, month, day) = days_to_date(days);

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(days_since_epoch: u64) -> (u64, u64, u64) {
    // Algorithm based on Howard Hinnant's civil_from_days.
    let z = days_since_epoch as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // day of era [0, 146096]
    let yoe =
        (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month index [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as u64, m, d)
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Naming / URL helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_archive_name() {
        assert_eq!(
            archive_name(&CacheBackend::Graph, "1.2.3"),
            "music-theory-cache-graph-v1.2.3.tar.gz"
        );
        assert_eq!(
            archive_name(&CacheBackend::Fts, "0.1.0"),
            "music-theory-cache-fts-v0.1.0.tar.gz"
        );
        assert_eq!(
            archive_name(&CacheBackend::Vector, "2.0.0-rc1"),
            "music-theory-cache-vector-v2.0.0-rc1.tar.gz"
        );
    }

    #[test]
    fn test_release_url() {
        let url = release_url(&CacheBackend::Graph, "1.0.0");
        assert_eq!(
            url,
            "https://github.com/oxur/ai-music-theory/releases/download/v1.0.0/\
             music-theory-cache-graph-v1.0.0.tar.gz"
        );
    }

    #[test]
    fn test_checksum_url() {
        let url = checksum_url(&CacheBackend::Fts, "1.0.0");
        assert_eq!(
            url,
            "https://github.com/oxur/ai-music-theory/releases/download/v1.0.0/\
             music-theory-cache-fts-v1.0.0.tar.gz.sha256"
        );
    }

    // -----------------------------------------------------------------------
    // CacheBackend Display / FromStr
    // -----------------------------------------------------------------------

    #[test]
    fn test_cache_backend_display() {
        assert_eq!(CacheBackend::Graph.to_string(), "graph");
        assert_eq!(CacheBackend::Fts.to_string(), "fts");
        assert_eq!(CacheBackend::Vector.to_string(), "vector");
    }

    #[test]
    fn test_cache_backend_from_str() {
        assert_eq!(CacheBackend::from_str("graph").unwrap(), CacheBackend::Graph);
        assert_eq!(CacheBackend::from_str("fts").unwrap(), CacheBackend::Fts);
        assert_eq!(CacheBackend::from_str("vector").unwrap(), CacheBackend::Vector);

        // Case-insensitive.
        assert_eq!(CacheBackend::from_str("GRAPH").unwrap(), CacheBackend::Graph);
        assert_eq!(CacheBackend::from_str("FTS").unwrap(), CacheBackend::Fts);
        assert_eq!(CacheBackend::from_str("Vector").unwrap(), CacheBackend::Vector);

        // Invalid name.
        assert!(CacheBackend::from_str("unknown").is_err());
        assert!(CacheBackend::from_str("").is_err());
    }

    // -----------------------------------------------------------------------
    // parse_backend_arg
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_backend_arg_all() {
        let backends = parse_backend_arg("all").unwrap();
        assert_eq!(backends.len(), 3);
        assert_eq!(backends[0], CacheBackend::Graph);
        assert_eq!(backends[1], CacheBackend::Fts);
        assert_eq!(backends[2], CacheBackend::Vector);

        // Case-insensitive "ALL".
        let backends = parse_backend_arg("ALL").unwrap();
        assert_eq!(backends.len(), 3);
    }

    #[test]
    fn test_parse_backend_arg_single() {
        let backends = parse_backend_arg("graph").unwrap();
        assert_eq!(backends, vec![CacheBackend::Graph]);

        let backends = parse_backend_arg("fts").unwrap();
        assert_eq!(backends, vec![CacheBackend::Fts]);

        let backends = parse_backend_arg("vector").unwrap();
        assert_eq!(backends, vec![CacheBackend::Vector]);
    }

    #[test]
    fn test_parse_backend_arg_invalid() {
        let result = parse_backend_arg("nope");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("nope"), "Error should mention the invalid name");
    }

    // -----------------------------------------------------------------------
    // Manifest serialization
    // -----------------------------------------------------------------------

    #[test]
    fn test_manifest_roundtrip() {
        let manifest = CacheManifest {
            graph: Some(CacheEntry {
                version: "1.0.0".to_string(),
                downloaded_at: "2025-01-15T10:30:00Z".to_string(),
                checksum: "abc123".to_string(),
            }),
            fts: None,
            vector: Some(CacheEntry {
                version: "1.0.0".to_string(),
                downloaded_at: "2025-01-15T11:00:00Z".to_string(),
                checksum: "def456".to_string(),
            }),
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();

        // FTS should be absent from JSON (skip_serializing_if).
        assert!(!json.contains("\"fts\""));

        let parsed: CacheManifest = serde_json::from_str(&json).unwrap();
        assert!(parsed.graph.is_some());
        assert!(parsed.fts.is_none());
        assert!(parsed.vector.is_some());
        assert_eq!(parsed.graph.unwrap().version, "1.0.0");
        assert_eq!(parsed.vector.unwrap().checksum, "def456");
    }

    #[test]
    fn test_manifest_load_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = load_manifest(dir.path()).unwrap();
        assert!(manifest.graph.is_none());
        assert!(manifest.fts.is_none());
        assert!(manifest.vector.is_none());
    }

    #[test]
    fn test_manifest_save_and_load() {
        let dir = tempfile::tempdir().unwrap();

        let manifest = CacheManifest {
            graph: Some(CacheEntry {
                version: "2.0.0".to_string(),
                downloaded_at: "2025-06-01T00:00:00Z".to_string(),
                checksum: "aabbcc".to_string(),
            }),
            fts: Some(CacheEntry {
                version: "2.0.0".to_string(),
                downloaded_at: "2025-06-01T00:05:00Z".to_string(),
                checksum: "ddeeff".to_string(),
            }),
            vector: None,
        };

        save_manifest(dir.path(), &manifest).unwrap();

        // Verify the file was created.
        let path = dir.path().join(MANIFEST_FILENAME);
        assert!(path.exists());

        // Load it back.
        let loaded = load_manifest(dir.path()).unwrap();
        assert_eq!(loaded.graph.as_ref().unwrap().version, "2.0.0");
        assert_eq!(loaded.fts.as_ref().unwrap().checksum, "ddeeff");
        assert!(loaded.vector.is_none());
    }

    // -----------------------------------------------------------------------
    // Cache status
    // -----------------------------------------------------------------------

    #[test]
    fn test_cache_status_empty() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = Config::default();
        config.paths.base = dir.path().to_string_lossy().to_string();

        let report = cache_status(&config).unwrap();

        assert!(!report.graph.files_present);
        assert!(!report.fts.files_present);
        assert!(!report.vector.files_present);
        assert!(report.graph.installed_version.is_none());
        assert!(report.fts.installed_version.is_none());
        assert!(report.vector.installed_version.is_none());

        // Display should not panic.
        let display = format!("{report}");
        assert!(display.contains("graph"));
        assert!(display.contains("fts"));
        assert!(display.contains("vector"));
        assert!(display.contains("not installed"));
    }

    // -----------------------------------------------------------------------
    // Manifest get/set helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_manifest_get_set() {
        let mut manifest = CacheManifest::default();
        assert!(manifest.get(&CacheBackend::Graph).is_none());

        let entry = CacheEntry {
            version: "1.0.0".to_string(),
            downloaded_at: "2025-01-01T00:00:00Z".to_string(),
            checksum: "abc".to_string(),
        };

        manifest.set(&CacheBackend::Graph, entry.clone());
        assert!(manifest.get(&CacheBackend::Graph).is_some());
        assert_eq!(manifest.get(&CacheBackend::Graph).unwrap().version, "1.0.0");

        manifest.set(&CacheBackend::Fts, entry.clone());
        assert!(manifest.get(&CacheBackend::Fts).is_some());

        manifest.set(&CacheBackend::Vector, entry);
        assert!(manifest.get(&CacheBackend::Vector).is_some());
    }

    // -----------------------------------------------------------------------
    // ISO 8601 timestamp helper
    // -----------------------------------------------------------------------

    #[test]
    fn test_iso8601_now_format() {
        let ts = iso8601_now();
        // Should match YYYY-MM-DDTHH:MM:SSZ pattern.
        assert_eq!(ts.len(), 20, "Timestamp should be 20 chars: {ts}");
        assert!(ts.ends_with('Z'));
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_days_to_date_epoch() {
        let (y, m, d) = days_to_date(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_date_known() {
        // 2025-01-01 is 20089 days after epoch.
        let (y, m, d) = days_to_date(20089);
        assert_eq!((y, m, d), (2025, 1, 1));
    }

    // -----------------------------------------------------------------------
    // CacheStatusReport Display
    // -----------------------------------------------------------------------

    #[test]
    fn test_cache_status_report_display_installed() {
        let report = CacheStatusReport {
            graph: BackendStatus {
                backend: CacheBackend::Graph,
                installed_version: Some("1.0.0".to_string()),
                files_present: true,
            },
            fts: BackendStatus {
                backend: CacheBackend::Fts,
                installed_version: None,
                files_present: false,
            },
            vector: BackendStatus {
                backend: CacheBackend::Vector,
                installed_version: Some("1.0.0".to_string()),
                files_present: false,
            },
        };

        let display = format!("{report}");
        assert!(display.contains("graph: installed (v1.0.0)"));
        assert!(display.contains("fts: not installed"));
        assert!(display.contains("vector: manifest says v1.0.0, but files missing"));
    }
}
