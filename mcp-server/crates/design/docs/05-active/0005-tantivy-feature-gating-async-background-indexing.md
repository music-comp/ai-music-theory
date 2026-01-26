---
number: 5
title: "Tantivy Feature Gating & Async Background Indexing"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-01-26
updated: 2026-01-26
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Tantivy Feature Gating & Async Background Indexing

**Version:** 2.0
**Created:** 2026-01-26
**Task:** Make Tantivy optional and add non-blocking async indexing
**Estimated Time:** 18-24 hours over 9 phases

---

## Executive Summary

**Current State:** Tantivy search fully implemented but required dependency with blocking startup indexing

**Goal:** Transform to optional FTS with graceful degradation and background indexing

**Key Changes:**

- Tantivy becomes opt-in via `--features fts`
- Index builds asynchronously (non-blocking startup)
- AppState tracks FTS readiness with dynamic backend switching
- CLI tools for index management (serve/index/status)
- Health MCP tool reports backend status
- Search responses include `"backend"` field

**Zero Breaking Changes:** All existing configurations continue to work

---

## Architecture Overview

### Current Implementation (from exploration)

**SearchBackend Trait:**

```rust
#[async_trait]
pub trait SearchBackend: Send + Sync {
    async fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>>;
}
```

**Two Backends:**

- SimpleSearch (O(n) linear scan, always available)
- TantivySearch (O(log n) indexed, requires existing index)

**Factory Function:**

```rust
pub async fn create_search_backend(config: &Config) -> Result<Box<dyn SearchBackend>>
```

**Current Startup:**

1. Load config
2. Initialize logging
3. **Synchronous index rebuild** (if configured) ← BLOCKS HERE
4. Create MCP server
5. Start serving

**Current Limitations:**

- Tantivy always compiled (not optional)
- Index building blocks server startup (5-10 seconds)
- Backend selected per-request (no persistent state)
- No index freshness tracking (rebuilds unnecessarily)

### Target Architecture

**AppState (New):**

```rust
#[derive(Clone)]
pub struct AppState {
    config: Config,
    simple_backend: Arc<SimpleSearch>,
    #[cfg(feature = "fts")]
    fts_backend: Arc<RwLock<Option<Arc<TantivySearch>>>>,  // Hot-swappable
    #[cfg(feature = "fts")]
    fts_ready: Arc<AtomicBool>,  // Readiness flag
}
```

**Target Startup:**

1. Load config
2. Initialize logging
3. Create AppState with backends
4. **Async background indexing** (if needed) ← NON-BLOCKING
5. Start MCP server immediately
6. Index completes in background, updates readiness flag

**Benefits:**

- Server responsive immediately (<1 second startup)
- Simple search available during indexing
- Automatic switch to FTS when ready
- Index freshness avoids unnecessary rebuilds

---

## Implementation Phases

### Phase 1: Cargo Feature Gating (2-3 hours)

#### Goals

- Make tantivy optional dependency
- Feature-gate all FTS code with `#[cfg(feature = "fts")]`
- Ensure compilation with/without feature

#### Critical Files

**1. Workspace Dependencies** (`Cargo.toml`)

```toml
[workspace.dependencies]
tantivy = "0.22"  # Move from commented to active
```

**2. Server Crate** (`crates/server/Cargo.toml`)

```toml
[features]
default = []
fts = ["dep:tantivy"]

[dependencies]
tantivy = { workspace = true, optional = true }
clap = { version = "4", features = ["derive"] }  # For CLI
```

**3. Search Module** (`crates/server/src/search/mod.rs`)

```rust
pub mod backend;
mod document;
mod simple_search;

#[cfg(feature = "fts")]
mod builder;
#[cfg(feature = "fts")]
mod indexer;
#[cfg(feature = "fts")]
mod query;
#[cfg(feature = "fts")]
mod schema;
#[cfg(feature = "fts")]
mod tantivy_search;
#[cfg(feature = "fts")]
mod freshness;  // NEW: index freshness tracking

pub use backend::create_search_backend;
pub use document::SearchDocument;
pub use simple_search::SimpleSearch;

#[cfg(feature = "fts")]
pub use builder::{build_index, IndexStats};
#[cfg(feature = "fts")]
pub use tantivy_search::TantivySearch;
```

