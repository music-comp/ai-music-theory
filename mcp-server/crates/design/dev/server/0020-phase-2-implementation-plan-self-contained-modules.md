# Phase 2 Implementation Plan: Self-Contained Modules

**Milestones:** 2.1 (Cache), 2.2 (Search Adapters), 2.3 (Graph Adapters)
**Estimated effort:** 3 sittings (one per milestone)
**Dependencies:** Phase 1 complete (1.2 specifically for iso8601_now used by cache)

---

## Milestone 2.1: Cache Module -> fabryk-cli

### Objective

Parameterize hardcoded paths, then move the entire cache management system (~500 lines) to fabryk-cli. Leave only project-specific constants in ai-music-theory.

### What Exists in Fabryk Today

fabryk-cli has zero cache management. `fabryk-core::ConfigProvider` has `cache_path(cache_type: &str)` which provides per-backend path resolution -- this is the hook for parameterizing the hardcoded paths.

### Pre-Work: Parameterize Hardcoded Paths

Before any code moves, fix the two hardcoded-path locations:

**`cache_status()` (cache.rs:230-255):**
Currently checks `data/graphs/concept_graph.json`, `.tantivy-index`, `.cache/vector/vector-cache.json`. Change to:

```rust
pub fn cache_status<C: ConfigProvider>(config: &C) -> Result<CacheStatusReport> {
    let base = config.base_path()?;
    let manifest = load_manifest(&base)?;
    
    let graph_present = config.cache_path("graph")
        .map(|p| p.join("concept_graph.json").exists())
        .unwrap_or(false);
    let fts_present = config.cache_path("fts")
        .map(|p| p.exists())
        .unwrap_or(false);
    let vector_present = config.cache_path("vector")
        .map(|p| p.join("vector-cache.json").exists())
        .unwrap_or(false);
    // ... rest unchanged
}
```

**`package_cache()` (cache.rs:437-466):**
Same treatment -- use `ConfigProvider::cache_path()` instead of hardcoded paths in the `match` arms.

### Changes in textrynum (fabryk-cli)

#### 1. Create `src/cache.rs`

Move from ai-music-theory cache.rs (~450 lines of logic):

**Types:**
- `CacheBackend` enum (Graph, Fts, Vector) + Display + FromStr
- `CacheManifest`, `CacheEntry` (serde structs)
- `BackendStatus`, `CacheStatusReport` + Display

**Functions (parameterized):**
- `archive_name(backend, version, project_prefix)` -- project_prefix is now a parameter
- `release_url(backend, version, base_url, project_prefix)` -- base_url is now a parameter
- `checksum_url(backend, version, base_url, project_prefix)`
- `load_manifest(base_path)`, `save_manifest(base_path, manifest)`
- `shell_download(url, dest)`, `verify_checksum(archive, expected_hash)`, `extract_archive(archive, target_dir)`
- `download_cache<C: ConfigProvider>(backend, config, project_prefix, base_url, force)`
- `package_cache<C: ConfigProvider>(backend, config, output_dir, version, project_prefix)`
- `parse_backend_arg(arg)`
- `cache_status<C: ConfigProvider>(config)`

#### 2. Create `src/cache_handlers.rs`

CLI handler functions for cache commands:
- `handle_cache_download(backend_arg, config, project_prefix, base_url, force)`
- `handle_cache_status(config)`
- `handle_cache_package(backend_arg, config, output_dir, version, project_prefix)`

#### 3. Update `src/cli.rs`

Add `CacheCommand` and `CacheSubcommand`:
```rust
#[derive(Parser, Debug)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub action: CacheAction,
}

#[derive(Subcommand, Debug)]
pub enum CacheAction {
    Download { backend: String, #[arg(long, short)] force: bool },
    Status,
    Package { backend: String, #[arg(long, short, default_value = "./dist")] output: String },
}
```

Add `Cache(CacheCommand)` to `BaseCommand`.

#### 4. Update `src/lib.rs`

- Add `pub mod cache;` and `pub mod cache_handlers;`
- Re-export: `CacheCommand`, `CacheAction`, `CacheBackend`, `CacheManifest`, etc.

