# Tantivy Feature Gating & Async Indexing Architecture

## Context

The music-theory MCP server has implemented Tantivy for full-text search, but needs architectural improvements for:

1. **Optional dependency** — Users shouldn't need Tantivy if they don't want FTS
2. **Non-blocking startup** — Don't block Claude Desktop on index building
3. **Graceful degradation** — Use simple search until FTS index is ready

## Current State

- Tantivy search backend implemented
- Indexing happens at startup (blocking)
- ~200 concept cards (growing)
- Config-based toggle for Tantivy (should be feature-based)

## Requirements

### 0. CLAUDE.md and the AI Rust Skill

For all of the below, you MUST use CLAUDE.md -> SKILL.md and the various guides (as applicable) referenced by SKILL.md. Note that the directory that SKILL.md points to is actually a symlink, so take that into account when reading directories and scanningn for files.

### 1. Cargo Feature Gating

**Goal**: `tantivy` and its dependencies are only compiled when explicitly enabled.

**Questions to resolve**:

- Same crate with `#[cfg(feature = "fts")]` conditionals?
- OR separate `server-tantivy` crate?

**Recommendation**: Start with same crate + feature flag. Split to separate crate only if:

- Compile times become problematic
- The FTS code grows large enough to warrant separation
- You want to publish the FTS backend independently

User's note: Since we're not going to be publishing these crates (the server is bundled as part of the skill, for which this is no packaing option other than a `git clone`), this is not a concern, so it seems like the best option is to keep the tantivy implementation where it is, as part of the larger `server` crate.

**Implementation pattern**:

```toml
# Cargo.toml
[features]
default = []
fts = ["tantivy"]

[dependencies]
tantivy = { version = "0.21", optional = true }
```

```rust
// src/search/mod.rs
mod simple;
#[cfg(feature = "fts")]
mod tantivy_backend;

pub use simple::SimpleSearch;
#[cfg(feature = "fts")]
pub use tantivy_backend::TantivySearch;
```

### 2. Search Backend Trait

If not already done, abstract over search implementations so they're swappable:

```rust
#[async_trait]
pub trait SearchBackend: Send + Sync {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;
    async fn index_concept(&self, concept: &ConceptCard) -> Result<()>;
    async fn reindex_all(&self, concepts: &[ConceptCard]) -> Result<()>;
    fn is_ready(&self) -> bool;
}
```

User's note: I believe Claude Code has already done this.

### 3. Application State with Dynamic Backend Switching

**Goal**: Track FTS readiness and switch backends dynamically.

```rust
pub struct AppState {
    /// Simple search backend (always available)
    simple_search: Arc<SimpleSearch>,

    /// FTS backend (only with feature, may not be ready)
    #[cfg(feature = "fts")]
    fts_search: Arc<TantivySearch>,

    /// Whether FTS index is ready for queries
    #[cfg(feature = "fts")]
    fts_ready: AtomicBool,

    // ... other state
}

impl AppState {
    pub fn search_backend(&self) -> &dyn SearchBackend {
        #[cfg(feature = "fts")]
        if self.fts_ready.load(Ordering::Relaxed) {
            return self.fts_search.as_ref();
        }

        self.simple_search.as_ref()
    }
}
```

### 4. Async Background Indexing

**Goal**: Start indexing in background, don't block server startup.

```rust
impl AppState {
    pub async fn start_background_indexing(self: &Arc<Self>) {
        #[cfg(feature = "fts")]
        {
            let state = Arc::clone(self);
            tokio::spawn(async move {
                tracing::info!("Starting background FTS indexing...");

                match state.build_fts_index().await {
                    Ok(()) => {
                        state.fts_ready.store(true, Ordering::Release);
                        tracing::info!("FTS index ready");
                    }
                    Err(e) => {
                        tracing::error!("FTS indexing failed: {}", e);
                        // Simple backend remains active
                    }
                }
            });
        }
    }

    #[cfg(feature = "fts")]
    async fn build_fts_index(&self) -> Result<()> {
        let concepts = self.load_all_concepts().await?;
        self.fts_search.reindex_all(&concepts).await
    }
}
```

User's note: I do not actually like the pattern of embedding logic in AppState struct like that. That's fine for demonstration purposes (e.g,. for showing intent), but we MUST NOT take that as implementation instruction. Much better:

- to have the ability to define any number of functions that operate on safe (copies? dunno -- follow Rust best practices) state data
- for the AppState to call those functions
- to use the results of those functions to update its state

The functions the AppState calls could conceivably be defined anywhere. For our purposes, they should be organised properly for the task at hand, and that probably means module-level functions somewhere near the AppState struct. Follow Rust best practices when making this decision.

### 5. CLI for Out-of-Band Indexing

**Goal**: Allow pre-building index before server starts.

