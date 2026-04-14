# Phase 1 Implementation Plan: Helpers & Plumbing

**Milestones:** 1.1 (MCP Helpers), 1.2 (Date Utilities)
**Estimated effort:** 2 sittings (one per milestone)
**Dependencies:** None -- these are the first things to move

---

## Milestone 1.1: MCP Helpers -> fabryk-mcp-core

### Objective

Make three internal helpers public in fabryk-mcp-core, add `McpErrorContextExt`, and move `tier_confidence_schema()`. Then update ai-music-theory to import from fabryk instead of defining locally.

### What Exists in Fabryk Today

| Item | Location in fabryk-mcp-core | Status |
|------|---------------------------|--------|
| `make_tool(name, desc)` | 6 internal copies (diagnostics.rs:20, server.rs:452, registry.rs:129, discoverable.rs:567, service_registry.rs:91, validate.rs:250) | Private, no-schema variant only |
| `serialize_response<T>()` | diagnostics.rs:28 | Private |
| `McpErrorExt` | error.rs | Public, no-context version |
| `McpErrorContextExt` | -- | Does not exist |
| `tier_confidence_schema()` | -- | Does not exist |

### Changes in textrynum (fabryk-mcp-core)

#### 1. Create `src/helpers.rs` -- public helper module

```
pub fn make_tool(name: &str, description: &str, schema: serde_json::Value) -> Tool
pub fn make_tool_no_params(name: &str, description: &str) -> Tool  // calls make_tool with empty_input_schema()
pub fn serialize_response<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData>
pub fn tier_confidence_schema() -> serde_json::Value
```

- `make_tool` accepts a `Value` schema parameter (more general than the existing no-schema internal versions)
- `make_tool_no_params` is a convenience that passes `empty_input_schema()` -- replaces the 6 internal copies
- `serialize_response` lifted from diagnostics.rs:28, made public
- `tier_confidence_schema` moved from ai-music-theory server.rs:853-871

#### 2. Update `src/lib.rs`

- Add `pub mod helpers;`
- Re-export key items: `pub use helpers::{make_tool, make_tool_no_params, serialize_response, tier_confidence_schema};`

#### 3. Update `src/error.rs` -- add McpErrorContextExt

Add the `McpErrorContextExt` trait alongside existing `McpErrorExt`:

```
pub trait McpErrorContextExt {
    fn to_mcp_error(&self, context: &str) -> ErrorData;
}

impl McpErrorContextExt for fabryk_core::Error {
    fn to_mcp_error(&self, context: &str) -> ErrorData {
        // Maps NotFound -> RESOURCE_NOT_FOUND, Config -> INVALID_PARAMS, other -> INTERNAL_ERROR
        // Includes context string in the message
    }
}
```

Re-export from lib.rs: `pub use error::McpErrorContextExt;`

#### 4. Update internal callers

Replace the 6 internal `make_tool` copies in test modules with imports from `crate::helpers::make_tool_no_params` (or `make_tool` where they pass a schema). Files:
- `src/tools/diagnostics.rs` -- remove local fn, use `crate::helpers::make_tool_no_params`
- `src/discoverable.rs` (test module) -- same
- `src/server.rs` (test module) -- same
- `src/registry.rs` (test module) -- same
- `src/service_registry.rs` (test module) -- same
- `src/validate.rs` (test module) -- uses `make_tool_with_schema`, may need signature adjustment

#### 5. Tests

Add to `src/helpers.rs`:
- `test_make_tool_with_schema` -- verifies tool name, description, input_schema
- `test_make_tool_no_params` -- verifies empty schema
- `test_serialize_response` -- verifies JSON serialization into CallToolResult
- `test_tier_confidence_schema` -- verifies schema structure

Add to `src/error.rs`:
- `test_mcp_error_context_not_found` -- NotFound maps to RESOURCE_NOT_FOUND with context
- `test_mcp_error_context_config` -- Config maps to INVALID_PARAMS with context
- `test_mcp_error_context_internal` -- Other maps to INTERNAL_ERROR with context