#### 5. Update `Cargo.toml`

Ensure `serde`, `serde_json` are dependencies (for manifest serialization). Likely already present.

#### 6. Tests

Move all cache tests from ai-music-theory cache.rs:
- `test_archive_name`, `test_release_url`, `test_checksum_url`
- `test_cache_backend_display`, `test_cache_backend_from_str`
- `test_parse_backend_arg_all`, `test_parse_backend_arg_single`, `test_parse_backend_arg_invalid`
- `test_manifest_roundtrip`, `test_manifest_load_missing_file`, `test_manifest_save_and_load`
- `test_manifest_get_set`
- `test_cache_status_empty`, `test_cache_status_report_display_installed`

Update tests to use parameterized signatures (pass project_prefix, base_url where needed).

### Changes in ai-music-theory

#### 1. Gut `cache.rs` to ~40 lines

Keep only:
```rust
//! Project-specific cache constants and re-exports.

pub use fabryk_cli::cache::*;

/// GitHub Release base URL for this project's cache archives.
pub const RELEASE_BASE_URL: &str = "https://github.com/oxur/ai-music-theory/releases/download";

/// Project prefix for cache archive names.
pub const PROJECT_PREFIX: &str = "music-theory";
```

#### 2. Update `cli.rs`

Update `handle_cache_command` to pass project-specific constants:
```rust
Commands::Cache(cache_cmds) => {
    fabryk_cli::cache_handlers::handle_cache(
        cache_cmds, &config, crate::cache::PROJECT_PREFIX, crate::cache::RELEASE_BASE_URL,
    ).await
}
```

#### 3. Remove all cache tests from local repo (they live in fabryk now)

### Verification

```bash
# In textrynum repo
make test
make lint

# In ai-music-theory repo
make test
make lint

# Functional test
cd mcp-server && cargo run -- cache status
```

### Risk Notes

- The parameterization of `cache_status` and `package_cache` is the main design work. Using `ConfigProvider::cache_path()` is cleanest but requires the cache module to be generic over `C: ConfigProvider`.
- `shell_download` uses `curl` -- ensure no platform assumptions in fabryk-cli's tests (or skip shell tests in CI).

---

## Milestone 2.2: Search Adapters -> fabryk-fts

### Objective

Eliminate local adapter types by enhancing fabryk-fts, or replace local orchestration with fabryk's existing `build_index_multi()`.

### What Exists in Fabryk Today

| Local item | Fabryk equivalent | Action |
|-----------|-------------------|--------|
| `IndexStats` (per-type counts) | `fabryk::fts::IndexStats` (flat counts) | Enhance fabryk's with `label_counts: HashMap<String, usize>` |
| `IndexMetadata` wrapper | `fabryk::fts::IndexMetadata` | Add convenience accessors to fabryk's |
| `build_index()` (multi-dir) | `fabryk::fts::build_index_multi()` | Verify equivalence, then delete local |
| `is_index_fresh()` | `fabryk::fts::is_index_fresh()` | Already delegating -- just remove wrapper |
| `to_fabryk_search_config()` | -- | Keep until Phase 3.1 unifies QueryMode |
| `to_fabryk_query_mode()` | -- | Keep until Phase 3.1 |

### Changes in textrynum (fabryk-fts)

#### 1. Enhance `IndexStats`

fabryk-fts's `IndexStats` already has a `label_counts: HashMap<String, usize>` field (confirmed from exploration). Verify it's populated by `build_index_multi()`. If so, the local adapter's per-type fields (`concept_cards`, `source_chapters`, etc.) are just reads from this map -- no fabryk change needed, just delete the local adapter.

#### 2. Add convenience accessors to `IndexMetadata`

If not already present, add:
```rust
impl IndexMetadata {
    pub fn doc_count(&self) -> usize { self.document_count }
    pub fn indexed_at_display(&self) -> &str { &self.indexed_at }
}
```

#### 3. Verify `build_index_multi()` handles first-build + append

The local `build_index()` does:
1. Collect content directories that exist
2. First directory: `builder.build(content_path, &index_path)`
3. Subsequent: `builder.build_append(content_path, &index_path)`

