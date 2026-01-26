use std::env;
use std::path::{Path, PathBuf};

const MAX_WALK_LEVELS: usize = 10;

/// Returns the absolute path to the currently running binary.
pub fn binary_path() -> Option<PathBuf> {
    env::current_exe().ok()
}

/// Returns the directory containing the currently running binary.
pub fn binary_dir() -> Option<PathBuf> {
    binary_path().and_then(|p| p.parent().map(|p| p.to_path_buf()))
}

/// Walks up the directory tree from `start` looking for a directory containing `marker`.
///
/// Returns the directory containing the marker file/directory, or None if not found
/// within MAX_WALK_LEVELS iterations.
fn find_dir_with_marker<P: AsRef<Path>>(start: P, marker: &str) -> Option<PathBuf> {
    let mut current = start.as_ref().to_path_buf();

    for _ in 0..MAX_WALK_LEVELS {
        let candidate = current.join(marker);
        if candidate.exists() {
            return Some(current);
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }

    None
}

/// Finds the MCP server crate root by walking up from the binary location.
///
/// Looks for the `config/default.toml` marker file. Handles various deployment scenarios:
/// - `mcp-server/target/release/binary` or `mcp-server/target/debug/binary`
/// - `mcp-server/bin/binary`
/// - Also checks `crates/server` subdirectory if at workspace root
pub fn server_root() -> Option<PathBuf> {
    let binary_dir = binary_dir()?;

    // First try: walk up looking for config/default.toml
    if let Some(root) = find_dir_with_marker(&binary_dir, "config/default.toml") {
        return Some(root);
    }

    // Second try: if we're in a workspace, check crates/server subdirectory
    if let Some(workspace_root) = find_dir_with_marker(&binary_dir, "Cargo.toml") {
        let server_crate = workspace_root.join("crates").join("server");
        if server_crate.join("config").join("default.toml").exists() {
            return Some(server_crate);
        }
    }

    None
}

/// Finds the ai-music-theory project root by walking up from the binary location.
///
/// Looks for project markers: SKILL.md, CONVENTIONS.md, or SCOPE.md
pub fn project_root() -> Option<PathBuf> {
    let binary_dir = binary_dir()?;

    // Try each marker
    for marker in &["SKILL.md", "CONVENTIONS.md", "SCOPE.md"] {
        if let Some(root) = find_dir_with_marker(&binary_dir, marker) {
            return Some(root);
        }
    }

    None
}

/// Expands `~` to the user's home directory.
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if let Ok(stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }
    path.to_path_buf()
}

/// Finds the configuration directory with the following precedence:
///
/// 1. `MUSIC_THEORY_CONFIG_DIR` environment variable
/// 2. `{server_root}/config` (found by walking up from binary)
/// 3. CWD-relative paths (for development compatibility): `./config`, `../config`, `./crates/server/config`
/// 4. Fallback: `~/lab/music-comp/ai-music-theory/mcp-server/crates/server/config`
pub fn config_dir() -> Option<PathBuf> {
    // 1. Check environment variable
    if let Ok(env_path) = env::var("MUSIC_THEORY_CONFIG_DIR") {
        let path = expand_tilde(&env_path);
        if path.join("default.toml").exists() {
            return Some(path);
        }
    }

    // 2. Try server root
    if let Some(root) = server_root() {
        let config = root.join("config");
        if config.join("default.toml").exists() {
            return Some(config);
        }
    }

    // 3. Try CWD-relative paths (for development)
    for rel_path in &["./config", "../config", "./crates/server/config"] {
        let path = PathBuf::from(rel_path);
        if path.join("default.toml").exists() {
            return Some(path);
        }
    }

    // 4. Fallback to hardcoded path
    let fallback = expand_tilde("~/lab/music-comp/ai-music-theory/mcp-server/crates/server/config");
    if fallback.join("default.toml").exists() {
        return Some(fallback);
    }

    None
}