### Changes in ai-music-theory

#### 1. Update `server.rs`

- Remove lines 24-45 (`make_tool`, `serialize_response`, `to_mcp_error`)
- Remove lines 853-871 (`tier_confidence_schema`)
- Add imports:
  ```
  use fabryk_mcp::{make_tool, serialize_response, tier_confidence_schema};
  ```
- The `to_mcp_error` wrapper (line 43-45) was just `e.to_mcp_error(context)` -- replace inline calls with direct `McpErrorContextExt` method calls, or keep a local one-liner wrapper if cleaner

#### 2. Update `error.rs`

- Remove `McpErrorContextExt` trait definition and impl (lines 17-44)
- Replace with: `pub use fabryk_mcp::McpErrorContextExt;`
- Keep the re-exports of `fabryk::core::Error` and `fabryk::core::Result`

#### 3. Update `lib.rs`

- The re-export `pub use fabryk::fts::SearchDocument;` stays
- Verify `McpErrorContextExt` is accessible via `crate::error::McpErrorContextExt`

### Verification

```bash
# In textrynum repo
make test
make lint

# In ai-music-theory repo
make test
make lint

# Grep to confirm no lingering local definitions
grep -rn "fn make_tool" mcp-server/crates/server/src/
grep -rn "fn serialize_response" mcp-server/crates/server/src/
grep -rn "trait McpErrorContextExt" mcp-server/crates/server/src/
grep -rn "fn tier_confidence_schema" mcp-server/crates/server/src/
# All should return zero results
```

### Risk Notes

- The `make_tool` signature change (adding `Value` parameter) means the 6 internal callers in fabryk need updating. Most are in test modules and trivial. The one in `diagnostics.rs` (non-test) needs care.
- `validate.rs:250` uses `make_tool_with_schema` which already takes a schema -- may just need renaming to use `make_tool`.

---

## Milestone 1.2: Date Utilities -> fabryk-core

### Objective

Move `iso8601_now()` and `days_to_date()` from ai-music-theory's cache.rs to a new `util::time` module in fabryk-core.

### What Exists in Fabryk Today

fabryk-core has `src/util/` with modules: `files`, `ids`, `paths`, `resolver`. No time/date utilities.

### Changes in textrynum (fabryk-core)

#### 1. Create `src/util/time.rs`

Move these two functions (currently at cache.rs:497-531):

```
pub fn iso8601_now() -> String
fn days_to_date(days_since_epoch: u64) -> (u64, u64, u64)  // keep private, implementation detail
```

Only `iso8601_now` needs to be public. `days_to_date` is a helper used only by `iso8601_now`.

#### 2. Update `src/util/mod.rs`

Add `pub mod time;`

#### 3. Tests

Move the existing tests from cache.rs:
- `test_iso8601_now_format` -- verifies YYYY-MM-DDTHH:MM:SSZ format
- `test_days_to_date_epoch` -- verifies (1970, 1, 1)
- `test_days_to_date_known` -- verifies (2025, 1, 1) = day 20089

### Changes in ai-music-theory

#### 1. Update `cache.rs`

- Remove `iso8601_now()` and `days_to_date()` (lines 497-531)
- Remove the 3 tests for these functions
- Add import: `use fabryk::core::util::time::iso8601_now;`
- The call site in `download_cache()` (line 402) is unchanged -- just the import path changes

### Verification

```bash
# In textrynum repo
make test
make lint

# In ai-music-theory repo
make test
make lint

# Confirm removal
grep -rn "fn iso8601_now" mcp-server/crates/server/src/
grep -rn "fn days_to_date" mcp-server/crates/server/src/
# Both should return zero results
```

### Risk Notes

None. Pure functions with no dependencies, no feature flags, no state. Simplest possible move.