Verify `build_index_multi()` does the same. If yes, direct replacement. If not, enhance it.

### Changes in ai-music-theory

#### 1. Update `search/mod.rs`

- Remove `IndexStats` adapter (lines 98-142) -- use `fabryk::fts::IndexStats` directly
- Remove `IndexMetadata` adapter (lines 152-189) -- use `fabryk::fts::IndexMetadata` directly
- Remove `build_index()` (lines 205-300) -- replace callers with `fabryk::fts::build_index_multi()`
- Remove `is_index_fresh()` wrapper (lines 311-320) -- use `fabryk::fts::is_index_fresh()` directly
- Keep `to_fabryk_search_config()` and `to_fabryk_query_mode()` (needed until Phase 3.1)
- Keep re-exports section at top

The file should shrink from ~540 lines to ~100 lines (re-exports + the two config conversion functions).

#### 2. Update callers

- `cli.rs` (handle_index_command) -- replace `crate::search::build_index(&config)` with fabryk's `build_index_multi()` call, passing content directories from config
- `cli.rs` (handle_status_command) -- replace `crate::search::IndexMetadata::load()` with `fabryk::fts::IndexMetadata::load()`
- `state.rs` (initialize_fts) -- if it calls `build_index`, update similarly

### Verification

```bash
# In textrynum repo
make test

# In ai-music-theory repo
make test
cargo run -- index --force     # verify index builds
cargo run -- status            # verify metadata display
```

### Risk Notes

- Need to confirm `build_index_multi()` signature matches what we need (accepts Vec of (path, label) tuples or similar). If it only takes a single content path, we need to enhance it.
- The `to_fabryk_search_config` / `to_fabryk_query_mode` functions stay for now -- they're a known tech-debt item cleaned up in Phase 3.1.

---

## Milestone 2.3: Graph Adapters -> fabryk-graph

### Objective

Add convenience methods to fabryk-graph's `Node` and `Relationship` types, add `LoadedGraph` type, then delete local adapter code.

### What Exists in Fabryk Today

| Local item | Fabryk equivalent | Action |
|-----------|-------------------|--------|
| `GraphStats` (concept/source counts) | `fabryk::graph::GraphStats` + `compute_stats()` | Enhance with type_counts |
| `LoadedGraph` (data + timestamp + stats) | -- | Add to fabryk-graph |
| `is_concept_node(node)` | -- | Add `Node::is_domain()` method |
| `is_source_node(node)` | -- | Add `Node::is_custom_type(name)` method |
| `node_id(node)`, `node_title(node)` | Direct field access (`node.id`, `node.title`) | Delete helpers, use fields |
| `node_category(node)` | -- | Add `Node::category()` accessor |
| `source_author(node)`, etc. | -- | Add `Node::metadata_str(key)`, `metadata_u64(key)`, `metadata_bool(key)` |
| `to_fabryk_relationship(name)` | `Relationship` has Display | Add `FromStr` impl |
| `from_fabryk_relationship(rel)` | `Relationship` has Display | Verify Display output matches |
| `load_concept_graph(data_dir)` | `load_graph()` exists | Keep local (thin config wrapper) |
| `compute_graph_stats(data)` | `compute_stats()` exists | Verify type_counts available, then delete local |
| `build_graph(config)` | `GraphBuilder` exists | Keep local (thin config wrapper) |

### Changes in textrynum (fabryk-graph)

#### 1. Add methods to `Node` (src/types.rs or wherever Node is defined)

```rust
impl Node {
    /// Check if this is a domain-type node.
    pub fn is_domain(&self) -> bool {
        matches!(self.node_type, NodeType::Domain)
    }
    
    /// Check if this is a custom-type node with the given type name.
    pub fn is_custom_type(&self, type_name: &str) -> bool {
        matches!(&self.node_type, NodeType::Custom(s) if s == type_name)
    }
    
    /// Get the category, defaulting to "unknown" if not set.
    pub fn category(&self) -> &str {
        self.category.as_deref().unwrap_or("unknown")
    }
    
    /// Get a string value from metadata.
    pub fn metadata_str(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).and_then(|v| v.as_str())
    }
    
    /// Get a u64 value from metadata.
    pub fn metadata_u64(&self, key: &str) -> Option<u64> {
        self.metadata.get(key).and_then(|v| v.as_u64())
    }
    
    /// Get a bool value from metadata.
    pub fn metadata_bool(&self, key: &str) -> Option<bool> {
        self.metadata.get(key).and_then(|v| v.as_bool())
    }
}
```

