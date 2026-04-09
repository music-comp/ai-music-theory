# Plan: MCP Server Slimming Round 2

## Context

Phases 0-8 of the fabryk migration are complete (34,172 → 13,553 lines). Four areas of further cleanup remain: replacing the local SimpleSearch with fabryk's, using FabrykCli's argument parsing, upstreaming a `BackendSlot<B>` to fabryk-core, and composing config with FabrykConfig. After analysis, two of these (config composition, graph CLI delegation) have poor ROI and should be skipped.

## Execution Order

| Phase | Work Item | Lines Saved | Where |
|-------|-----------|-------------|-------|
| A | Delete local SimpleSearch, use fabryk's | ~844 | mcp-server |
| B | Upstream BackendSlot<B> to fabryk-core | ~100-150 | both repos |
| C | Use FabrykCli arg parsing in cli.rs | ~140 | mcp-server |

**Skipped:**

- Config composition with FabrykConfig — marginal benefit, high TOML-breaking risk. Config already implements ConfigProvider/ConfigManager cleanly.
- Graph CLI handler delegation — path mismatch (`concept_graph.json` vs `graph.json`), domain-specific stats display. Only ~100 lines savings.

---

## Phase A: Delete Local SimpleSearch (~844 lines)

### Why

`search/simple_search.rs` duplicates fabryk-fts's `SimpleSearch` (added in Milestone 3.3b). The only difference: local takes `Config`, fabryk's takes `SearchConfig` + extractor.

### Files to modify

**Delete:**

- `mcp-server/crates/server/src/search/simple_search.rs` (844 lines)

**Modify:**

- `mcp-server/crates/server/src/search/mod.rs`
  - Remove `mod simple_search;` (line 21)
  - Change `pub use simple_search::SimpleSearch;` → `pub use fabryk::fts::SimpleSearch;`
  - Remove local SimpleSearch tests at bottom

- `mcp-server/crates/server/src/state.rs` (line 104)
  - Old: `Arc::new(SimpleSearch::new(config.clone()))`
  - New: Build `SearchConfig` with `content_path` set, then `Arc::new(fabryk::fts::SimpleSearch::with_default_extractor(&search_config))`
  - Use `to_fabryk_search_config(&config.search)?` then set `content_path`

- `mcp-server/crates/server/src/lib.rs` — no change needed (search module stays)

### Alias boost gap

The local SimpleSearch has alias boosting (lines 117-126) that fabryk's doesn't. Accept this minor relevance difference — TantivySearch is the primary backend, SimpleSearch is just the fallback. Can upstream the alias boost to fabryk later if needed.

### Verification

```bash
cd mcp-server && cargo build --features fts,graph && cargo test --features fts,graph && cargo clippy --features fts,graph
```

---

## Phase B: Upstream BackendSlot<B> to fabryk-core (~100-150 lines saved)

### Why

`state.rs` repeats a pattern 3 times: `ServiceHandle` + `Arc<RwLock<Option<Backend>>>` with `update_*`, `require_*`, `is_*_ready`, `set_*_ready` methods. A generic `BackendSlot<B>` eliminates this boilerplate.

### Files to create/modify

**In fabryk (textrynum repo):**

- Create `crates/fabryk-core/src/slot.rs` — new file:

  ```rust
  pub struct BackendSlot<B> {
      service: ServiceHandle,
      backend: Arc<RwLock<Option<B>>>,
  }
  ```

  Methods: `new(name)`, `service()`, `is_ready()`, `set(value)`, `require() -> Result<RwLockReadGuard>`
- Update `crates/fabryk-core/src/lib.rs` — add `pub mod slot;` and re-export

**In mcp-server:**

- `state.rs` — Replace per-backend fields:
  - `fts_backend: Arc<StdRwLock<Option<Arc<TantivySearch>>>>` + `fts_service: ServiceHandle` → `fts_slot: BackendSlot<Arc<TantivySearch>>`
  - `graph_data: Arc<RwLock<Option<LoadedGraph>>>` + `graph_service: ServiceHandle` → `graph_slot: BackendSlot<LoadedGraph>`
  - `vector_backend: Arc<RwLock<Option<Arc<dyn VectorBackend>>>>` + `vector_service: ServiceHandle` → `vector_slot: BackendSlot<Arc<dyn VectorBackend>>`
  - Delete: `update_fts_backend()`, `update_vector_backend()`, `require_graph()`, `require_vector()`, `is_fts_ready()`, `is_vector_ready()`, `set_vector_ready()`
  - Callers change: `state.fts_service.state()` → `state.fts_slot.service().state()`, `state.update_fts_backend(b)` → `state.fts_slot.set(Arc::new(b))`

**Note:** Graph and vector have dual-lock patterns (`shared_graph` uses tokio::sync::RwLock alongside the std RwLock). The `BackendSlot` handles the std lock; the tokio mirror slot stays as a separate field updated in lockstep.

### Verification

```bash
cd ~/lab/oxur/textrynum && cargo build -p fabryk-core && cargo test -p fabryk-core
cd mcp-server && cargo build --features fts,graph && cargo test --features fts,graph
```

---

## Phase C: Use FabrykCli Arg Parsing (~140 lines saved)

### Why

The mcp-server's `cli.rs` has a hand-rolled `Cli` struct and `Commands` enum that duplicates FabrykCli's `CliArgs` and `BaseCommand`. 7 of 8 commands are already in BaseCommand.

### Approach

Use FabrykCli's `CliArgs` + `BaseCommand` for parsing, but keep custom dispatch (FabrykCli's `run()` has placeholder implementations for Serve/Index that don't do anything useful). Only `Cache` is a domain-specific command.

### Key constraint

FabrykCli's `CliArgs` has `--verbose`/`--quiet` but not `--log-level` or `--transport`. The mcp-server needs these. Solution: a wrapper struct:

```rust
#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    base: CliArgs,

    #[arg(short, long)]
    log_level: Option<String>,

    #[cfg(feature = "http")]
    #[arg(long, default_value = "stdio")]
    transport: String,

    // Add Cache as additional subcommand alongside BaseCommand
}
```

### What gets deleted

- The `Commands` enum (~40 lines) — replaced by `BaseCommand` + Cache extension
- Config handler dispatch (~20 lines) — delegate to `fabryk_cli::config_handlers`
- Sources handler bridging (~35 lines) — already delegating, just remove boilerplate
- Vectordb handler dispatch (~15 lines) — delegate to fabryk_cli
- Serve/Index argument definitions (~30 lines) — use BaseCommand's

### What stays

- All handler functions (handle_serve, handle_index, handle_status, handle_graph, handle_cache)
- Domain-specific serve logic (build_server + MCP transport)
- All test code (tests domain behavior, not CLI parsing)

### Files to modify

- `mcp-server/crates/server/src/cli.rs` — replace Cli/Commands, simplify dispatch
- `mcp-server/crates/server/src/main.rs` — update to use new Cli wrapper

### Verification

```bash
cd mcp-server && cargo build --features fts,graph && cargo test --features fts,graph
# Manual: test each CLI command still works
```

---

## Summary

| Phase | Lines Before | Lines After | Files Changed |
|-------|-------------|-------------|---------------|
| A (SimpleSearch) | 13,553 | ~12,709 | 3 files modified, 1 deleted |
| B (BackendSlot) | 12,709 | ~12,559 | 1 created in fabryk, 1 modified in mcp-server |
| C (FabrykCli) | 12,559 | ~12,419 | 2 files modified in mcp-server |
| **Total** | **13,553** | **~12,419** | **~1,134 lines removed** |
