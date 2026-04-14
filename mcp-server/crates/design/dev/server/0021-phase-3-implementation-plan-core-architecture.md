# Phase 3 Implementation Plan: Core Architecture

**Milestones:** 3.1 (Config), 3.2 (AppState), 3.3 (CLI), 3.4 (Server Builder)
**Estimated effort:** 4 sittings (one per milestone)
**Dependencies:** Phases 1-2 complete. Within Phase 3: 3.1 first, then 3.2 and 3.3 (parallel), then 3.4 last.

---

## Milestone 3.1: Config Types -> fabryk-core / fabryk-fts

### Objective

Eliminate the duplicate `QueryMode` enum, unify `SearchConfig`, move `LanceDbConfig`, and clean up the config conversion adapters that become unnecessary.

### What Exists in Fabryk Today

| Local item | Fabryk equivalent | Action |
|-----------|-------------------|--------|
| `QueryMode` (Or, And, MinimumMatch(f32), Smart) | `fabryk::fts::QueryMode` (Or, And, MinimumMatch, Smart) | **(a) delete local**, use fabryk's |
| `SearchConfig` (~140 lines, 14 fields) | `fabryk::fts::SearchConfig` (~12 fields) | **(b) enhance fabryk's** with missing fields |
| `LanceDbConfig` (embedding_model, cache_dir, enabled) | -- | **(b) add to fabryk-vector** or fabryk-cli |
| `PathsConfig` (concept_cards, sources_md, etc.) | `ConfigProvider::content_path(type)` | **(c) keep local** -- typed struct is project-specific |
| `ConfigProvider` / `ConfigManager` impls | Traits defined in fabryk-core | **(c) keep local** -- impls are project-specific |
| `to_fabryk_search_config()` | -- | **(a) delete** -- unnecessary once types unified |
| `to_fabryk_query_mode()` | -- | **(a) delete** -- unnecessary once types unified |

### Changes in textrynum

#### 1. Enhance `SearchConfig` in fabryk-fts

Compare field-by-field and add any missing to fabryk's:

| ai-music-theory field | fabryk-fts field | Status |
|----------------------|------------------|--------|
| `backend` | `backend` | Exists |
| `index_path` (String) | `index_path` (Option\<String\>) | Exists |
| `rebuild_on_startup` | -- | **Add** |
| `snippet_size` | `snippet_length` | Exists (different name) |
| `fuzzy_search` | `fuzzy_enabled` | Exists (different name) |
| `fuzzy_distance` | `fuzzy_distance` | Exists |
| `query_mode` | `query_mode` | Exists |
| `minimum_match_percent` | -- | **Add** (or keep in local config) |
| `enable_stopwords` | `stopwords_enabled` | Exists (different name) |
| `custom_stopwords` | `custom_stopwords` | Exists |
| `stopword_allowlist` | `allowlist` | Exists (different name) |
| `field_boost_title` | -- | **Add** |
| `field_boost_description` | -- | **Add** |
| `field_boost_content` | -- | **Add** |

Add to fabryk's `SearchConfig`:
- `rebuild_on_startup: bool` (default false)
- `minimum_match_percent: f32` (default 0.6)
- `field_boost_title: f32` (default 3.0)
- `field_boost_description: f32` (default 2.0)
- `field_boost_content: f32` (default 1.0)

**Critical:** Use `#[serde(alias = "...")]` for the field name differences (`snippet_size` vs `snippet_length`, etc.) to maintain backward compatibility with existing TOML config files.