**4. Backend Factory** (`crates/server/src/search/backend.rs`)

```rust
#[cfg(feature = "fts")]
"tantivy" => { /* create TantivySearch */ }
_ => { /* default to SimpleSearch */ }
```

**5. Feature-Gate FTS Files**
Add `#![cfg(feature = "fts")]` at top of:

- `builder.rs`
- `indexer.rs`
- `query.rs`
- `schema.rs`
- `tantivy_search.rs`

**6. Main Startup** (`crates/server/src/main.rs`)

```rust
#[cfg(feature = "fts")]
if config.search.backend == "tantivy" && config.search.rebuild_on_startup {
    // Index building (will be moved to background in Phase 3)
}
```

#### Testing

```bash
cargo build                    # Without FTS
cargo build --features fts     # With FTS
cargo test                     # Without FTS
cargo test --features fts      # With FTS
```

#### Verification

- [ ] Compiles without fts feature
- [ ] Compiles with fts feature
- [ ] All tests pass both ways
- [ ] Binary size smaller without fts

---

### Phase 2: AppState Architecture (3-4 hours)

#### Goals

- Introduce shared application state
- Track FTS readiness dynamically
- Backend selection via state (not per-request factory)
- Module-level functions (not embedded in AppState)

#### Critical Files

**1. Create State Module** (`crates/server/src/state.rs` - NEW)

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    simple_backend: Arc<SimpleSearch>,

    #[cfg(feature = "fts")]
    fts_backend: Arc<RwLock<Option<Arc<TantivySearch>>>>,

    #[cfg(feature = "fts")]
    fts_ready: Arc<AtomicBool>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        let simple_backend = Arc::new(SimpleSearch::new(config.clone()));

        #[cfg(feature = "fts")]
        let (fts_backend, fts_ready) = initialize_fts_backend(&config)?;

        Ok(Self {
            config,
            simple_backend,
            #[cfg(feature = "fts")]
            fts_backend,
            #[cfg(feature = "fts")]
            fts_ready,
        })
    }

    /// Get currently active search backend
    pub fn search_backend(&self) -> Arc<dyn SearchBackend> {
        #[cfg(feature = "fts")]
        if self.fts_ready.load(Ordering::Acquire) {
            if let Ok(guard) = self.fts_backend.read() {
                if let Some(ref backend) = *guard {
                    return Arc::clone(backend) as Arc<dyn SearchBackend>;
                }
            }
        }

        Arc::clone(&self.simple_backend) as Arc<dyn SearchBackend>
    }

    pub fn active_backend_name(&self) -> &'static str {
        #[cfg(feature = "fts")]
        if self.fts_ready.load(Ordering::Acquire) {
            return "tantivy";
        }
        "simple"
    }

    #[cfg(feature = "fts")]
    pub fn is_fts_ready(&self) -> bool {
        self.fts_ready.load(Ordering::Acquire)
    }

    #[cfg(feature = "fts")]
    pub(crate) fn set_fts_ready(&self, ready: bool) {
        self.fts_ready.store(ready, Ordering::Release);
    }

    #[cfg(feature = "fts")]
    pub(crate) fn update_fts_backend(&self, backend: TantivySearch) -> Result<()> {
        let mut guard = self.fts_backend.write()
            .map_err(|_| Error::internal("Failed to acquire write lock"))?;
        *guard = Some(Arc::new(backend));
        Ok(())
    }
}