```rust
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "music-theory-mcp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the MCP server (default)
    Serve,

    /// Build or rebuild the FTS index
    #[cfg(feature = "fts")]
    Index {
        /// Force full reindex even if index exists
        #[arg(long)]
        force: bool,
    },

    /// Check index status
    #[cfg(feature = "fts")]
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => run_server().await,

        #[cfg(feature = "fts")]
        Commands::Index { force } => {
            let state = AppState::new().await?;
            if force || !state.fts_search.index_exists() {
                state.build_fts_index().await?;
                println!("Index built successfully");
            } else {
                println!("Index already exists. Use --force to rebuild.");
            }
            Ok(())
        }

        #[cfg(feature = "fts")]
        Commands::Status => {
            let state = AppState::new().await?;
            let status = state.fts_search.index_status()?;
            println!("Index status: {:?}", status);
            println!("Documents: {}", status.doc_count);
            println!("Last updated: {:?}", status.last_modified);
            Ok(())
        }
    }
}
```

There is a CLI guide linked to via CLAUDE.md -> SKILL.md. This MUST be used to design this portion of the proposed functionality.

### 6. Index Persistence & Freshness

**Goal**: Don't rebuild index every startup if content hasn't changed.

One possible implementation:

```rust
#[cfg(feature = "fts")]
pub struct IndexMetadata {
    pub doc_count: usize,
    pub last_indexed: SystemTime,
    pub content_hash: String,  // Hash of all concept file mtimes
}

impl TantivySearch {
    /// Check if index exists and is fresh
    pub fn index_is_fresh(&self, concepts_dir: &Path) -> Result<bool> {
        let metadata = self.load_metadata()?;
        let current_hash = compute_content_hash(concepts_dir)?;
        Ok(metadata.content_hash == current_hash)
    }
}
```

Examine for strengths and possible improvements or adjustments needed for the current state of the server code.

**Startup logic**: One possible implementation:

```rust
#[cfg(feature = "fts")]
async fn initialize_fts(state: &Arc<AppState>) {
    if state.fts_search.index_is_fresh(&state.concepts_dir).unwrap_or(false) {
        // Index exists and is current
        state.fts_ready.store(true, Ordering::Release);
        tracing::info!("FTS index loaded from disk");
    } else {
        // Need to build/rebuild - do it in background
        state.start_background_indexing().await;
    }
}
```

Examine for strengths and possible improvements or adjustments needed for the current state of the server code.

## Implementation Plan

### Phase 1: Feature Gating

1. Add `fts` feature to Cargo.toml with optional tantivy dependency
2. Wrap all tantivy-related code in `#[cfg(feature = "fts")]`
3. Create `SearchBackend` trait (if not already done)
4. Ensure project compiles with and without `--features fts`
5. Update the Makefile and the .github CI file to support/test both configurations

### Phase 2: Application State Refactor

1. Create `AppState` struct with shared state
2. Add `Arc<AtomicBool>` for FTS readiness
3. Implement `search_backend()` method with dynamic dispatch
4. Update all tool handlers to use `state.search_backend()`

### Phase 3: Async Background Indexing

1. Move indexing logic to async function
2. Spawn indexing as background task on startup
3. Update readiness flag when complete
4. Add proper error handling and logging
5. Test that server responds immediately while indexing

### Phase 4: CLI Commands

1. Add clap dependency
2. Implement `serve`, `index`, and `status` subcommands
3. Feature-gate the index-related commands
4. Document CLI usage in README

### Phase 5: Index Persistence

1. Add metadata file alongside index
2. Implement content hash computation
3. Add freshness check on startup
4. Skip background indexing if index is fresh

### Phase 6: Testing & Documentation

1. Test feature combinations
2. Test graceful degradation
3. Test background indexing completion
4. Document feature flags in README
5. Document CLI commands

## Dependencies to Add

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }

[dependencies.tantivy]
version = "0.21"
optional = true
```

## Testing Checklist

- [ ] `cargo build` succeeds (no FTS)
- [ ] `cargo build --features fts` succeeds
- [ ] Server starts immediately with `--features fts` (doesn't block)
- [ ] Simple search works while index builds
- [ ] FTS search works after index completes
- [ ] `music-theory-mcp index` builds index
- [ ] `music-theory-mcp status` shows index info
- [ ] Pre-built index loads instantly on restart
- [ ] Index rebuilds when concepts change

## Key Architectural Decisions

1. **Same crate, not separate** — Simpler to maintain, feature flags handle optionality
2. **Trait-based backends** — Clean abstraction, easy to add more backends later
3. **AtomicBool for readiness** — Simple, lock-free, sufficient for boolean state
4. **Background task, not thread** — Tokio-native, integrates with async runtime
5. **Content hash for freshness** — Avoids timestamp issues, detects actual changes
6. **CLI for manual indexing** — Gives operators control, useful for CI/deployment

## Notes for Implementation

- Use `twyg` with structured logs for all logging (you likely already do)
- Consider adding a `/health` or status tool that reports FTS readiness (User: yes, please!)
- The search tool response could include a `"backend": "simple"` or `"backend": "fts"` field (User: this is a GREAT idea -- please do this!)
- Index path should be in config or derived from `paths::skill_root()`