#### 2. Add `LanceDbConfig` to fabryk-vector (or fabryk-cli)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LanceDbConfig {
    pub embedding_model: String,      // default: "bge-small-en-v1.5"
    pub embedding_cache_dir: Option<String>,
    pub enabled: bool,                // default: true
}
```

Feature-gate behind `vector` or `vector-fastembed`.

### Changes in ai-music-theory

#### 1. Update `config.rs`

- **Delete** `QueryMode` enum (lines 116-127) -- replace with `pub use fabryk::fts::QueryMode;`
- **Delete** `SearchConfig` struct and its impl/defaults (lines 131-270, ~140 lines) -- replace with `pub use fabryk::fts::SearchConfig;`
- **Delete** `LanceDbConfig` (lines 273-308) -- replace with import from fabryk
- **Keep** `PathsConfig`, `SourcesConfig`, `ServerConfig`, `Config` struct, `ConfigProvider`/`ConfigManager` impls
- **Keep** `default_stopword_allowlist()` -- music-specific, remains in local `Config::default()` or passed to fabryk's SearchConfig

**Note on serde compatibility:** The local config TOML uses field names like `snippet_size`, `fuzzy_search`, `enable_stopwords`. Fabryk uses `snippet_length`, `fuzzy_enabled`, `stopwords_enabled`. The `#[serde(alias)]` approach in fabryk ensures existing config files still parse.

#### 2. Update `search/mod.rs`

- **Delete** `to_fabryk_search_config()` and `to_fabryk_query_mode()` (lines 44-87) -- no longer needed since config types are unified
- The file is now just re-exports (~30 lines)

#### 3. Update callers

- Anywhere that calls `to_fabryk_search_config()` or `to_fabryk_query_mode()` -- replace with direct use of the unified config types
- `state.rs` search backend initialization -- pass `&config.search` directly

### Verification

```bash
make test   # both repos
make lint   # both repos

# Critical: verify existing config files still parse
cd mcp-server && cargo run -- config get
cd mcp-server && cargo run -- config get search.query_mode
```

### Risk Notes

- **Highest risk in entire migration.** Serde field name differences between local and fabryk configs could break TOML deserialization for existing users. The `#[serde(alias)]` strategy mitigates this but must be tested thoroughly.
- `minimum_match_percent` is used locally with `MinimumMatch(f32)` but fabryk's `MinimumMatch` has no parameter. Need to decide: add the f32 to fabryk's variant, or keep `minimum_match_percent` as a separate config field (not embedded in the enum variant). Recommend the latter for simplicity.

---

## Milestone 3.2: AppState Pattern -> fabryk-mcp-core

### Objective

Create a `FabrykMcpState<C>` in fabryk-mcp-core that provides the generic application state management pattern (search backends, graph lifecycle, vector lifecycle), then make ai-music-theory's `AppState` a thin wrapper or type alias.

### What Exists in Fabryk Today

- `fabryk_core::AppState<C>` -- minimal Arc-wrapped config container (~60 lines)
- `fabryk_core::ServiceHandle` -- lifecycle tracking (Stopped/Starting/Ready/Failed)
- `fabryk_core::ServiceOrchestrator`, `ManagedService` -- orchestration primitives

ai-music-theory's `AppState` (~300 lines) adds:
- `simple_backend: Arc<SimpleSearch>` (always available)
- `fts_backend: Arc<StdRwLock<Option<Arc<TantivySearch>>>>` (feature-gated)
- `fts_service: ServiceHandle`
- `graph_data`, `shared_graph`, `graph_service` (feature-gated)
- `vector_backend`, `vector_slot`, `vector_service` (feature-gated)
- Methods: `search_backend()`, `active_backend_name()`, `is_fts_ready()`, `update_fts_backend()`, `require_graph()`, `require_vector()`, `update_vector_backend()`

### Changes in textrynum (fabryk-mcp-core)

#### 1. Create `src/state.rs`

```rust
/// Generic MCP application state with optional backend management.
///
/// Provides search backend switching (simple -> FTS), graph lifecycle,
/// and vector lifecycle -- all feature-gated.
pub struct FabrykMcpState<C: ConfigProvider> {
    pub config: C,
    simple_backend: Arc<dyn SearchBackend + Send + Sync>,
    
    #[cfg(feature = "fts")]
    fts_backend: Arc<StdRwLock<Option<Arc<TantivySearch>>>>,
    #[cfg(feature = "fts")]
    pub fts_service: ServiceHandle,
    
    #[cfg(feature = "graph")]
    pub graph_service: ServiceHandle,
    #[cfg(feature = "graph")]
    pub shared_graph: Arc<tokio::sync::RwLock<GraphData>>,
    
    #[cfg(feature = "vector")]
    pub vector_service: ServiceHandle,
    #[cfg(feature = "vector")]
    pub vector_backend: Arc<RwLock<Option<Arc<dyn VectorBackend>>>>,
    #[cfg(feature = "vector")]
    pub vector_slot: VectorSlot,
}
```

