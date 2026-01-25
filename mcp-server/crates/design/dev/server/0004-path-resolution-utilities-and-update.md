# Implementation Plan: Fix Path Resolution for Claude Desktop

**Date:** 2026-01-25
**Goal:** Fix configuration loading failure when MCP server is run by Claude Desktop from arbitrary working directory
**Error:** `Configuration error: Failed to build config: no configurations added`

---

## Problem Statement

The MCP server fails to start when invoked by Claude Desktop because config loading uses CWD-relative paths (`./config`, `../config`, `./crates/server/config`). When Claude Desktop runs the binary, the current working directory is arbitrary (not the project root), causing config file lookup to fail.

**Current behavior:**

- Binary location: `mcp-server/target/release/music-theory-mcp` or `mcp-server/bin/music-theory-mcp`
- Config file location: `mcp-server/crates/server/config/default.toml`
- Problem: Config loading searches relative to CWD, not relative to binary location

**Required behavior:**

- Config loading should work regardless of CWD
- Find config by walking up from binary location
- Support environment variable overrides
- Fall back to sensible defaults

---

## Solution Overview

Create `util/paths.rs` module that:

1. Finds binary location using `std::env::current_exe()`
2. Walks up directory tree looking for project markers
3. Provides functions for finding config dir and skill content root
4. Supports environment variable overrides (`MUSIC_THEORY_CONFIG_DIR`, `MUSIC_THEORY_SKILL_ROOT`)

---

## Implementation Steps

### Step 1: Add Dependencies

**File:** `mcp-server/Cargo.toml` (workspace root)

Add to `[workspace.dependencies]`:

```toml
dirs = "5"
```

**File:** `mcp-server/crates/server/Cargo.toml`

Add to `[dependencies]`:

```toml
dirs = { workspace = true }
```

---

### Step 2: Create Path Resolution Module

**File:** `mcp-server/crates/server/src/util/paths.rs` (NEW)

Create complete module with:

- `binary_path()` - Get absolute path to running binary
- `binary_dir()` - Get directory containing binary
- `server_root()` - Find MCP server crate root by walking up from binary
  - Looks for marker: `config/default.toml`
  - Handles: `mcp-server/target/{release,debug}/binary`, `mcp-server/bin/binary`
  - Also checks `crates/server` subdirectory if at workspace root
- `project_root()` - Find ai-music-theory project root
  - Looks for markers: `SKILL.md`, `CONVENTIONS.md`, `SCOPE.md` (all exist)
- `config_dir()` - Find config directory with precedence:
  1. `MUSIC_THEORY_CONFIG_DIR` env var
  2. `{server_root}/config` (found by walking up from binary)
  3. CWD-relative paths (for development compatibility)
  4. Fallback to `~/lab/music-comp/ai-music-theory/mcp-server/crates/server/config`
- `skill_root()` - Find skill content root with precedence:
  1. `MUSIC_THEORY_SKILL_ROOT` env var
  2. Walk up from binary to find project root
  3. Fallback to `~/lab/music-comp/ai-music-theory`
- `expand_tilde()` - Resolve `~` to home directory
- `debug_paths()` - Human-readable debug output

**Include comprehensive tests:**

- `test_binary_path_exists`
- `test_binary_dir_exists`
- `test_expand_tilde_*` (3 tests)
- `test_debug_paths_does_not_panic`

---

### Step 3: Register Module

**File:** `mcp-server/crates/server/src/util/mod.rs`

Change from:

```rust
pub mod files;
```

To:

```rust
pub mod files;
pub mod paths;
```

---

### Step 4: Update Config Loading

**File:** `mcp-server/crates/server/src/config.rs`

**Current code (lines 139-156):**

```rust
pub fn load() -> Result<Self> {
    let mut opts = conf::Options::default();
    opts.add_path("./config")
        .add_path("../config")
        .add_path("./crates/server/config");

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
```

**New code:**

```rust
pub fn load() -> Result<Self> {
    use crate::util::paths;

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
```

**Key changes:**

- Import `crate::util::paths`
- Call `paths::config_dir()` to find config directory
- Use single `add_path()` with found directory instead of multiple CWD-relative paths
- Include `paths::debug_paths()` in error message for troubleshooting

---

### Step 5: Add Optional Debug Logging

**File:** `mcp-server/crates/server/src/main.rs`

