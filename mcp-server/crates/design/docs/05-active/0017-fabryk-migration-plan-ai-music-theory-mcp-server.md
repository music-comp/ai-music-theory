---
number: 17
title: "Fabryk Migration Plan: ai-music-theory MCP Server"
author: "ensuring the"
component: All
tags: [change-me]
created: 2026-04-14
updated: 2026-04-14
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Fabryk Migration Plan: ai-music-theory MCP Server

**Date:** 2026-04-14
**Based on:** `workbench/fabryk-migration-audit-v2.md`
**Scope:** Move ~2,060 lines of domain-agnostic code from `mcp-server/crates/server/` into the appropriate `fabryk-*` crates, leaving only truly domain-specific code (~3,160 lines) in this repo.

## Principles

- **Each milestone is one sitting.** Scoped so that a single conversation session can complete it, including testing.
- **Every milestone leaves both repos green.** After each milestone, `make test` passes in both ai-music-theory and textrynum. No broken intermediate states.
- **Move, don't rewrite.** The code is already correct. We're relocating it and updating imports/re-exports. Resist the urge to refactor during the move.
- **Abstraction check at every decision point.** Before classifying anything as "stays here," apply the structural question: "Does this require domain-specific *knowledge* to function?" If the answer is just domain *data* (config values, display strings), it moves.

## Overview

| Phase | Theme | Milestones | Approx. Lines Moved |
|-------|-------|------------|---------------------|
| 1 | Helpers & Plumbing | 2 | ~120 |
| 2 | Self-Contained Modules | 3 | ~1,400 |
| 3 | Core Architecture | 4 | ~540 |

---

## Phase 1: Helpers & Plumbing

**Goal:** Move the smallest, most self-contained items first. These have zero dependencies on other items being moved, so they validate the workflow (move code, update imports, test both repos) with minimal risk.

### Milestone 1.1: MCP Helpers -> `fabryk-mcp-core`

**What moves:**
- `error.rs:17-44` -- `McpErrorContextExt` trait (already has a TODO for this)
- `server.rs:24-46` -- `make_tool()`, `serialize_response()`, `to_mcp_error()` helper functions
- `server.rs:853-871` -- `tier_confidence_schema()` (generic metadata filter schema)

**What stays:**
- `error.rs:1-15` -- The re-exports of `fabryk::core::Error` / `Result` (these are just `pub use` lines)

**Why this order:** These are pure functions with no state, no config dependencies, and no interaction with other modules being moved. `McpErrorContextExt` already has a TODO. The `make_tool`/`serialize_response` helpers are used by both `MusicTheoryToolsRegistry` and `OthToolsRegistry`, so after the move, both registries will import from `fabryk_mcp` instead of local `crate::server`.

**After this milestone:**
- `server.rs` imports `make_tool`, `serialize_response`, `to_mcp_error` from `fabryk_mcp`
- `McpErrorContextExt` is available to any fabryk-mcp-based project
- Local `error.rs` shrinks to just re-exports

**Estimated size:** ~70 lines of logic + tests

---

### Milestone 1.2: Date Utilities -> `fabryk-core`

**What moves:**
- `cache.rs:497-531` -- `iso8601_now()`, `days_to_date()` (pure date/time functions)

**Why this order:** Zero dependencies. Pure functions. Currently only used by `download_cache()` in the same file, so the import change is trivial.

**After this milestone:**
- `cache.rs` imports `iso8601_now` from `fabryk::core::util` (or wherever it lands)
- Any fabryk project that needs a lightweight ISO 8601 timestamp without pulling in chrono gets it for free

**Estimated size:** ~50 lines of logic + tests

---

## Phase 2: Self-Contained Modules

**Goal:** Move the three largest self-contained blocks. Each is a coherent unit that can be extracted without touching the others.

### Milestone 2.1: Cache Module -> `fabryk-cli`

**What moves:**
- `cache.rs` -- nearly the entire file:
  - Types: `CacheBackend`, `CacheManifest`, `CacheEntry`, `BackendStatus`, `CacheStatusReport`
  - URL helpers: `archive_name()`, `release_url()`, `checksum_url()` (parameterize project prefix)
  - Persistence: `load_manifest()`, `save_manifest()`
  - Shell helpers: `shell_download()`, `verify_checksum()`, `extract_archive()`
  - Orchestration: `download_cache()`, `package_cache()`
  - Parsing: `parse_backend_arg()`
  - Status: `cache_status()`

**Pre-work (within this milestone):**
- Parameterize `DEFAULT_PROJECT_PREFIX` -- make it a function parameter or config-driven
- Parameterize hardcoded paths in `cache_status()` and `package_cache()` -- accept a `BackendPaths` config or read from `ConfigProvider::cache_path()`