// Module-level initialization function
#[cfg(feature = "fts")]
fn initialize_fts_backend(config: &Config)
    -> Result<(Arc<RwLock<Option<Arc<TantivySearch>>>>, Arc<AtomicBool>)>
{
    if config.search.backend == "tantivy" {
        let index_path = config.search.index_path()?;
        match TantivySearch::new(&index_path, config.search.clone()) {
            Ok(backend) => {
                // Index exists and is loadable
                Ok((
                    Arc::new(RwLock::new(Some(Arc::new(backend)))),
                    Arc::new(AtomicBool::new(true))
                ))
            }
            Err(_) => {
                // Index doesn't exist yet - will build in background
                Ok((
                    Arc::new(RwLock::new(None)),
                    Arc::new(AtomicBool::new(false))
                ))
            }
        }
    } else {
        Ok((
            Arc::new(RwLock::new(None)),
            Arc::new(AtomicBool::new(false))
        ))
    }
}
```

**Key Design:**

- `search_backend()` returns Arc (shared ownership, not per-request creation)
- `fts_backend` uses RwLock for hot-swapping after indexing
- `fts_ready` uses AtomicBool for lock-free readiness checks
- Initialization logic in module-level function (not embedded in AppState)

**2. Update MusicTheoryServer** (`crates/server/src/server.rs`)

```rust
use crate::state::AppState;

#[derive(Clone)]
pub struct MusicTheoryServer {
    pub state: AppState,  // Changed from config
    tool_router: ToolRouter<Self>,
}

impl MusicTheoryServer {
    pub fn new(state: AppState) -> Self {
        let tool_router = Self::tool_router();
        // ... logging ...
        Self { state, tool_router }
    }
}
```

**3. Update Search Tool** (`crates/server/src/tools/search.rs`)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchConceptsResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub query: String,
    pub backend: String,  // NEW: "simple" or "tantivy"
}

pub async fn search_concepts(
    state: &AppState,  // Changed from config
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    let backend = state.search_backend();
    let backend_name = state.active_backend_name();

    let results = backend.search(&params).await?;

    Ok(SearchConceptsResponse {
        results,
        total: results.len(),
        query: params.query,
        backend: backend_name.to_string(),  // NEW
    })
}
```

**4. Update Tool Handler** (`crates/server/src/server.rs`)

```rust
#[tool(description = "Search concept cards")]
async fn search_concepts(&self, params: Parameters<SearchConceptsParams>)
    -> Result<CallToolResult, ErrorData>
{
    let response = tools::search::search_concepts(&self.state, params.0)  // Use state
        .await
        .map_err(|e| e.to_mcp_error("Error searching concepts"))?;

    // ... serialize and return ...
}
```

**5. Update Main** (`crates/server/src/main.rs`)

```rust
mod state;
use state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    // Initialize logging
    twyg::setup(config.logging.to_twyg()?)?;

    log::info!("Creating application state");
    let state = Arc::new(AppState::new(config).await?);

    // Note: Background indexing initialization will be added in Phase 3

    log::info!("Starting MCP server");
    let service = MusicTheoryServer::new((*state).clone())
        .serve(stdio())
        .await?;

    // ... shutdown handling ...
}
```

#### Testing

```bash
cargo test --features fts
# Verify AppState creation
# Verify backend selection
# Verify state cloning works
```

#### Verification

- [ ] AppState creates successfully
- [ ] search_backend() returns correct backend
- [ ] Backend name matches configuration
- [ ] Search responses include backend field
- [ ] State cloning doesn't panic

---

### Phase 3: Async Background Indexing (4-5 hours)

#### Goals

- Non-blocking index building
- Content hash for freshness tracking
- Automatic backend switching when ready
- Module-level functions for logic

#### Critical Files

**1. Create Freshness Module** (`crates/server/src/search/freshness.rs` - NEW)

```rust
#![cfg(feature = "fts")]

use std::path::Path;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub doc_count: usize,
    pub last_indexed: SystemTime,
    pub content_hash: String,
}

/// Compute hash of all concept card modification times
pub async fn compute_content_hash(config: &Config) -> Result<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use async_walkdir::WalkDir;
    use futures::StreamExt;

    let concept_cards_path = config.paths.concept_cards_path()?;
    let mut hasher = DefaultHasher::new();
    let mut entries = WalkDir::new(concept_cards_path);

    while let Some(entry) = entries.next().await {
        if let Ok(entry) = entry {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(modified) = metadata.modified() {
                        entry.path().hash(&mut hasher);
                        modified.hash(&mut hasher);
                    }
                }
            }
        }
    }

    Ok(format!("{:x}", hasher.finish()))
}

/// Check if index is fresh (content hasn't changed)
pub async fn is_index_fresh(index_path: &Path, config: &Config) -> Result<bool> {
    let metadata_path = index_path.join("metadata.json");

    if !metadata_path.exists() {
        return Ok(false);
    }

    let json = tokio::fs::read_to_string(&metadata_path).await?;
    let metadata: IndexMetadata = serde_json::from_str(&json)?;

    let current_hash = compute_content_hash(config).await?;

    Ok(metadata.content_hash == current_hash)
}

/// Save index metadata after building
pub async fn save_metadata(index_path: &Path, metadata: &IndexMetadata) -> Result<()> {
    let metadata_path = index_path.join("metadata.json");
    let json = serde_json::to_string_pretty(metadata)?;
    tokio::fs::write(&metadata_path, json).await?;
    Ok(())
}
```