After logging initialization (around line 30), add:

```rust
log::debug!(
    binary = ?crate::util::paths::binary_path(),
    server_root = ?crate::util::paths::server_root(),
    config_dir = ?crate::util::paths::config_dir(),
    skill_root = ?crate::util::paths::skill_root();
    "Path resolution"
);
```

This helps debug path issues during development and troubleshooting.

---

## Files Changed Summary

| File | Type | Change |
|------|------|--------|
| `Cargo.toml` | Modify | Add `dirs = "5"` to workspace dependencies |
| `crates/server/Cargo.toml` | Modify | Add `dirs = { workspace = true }` |
| `crates/server/src/util/paths.rs` | Create | New path resolution module (~275 lines) |
| `crates/server/src/util/mod.rs` | Modify | Add `pub mod paths;` |
| `crates/server/src/config.rs` | Modify | Update `Config::load()` to use `paths::config_dir()` |
| `crates/server/src/main.rs` | Modify | Add debug logging of resolved paths (optional) |

---

## Environment Variables

After this change, users can override path resolution:

| Variable | Purpose | Example |
|----------|---------|---------|
| `MUSIC_THEORY_CONFIG_DIR` | Override config directory | `/custom/path/to/config` |
| `MUSIC_THEORY_SKILL_ROOT` | Override skill content root | `/custom/path/to/ai-music-theory` |

**Claude Desktop config example:**

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

---

## Verification Plan

### 1. Unit Tests

```bash
cargo test
```

Should see 6 new tests pass in `util::paths::tests`.

### 2. Build and Run

```bash
cargo build --release
```

### 3. Test from Arbitrary Directory

```bash
# Should fail BEFORE fix, succeed AFTER
cd /
~/lab/music-comp/ai-music-theory/mcp-server/target/release/music-theory-mcp

# Should show resolved paths in debug output
cd /tmp
RUST_LOG=debug ~/lab/music-comp/ai-music-theory/mcp-server/target/release/music-theory-mcp
```

### 4. Test with Claude Desktop

1. Update `claude_desktop_config.json`:

   ```json
   {
     "mcpServers": {
       "music-theory": {
         "command": "/Users/YOUR_USERNAME/lab/music-comp/ai-music-theory/mcp-server/target/release/music-theory-mcp"
       }
     }
   }
   ```

2. Restart Claude Desktop (quit and relaunch)

3. Open new conversation and verify:
   - Server appears in MCP servers list
   - No error in developer console
   - Tools are available and working (`list_concepts`, etc.)

4. Check logs in `~/Library/Logs/Claude/mcp-server-music-theory.log`:
   - Should see successful config loading
   - Should NOT see "no configurations added" error
   - With `RUST_LOG=debug`, should see path resolution debug output

### 5. Test Environment Variable Override

```bash
# Create alternate config
mkdir -p /tmp/test-config
cp ~/lab/music-comp/ai-music-theory/mcp-server/crates/server/config/default.toml /tmp/test-config/

# Test override
MUSIC_THEORY_CONFIG_DIR=/tmp/test-config \
  ~/lab/music-comp/ai-music-theory/mcp-server/target/release/music-theory-mcp
```

Should use config from `/tmp/test-config/default.toml`.

---

## Success Criteria

- ✅ Binary can find config from any working directory
- ✅ All tests pass (including 6 new path tests)
- ✅ No clippy warnings
- ✅ Claude Desktop successfully connects to server
- ✅ No "no configurations added" error in logs
- ✅ All 8 MCP tools work correctly
- ✅ Environment variable overrides work
- ✅ Debug logging shows correct path resolution

---

## Rollback Plan

If issues occur:

1. Revert changes to `config.rs` (restore CWD-relative paths)
2. Remove `util/paths.rs`
3. Remove `pub mod paths;` from `util/mod.rs`
4. Remove `dirs` dependency
5. Git revert the commit

This restores original behavior (works from project root, fails from arbitrary directory).

---

## Notes

- The `dirs` crate is lightweight (no platform-specific issues)
- Path walking is limited to 10 levels for safety
- Fallback paths use the current user's home directory
- Project markers (SKILL.md, etc.) already exist at correct locations
- Config file already exists at expected location
- All current tool implementations continue to work unchanged (they use `config.paths.*_path()`)