**What stays:**
- `cache.rs:32-35` -- `DEFAULT_RELEASE_BASE_URL` and `DEFAULT_PROJECT_PREFIX` constants (domain-specific values passed to the now-generic fabryk functions)
- A thin local module that re-exports fabryk cache types and provides the project-specific constants

**Why this order:** Largest single block of generic code (~500 lines). Self-contained -- only depends on `Config` for path resolution, which can be abstracted via `ConfigProvider`. The hardcoded-path fix is a prerequisite that naturally happens as part of the extraction.

**After this milestone:**
- Any fabryk-cli-based project gets cache download/package/status for free
- Local `cache.rs` shrinks to constants + re-exports

**Estimated size:** ~500 lines of logic + ~300 lines of tests

---

### Milestone 2.2: Search Adapters -> `fabryk-fts`

**What moves:**
- `search/mod.rs:44-87` -- `to_fabryk_search_config()`, `to_fabryk_query_mode()`
- `search/mod.rs:98-189` -- `IndexStats` adapter, `IndexMetadata` adapter
- `search/mod.rs:205-320` -- `build_index()`, `is_index_fresh()`

**What stays:**
- `search/mod.rs:1-40` -- Re-exports from fabryk (already just `pub use` lines)
- Any project-specific search configuration that doesn't generalize

**Why this order:** The search module is already 90% delegation to fabryk. These adapters are the remaining local logic. `SimpleSearch` was already moved (deleted in the recent update). After this, `search/mod.rs` becomes pure re-exports.

**After this milestone:**
- `search/mod.rs` is either deleted entirely or reduced to `pub use fabryk::fts::*` plus any project-specific re-exports
- Any fabryk-fts user gets `build_index()` multi-directory orchestration and backward-compatible adapters

**Estimated size:** ~280 lines of logic + ~170 lines of tests

---

### Milestone 2.3: Graph Adapters -> `fabryk-graph`

**What moves:**
- `graph/mod.rs:73-96` -- `GraphStats`, `LoadedGraph` adapter types
- `graph/mod.rs:102-154` -- Node discrimination helpers (`is_concept_node`, `is_source_node`, `node_id`, `node_title`, `node_category`, `source_author`, `source_year`, `source_is_converted`)
- `graph/mod.rs:160-198` -- `to_fabryk_relationship()`, `from_fabryk_relationship()` (relationship mapping/display)
- `graph/mod.rs:217-296` -- `load_concept_graph()`, `compute_graph_stats()`, `build_graph()`

**What stays:**
- `graph/mod.rs:29-60` -- Re-exports (already just `pub use` lines)
- Any truly project-specific graph CLI handlers

**Design note:** The node helpers (`is_concept_node`, `node_title`, etc.) are strong candidates for methods on fabryk's `Node` type rather than free functions. This milestone may involve a small API addition to `fabryk-graph` rather than a pure move.

**After this milestone:**
- `graph/mod.rs` is either deleted or reduced to re-exports + project-specific CLI handlers
- Any fabryk-graph user gets `LoadedGraph`, node inspection helpers, and graph loading/building

**Estimated size:** ~220 lines of logic + ~200 lines of tests (estimated, partly behind feature gates)

---

## Phase 3: Core Architecture

**Goal:** Move the structural patterns that other modules depend on. These are done last because they touch more surface area and because earlier phases may reveal adjustments needed.

### Milestone 3.1: Config Types -> `fabryk-core` / `fabryk-cli`