**2. Update Builder** (`crates/server/src/search/builder.rs`)

```rust
use crate::search::freshness;

pub async fn build_index(config: &Config) -> Result<IndexStats> {
    // ... existing indexing logic ...

    indexer.commit()?;

    // Save metadata for freshness tracking
    let content_hash = freshness::compute_content_hash(config).await?;
    let metadata = freshness::IndexMetadata {
        doc_count: stats.indexed,
        last_indexed: SystemTime::now(),
        content_hash,
    };
    freshness::save_metadata(&index_path, &metadata).await?;

    Ok(stats)
}
```

**3. Add Background Indexing Functions** (`crates/server/src/state.rs`)

```rust
// Module-level function for FTS initialization
#[cfg(feature = "fts")]
pub async fn initialize_fts(state: &Arc<AppState>) -> Result<()> {
    if state.config.search.backend != "tantivy" {
        return Ok(());
    }

    let index_path = state.config.search.index_path()?;

    // Check if index exists and is fresh
    if index_exists_and_fresh(&index_path, &state.config).await? {
        log::info!("FTS index found and is current");
        state.set_fts_ready(true);
        Ok(())
    } else {
        log::info!("FTS index needs building - starting background task");
        start_background_indexing(Arc::clone(state));
        Ok(())
    }
}

#[cfg(feature = "fts")]
async fn index_exists_and_fresh(index_path: &Path, config: &Config) -> Result<bool> {
    use crate::search::freshness;

    if !index_path.exists() {
        return Ok(false);
    }

    freshness::is_index_fresh(index_path, config).await
}

#[cfg(feature = "fts")]
fn start_background_indexing(state: Arc<AppState>) {
    tokio::spawn(async move {
        log::info!("Background indexing started");

        match build_fts_index_for_state(&state).await {
            Ok(stats) => {
                log::info!(
                    indexed = stats.indexed,
                    errors = stats.errors;
                    "Background indexing complete"
                );

                // Load newly built index
                if let Ok(index_path) = state.config.search.index_path() {
                    match TantivySearch::new(&index_path, state.config.search.clone()) {
                        Ok(backend) => {
                            if state.update_fts_backend(backend).is_ok() {
                                state.set_fts_ready(true);
                                log::info!("FTS backend now active");
                            }
                        }
                        Err(e) => {
                            log::error!(error = %e; "Failed to load newly built index");
                        }
                    }
                }
            }
            Err(e) => {
                log::error!(error = %e; "Background indexing failed");
                // Simple backend remains active
            }
        }
    });
}

#[cfg(feature = "fts")]
async fn build_fts_index_for_state(state: &AppState) -> Result<crate::search::IndexStats> {
    use crate::search::build_index;
    build_index(&state.config).await
}
```

**4. Update Main** (`crates/server/src/main.rs`)

```rust
// Initialize FTS (may start background indexing)
#[cfg(feature = "fts")]
state::initialize_fts(&state).await?;

log::info!("Starting MCP server (FTS may be indexing in background)");
let service = MusicTheoryServer::new((*state).clone())
    .serve(stdio())
    .await?;
```

#### Testing

```bash
# Test background indexing completes
cargo test --features fts test_background_indexing

# Manually verify non-blocking startup
cargo run --features fts
# Server should respond immediately
# Check logs for "Background indexing started"
# After completion: "FTS backend now active"
```

#### Verification

- [ ] Server starts in <1 second
- [ ] Simple search works immediately
- [ ] Background task logs progress
- [ ] FTS switches when ready
- [ ] Index freshness prevents unnecessary rebuilds