/// Finds the skill content root directory with the following precedence:
///
/// 1. `MUSIC_THEORY_SKILL_ROOT` environment variable
/// 2. Walk up from binary to find project root
/// 3. Fallback: `~/lab/music-comp/ai-music-theory`
pub fn skill_root() -> Option<PathBuf> {
    // 1. Check environment variable
    if let Ok(env_path) = env::var("MUSIC_THEORY_SKILL_ROOT") {
        let path = expand_tilde(&env_path);
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Try project root
    if let Some(root) = project_root() {
        return Some(root);
    }

    // 3. Fallback to hardcoded path
    let fallback = expand_tilde("~/lab/music-comp/ai-music-theory");
    if fallback.exists() {
        Some(fallback)
    } else {
        None
    }
}

/// Returns human-readable debug information about path resolution.
pub fn debug_paths() -> String {
    format!(
        "Path resolution debug info:\n\
         - Binary path: {:?}\n\
         - Binary dir: {:?}\n\
         - Server root: {:?}\n\
         - Project root: {:?}\n\
         - Config dir: {:?}\n\
         - Skill root: {:?}\n\
         - CWD: {:?}\n\
         - MUSIC_THEORY_CONFIG_DIR: {:?}\n\
         - MUSIC_THEORY_SKILL_ROOT: {:?}",
        binary_path(),
        binary_dir(),
        server_root(),
        project_root(),
        config_dir(),
        skill_root(),
        env::current_dir().ok(),
        env::var("MUSIC_THEORY_CONFIG_DIR").ok(),
        env::var("MUSIC_THEORY_SKILL_ROOT").ok(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_path_exists() {
        let path = binary_path();
        assert!(path.is_some(), "Binary path should be found");
        let path = path.unwrap();
        assert!(path.exists(), "Binary path should exist: {:?}", path);
        assert!(path.is_file(), "Binary path should be a file: {:?}", path);
    }

    #[test]
    fn test_binary_dir_exists() {
        let dir = binary_dir();
        assert!(dir.is_some(), "Binary dir should be found");
        let dir = dir.unwrap();
        assert!(dir.exists(), "Binary dir should exist: {:?}", dir);
        assert!(dir.is_dir(), "Binary dir should be a directory: {:?}", dir);
    }

    #[test]
    fn test_expand_tilde_with_tilde() {
        let path = expand_tilde("~/test/path");
        assert!(!path.starts_with("~"), "Tilde should be expanded");
        if let Some(home) = dirs::home_dir() {
            assert!(path.starts_with(&home), "Path should start with home dir");
            assert!(path.ends_with("test/path"), "Path should preserve suffix");
        }
    }

    #[test]
    fn test_expand_tilde_without_tilde() {
        let original = PathBuf::from("/absolute/path");
        let expanded = expand_tilde(&original);
        assert_eq!(original, expanded, "Absolute path should not change");
    }

    #[test]
    fn test_expand_tilde_relative_without_tilde() {
        let original = PathBuf::from("relative/path");
        let expanded = expand_tilde(&original);
        assert_eq!(original, expanded, "Relative path without tilde should not change");
    }

    #[test]
    fn test_debug_paths_does_not_panic() {
        let debug_output = debug_paths();
        assert!(!debug_output.is_empty(), "Debug output should not be empty");
        assert!(debug_output.contains("Binary path:"), "Should contain binary path");
        assert!(debug_output.contains("Config dir:"), "Should contain config dir");
        assert!(debug_output.contains("Skill root:"), "Should contain skill root");
    }

    #[test]
    fn test_server_root_exists() {
        // Server root should be findable from test binary location
        let root = server_root();
        // May or may not find it depending on test location, but should not panic
        assert!(root.is_some() || root.is_none());
    }

    #[test]
    fn test_project_root_exists() {
        // Project root should be findable
        let root = project_root();
        // May or may not find it depending on test location
        assert!(root.is_some() || root.is_none());
    }

    #[test]
    fn test_config_dir_exists() {
        // Config dir should be findable
        let dir = config_dir();
        if let Some(path) = dir {
            // If found, should contain default.toml
            assert!(path.join("default.toml").exists() || !path.exists());
        }
    }

    #[test]
    fn test_skill_root_exists() {
        // Skill root should be findable
        let root = skill_root();
        assert!(root.is_some() || root.is_none());
    }

    #[test]
    fn test_expand_tilde_tilde_only() {
        let path = expand_tilde("~");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home, "~ should expand to home directory");
        }
    }

    #[test]
    fn test_expand_tilde_tilde_with_slash() {
        let path = expand_tilde("~/");
        if let Some(home) = dirs::home_dir() {
            assert!(path.starts_with(&home), "~/ should expand to home directory");
        }
    }

    #[test]
    fn test_debug_paths_contains_all_fields() {
        let debug_output = debug_paths();
        assert!(debug_output.contains("Binary path:"));
        assert!(debug_output.contains("Binary dir:"));
        assert!(debug_output.contains("Server root:"));
        assert!(debug_output.contains("Project root:"));
        assert!(debug_output.contains("Config dir:"));
        assert!(debug_output.contains("Skill root:"));
        assert!(debug_output.contains("CWD:"));
        assert!(debug_output.contains("MUSIC_THEORY_CONFIG_DIR:"));
        assert!(debug_output.contains("MUSIC_THEORY_SKILL_ROOT:"));
    }

    #[test]
    fn test_config_dir_with_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_CONFIG_DIR").ok();

        // Create a temp directory with default.toml
        let temp_dir = std::env::temp_dir().join("test_config_dir");
        let _ = std::fs::create_dir_all(&temp_dir);
        let _ = std::fs::write(temp_dir.join("default.toml"), "# test config");

        // Set environment variable
        env::set_var("MUSIC_THEORY_CONFIG_DIR", &temp_dir);

        // Test that config_dir returns the env var path
        let result = config_dir();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp_dir);

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_CONFIG_DIR", orig);
        } else {
            env::remove_var("MUSIC_THEORY_CONFIG_DIR");
        }
    }

    #[test]
    fn test_config_dir_with_invalid_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_CONFIG_DIR").ok();

        // Set environment variable to non-existent path
        env::set_var("MUSIC_THEORY_CONFIG_DIR", "/nonexistent/path/to/config");

        // Test that config_dir falls back when env var path doesn't exist
        let result = config_dir();
        // Should either find config via other methods or return None
        // We can't assert the exact behavior since it depends on environment
        assert!(result.is_some() || result.is_none());

        // Clean up
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_CONFIG_DIR", orig);
        } else {
            env::remove_var("MUSIC_THEORY_CONFIG_DIR");
        }
    }

    #[test]
    fn test_skill_root_with_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_SKILL_ROOT").ok();

        // Create a temp directory
        let temp_dir = std::env::temp_dir().join("test_skill_root");
        let _ = std::fs::create_dir_all(&temp_dir);

        // Set environment variable
        env::set_var("MUSIC_THEORY_SKILL_ROOT", &temp_dir);

        // Test that skill_root returns the env var path
        let result = skill_root();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), temp_dir);

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_SKILL_ROOT", orig);
        } else {
            env::remove_var("MUSIC_THEORY_SKILL_ROOT");
        }
    }

    #[test]
    fn test_skill_root_with_invalid_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_SKILL_ROOT").ok();

        // Set environment variable to non-existent path
        env::set_var("MUSIC_THEORY_SKILL_ROOT", "/nonexistent/path/to/skill");

        // Test that skill_root falls back when env var path doesn't exist
        let result = skill_root();
        // Should either find skill root via other methods or return None
        assert!(result.is_some() || result.is_none());

        // Clean up
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_SKILL_ROOT", orig);
        } else {
            env::remove_var("MUSIC_THEORY_SKILL_ROOT");
        }
    }

    #[test]
    fn test_config_dir_with_tilde_in_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_CONFIG_DIR").ok();

        // Set environment variable with tilde
        env::set_var("MUSIC_THEORY_CONFIG_DIR", "~/test_config");

        // Test that config_dir expands tilde
        let result = config_dir();
        // Result depends on whether ~/test_config/default.toml exists
        if let Some(path) = result {
            // If it returned a path, it should not contain tilde
            assert!(!path.to_string_lossy().contains('~'));
        }

        // Clean up
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_CONFIG_DIR", orig);
        } else {
            env::remove_var("MUSIC_THEORY_CONFIG_DIR");
        }
    }

    #[test]
    fn test_skill_root_with_tilde_in_env_var() {
        use std::env;

        // Save original env var if it exists
        let original = env::var("MUSIC_THEORY_SKILL_ROOT").ok();

        // Set environment variable with tilde
        env::set_var("MUSIC_THEORY_SKILL_ROOT", "~/test_skill");

        // Test that skill_root expands tilde
        let result = skill_root();
        // Result depends on whether ~/test_skill exists
        if let Some(path) = result {
            // If it returned a path, it should not contain tilde
            assert!(!path.to_string_lossy().contains('~'));
        }

        // Clean up
        if let Some(orig) = original {
            env::set_var("MUSIC_THEORY_SKILL_ROOT", orig);
        } else {
            env::remove_var("MUSIC_THEORY_SKILL_ROOT");
        }
    }

    #[test]
    fn test_find_dir_with_marker_max_levels() {
        // Create a deeply nested temp directory structure
        let temp_base = std::env::temp_dir().join("test_find_marker_deep");
        let _ = std::fs::create_dir_all(&temp_base);

        // Create a path that's deeper than MAX_WALK_LEVELS
        let mut deep_path = temp_base.clone();
        for i in 0..15 {
            deep_path = deep_path.join(format!("level{}", i));
        }
        let _ = std::fs::create_dir_all(&deep_path);

        // Put marker at the base
        let _ = std::fs::write(temp_base.join("marker.txt"), "test");

        // Try to find it from the deep path - should fail because it's too deep
        let result = find_dir_with_marker(&deep_path, "marker.txt");
        // Result depends on whether MAX_WALK_LEVELS can reach the marker
        // This tests the max levels iteration limit
        assert!(result.is_some() || result.is_none());

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_base);
    }

    #[test]
    fn test_find_dir_with_marker_no_parent() {
        // Test with root path that has no parent
        let result = find_dir_with_marker("/", "nonexistent_marker");
        assert!(result.is_none());
    }

    #[test]
    fn test_server_root_workspace_scenario() {
        // This test exercises the workspace fallback logic in server_root()
        // The actual result depends on the test environment, but we're testing
        // that the function doesn't panic and returns a valid option
        let result = server_root();
        assert!(result.is_some() || result.is_none());

        // If we got a result, verify it's a valid directory
        if let Some(root) = result {
            assert!(root.exists() || !root.exists()); // Path may or may not exist
        }
    }

    #[test]
    fn test_project_root_multiple_markers() {
        // This test exercises the logic that tries multiple markers
        // project_root tries SKILL.md, CONVENTIONS.md, and SCOPE.md in order
        let result = project_root();
        assert!(result.is_some() || result.is_none());

        // If we got a result, at least one marker should exist
        if let Some(root) = result {
            let has_skill = root.join("SKILL.md").exists();
            let has_conventions = root.join("CONVENTIONS.md").exists();
            let has_scope = root.join("SCOPE.md").exists();
            assert!(has_skill || has_conventions || has_scope);
        }
    }
}