Implement the core methods:
- `new(config) -> Result<Self>`
- `search_backend() -> Arc<dyn SearchBackend + Send + Sync>`
- `active_backend_name() -> &'static str`
- `update_fts_backend(backend)`, `is_fts_ready()`
- `require_graph()`, `require_vector()`
- `update_vector_backend(backend)`

#### 2. Update `src/lib.rs`

- Add `pub mod state;`
- Re-export `FabrykMcpState`

#### 3. Feature flags in `Cargo.toml`

Ensure `fts`, `graph`, `vector` features exist and pull in the right dependencies.

### Changes in ai-music-theory

#### 1. Update `state.rs`

Replace the bulk of `AppState` with:
```rust
pub type AppState = FabrykMcpState<Config>;
```

Or, if project-specific fields are needed (e.g., `graph_data: Arc<RwLock<Option<LoadedGraph>>>`), use a newtype:
```rust
pub struct AppState {
    inner: FabrykMcpState<Config>,
    #[cfg(feature = "graph")]
    pub graph_data: Arc<RwLock<Option<LoadedGraph>>>,
}

impl std::ops::Deref for AppState {
    type Target = FabrykMcpState<Config>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
```

Estimated reduction: ~200 lines.

### Verification

```bash
make test   # both repos

# Functional: start server, verify backends initialize
cd mcp-server && cargo run -- serve --test
# Ctrl+C to verify graceful shutdown
```

### Risk Notes

- The `shared_graph: Arc<tokio::sync::RwLock<GraphData>>` pattern (needed by `GraphTools::with_shared`) is a coupling point. The fabryk version must support this dual-slot pattern.
- `vector_slot: VectorSlot` (fabryk_mcp::semantic::VectorSlot) creates a dependency from fabryk-mcp-core on fabryk-mcp-semantic. May need to be generic (`Arc<tokio::sync::RwLock<Option<Arc<dyn VectorBackend>>>>`) to avoid circular deps.
- This is design-heavy. Recommend a discussion before implementation to nail down the exact generic type structure.

---

## Milestone 3.3: CLI Deduplication -> fabryk-cli

### Objective

Replace local CLI type definitions with fabryk-cli's existing types, adopt the `CliExtension` trait for project-specific commands.

### What Exists

| Local type | fabryk-cli equivalent | Action |
|-----------|----------------------|--------|
| `Cli` struct | `CliArgs` | Adopt fabryk's, extend via `CliExtension` |
| `Commands` enum | `BaseCommand` | Use fabryk's + extension commands |
| `GraphCommands`/`GraphSubcommand` | `GraphCommand`/`GraphSubcommand` | Use fabryk's (enhance if needed) |
| `VectordbCommands`/`VectordbSubcommand` | `VectordbCommand` | Use fabryk's |
| `CacheCommands`/`CacheSubcommand` | `CacheCommand` (added in 2.1) | Use fabryk's |
| `handle_command()` | `FabrykCli::run()` | Adopt fabryk's dispatch + extension |
| `handle_config_command()` | Already delegates to fabryk | Delete local |

### Changes in textrynum (fabryk-cli)

#### 1. Enhance `GraphSubcommand` if needed

Local has `Build { dry_run, verbose }`, `Validate`, `Stats`, `Compile`. Fabryk has `Build { force }`, `Validate`, `Stats`, `Query { ... }`.

Add to fabryk's `GraphSubcommand::Build`:
- `#[arg(long)] dry_run: bool`
- `#[arg(long, short)] verbose: bool`