---

### Phase 4: CLI Commands (3-4 hours)

#### Goals

- Add clap for argument parsing
- Implement serve/index/status subcommands
- Feature-gate FTS commands
- Follow CLAUDE.md CLI guide

#### Critical Files

**1. Create CLI Module** (`crates/server/src/cli.rs` - NEW)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "music-theory-mcp",
    version,
    about = "Music Theory AI Skill MCP Server",
    long_about = "MCP server providing access to music theory educational materials"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the MCP server (default mode)
    Serve,

    #[cfg(feature = "fts")]
    /// Build or rebuild the full-text search index
    Index {
        /// Force rebuild even if index is current
        #[arg(long, short)]
        force: bool,

        /// Display verbose progress information
        #[arg(long, short)]
        verbose: bool,
    },

    #[cfg(feature = "fts")]
    /// Display FTS index status and statistics
    Status,
}
```

**2. Add CLI Handlers** (`crates/server/src/cli.rs`)

```rust
pub async fn handle_command(cli: Cli) -> Result<()> {
    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => run_server().await,

        #[cfg(feature = "fts")]
        Commands::Index { force, verbose } => handle_index_command(force, verbose).await,

        #[cfg(feature = "fts")]
        Commands::Status => handle_status_command().await,
    }
}

#[cfg(feature = "fts")]
async fn handle_index_command(force: bool, _verbose: bool) -> Result<()> {
    let config = Config::load()?;
    twyg::setup(config.logging.to_twyg()?)?;

    let index_path = config.search.index_path()?;

    if !force && freshness::is_index_fresh(&index_path, &config).await? {
        println!("Index is up to date. Use --force to rebuild anyway.");
        return Ok(());
    }

    println!("Building FTS index...");
    let stats = build_index(&config).await?;

    println!("Index build complete:");
    println!("  Files found: {}", stats.files_found);
    println!("  Documents indexed: {}", stats.indexed);
    println!("  Errors: {}", stats.errors);

    Ok(())
}

#[cfg(feature = "fts")]
async fn handle_status_command() -> Result<()> {
    let config = Config::load()?;
    let index_path = config.search.index_path()?;

    if !index_path.exists() {
        println!("No index found at {:?}", index_path);
        println!("Run 'music-theory-mcp index' to build one.");
        return Ok(());
    }

    let metadata_path = index_path.join("metadata.json");
    if let Ok(json) = tokio::fs::read_to_string(&metadata_path).await {
        if let Ok(metadata) = serde_json::from_str::<freshness::IndexMetadata>(&json) {
            println!("Index Status:");
            println!("  Location: {:?}", index_path);
            println!("  Documents: {}", metadata.doc_count);
            println!("  Last indexed: {:?}", metadata.last_indexed);

            let is_fresh = freshness::is_index_fresh(&index_path, &config).await?;
            println!("  Current: {}", if is_fresh { "yes" } else { "no (rebuild recommended)" });
        }
    }

    Ok(())
}

async fn run_server() -> Result<()> {
    // Move existing main() logic here
    // (Same as current implementation in main.rs)
}
```

**3. Update Main** (`crates/server/src/main.rs`)

```rust
mod cli;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::handle_command(cli).await
}
```

#### Testing

```bash
# Test CLI parsing
cargo test --features fts cli_tests

