# Path Resolution Utilities and Update

## Overview

Implement a `util/paths.rs` module that provides robust path resolution for the MCP server. This solves the problem of locating config files and skill content regardless of:

- Where the binary is run from
- Whether it's invoked by Claude Desktop (arbitrary working directory)
- Whether it's run during development or after installation

## The Problem

Currently, config loading uses relative paths that fail when Claude Desktop runs the binary:

```
Error: Configuration error: Failed to build config: no configurations added
```

The binary can't find `config/default.toml` because the working directory isn't what we expect.

## Solution

Create `util/paths.rs` with functions that:

1. Find the binary's location using `std::env::current_exe()`
2. Walk up the directory tree to find project markers
3. Support environment variable overrides
4. Provide fallbacks for robustness

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dirs = "5"
```

## Implementation

Create `src/util/paths.rs`:

```rust
//! Path resolution utilities for finding project resources.
//!
//! Handles the challenge of locating config files and resources whether
//! the binary is run from the workspace, installed globally, or invoked
//! by Claude Desktop from an arbitrary working directory.

use std::path::{Path, PathBuf};

/// Markers that indicate we've found the project root (ai-music-theory)
const PROJECT_MARKERS: &[&str] = &[
    "SKILL.md",
    "CONVENTIONS.md",
    "SCOPE.md",
];

/// Markers that indicate we've found the MCP server crate root
const SERVER_MARKERS: &[&str] = &[
    "config/default.toml",
];

/// Find the absolute path to the currently running binary.
pub fn binary_path() -> Option<PathBuf> {
    std::env::current_exe().ok()
}

/// Find the directory containing the currently running binary.
pub fn binary_dir() -> Option<PathBuf> {
    binary_path().and_then(|p| p.parent().map(Path::to_path_buf))
}

/// Find the MCP server crate root by walking up from the binary location.
///
/// Looks for `config/default.toml` as a marker.
///
/// Handles common binary locations:
/// - `mcp-server/bin/music-theory-mcp` (installed)
/// - `mcp-server/target/release/music-theory-mcp` (cargo build --release)
/// - `mcp-server/target/debug/music-theory-mcp` (cargo build)
pub fn server_root() -> Option<PathBuf> {
    let binary_dir = binary_dir()?;

    // Walk up from binary location, checking each directory
    let mut current = binary_dir.as_path();

    for _ in 0..10 {  // Limit search depth
        // Check if this looks like the server crate root
        if SERVER_MARKERS.iter().any(|marker| current.join(marker).exists()) {
            return Some(current.to_path_buf());
        }

        // Also check crates/server subdirectory (if we're at workspace root)
        let crates_server = current.join("crates/server");
        if SERVER_MARKERS.iter().any(|marker| crates_server.join(marker).exists()) {
            return Some(crates_server);
        }

        current = current.parent()?;
    }

    None
}

/// Find the project root (ai-music-theory) by walking up from the binary location.
///
/// Looks for SKILL.md, CONVENTIONS.md, or SCOPE.md as markers.
pub fn project_root() -> Option<PathBuf> {
    let binary_dir = binary_dir()?;

    let mut current = binary_dir.as_path();

    for _ in 0..10 {  // Limit search depth
        if PROJECT_MARKERS.iter().any(|marker| current.join(marker).exists()) {
            return Some(current.to_path_buf());
        }

        current = current.parent()?;
    }

    None
}