Add `Compile` variant to fabryk's `GraphSubcommand`.

#### 2. Verify `CacheCommand` was added in Milestone 2.1

If not, add it now.

### Changes in ai-music-theory

#### 1. Restructure `cli.rs`

Replace the current approach (custom `Cli` struct + `Commands` enum) with fabryk-cli's `CliExtension` pattern:

```rust
use fabryk_cli::{CliArgs, BaseCommand, CliExtension, FabrykCli};

/// Project-specific CLI extension commands.
#[derive(Subcommand)]
pub enum MusicTheoryCommand {
    /// Run in test mode (no MCP protocol)
    ServeTest,
}

impl CliExtension for MusicTheoryCommand {
    // ... implement dispatch for project-specific commands
}
```

The top-level binary becomes:
```rust
fn main() {
    let args = CliArgs::parse();
    let cli = FabrykCli::<Config>::from_args("music-theory-mcp", &args)?;
    cli.run_with_extension::<MusicTheoryCommand>(args).await
}
```

#### 2. Delete local type definitions

Remove from `cli.rs`:
- `Cli` struct (~20 lines)
- `Commands` enum (~50 lines)
- `GraphCommands`/`GraphSubcommand` (~30 lines)
- `VectordbCommands`/`VectordbSubcommand` (~20 lines)
- `CacheCommands`/`CacheSubcommand` (~35 lines)
- `handle_config_command()` (~25 lines)

Keep (but simplify):
- Project-specific `handle_command()` logic for Sources (with category mapping)
- The `--log-level` and `--transport` args (may need to be extension args)

Estimated reduction: ~150 lines.

#### 3. Update `main.rs`

Adopt `FabrykCli` pattern instead of direct `Cli::parse()`.

### Verification

```bash
make test   # both repos

# Test every CLI subcommand:
cargo run -- serve --test        # Ctrl+C
cargo run -- index --force
cargo run -- status
cargo run -- graph stats
cargo run -- graph validate
cargo run -- graph build --dry-run
cargo run -- graph compile
cargo run -- vectordb status
cargo run -- cache status
cargo run -- config path
cargo run -- config get
cargo run -- sources list
```

### Risk Notes

- **Most design-heavy milestone.** The `CliExtension` trait pattern may not perfectly accommodate all of ai-music-theory's current CLI features (e.g., the `--log-level` global arg, the `--transport` HTTP arg, the `serve --test` flag).
- Recommend studying an existing `CliExtension` implementation in another fabryk consumer before starting.
- If `CliExtension` doesn't fit, an alternative is to keep the local `Cli` struct but compose it with fabryk-cli types internally (less clean but lower risk).

---

## Milestone 3.4: Server Builder Pattern -> fabryk-mcp

### Objective

Extract the generic registry-composition pattern from `build_server()` into a reusable `ServerBuilder` in fabryk-mcp-core, so new fabryk-mcp projects get all standard tools wired up automatically.

### What Moves

The generic portion of `build_server()` (lines 894-1247, ~350 lines):
- ContentTools wiring (provider creation, name/description maps, extra schema)
- SourceTools wiring
- GuideTools wiring
- FtsTools wiring
- SemanticSearchTools wiring
- QuestionSearchTools wiring
- HealthTools wiring (backend probes, search config info)
- GraphTools wiring (shared graph, node filter, name/description maps, extra schemas)
- StaticResources wiring

### What Stays

- `MusicTheoryToolsRegistry` and `OthToolsRegistry` (domain-specific)
- The `.add(music_theory_tools)` and `.add(oth_tools)` calls
- The `.with_description("Music Theory AI Skill - ...")` branding
- Resource fallback content (`default_conventions()`, etc.)

### Changes in textrynum (fabryk-mcp-core)

#### 1. Create `src/builder.rs`