# Manual testing
cargo build --features fts --release
./target/release/music-theory-mcp serve        # Default
./target/release/music-theory-mcp index        # Build index
./target/release/music-theory-mcp index --force
./target/release/music-theory-mcp status       # Show stats
```

#### Verification

- [ ] `serve` command runs server
- [ ] `index` builds index
- [ ] `index --force` rebuilds even if fresh
- [ ] `status` shows index info
- [ ] Commands feature-gated correctly

---

### Phase 5: Health/Status MCP Tool (2-3 hours)

#### Goals

- Add `health` tool to MCP interface
- Report FTS readiness and active backend
- Include index statistics if available

#### Critical Files

**1. Create Health Tool** (`crates/server/src/tools/health.rs` - NEW)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub backend: BackendStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendStatus {
    pub active: String,  // "simple" or "tantivy"
    #[cfg(feature = "fts")]
    pub fts_enabled: bool,
    #[cfg(feature = "fts")]
    pub fts_ready: bool,
    #[cfg(feature = "fts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_stats: Option<IndexStats>,
}

#[cfg(feature = "fts")]
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub doc_count: usize,
    pub last_indexed: Option<String>,
}

pub async fn get_health(state: &AppState) -> Result<HealthResponse> {
    let active_backend = state.active_backend_name();

    #[cfg(feature = "fts")]
    let (fts_enabled, fts_ready, index_stats) = {
        let enabled = state.config.search.backend == "tantivy";
        let ready = state.is_fts_ready();
        let stats = if ready {
            load_index_stats(&state.config).await.ok()
        } else {
            None
        };
        (enabled, ready, stats)
    };

    Ok(HealthResponse {
        status: "ok".to_string(),
        backend: BackendStatus {
            active: active_backend.to_string(),
            #[cfg(feature = "fts")]
            fts_enabled,
            #[cfg(feature = "fts")]
            fts_ready,
            #[cfg(feature = "fts")]
            index_stats,
        },
    })
}

#[cfg(feature = "fts")]
async fn load_index_stats(config: &Config) -> Result<IndexStats> {
    use crate::search::freshness;

    let index_path = config.search.index_path()?;
    let metadata_path = index_path.join("metadata.json");

    if let Ok(json) = tokio::fs::read_to_string(&metadata_path).await {
        if let Ok(metadata) = serde_json::from_str::<freshness::IndexMetadata>(&json) {
            return Ok(IndexStats {
                doc_count: metadata.doc_count,
                last_indexed: Some(format!("{:?}", metadata.last_indexed)),
            });
        }
    }

    Ok(IndexStats {
        doc_count: 0,
        last_indexed: None,
    })
}
```

**2. Register Tool** (`crates/server/src/server.rs`)

```rust
#[tool(description = "Get server health and search backend status")]
async fn health(&self) -> Result<CallToolResult, ErrorData> {
    let response = tools::health::get_health(&self.state)
        .await
        .map_err(|e| e.to_mcp_error("Error getting health status"))?;

    let content = serde_json::to_string_pretty(&response)
        .map_err(serialization_error)?;

    Ok(CallToolResult::success(vec![Content::text(content)]))
}
```

**3. Update Tools Module** (`crates/server/src/tools/mod.rs`)

```rust
pub mod health;
```

#### Example Response

```json
{
  "status": "ok",
  "backend": {
    "active": "tantivy",
    "fts_enabled": true,
    "fts_ready": true,
    "index_stats": {
      "doc_count": 187,
      "last_indexed": "SystemTime { tv_sec: 1706281496, tv_nsec: 0 }"
    }
  }
}
```

#### Testing

```bash
# Test health tool
cargo test --features fts health_tool_test

# Manual testing via MCP client
# Call health tool, verify response includes backend status
```

#### Verification

- [ ] Health tool returns status
- [ ] Shows active backend name
- [ ] FTS fields present when feature enabled
- [ ] Index stats included when ready

---

### Phase 6: Configuration & Documentation (1-2 hours)

#### Goals

- Document feature flags
- Update configuration guide
- Add CLI usage examples

#### Critical Files

**1. Update Default Config** (`crates/server/config/default.toml`)

Add comments explaining async indexing:

```toml
[search]
# Backend selection: "simple" or "tantivy"
# Note: "tantivy" requires building with --features fts
backend = "simple"

# Tantivy index storage directory
index_path = ".tantivy-index"

# Rebuild index on startup
# With async background indexing, this is less critical
# Set to true for initial build, then false
rebuild_on_startup = false

# Snippet size, fuzzy search, etc.
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
```

**2. Update README** (`crates/server/README.md`)

Add sections for:

- Feature flags (`--features fts`)
- CLI commands
- Health tool
- Backend field in search responses

---

### Phase 7: Testing & Validation (3-4 hours)

#### Test Coverage

**Feature Flag Tests:**

```bash
cargo build                    # Without FTS
cargo build --features fts     # With FTS
cargo test                     # Without FTS tests
cargo test --features fts      # With FTS tests
```

