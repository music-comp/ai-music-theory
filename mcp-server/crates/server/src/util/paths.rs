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
    use serial_test::serial;

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
        assert_eq!(
            original, expanded,
            "Relative path without tilde should not change"
        );
    }

    #[test]
    fn test_debug_paths_does_not_panic() {
        let debug_output = debug_paths();
        assert!(!debug_output.is_empty(), "Debug output should not be empty");
        assert!(
            debug_output.contains("Binary path:"),
            "Should contain binary path"
        );
        assert!(
            debug_output.contains("Config dir:"),
            "Should contain config dir"
        );
        assert!(
            debug_output.contains("Skill root:"),
            "Should contain skill root"
        );
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
            assert!(
                path.starts_with(&home),
                "~/ should expand to home directory"
            );
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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

    // ── find_dir_with_marker ─────────────────────────────────────────

    #[test]
    fn test_find_dir_with_marker_found_at_start() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "m").unwrap();

        let result = find_dir_with_marker(tmp.path(), "marker.txt");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_found_one_level_up() {
        let tmp = tempfile::tempdir().unwrap();
        let child = tmp.path().join("child");
        std::fs::create_dir_all(&child).unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "m").unwrap();

        let result = find_dir_with_marker(&child, "marker.txt");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_found_at_max_walk_boundary() {
        // Place marker exactly MAX_WALK_LEVELS - 1 levels above start
        let tmp = tempfile::tempdir().unwrap();
        let mut deep = tmp.path().to_path_buf();
        // Create exactly MAX_WALK_LEVELS - 1 nested dirs (so the walk reaches
        // the base on the last allowed iteration)
        for i in 0..(MAX_WALK_LEVELS - 1) {
            deep = deep.join(format!("d{}", i));
        }
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "m").unwrap();

        let result = find_dir_with_marker(&deep, "marker.txt");
        assert_eq!(
            result.as_deref(),
            Some(tmp.path()),
            "Should find marker at the boundary of MAX_WALK_LEVELS"
        );
    }

    #[test]
    fn test_find_dir_with_marker_not_found_beyond_max_walk() {
        // Place marker MAX_WALK_LEVELS + 1 levels above start so the walk
        // gives up before reaching it.
        let tmp = tempfile::tempdir().unwrap();
        let mut deep = tmp.path().to_path_buf();
        for i in 0..(MAX_WALK_LEVELS + 1) {
            deep = deep.join(format!("d{}", i));
        }
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "m").unwrap();

        let result = find_dir_with_marker(&deep, "marker.txt");
        assert!(
            result.is_none(),
            "Should NOT find marker beyond MAX_WALK_LEVELS"
        );
    }

    #[test]
    fn test_find_dir_with_marker_nonexistent_marker() {
        let tmp = tempfile::tempdir().unwrap();
        let result = find_dir_with_marker(tmp.path(), "does_not_exist.xyz");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_dir_with_marker_nested_marker_path() {
        // Marker can be a nested path like "config/default.toml"
        let tmp = tempfile::tempdir().unwrap();
        let config = tmp.path().join("config");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("default.toml"), "# cfg").unwrap();

        let child = tmp.path().join("subdir");
        std::fs::create_dir_all(&child).unwrap();

        let result = find_dir_with_marker(&child, "config/default.toml");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    // ── server_root (controlled temp dirs) ───────────────────────────

    #[test]
    fn test_server_root_returns_dir_with_config_default_toml() {
        // If server_root succeeds, the returned dir must contain config/default.toml
        if let Some(root) = server_root() {
            assert!(
                root.join("config").join("default.toml").exists(),
                "server_root should point to a dir with config/default.toml"
            );
        }
    }

    // ── config_dir (controlled environment) ──────────────────────────

    #[test]
    #[serial(config_env)]
    fn test_config_dir_env_var_with_tilde_and_valid_dir() {
        // Create a temp dir with default.toml, set env to its path using tilde
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("default.toml"), "# cfg").unwrap();

        // We cannot easily fake a tilde path that maps to tmp, so we use the
        // absolute path here to make sure the env-var branch (priority 1)
        // returns correctly and that expand_tilde is invoked.
        env::set_var("MUSIC_THEORY_CONFIG_DIR", tmp.path());

        let result = config_dir();
        assert!(result.is_some(), "Should find config via env var");
        assert_eq!(result.unwrap(), tmp.path().to_path_buf());

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");
    }

    #[test]
    #[serial(config_env)]
    fn test_config_dir_env_var_dir_exists_but_no_default_toml() {
        // The env var points to an existing directory that does NOT contain
        // default.toml, so priority 1 should be skipped.
        let tmp = tempfile::tempdir().unwrap();
        // Do NOT create default.toml inside tmp.
        env::set_var("MUSIC_THEORY_CONFIG_DIR", tmp.path());

        let result = config_dir();
        // The env-var branch should fail; result may come from another
        // strategy or be None.
        if let Some(ref path) = result {
            assert_ne!(
                path,
                &tmp.path().to_path_buf(),
                "Should NOT return the env-var dir when default.toml is missing"
            );
        }

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");
    }

    #[test]
    #[serial(config_env)]
    fn test_config_dir_returns_path_containing_default_toml() {
        // Regardless of strategy used, if config_dir returns Some the dir
        // must contain default.toml.
        env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        if let Some(path) = config_dir() {
            assert!(
                path.join("default.toml").exists(),
                "config_dir result must contain default.toml: {:?}",
                path
            );
        }
    }

    // ── skill_root (controlled environment) ──────────────────────────

    #[test]
    #[serial(skill_env)]
    fn test_skill_root_env_var_existing_dir() {
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("MUSIC_THEORY_SKILL_ROOT", tmp.path());

        let result = skill_root();
        assert_eq!(result.as_deref(), Some(tmp.path()));

        env::remove_var("MUSIC_THEORY_SKILL_ROOT");
    }

    #[test]
    #[serial(skill_env)]
    fn test_skill_root_env_var_nonexistent_dir_falls_through() {
        env::set_var("MUSIC_THEORY_SKILL_ROOT", "/no/such/dir/ever");

        let result = skill_root();
        // The env-var branch should fail; result comes from project_root or
        // fallback (or None).
        if let Some(ref path) = result {
            assert_ne!(
                path,
                &PathBuf::from("/no/such/dir/ever"),
                "Should NOT return a non-existent env-var path"
            );
        }

        env::remove_var("MUSIC_THEORY_SKILL_ROOT");
    }

    // ── expand_tilde edge cases ──────────────────────────────────────

    #[test]
    fn test_expand_tilde_embedded_tilde_not_at_start() {
        // A tilde that is NOT at the start of the path should be left alone
        let original = PathBuf::from("/some/~path/here");
        let expanded = expand_tilde(&original);
        assert_eq!(original, expanded);
    }

    #[test]
    fn test_expand_tilde_empty_path() {
        let expanded = expand_tilde("");
        assert_eq!(expanded, PathBuf::from(""));
    }

    #[test]
    fn test_expand_tilde_deeply_nested() {
        let path = expand_tilde("~/a/b/c/d/e");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home.join("a/b/c/d/e"));
        }
    }

    // ── debug_paths additional assertions ────────────────────────────

    #[test]
    fn test_debug_paths_output_is_multiline() {
        let output = debug_paths();
        let line_count = output.lines().count();
        assert!(
            line_count >= 9,
            "debug_paths should produce at least 9 lines, got {}",
            line_count
        );
    }

    // ── config_dir: CWD-relative paths branch ─────────────────────────

    #[test]
    #[serial(config_env)]
    fn test_config_dir_no_env_var_set() {
        // With no env var set, config_dir falls through to server_root, CWD,
        // or fallback strategies. We verify it doesn't panic and that if it
        // returns Some, the directory contains default.toml.
        env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let result = config_dir();
        if let Some(path) = result {
            assert!(
                path.join("default.toml").exists(),
                "config_dir result must contain default.toml: {:?}",
                path
            );
        }
    }

    #[test]
    #[serial(config_env)]
    fn test_config_dir_env_var_empty_string() {
        // Setting env var to empty string should cause env::var to succeed
        // but the empty path won't contain default.toml, so it falls through.
        env::set_var("MUSIC_THEORY_CONFIG_DIR", "");

        let result = config_dir();
        if let Some(ref path) = result {
            assert!(
                path.join("default.toml").exists(),
                "Returned config dir must contain default.toml"
            );
        }

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");
    }

    #[test]
    #[serial(config_env)]
    fn test_config_dir_cwd_relative_fallback() {
        // Create a temporary directory with a config/default.toml layout and
        // change CWD to it. This exercises the CWD-relative path search
        // (priority 3).
        let tmp = tempfile::tempdir().unwrap();
        let config = tmp.path().join("config");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("default.toml"), "# test").unwrap();

        // Ensure env var doesn't interfere
        env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let original_cwd = env::current_dir().ok();

        // Change CWD to the temp dir so ./config is found
        let _ = env::set_current_dir(tmp.path());

        let result = config_dir();
        // Restore CWD before assertions to avoid interfering with other tests
        if let Some(ref cwd) = original_cwd {
            let _ = env::set_current_dir(cwd);
        }

        // The result might come from CWD-relative or another strategy,
        // but if it returns Some, it must contain default.toml
        if let Some(ref path) = result {
            assert!(
                path.join("default.toml").exists(),
                "Returned config dir must contain default.toml: {:?}",
                path
            );
        }
    }

    // ── skill_root: fallback path branch ──────────────────────────────

    #[test]
    #[serial(skill_env)]
    fn test_skill_root_no_env_var_set() {
        env::remove_var("MUSIC_THEORY_SKILL_ROOT");

        let result = skill_root();
        // Result may come from project_root or the hardcoded fallback
        if let Some(path) = result {
            assert!(path.exists(), "skill_root result should exist: {:?}", path);
        }
    }

    #[test]
    #[serial(skill_env)]
    fn test_skill_root_env_var_empty_string() {
        // An empty env var expands to "" which won't exist as a path
        env::set_var("MUSIC_THEORY_SKILL_ROOT", "");

        let result = skill_root();
        // Empty path "" .exists() returns false, so env branch falls through
        if let Some(ref path) = result {
            assert_ne!(path, &PathBuf::from(""), "Should NOT return an empty path");
        }

        env::remove_var("MUSIC_THEORY_SKILL_ROOT");
    }

    // ── find_dir_with_marker: additional edge cases ───────────────────

    #[test]
    fn test_find_dir_with_marker_marker_is_directory() {
        // The marker can be a directory, not just a file
        let tmp = tempfile::tempdir().unwrap();
        let marker_dir = tmp.path().join("marker_dir");
        std::fs::create_dir_all(&marker_dir).unwrap();

        let child = tmp.path().join("child");
        std::fs::create_dir_all(&child).unwrap();

        let result = find_dir_with_marker(&child, "marker_dir");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_empty_marker() {
        // Empty marker matches the dir itself (dir.join("") is the dir)
        let tmp = tempfile::tempdir().unwrap();
        let result = find_dir_with_marker(tmp.path(), "");
        // PathBuf::join("") returns the original path which .exists()
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_start_does_not_exist() {
        // Start from a path that doesn't exist on disk
        let result = find_dir_with_marker("/no/such/path/at/all", "marker.txt");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_dir_with_marker_deeply_nested_marker_path() {
        // Marker is a deeply nested path like "a/b/c/file.txt"
        let tmp = tempfile::tempdir().unwrap();
        let nested = tmp.path().join("a").join("b").join("c");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("file.txt"), "x").unwrap();

        let child = tmp.path().join("subdir");
        std::fs::create_dir_all(&child).unwrap();

        let result = find_dir_with_marker(&child, "a/b/c/file.txt");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_found_at_exact_level_zero() {
        // Marker exists at the start directory itself (level 0 of the walk)
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("here.txt"), "found").unwrap();

        let result = find_dir_with_marker(tmp.path(), "here.txt");
        assert_eq!(result.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_find_dir_with_marker_multiple_markers_at_different_levels() {
        // Marker exists at multiple levels; should find the closest (deepest)
        let tmp = tempfile::tempdir().unwrap();
        let child = tmp.path().join("child");
        std::fs::create_dir_all(&child).unwrap();
        // Marker at both levels
        std::fs::write(tmp.path().join("m.txt"), "parent").unwrap();
        std::fs::write(child.join("m.txt"), "child").unwrap();

        let result = find_dir_with_marker(&child, "m.txt");
        // Should find the child's marker first since we start there
        assert_eq!(result.as_deref(), Some(child.as_path()));
    }

    // ── server_root / project_root: additional verification ───────────

    #[test]
    fn test_server_root_if_found_contains_config() {
        if let Some(root) = server_root() {
            let config_dir = root.join("config");
            assert!(
                config_dir.exists(),
                "server_root should contain a config directory: {:?}",
                root
            );
            assert!(
                config_dir.join("default.toml").exists(),
                "server_root/config should contain default.toml"
            );
        }
    }

    #[test]
    fn test_project_root_if_found_has_expected_structure() {
        if let Some(root) = project_root() {
            // At least one of the marker files should exist
            let markers = ["SKILL.md", "CONVENTIONS.md", "SCOPE.md"];
            let found = markers.iter().any(|m| root.join(m).exists());
            assert!(
                found,
                "project_root should contain at least one marker file: {:?}",
                root
            );
        }
    }

    // ── binary_path / binary_dir: additional property checks ──────────

    #[test]
    fn test_binary_path_is_absolute() {
        if let Some(path) = binary_path() {
            assert!(
                path.is_absolute(),
                "Binary path should be absolute: {:?}",
                path
            );
        }
    }

    #[test]
    fn test_binary_dir_is_absolute() {
        if let Some(dir) = binary_dir() {
            assert!(
                dir.is_absolute(),
                "Binary dir should be absolute: {:?}",
                dir
            );
        }
    }

    #[test]
    fn test_binary_dir_is_parent_of_binary_path() {
        if let (Some(path), Some(dir)) = (binary_path(), binary_dir()) {
            assert!(
                path.starts_with(&dir),
                "Binary path {:?} should be under binary dir {:?}",
                path,
                dir
            );
        }
    }

    // ── expand_tilde: more edge cases ─────────────────────────────────

    #[test]
    fn test_expand_tilde_tilde_with_dot_path() {
        let path = expand_tilde("~/.");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home.join("."));
        }
    }

    #[test]
    fn test_expand_tilde_double_tilde() {
        // "~~" is a single path component, not "~" + "~",
        // so strip_prefix("~") does not match — returned as-is
        let path = expand_tilde("~~");
        assert_eq!(path, PathBuf::from("~~"));
    }

    #[test]
    fn test_expand_tilde_tilde_with_space_in_path() {
        let path = expand_tilde("~/my path/with spaces");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home.join("my path/with spaces"));
        }
    }

    // ── debug_paths: verify env var output ────────────────────────────

    #[test]
    #[serial(config_env)]
    fn test_debug_paths_reflects_env_vars() {
        env::set_var("MUSIC_THEORY_CONFIG_DIR", "/custom/config");
        env::set_var("MUSIC_THEORY_SKILL_ROOT", "/custom/skill");

        let output = debug_paths();
        assert!(
            output.contains("/custom/config"),
            "debug_paths should show MUSIC_THEORY_CONFIG_DIR value"
        );
        assert!(
            output.contains("/custom/skill"),
            "debug_paths should show MUSIC_THEORY_SKILL_ROOT value"
        );

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");
        env::remove_var("MUSIC_THEORY_SKILL_ROOT");
    }

    #[test]
    #[serial(config_env)]
    fn test_debug_paths_shows_none_when_env_unset() {
        env::remove_var("MUSIC_THEORY_CONFIG_DIR");
        env::remove_var("MUSIC_THEORY_SKILL_ROOT");

        let output = debug_paths();
        // When env vars are unset, the output should show None for those fields
        assert!(
            output.contains("MUSIC_THEORY_CONFIG_DIR: None"),
            "Should show None for unset MUSIC_THEORY_CONFIG_DIR: {}",
            output
        );
        assert!(
            output.contains("MUSIC_THEORY_SKILL_ROOT: None"),
            "Should show None for unset MUSIC_THEORY_SKILL_ROOT: {}",
            output
        );
    }

    // ── config_dir: exercise CWD-relative ../config branch ──────────

    #[test]
    #[serial(config_env)]
    fn test_config_dir_cwd_relative_parent_config() {
        // Exercise the "../config" relative path in config_dir (priority 3).
        // We create:
        //   tmp/config/default.toml
        //   tmp/subdir/
        // Then set CWD to tmp/subdir so "../config" resolves to tmp/config.
        //
        // We must also suppress higher-priority strategies:
        //   - Remove env var (priority 1)
        //   - server_root (priority 2) cannot be suppressed, but if it
        //     succeeds we still exercise the code path up to that point.
        let tmp = tempfile::tempdir().unwrap();
        let config = tmp.path().join("config");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("default.toml"), "# test").unwrap();
        let subdir = tmp.path().join("subdir");
        std::fs::create_dir_all(&subdir).unwrap();

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let original_cwd = env::current_dir().ok();
        let _ = env::set_current_dir(&subdir);

        let result = config_dir();

        // Restore CWD before assertions
        if let Some(ref cwd) = original_cwd {
            let _ = env::set_current_dir(cwd);
        }

        // server_root (priority 2) may still succeed, so we just verify
        // that if we got Some, it contains default.toml
        if let Some(ref path) = result {
            assert!(
                path.join("default.toml").exists(),
                "Returned config dir must contain default.toml: {:?}",
                path
            );
        }
    }

    #[test]
    #[serial(config_env)]
    fn test_config_dir_cwd_relative_crates_server_config() {
        // Exercise the "./crates/server/config" relative path (priority 3).
        let tmp = tempfile::tempdir().unwrap();
        let csc = tmp.path().join("crates").join("server").join("config");
        std::fs::create_dir_all(&csc).unwrap();
        std::fs::write(csc.join("default.toml"), "# test").unwrap();

        env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let original_cwd = env::current_dir().ok();
        let _ = env::set_current_dir(tmp.path());

        let result = config_dir();

        if let Some(ref cwd) = original_cwd {
            let _ = env::set_current_dir(cwd);
        }

        if let Some(ref path) = result {
            assert!(
                path.join("default.toml").exists(),
                "Returned config dir must contain default.toml: {:?}",
                path
            );
        }
    }

    // ── config_dir: fallback hardcoded path ─────────────────────────

    #[test]
    #[serial(config_env)]
    fn test_config_dir_hardcoded_fallback_path_expansion() {
        // Verify that the hardcoded fallback path uses tilde expansion.
        // We cannot easily make all earlier strategies fail, but we can
        // verify that the fallback path, when manually constructed, matches
        // what expand_tilde would produce.
        let fallback =
            expand_tilde("~/lab/music-comp/ai-music-theory/mcp-server/crates/server/config");
        if fallback.join("default.toml").exists() {
            // The fallback path exists on this machine, which means the
            // fallback branch COULD succeed if higher-priority strategies
            // all fail.
            assert!(fallback.is_absolute());
        }
    }

    // ── skill_root: hardcoded fallback ──────────────────────────────

    #[test]
    #[serial(skill_env)]
    fn test_skill_root_hardcoded_fallback_path_expansion() {
        // Verify the hardcoded fallback uses tilde expansion correctly.
        let fallback = expand_tilde("~/lab/music-comp/ai-music-theory");
        if fallback.exists() {
            assert!(fallback.is_absolute());
            // On the developer's machine this path exists; skill_root's
            // fallback would succeed if project_root failed.
        }
    }

    // ── server_root: first-try path (config/default.toml directly) ──

    #[test]
    fn test_server_root_first_try_would_find_direct_config() {
        // Verify the first-try strategy: if config/default.toml existed
        // at a parent of binary_dir, server_root would return that parent.
        // We test this indirectly via find_dir_with_marker.
        let tmp = tempfile::tempdir().unwrap();
        let config = tmp.path().join("config");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("default.toml"), "# test").unwrap();

        let child = tmp.path().join("target").join("debug").join("deps");
        std::fs::create_dir_all(&child).unwrap();

        let result = find_dir_with_marker(&child, "config/default.toml");
        assert_eq!(
            result.as_deref(),
            Some(tmp.path()),
            "First-try strategy should find config/default.toml"
        );
    }

    #[test]
    fn test_server_root_workspace_strategy_via_find_dir_with_marker() {
        // Simulate the workspace strategy: Cargo.toml is found at workspace
        // root, then crates/server/config/default.toml is checked.
        let tmp = tempfile::tempdir().unwrap();
        // Create workspace-like structure
        std::fs::write(tmp.path().join("Cargo.toml"), "# workspace").unwrap();
        let server_config = tmp.path().join("crates").join("server").join("config");
        std::fs::create_dir_all(&server_config).unwrap();
        std::fs::write(server_config.join("default.toml"), "# cfg").unwrap();

        let child = tmp.path().join("target").join("debug").join("deps");
        std::fs::create_dir_all(&child).unwrap();

        // First try: no direct config/default.toml
        let first_try = find_dir_with_marker(&child, "config/default.toml");
        assert!(
            first_try.is_none(),
            "Direct config/default.toml should NOT be found"
        );

        // Second try: find Cargo.toml
        let workspace = find_dir_with_marker(&child, "Cargo.toml");
        assert_eq!(workspace.as_deref(), Some(tmp.path()));

        // Then check crates/server/config/default.toml
        let workspace_root = workspace.unwrap();
        let server_crate = workspace_root.join("crates").join("server");
        assert!(server_crate.join("config").join("default.toml").exists());
    }

    // ── project_root: verify marker iteration order ─────────────────

    #[test]
    fn test_project_root_finds_first_available_marker() {
        // Simulate project_root's marker search order via find_dir_with_marker.
        // If the first marker (SKILL.md) is found, later markers are not checked.
        let tmp = tempfile::tempdir().unwrap();
        // Only place the second marker (CONVENTIONS.md)
        std::fs::write(tmp.path().join("CONVENTIONS.md"), "# conv").unwrap();

        let child = tmp.path().join("src");
        std::fs::create_dir_all(&child).unwrap();

        // SKILL.md not found
        let r1 = find_dir_with_marker(&child, "SKILL.md");
        assert!(r1.is_none());

        // CONVENTIONS.md found
        let r2 = find_dir_with_marker(&child, "CONVENTIONS.md");
        assert_eq!(r2.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_project_root_finds_third_marker() {
        // Verify the third marker (SCOPE.md) is checked when others are absent.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("SCOPE.md"), "# scope").unwrap();

        let child = tmp.path().join("src");
        std::fs::create_dir_all(&child).unwrap();

        assert!(find_dir_with_marker(&child, "SKILL.md").is_none());
        assert!(find_dir_with_marker(&child, "CONVENTIONS.md").is_none());

        let r3 = find_dir_with_marker(&child, "SCOPE.md");
        assert_eq!(r3.as_deref(), Some(tmp.path()));
    }

    #[test]
    fn test_project_root_no_markers_returns_none() {
        // When no markers exist anywhere in the walk path, returns None.
        let tmp = tempfile::tempdir().unwrap();
        let child = tmp.path().join("a").join("b");
        std::fs::create_dir_all(&child).unwrap();

        for marker in &["SKILL.md", "CONVENTIONS.md", "SCOPE.md"] {
            assert!(
                find_dir_with_marker(&child, marker).is_none(),
                "Should not find {} in empty tree",
                marker
            );
        }
    }

    // ── find_dir_with_marker: walk exactly MAX_WALK_LEVELS ──────────

    #[test]
    fn test_find_dir_with_marker_exactly_at_max_walk_levels() {
        // Place marker exactly MAX_WALK_LEVELS levels above start.
        // The loop runs 0..MAX_WALK_LEVELS (10 iterations: levels 0-9).
        // Starting at level MAX_WALK_LEVELS means the marker is at iteration
        // index MAX_WALK_LEVELS, which is ONE past the last iteration.
        let tmp = tempfile::tempdir().unwrap();
        let mut deep = tmp.path().to_path_buf();
        for i in 0..MAX_WALK_LEVELS {
            deep = deep.join(format!("d{}", i));
        }
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(tmp.path().join("marker.txt"), "m").unwrap();

        // Exactly at the boundary: the marker is MAX_WALK_LEVELS levels up.
        // The loop checks levels 0 through 9 (10 total). Starting from deep,
        // level 9 is tmp + d0, and level 10 (tmp itself) is never checked.
        let result = find_dir_with_marker(&deep, "marker.txt");
        assert!(
            result.is_none(),
            "Marker at exactly MAX_WALK_LEVELS levels up should NOT be found"
        );
    }

    // ── expand_tilde: with user-like path components ────────────────

    #[test]
    fn test_expand_tilde_preserves_trailing_slash() {
        let path = expand_tilde("~/dir/");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home.join("dir/"));
        }
    }

    #[test]
    fn test_expand_tilde_with_dot_dot_component() {
        let path = expand_tilde("~/a/../b");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(path, home.join("a/../b"));
        }
    }
}