```rust
pub struct ServerBuilder<C: ConfigProvider> {
    config: Arc<C>,
    state: FabrykMcpState<C>,
    registry: CompositeRegistry,
    resources: Vec<StaticResourceDef>,
}

impl<C: ConfigProvider> ServerBuilder<C> {
    pub fn new(state: FabrykMcpState<C>) -> Self { ... }
    
    /// Wire up ContentTools from a content path with custom names.
    pub fn with_content_tools(self, names: HashMap<String, String>, 
                              descriptions: HashMap<String, String>) -> Self { ... }
    
    /// Wire up SourceTools from a sources path.
    pub fn with_source_tools(self, names: HashMap<String, String>,
                             descriptions: HashMap<String, String>) -> Self { ... }
    
    /// Wire up GuideTools.
    pub fn with_guide_tools(self) -> Self { ... }
    
    /// Wire up FtsTools with extra search schema.
    pub fn with_fts_tools(self, names: HashMap<String, String>,
                          extra_schema: Option<Value>) -> Self { ... }
    
    /// Wire up GraphTools with custom names and node filter.
    #[cfg(feature = "graph")]
    pub fn with_graph_tools(self, names: HashMap<String, String>,
                            descriptions: HashMap<String, String>,
                            extra_schemas: Vec<(&str, Value)>) -> Self { ... }
    
    /// Wire up HealthTools with backend probes.
    pub fn with_health_tools(self) -> Self { ... }
    
    /// Add a domain-specific ToolRegistry.
    pub fn add_registry(self, registry: impl ToolRegistry + 'static) -> Self { ... }
    
    /// Add a static resource.
    pub fn with_resource(self, def: StaticResourceDef) -> Self { ... }
    
    /// Build the final FabrykMcpServer.
    pub fn build(self, name: &str, version: &str, description: &str) -> FabrykMcpServer { ... }
}
```

#### 2. Provide a convenience `standard_server()` function

For the common case where all standard tools are wanted:
```rust
pub fn standard_server<C: ConfigProvider>(state: FabrykMcpState<C>) -> ServerBuilder<C> {
    ServerBuilder::new(state)
        .with_content_tools(/* defaults */)
        .with_source_tools(/* defaults */)
        .with_guide_tools()
        .with_fts_tools(/* defaults */)
        .with_health_tools()
}
```

### Changes in ai-music-theory

#### 1. Simplify `server.rs`

Replace ~350 lines of composition with:
```rust
pub fn build_server(state: AppState) -> FabrykMcpServer {
    fabryk_mcp::ServerBuilder::new(state)
        .with_content_tools(concept_names(), concept_descriptions())
        .with_source_tools(source_names(), source_descriptions())
        .with_guide_tools()
        .with_fts_tools(fts_names(), Some(fts_extra_schema()))
        .with_graph_tools(graph_names(), graph_descriptions(), graph_extra_schemas())
        .with_health_tools()
        .add_registry(MusicTheoryToolsRegistry)
        .add_registry(OthToolsRegistry)
        .with_resource(conventions_resource())
        .with_resource(scope_resource())
        .with_resource(sources_resource())
        .with_resource(index_resource())
        .build("music-theory-skill", env!("CARGO_PKG_VERSION"),
               "Music Theory AI Skill - Access comprehensive music theory materials...")
}
```

The name/description HashMaps move to local helper functions for readability.

### Verification

```bash
make test   # both repos

# Start server, verify all tools are registered
cd mcp-server && cargo run -- serve --test
# In another terminal, connect an MCP client and list tools
# Verify tool count matches expected (53 with graph, 36 without)
```

### Risk Notes

- **Most architectural milestone.** The builder API design determines ergonomics for all future fabryk projects.
- The builder should NOT try to hide `CompositeRegistry` -- projects must be able to `.add_registry()` their domain registries.
- Graph tool wiring is complex (node filters, extra schemas per slot, custom names per slot). The builder API must be flexible enough without becoming a mirror of the raw GraphTools builder.
- Consider whether the builder should auto-detect available features (has graph? has vector?) or require explicit opt-in. Recommend explicit -- less magic, more predictable.
- This milestone benefits most from all prior milestones being complete, since the remaining local code will be minimal and the pattern clear.
