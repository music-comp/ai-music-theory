# Plan: Post-Migration Cleanup (7 Items from Audit v3)

## Context

After completing the 9-milestone Fabryk extraction migration, audit v3 (`workbench/fabryk-migration-audit-v3.md`) identified 7 actionable cleanup items. This plan addresses all of them in dependency order.

## Items (in execution order)

### 1. Remove redundant inner `#[cfg(feature = "fts")]` (trivial)

**File:** `mcp-server/crates/server/src/search/mod.rs:82`

The `#[cfg(feature = "fts")]` on `concepts_unified_path()` is inside a function already gated by `#[cfg(feature = "fts")]`. Remove the inner gate.

### 2. Promote `SameAs` and `Cites` to first-class Relationship variants in fabryk

**File:** `/Users/oubiwann/lab/oxur/textrynum/crates/fabryk-graph/src/types.rs`

Currently `"SameAs"` parses to `Custom("same_as")` and displays as `"same_as"` (lossy round-trip). Fix by:
- Add `SameAs` and `Cites` variants to the `Relationship` enum (after `AnswersQuestion`, before `Custom`)
- Add serde aliases: `#[serde(alias = "same_as")]` and `#[serde(alias = "cites")]`
- Update `Display` impl: `SameAs => "SameAs"`, `Cites => "Cites"`
- Update `FromStr` impl: `"sameas" | "same_as" => Self::SameAs`, `"cites" => Self::Cites`
- Update `name()` method: `SameAs => "same_as"`, `Cites => "cites"`
- Update `default_weight()`: both get `0.7` (same as current Custom weight)
- Add tests for round-trip fidelity

**Then in ai-music-theory** `graph/mod.rs`: The `to_fabryk_relationship` and `from_fabryk_relationship` wrappers handle SameAs/Cites specially -- after this change, they become pure delegations (no special cases). This feeds into item 4.

### 3. Inline graph free-function wrappers (~50 lines)

**File:** `mcp-server/crates/server/src/graph/mod.rs:74-123`

Remove these 10 functions:
- `is_concept_node(node)` → callers use `node.is_domain()`
- `is_source_node(node)` → callers use `node.is_custom_type("source")`
- `node_id(node)` → callers use `&node.id`
- `node_title(node)` → callers use `&node.title`
- `node_category(node)` → callers use `node.category_or_default()`
- `source_author(node)` → callers use `node.metadata_str("author").unwrap_or("Unknown")`
- `source_year(node)` → callers use `node.metadata_u64("year").map(|y| y as u16)`
- `source_is_converted(node)` → callers use `node.metadata_bool("is_converted").unwrap_or(false)`
- `to_fabryk_relationship(name)` → callers use `name.parse::<Relationship>().unwrap_or(...)`
- `from_fabryk_relationship(rel)` → callers use `rel.to_string()`

Search all callers in graph/mod.rs (CLI handlers + tests) and update. The tests that exercise these wrappers directly can be deleted (the underlying Node methods are tested in fabryk-graph).

### 4. Remove redundant graph re-export aliases (~35 lines)

**File:** `mcp-server/crates/server/src/graph/mod.rs:29-54`

Remove aliased re-exports:
- `Edge as FabrykEdge` → just `Edge`
- `EdgeOrigin as FabrykEdgeOrigin` → just `EdgeOrigin`
- `Node as FabrykNode` → just `Node` (callers import from `fabryk::graph` directly)
- `Relationship as FabrykRelationship` → just `Relationship`
- `GraphStats as FabrykGraphStats` → just `GraphStats`

Check if any code still uses the aliased names. If so, update those callers to use the unaliased names.

### 5. Inline `to_mcp_error()` wrapper in server.rs

**File:** `mcp-server/crates/server/src/server.rs:24-26`

Replace `to_mcp_error(e, "context")` calls with `e.to_mcp_error_with_context("context")` at all ~30 call sites in MusicTheoryToolsRegistry and OthToolsRegistry dispatch. Delete the wrapper function.

### 6. Move `LanceDbConfig` to fabryk

**Files:** 
- Create in: `/Users/oubiwann/lab/oxur/textrynum/crates/fabryk-fts/src/types.rs` (or fabryk-vector)
- Update: `mcp-server/crates/server/src/config.rs`

Move the 3-field struct + defaults. Replace local definition with `pub use fabryk::fts::LanceDbConfig` (or wherever it lands).

### 7. Replace local `SearchConfig` with fabryk's

**Files:**
- `mcp-server/crates/server/src/config.rs` — delete local SearchConfig (~130 lines)
- `mcp-server/crates/server/src/search/mod.rs` — delete `to_fabryk_search_config()` (~25 lines)

Change `Config.search` field to `fabryk::fts::SearchConfig`. The `index_path()` method becomes a standalone helper. The only behavioral change: `index_path` goes from `String` to `Option<String>`. The TOML file uses serde aliases already added to fabryk's type, so existing configs parse correctly.

Handle the default differences:
- Local default backend is `"simple"`, fabryk's is `"tantivy"` — set project default explicitly in `Config::default()`
- Local default fuzzy_distance is 2, fabryk's is 1 — same treatment
- `default_stopword_allowlist()` with music terms — set in project's `Config::default()`

## Verification

```bash
# After each item:
cd mcp-server && make check   # clippy + fmt + tests for all feature combos

# After item 2 (fabryk change):
cd /path/to/textrynum && cargo test -p fabryk-graph
```

## Files Modified

**fabryk (textrynum):**
- `crates/fabryk-graph/src/types.rs` — add SameAs/Cites variants (item 2)
- `crates/fabryk-fts/src/types.rs` — add LanceDbConfig (item 6)

**ai-music-theory:**
- `mcp-server/crates/server/src/search/mod.rs` — items 1, 7
- `mcp-server/crates/server/src/graph/mod.rs` — items 3, 4
- `mcp-server/crates/server/src/server.rs` — item 5
- `mcp-server/crates/server/src/config.rs` — items 6, 7
- `mcp-server/crates/server/src/lib.rs` — update re-exports if needed