**Unit Tests:**

- AppState creation (`state.rs`)
- Backend selection logic
- Freshness detection
- CLI parsing (`cli.rs`)
- Health tool responses

**Integration Tests:**

- Background indexing completion (`tests/background_indexing.rs`)
- Backend switching (simple → FTS)
- Concurrent access during indexing
- Index rebuild with --force

**Manual Testing:**

1. Build without FTS → verify simple search works
2. Build with FTS → verify FTS indexing
3. Call health tool → verify status
4. CLI commands → verify index/status work
5. Restart server → verify index persistence

#### Verification Checklist

- [ ] Compiles without fts feature
- [ ] Compiles with fts feature
- [ ] All tests pass (both configs)
- [ ] Server starts in <1 second
- [ ] Background indexing completes
- [ ] Simple → FTS transition works
- [ ] Health tool reports correctly
- [ ] CLI commands work
- [ ] Index freshness prevents rebuilds
- [ ] Search responses include backend field
- [ ] No clippy warnings

---

### Phase 8: Build System Updates (1-2 hours)

#### Goals

- Update Makefile for feature flags
- Add CI matrix testing
- Document build commands

#### Critical Files

**1. Makefile** (`Makefile`)

```makefile
.PHONY: build build-fts test test-fts check check-fts

# Build targets
build:
 cargo build --release

build-fts:
 cargo build --release --features fts

# Test targets
test:
 cargo test

test-fts:
 cargo test --features fts

test-all: test test-fts

# Check targets
check:
 cargo check
 cargo clippy -- -D warnings

check-fts:
 cargo check --features fts
 cargo clippy --features fts -- -D warnings

# CLI helpers (require FTS)
index:
 cargo run --features fts -- index

index-force:
 cargo run --features fts -- index --force

status:
 cargo run --features fts -- status
```

**2. CI Workflow** (`.github/workflows/ci.yml`)

Add feature matrix if CI exists:

```yaml
jobs:
  test:
    strategy:
      matrix:
        features: ['', 'fts']
    steps:
      - name: Build
        run: cargo build ${{ matrix.features && format('--features {0}', matrix.features) }}
      - name: Test
        run: cargo test ${{ matrix.features && format('--features {0}', matrix.features) }}
```

---

### Phase 9: Documentation & Migration Guide (1-2 hours)

#### Goals

- Document all changes
- Provide migration steps
- Explain feature flags and CLI

#### Critical Files

**1. README Updates** (`README.md`)

Add sections:

- **Features** - FTS is optional, how to enable
- **Building** - Commands with/without fts
- **CLI** - serve/index/status usage
- **MCP Tools** - health tool documentation
- **Configuration** - Backend selection guide

**2. Migration Guide** (`MIGRATION.md` - NEW)

Document:

- Zero breaking changes
- How to enable FTS
- Pre-building index
- Background indexing behavior
- Testing both configurations

---

## Implementation Order

### Dependency Graph

```
Phase 1 (Feature Gating)
    ↓
Phase 2 (AppState)
    ↓
Phase 3 (Background Indexing) ← Phase 5 (Health Tool)
    ↓
Phase 4 (CLI)
    ↓
Phase 6 (Config/Docs)
    ↓
Phase 7 (Testing)
    ↓
Phase 8 (Build System)
    ↓
Phase 9 (Documentation)
```

### Recommended Sequence

1. **Phase 1** - Feature gating (enables compilation testing)
2. **Phase 2** - AppState (architectural foundation)
3. **Phase 3** - Background indexing (core functionality)
4. **Phase 5** - Health tool (observability for testing)
5. **Phase 4** - CLI (operator tooling)
6. **Phase 6** - Config updates
7. **Phase 7** - Comprehensive testing
8. **Phase 8** - Build automation
9. **Phase 9** - Final documentation

---

## Critical Files Summary

### New Files (Create)

- `crates/server/src/state.rs` (~250 lines) - AppState and init functions
- `crates/server/src/search/freshness.rs` (~150 lines) - Index metadata
- `crates/server/src/cli.rs` (~200 lines) - CLI structure and handlers
- `crates/server/src/tools/health.rs` (~100 lines) - Health tool
- `crates/server/tests/background_indexing.rs` (~150 lines) - Integration tests
- `MIGRATION.md` (~100 lines) - Migration guide