#### 2. Add `FromStr` to `Relationship`

```rust
impl FromStr for Relationship {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace('_', "").as_str() {
            "prerequisite" => Ok(Self::Prerequisite),
            "relatesto" => Ok(Self::RelatesTo),
            "extends" => Ok(Self::Extends),
            "introduces" => Ok(Self::Introduces),
            "covers" => Ok(Self::Covers),
            "contrastswith" => Ok(Self::ContrastsWith),
            "leadsto" => Ok(Self::LeadsTo),
            "variantof" => Ok(Self::VariantOf),
            "answersquestion" => Ok(Self::AnswersQuestion),
            other => Ok(Self::Custom(other.to_string())),
        }
    }
}
```

Verify existing `Display` impl outputs PascalCase (matching what `from_fabryk_relationship` currently returns).

#### 3. Enhance `GraphStats` with type counts

If `compute_stats()` doesn't already count by `NodeType`, add:
```rust
pub struct GraphStats {
    // ... existing fields ...
    pub type_counts: HashMap<String, u32>,  // e.g., {"domain": 150, "source": 12}
}
```

Update `compute_stats()` to populate this.

#### 4. Add `LoadedGraph` type

```rust
pub struct LoadedGraph {
    pub data: GraphData,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub stats: GraphStats,
}
```

This is a simple composite -- no methods beyond construction.

#### 5. Tests

- `test_node_is_domain`, `test_node_is_custom_type`
- `test_node_category_default`, `test_node_metadata_accessors`
- `test_relationship_from_str_round_trip`
- `test_graph_stats_type_counts`

### Changes in ai-music-theory

#### 1. Update `graph/mod.rs`

- Remove free functions (lines 102-198, ~100 lines):
  - `is_concept_node` -> `node.is_domain()`
  - `is_source_node` -> `node.is_custom_type("source")`
  - `node_id`, `node_title` -> direct field access
  - `node_category` -> `node.category()`
  - `source_author` -> `node.metadata_str("author")`
  - `source_year` -> `node.metadata_u64("year").map(|y| y as u16)`
  - `source_is_converted` -> `node.metadata_bool("is_converted")`
  - `to_fabryk_relationship` -> `Relationship::from_str(name)`
  - `from_fabryk_relationship` -> `format!("{}", rel)` (Display)

- Remove local `GraphStats` and `LoadedGraph` (lines 73-96)
- Import from fabryk: `pub use fabryk::graph::{GraphStats, LoadedGraph};`

- Remove `compute_graph_stats()` (lines 236-254) if fabryk's `compute_stats()` now provides type_counts

- Keep `load_concept_graph()` and `build_graph()` -- these are thin wrappers that couple project config to fabryk calls. They shrink because they use fabryk types directly.

#### 2. Update callers

Grep for `is_concept_node`, `is_source_node`, `node_id`, `node_title`, `node_category`, `source_author`, `source_year`, `source_is_converted`, `to_fabryk_relationship`, `from_fabryk_relationship` across the codebase. Update each call site to use the new method syntax.

### Verification

```bash
# In textrynum repo
make test

# In ai-music-theory repo  
make test
cargo run -- graph stats       # verify stats display
cargo run -- graph validate    # verify validation works
```

### Risk Notes

- Adding methods to `Node` is purely additive -- no breaking changes for other fabryk consumers.
- `FromStr` for `Relationship` with `Custom(String)` fallback means any unrecognized string becomes a custom relationship rather than an error. This is intentional (matches current behavior of `to_fabryk_relationship`).
- `LoadedGraph` requires `chrono` -- verify it's already a dependency of fabryk-graph (it is, based on Cargo.toml).