/// Find the config directory for the MCP server.
///
/// Search order:
/// 1. `MUSIC_THEORY_CONFIG_DIR` environment variable
/// 2. Relative to server root: `{server_root}/config`
/// 3. Common relative paths from current working directory
/// 4. Fallback to home directory path
pub fn config_dir() -> Option<PathBuf> {
    // 1. Environment variable override
    if let Ok(path) = std::env::var("MUSIC_THEORY_CONFIG_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Relative to server root (found by walking up from binary)
    if let Some(root) = server_root() {
        let config = root.join("config");
        if config.exists() {
            return Some(config);
        }
    }

    // 3. Relative paths from current directory (for development)
    for relative in &["./config", "../config", "./crates/server/config"] {
        let path = PathBuf::from(relative);
        if path.exists() {
            if let Ok(canonical) = path.canonicalize() {
                return Some(canonical);
            }
        }
    }

    // 4. Fallback to hardcoded home path
    if let Some(home) = dirs::home_dir() {
        let path = home.join("lab/music-comp/ai-music-theory/mcp-server/crates/server/config");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Find the skill content root (where sources-md, concept-cards, etc. live).
///
/// This is the ai-music-theory project root, not the mcp-server subdirectory.
///
/// Search order:
/// 1. `MUSIC_THEORY_SKILL_ROOT` environment variable
/// 2. Walk up from binary to find project root
/// 3. Fallback to home directory path
pub fn skill_root() -> Option<PathBuf> {
    // 1. Environment variable override
    if let Ok(path) = std::env::var("MUSIC_THEORY_SKILL_ROOT") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Walk up from binary to find project root
    if let Some(root) = project_root() {
        return Some(root);
    }

    // 3. Fallback to hardcoded home path
    if let Some(home) = dirs::home_dir() {
        let path = home.join("lab/music-comp/ai-music-theory");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Resolve a path that might contain `~` to an absolute path.
pub fn expand_tilde(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    if let Ok(stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }

    path.to_path_buf()
}

/// Get a human-readable description of resolved paths (for debugging/logging).
pub fn debug_paths() -> String {
    format!(
        "Path resolution:\n  \
         binary: {:?}\n  \
         server_root: {:?}\n  \
         project_root: {:?}\n  \
         config_dir: {:?}\n  \
         skill_root: {:?}",
        binary_path(),
        server_root(),
        project_root(),
        config_dir(),
        skill_root(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_path_exists() {
        let path = binary_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_binary_dir_exists() {
        let dir = binary_dir();
        assert!(dir.is_some());
        assert!(dir.unwrap().is_dir());
    }

    #[test]
    fn test_expand_tilde_with_home() {
        let expanded = expand_tilde("~/some/path");
        assert!(!expanded.to_string_lossy().contains('~'));
        assert!(expanded.is_absolute() || dirs::home_dir().is_none());
    }

    #[test]
    fn test_expand_tilde_preserves_absolute() {
        let path = "/usr/local/bin";
        assert_eq!(expand_tilde(path), PathBuf::from(path));
    }

    #[test]
    fn test_expand_tilde_preserves_relative() {
        let path = "./relative/path";
        assert_eq!(expand_tilde(path), PathBuf::from(path));
    }

    #[test]
    fn test_debug_paths_does_not_panic() {
        // Just ensure it doesn't panic
        let _ = debug_paths();
    }
}
```

## Update `util/mod.rs`

Add the new module:

```rust
pub mod files;
pub mod markdown;
pub mod paths;  // Add this line
```

## Update Config Loading

Refactor `src/config.rs` (or wherever `Config::load()` lives) to use the new utilities:

```rust
use crate::util::paths;

impl Config {
    /// Load configuration from the default location.
    pub fn load() -> Result<Self> {
        let config_dir = paths::config_dir()
            .ok_or_else(|| Error::config(
                format!("Could not locate config directory.\n{}", paths::debug_paths())
            ))?;

        let mut opts = conf::Options::default();
        opts.add_path(&config_dir);

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
```

## Update Tool Implementations

Any tool that needs to find skill content (sources, concepts, guides) should use `paths::skill_root()`:

```rust
use crate::util::paths;

// Example: in tools/concepts.rs
fn get_concepts_dir() -> Result<PathBuf> {
    let skill_root = paths::skill_root()
        .ok_or_else(|| Error::config(
            format!("Could not locate skill root directory.\n{}", paths::debug_paths())
        ))?;

    Ok(skill_root.join("concept-cards"))
}

// Example: in tools/sources.rs
fn get_sources_dir() -> Result<PathBuf> {
    let skill_root = paths::skill_root()
        .ok_or_else(|| Error::config(
            format!("Could not locate skill root directory.\n{}", paths::debug_paths())
        ))?;

    Ok(skill_root.join("sources-md"))
}
```

## Optional: Add Startup Logging

In `main.rs`, log the resolved paths at startup for debugging:

```rust
use crate::util::paths;

fn main() -> Result<()> {
    // Initialize logging first
    // ...

    // Log path resolution for debugging
    tracing::debug!("{}", paths::debug_paths());

    // Rest of startup...
}
```

## Environment Variables Supported

After this change, these environment variables can override path resolution:

| Variable | Purpose |
|----------|---------|
| `MUSIC_THEORY_CONFIG_DIR` | Override config directory location |
| `MUSIC_THEORY_SKILL_ROOT` | Override skill content root location |

Example usage in Claude Desktop config:

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "/path/to/music-theory-mcp",
      "env": {
        "MUSIC_THEORY_SKILL_ROOT": "/custom/path/to/ai-music-theory"
      }
    }
  }
}
```

## Checklist

- [ ] Add `dirs = "5"` to Cargo.toml
- [ ] Create `src/util/paths.rs` with the implementation above
- [ ] Add `pub mod paths;` to `src/util/mod.rs`
- [ ] Update `Config::load()` to use `paths::config_dir()`
- [ ] Update tool implementations to use `paths::skill_root()`
- [ ] Add debug logging of paths at startup (optional)
- [ ] Run tests: `cargo test`
- [ ] Test manually: run binary from `/` to verify it finds config
- [ ] Restart Claude Desktop and verify connection succeeds

## Testing

After implementation, verify it works from an arbitrary directory:

```bash
# Should fail before fix, succeed after
cd /
~/lab/music-comp/ai-music-theory/mcp-server/bin/music-theory-mcp

# Should show resolved paths in debug output
RUST_LOG=debug ~/lab/music-comp/ai-music-theory/mcp-server/bin/music-theory-mcp
```