### Modified Files

- `Cargo.toml` - Add tantivy to workspace deps
- `crates/server/Cargo.toml` - Features + optional tantivy
- `crates/server/src/search/mod.rs` - Feature gates
- `crates/server/src/search/backend.rs` - Feature-gate tantivy case
- `crates/server/src/search/builder.rs` - Save metadata
- `crates/server/src/server.rs` - Use AppState, register health tool
- `crates/server/src/tools/search.rs` - Use AppState, add backend field
- `crates/server/src/main.rs` - CLI entry point, AppState init
- `crates/server/config/default.toml` - Updated comments
- `README.md` - Feature docs, CLI docs
- `Makefile` - Feature flag targets

**Total:** ~1,000 new lines, ~200 modified lines

---

## Success Criteria

### Functional Requirements

- [ ] Compiles without fts feature (simple search only)
- [ ] Compiles with fts feature (both backends)
- [ ] All tests pass (both configurations)
- [ ] Server starts in <1 second (non-blocking)
- [ ] Background indexing completes successfully
- [ ] Backend switches from simple → FTS when ready
- [ ] Index freshness prevents unnecessary rebuilds
- [ ] CLI commands work (index, status)
- [ ] Health tool reports correct status
- [ ] Search responses include backend field

### Quality Requirements

- [ ] No breaking changes to existing API
- [ ] Zero downtime during indexing
- [ ] Graceful degradation (simple fallback)
- [ ] Comprehensive test coverage (≥95%)
- [ ] No clippy warnings
- [ ] Documentation complete
- [ ] CI passes for both feature configs

### Performance Requirements

- [ ] Startup time <1 second
- [ ] Index build ~1-2 seconds for 200 docs (background)
- [ ] Memory usage <100MB during indexing
- [ ] No latency impact on simple search during indexing

---

## Risk Mitigation

### Identified Risks

1. **RwLock Contention**
   - Risk: Read lock on fts_backend could block during switch
   - Mitigation: Writes are rare (only after indexing), reads lock-free via AtomicBool check first
   - Alternative: Accept limitation that backend loads on next restart

2. **Content Hash Performance**
   - Risk: Walking all files could be slow
   - Mitigation: Async traversal, only reads mtimes (not contents)
   - Optimization: Can cache hash in memory after first computation

3. **Index Corruption**
   - Risk: Server crashes during indexing
   - Mitigation: Tantivy uses atomic commits; partial writes don't corrupt
   - Recovery: Rebuild with CLI tool

4. **Race on First Query**
   - Risk: Query arrives before FTS ready
   - Mitigation: AtomicBool with Acquire/Release ordering
   - Fallback: SimpleSearch handles queries until FTS ready

### Testing Priorities

1. Feature flag compilation (both variants)
2. Background indexing completion
3. Graceful degradation (simple → FTS)
4. CLI functionality
5. Concurrent access during indexing
6. Index freshness detection

---

## Verification Steps (End-to-End)

### Without FTS Feature

```bash
# Build
cargo build --release

# Verify
ls -lh target/release/music-theory-mcp  # Should be smaller
./target/release/music-theory-mcp serve  # Should work

# Test search tool (should use simple backend)
# Response should show "backend": "simple"
```

### With FTS Feature

```bash
# Build
cargo build --release --features fts

# Initial index build
./target/release/music-theory-mcp index

# Check status
./target/release/music-theory-mcp status

# Run server
./target/release/music-theory-mcp serve

# Test health tool
# Should show "fts_ready": true after indexing

# Test search tool
# Should show "backend": "tantivy"

# Restart server (fast startup)
# Index should load instantly from disk
```

---

## Notes for Implementation

1. **Use CLAUDE.md → SKILL.md guides** throughout
2. **Module-level functions** - Keep logic out of AppState
3. **Incremental commits** - One phase per commit
4. **Test continuously** - Both feature configs
5. **Document as you go** - Update README incrementally

**This plan provides a complete path to optional FTS with async background indexing while maintaining zero breaking changes.**