**What moves:**
- `config.rs:33-76` -- `PathsConfig` (named content paths with expansion)
- `config.rs:116-270` -- `QueryMode`, `SearchConfig` (the local `QueryMode` is a duplicate of fabryk's)
- `config.rs:273-308` -- `LanceDbConfig`
- `config.rs:360-448` -- `ConfigProvider` impl, `ConfigManager` impl

**What stays:**
- `config.rs:14-31` -- `Config` struct definition (project-specific aggregation of sub-configs)
- `config.rs:79-100` -- `SourcesConfig` with `oxford`/`general`/`papers` (project-specific taxonomy)
- `config.rs:220-229` -- `default_stopword_allowlist()` (music-specific terms)
- `config.rs:335-355` -- `ServerConfig` default with `"music-theory-skill"` name
- `config.rs:439` -- `MUSIC_THEORY_CONFIG_DIR` env var

**Design consideration:** `PathsConfig` field names (`concept_cards`, `sources_md`, etc.) are generic Fabryk vocabulary, but the specific *set* of fields is project-specific. The fabryk version might use a `HashMap<String, String>` or a trait, while this project keeps its typed struct. Needs discussion during implementation.

**After this milestone:**
- Eliminates the local `QueryMode` duplicate
- Generic search/vector config available to all fabryk projects
- Project keeps only domain-specific config aggregation

**Estimated size:** ~200 lines of logic + tests

---

### Milestone 3.2: AppState Pattern -> `fabryk-mcp` or `fabryk-core`

**What moves:**
- `state.rs` -- The `AppState` struct and its methods: search backend management, FTS readiness tracking, graph data lifecycle (via ServiceHandle), vector backend lifecycle, dynamic backend switching.

**What stays:**
- Any project-specific state initialization details

**Design consideration:** `AppState` is parameterized over the specific backends (FTS, graph, vector) via feature flags. The fabryk version needs to preserve this flexibility. It may make sense to extract a `FabrykAppState<C: ConfigProvider>` that projects extend, rather than moving the exact struct.

**After this milestone:**
- New fabryk-mcp projects get production-ready state management with service lifecycle, backend switching, and graceful degradation out of the box

**Estimated size:** ~150 lines of logic + tests

---

### Milestone 3.3: CLI Deduplication -> `fabryk-cli`

**What moves / merges:**
- `cli.rs:96-101` -- `GraphCommands`/`GraphSubcommand` -- reconcile with fabryk-cli's existing `GraphCommand`/`GraphSubcommand`
- `cli.rs:129-149` -- `VectordbCommands`/`VectordbSubcommand` -- reconcile with fabryk-cli's `VectordbCommand`
- `cli.rs:152-185` -- `CacheCommands`/`CacheSubcommand` -- move to fabryk-cli (new, not yet in fabryk)
- `cli.rs:276-299` -- `handle_config_command()` -- already delegates to `fabryk_cli::config_handlers`, may be fully replaceable
- `cli.rs:199-273` -- `handle_command()` -- generic dispatch logic

**What stays:**
- `cli.rs:22-42` -- `Cli` struct with project-specific naming
- Any project-specific command handling that can't be generalized

**Design consideration:** fabryk-cli already has `FabrykCli<C>` with `BaseCommand` (Serve, Index, Version, Health, Graph, Config, Sources, Vectordb). The local `cli.rs` has a parallel `Commands` enum. The goal is to use `FabrykCli<C>` directly (or via `CliExtension` trait) and add only the project-specific commands. This may be the most design-heavy milestone.

**After this milestone:**
- Local CLI reduced to project-specific naming + any extension commands
- CacheCommands available to all fabryk-cli projects

**Estimated size:** ~100 lines moved/deleted + reconciliation work

---

### Milestone 3.4: Server Builder Pattern -> `fabryk-mcp`

**What moves:**
- `server.rs:894-1247` -- The generic portion of `build_server()`: composition of ContentTools, SourceTools, GuideTools, FtsTools, GraphTools, SemanticSearchTools, QuestionSearchTools, HealthTools with config-driven names/descriptions/schemas. The pattern of wiring providers, setting names/descriptions via HashMaps, adding extra schemas, building backend probes, and assembling a CompositeRegistry.

**What stays:**
- `server.rs:51-847` -- `MusicTheoryToolsRegistry` and `OthToolsRegistry` (domain-specific tool registries)
- `server.rs:1249-1257` -- The `.with_description("Music Theory AI Skill - ...")` and resource fallback wiring
- The two `.add(music_theory_tools)` / `.add(oth_tools)` calls

**Design consideration:** This is the most architectural milestone. The extracted pattern might be a `FabrykServerBuilder` that takes a config and produces a pre-wired `CompositeRegistry` with all the standard Fabryk tools (content, source, guide, FTS, graph, semantic, health), letting projects just `.add()` their domain-specific registries and resources. Alternatively, it could be a set of builder functions/macros. Needs design discussion.

**After this milestone:**
- New fabryk-mcp projects get a one-call server builder that wires up all standard tools
- Local `server.rs` reduced to domain-specific registries + the final composition call

**Estimated size:** ~350 lines extracted into a reusable pattern + tests

---

## Dependency Graph

```
Phase 1 (no dependencies)
  1.1  MCP Helpers
  1.2  Date Utilities

Phase 2 (depends on Phase 1 for workflow validation, but not code dependencies)
  2.1  Cache Module        (depends on 1.2 for iso8601_now)
  2.2  Search Adapters     (independent)
  2.3  Graph Adapters      (independent)

Phase 3 (depends on Phase 2 items being stable)
  3.1  Config Types        (independent within phase)
  3.2  AppState            (depends on 3.1 for config types)
  3.3  CLI Deduplication   (depends on 2.1 for CacheCommands)
  3.4  Server Builder      (depends on 1.1 for MCP helpers, benefits from 3.1-3.3 being done)
```

## Risk Notes

- **Milestone 3.3 (CLI)** has the highest design risk -- reconciling two parallel CLI structures. May need a design discussion before implementation.
- **Milestone 3.4 (Server Builder)** is the most architectural -- extracting a reusable pattern from concrete code. Benefits from all prior milestones being done so the remaining local code is minimal and the pattern is clear.
- **Milestone 3.1 (Config)** has a design question around `PathsConfig` (typed struct vs. HashMap). Needs discussion.
- **All milestones** should be preceded by ensuring the test suite is green in both repos, and followed by the same check.
