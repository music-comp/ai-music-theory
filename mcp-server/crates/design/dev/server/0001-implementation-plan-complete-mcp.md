# Implementation Plan: Remove Dead Code Attributes and Complete MCP Integration

**Goal:** Implement all features currently marked with `#[allow(dead_code)]` or `#[allow(unused_imports)]` to eliminate "looking the other way" at clippy warnings.

**Date:** 2026-01-25
**Estimated Effort:** 4-6 hours
**Complexity:** Medium

## Executive Summary

Analysis of the codebase reveals that most `#[allow]` attributes fall into three categories:

1. **Resources** (Priority 1) - Fully implemented but not wired into ServerHandler
2. **Error Handling** (Priority 2) - Inspection methods exist but unused; could improve error messages
3. **Future Features** (Priority 3) - Config paths for planned features (concepts_unified, skill_docs)

**Key Finding:** All 8 tools are already fully implemented and registered via rmcp macros. The main gap is resources support.

---

## Phase 1: Resources Integration (Priority: HIGH)

### Status
- ✅ Code complete in `resources/mod.rs` (217 lines)
- ✅ 4 resources with default content
- ❌ NOT registered in ServerHandler
- ❌ Resources capability NOT enabled

### Implementation

**File:** `crates/server/src/server.rs`

#### Step 1.1: Enable Resources Capability
**Lines:** 271-275 (in `get_info()` method)

**Current:**
```rust
capabilities: ServerCapabilities::builder()
    .enable_tools()
    .build(),
```

**Change to:**
```rust
capabilities: ServerCapabilities::builder()
    .enable_tools()
    .enable_resources()
    .build(),
```

#### Step 1.2: Add Required Imports
**Lines:** 1-12 (at top of file)

**Add:**
```rust
use rmcp::model::{
    // ... existing imports ...
    ListResourcesResult, ReadResourceRequestParams, ReadResourceResult,
    PaginatedRequestParams, RawResource, RawTextContent,
};
use rmcp::service::{RequestContext, RoleServer};
```

#### Step 1.3: Implement list_resources() Method
**Lines:** After 286 (in ServerHandler impl block)

**Add:**
```rust
fn list_resources(
    &self,
    _request: Option<PaginatedRequestParams>,
    _context: RequestContext<RoleServer>,
) -> impl Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
    async {
        let resources = resources::list_resources()
            .into_iter()
            .map(|info| RawResource {
                uri: info.uri,
                name: info.name,
                description: Some(info.description),
                mime_type: Some(info.mime_type),
                ..Default::default()
            })
            .collect();

        Ok(ListResourcesResult {
            resources,
            ..Default::default()
        })
    }
}
```

#### Step 1.4: Implement read_resource() Method
**Lines:** After list_resources()

**Add:**
```rust
fn read_resource(
    &self,
    request: ReadResourceRequestParams,
    _context: RequestContext<RoleServer>,
) -> impl Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
    let config = self.config.clone();
    async move {
        match resources::get_resource(&config, &request.uri) {
            Ok(content) => Ok(ReadResourceResult {
                contents: vec![RawTextContent {
                    text: content,
                    mime_type: Some("text/markdown".to_string()),
                }
                .into()],
                ..Default::default()
            }),
            Err(_) => Err(ErrorData::new(
                ErrorCode::RESOURCE_NOT_FOUND,
                format!("Resource not found: {}", request.uri),
                None,
            )
            .into()),
        }
    }
}
```

#### Step 1.5: Remove Dead Code Attributes from resources/mod.rs
**File:** `crates/server/src/resources/mod.rs`

**Remove all `#[allow(dead_code)]` attributes from:**
- Line 9: `struct ResourceInfo`
- Line 20: `fn list_resources()`
- Line 52: `fn get_resource()`
- Line 84: `fn default_conventions()`
- Line 111: `fn default_scope()`
- Line 151: `fn default_sources()`
- Line 183: `fn default_index()`

**Testing:**
```bash
# Verify resources are registered
cargo run 2>&1 | grep -i resource

# Manual test with MCP client (if available)
# Should see 4 resources listed
```

**Expected Output:**
- Server logs should show resources capability enabled
- MCP clients should be able to list and read 4 resources

---

## Phase 2: Enhanced Error Handling (Priority: MEDIUM)

### Status
- ✅ Error inspection methods defined (is_io, is_not_found, is_config)
- ✅ ParseError, SearchError variants defined
- ❌ None are currently used
- ❌ All errors wrapped as generic INTERNAL_ERROR

### Implementation

**File:** `crates/server/src/server.rs`

#### Step 2.1: Use Error Inspection for Better Messages
**Lines:** Throughout tool handlers (85-264)

**Pattern to apply to all tool handlers:**

**Current:**
```rust
.map_err(|e| {
    ErrorData::new(
        ErrorCode::INTERNAL_ERROR,
        format!("Error listing sources: {}", e),
        None,
    )
})?
```

**Change to:**
```rust
.map_err(|e| {
    let (code, msg) = if e.is_not_found() {
        (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
    } else if e.is_config() {
        (ErrorCode::INVALID_PARAMS, format!("Configuration error: {}", e))
    } else {
        (ErrorCode::INTERNAL_ERROR, format!("Error: {}", e))
    };
    ErrorData::new(code, msg, None)
})?
```

**Apply this pattern to all 8 tool handlers:**
1. list_sources (line 85)
2. get_source_chapter (line 103)
3. get_source_pdf_path (line 127)
4. list_concepts (line 148)
5. get_concept (line 173)
6. search_concepts (line 195)
7. list_guides (line 220)
8. get_guide (line 240)

#### Step 2.2: Remove Dead Code Attributes from Error Methods
**File:** `crates/server/src/error.rs`

**Remove `#[allow(dead_code)]` from:**
- Line 84: `pub fn is_io(&self)`
- Line 91: `pub fn is_not_found(&self)`
- Line 98: `pub fn is_config(&self)`

**Note:** Keep ParseError and SearchError with `#[allow(dead_code)]` for now - they're future features for when we add:
- Markdown frontmatter parsing (ParseError)
- Advanced search validation (SearchError)

**Testing:**
```bash
# Test error handling
cargo test

# Try accessing non-existent concept
# Should get RESOURCE_NOT_FOUND instead of INTERNAL_ERROR
```

---

## Phase 3: Clean Up Misleading Attributes (Priority: LOW)

### Status
- Tools ARE being used via #[tool] macros
- `#[allow(unused_imports)]` is misleading
- Better documentation needed

### Implementation

**File:** `crates/server/src/tools/mod.rs`

#### Step 3.1: Replace Allow Attributes with Better Comments
**Lines:** 6-19

**Current:**
```rust
// Allow unused imports - these are part of the planned API and will be used
// when the full tool registration is implemented
#[allow(unused_imports)]
pub use concepts::{get_concept, list_concepts};
#[allow(unused_imports)]
pub use guides::{get_guide, list_guides};
#[allow(unused_imports)]
pub use search::search_concepts;
#[allow(unused_imports)]
pub use sources::{get_source_chapter, get_source_pdf_path, list_sources};

// Re-export common types
#[allow(unused_imports)]
pub use crate::error::{Error, Result};
```

**Change to:**
```rust
// Tool function exports - Used by server.rs via #[tool] macro expansion
// The macro system uses these via qualified paths, so they appear unused to clippy
pub use concepts::{get_concept, list_concepts};
pub use guides::{get_guide, list_guides};
pub use search::search_concepts;
pub use sources::{get_source_chapter, get_source_pdf_path, list_sources};

// Re-export common types for tool implementations
pub use crate::error::{Error, Result};
```

**Note:** This will cause clippy warnings, but they're false positives due to macro usage. Alternative: keep `#[allow(unused_imports)]` but with accurate comments.

**Decision point for user:** Keep the allow attributes with better comments, or remove them and accept the clippy false positives?

---

## Phase 4: Future Config Paths (Priority: VERY LOW)

### Status
- Config paths defined for future features
- Not currently used
- Need design work before implementation

### Analysis

**File:** `crates/server/src/config.rs`

**Paths with #[allow(dead_code)]:**
1. `base` (line 30) - Base path for skill repository
2. `concepts_unified` (line 34) - Unified concept storage (future)
3. `skill_docs` (line 37) - Skill documentation directory (future)
4. `base_path()` method (line 44)
5. `skill_docs_path()` method (line 66)

**Recommendation:** **KEEP** these attributes for now because:
1. Design doc clearly marks these as "(future)" features
2. No clear requirements yet for concepts_unified
3. skill_docs requires actual markdown documentation to be created
4. Removing them would delete planned functionality

**Alternative approach:**
- Create stub documentation files in `${paths.base}/` directory
- Implement skill_docs resource reading from actual files
- This would make skill_docs_path() used

**Action:** Ask user if they want to implement skill_docs now or keep as future work.

---

## Critical Files Modified

| File | Lines Changed | Purpose |
|------|--------------|---------|
| `crates/server/src/server.rs` | ~80 additions | Add resource handlers, improve error handling |
| `crates/server/src/resources/mod.rs` | 7 deletions | Remove dead code attributes |
| `crates/server/src/error.rs` | 3 deletions | Remove dead code attributes |
| `crates/server/src/tools/mod.rs` | ~10 changes | Update comments or remove attributes |

**Total estimated changes:** ~100 lines

---

## Testing Strategy

### Unit Tests
```bash
# All existing tests should pass
make test

# Should show 16 tests passing (current count)
```

### Integration Tests

**Test 1: Resource Listing**
```bash
# Start server
cargo run

# With MCP client, call list_resources
# Should return 4 resources
```

**Test 2: Resource Reading**
```bash
# With MCP client, read resource
# read_resource("skill://conventions")
# Should return markdown content
```

**Test 3: Error Handling**
```bash
# Try to get non-existent concept
# get_concept("nonexistent")
# Should return RESOURCE_NOT_FOUND (not INTERNAL_ERROR)
```

**Test 4: Clippy Check**
```bash
make lint

# Should pass with fewer #[allow] attributes
```

### Manual Verification

1. **Check server startup logs:**
   ```
   INFO Registered tools count=8
   INFO Resources capability enabled
   ```

2. **Verify resources accessible:**
   - skill://conventions
   - skill://scope
   - skill://sources
   - skill://index

3. **Test error messages:**
   - Not found errors should be specific
   - Config errors should indicate invalid params
   - I/O errors should show internal error

---

## Rollback Plan

If issues arise:

1. **Git restore:** All changes in single commit
   ```bash
   git restore crates/server/src/server.rs
   git restore crates/server/src/resources/mod.rs
   git restore crates/server/src/error.rs
   ```

2. **Minimal working state:** Resources are optional, tools will still work

3. **Testing checkpoint:** Run `make test` before and after each phase

---

## Success Criteria

**Phase 1 (Resources):**
- ✅ ServerCapabilities includes resources
- ✅ list_resources() returns 4 resources
- ✅ read_resource() returns markdown content
- ✅ No #[allow(dead_code)] in resources/mod.rs
- ✅ All tests pass

**Phase 2 (Error Handling):**
- ✅ NOT_FOUND errors use RESOURCE_NOT_FOUND code
- ✅ Config errors use INVALID_PARAMS code
- ✅ No #[allow(dead_code)] on error inspection methods
- ✅ All tests pass

**Phase 3 (Cleanup):**
- ✅ Better comments in tools/mod.rs
- ✅ Clippy warnings addressed or documented
- ✅ All tests pass

**Phase 4 (Future):**
- ✅ Decision made on skill_docs implementation
- ✅ Attributes kept or removed with rationale

---

## Follow-up Work (Out of Scope)

1. **Create actual skill documentation files** (CONVENTIONS.md, SCOPE.md, etc.)
2. **Implement concepts_unified** feature (requires design work)
3. **Add resource change notifications** (when content updates)
4. **Add ParseError usage** (when adding frontmatter parsing)
5. **Add SearchError usage** (when adding query validation)
6. **Consider Tantivy integration** for advanced search

---

## Dependencies on External Systems

**None** - All changes are internal to the Rust codebase.

**rmcp v0.14** already supports resources; just need to implement the methods.

---

## Notes for Implementation

1. **Follow Rucksack pattern:** The resources implementation mirrors how Rucksack handles similar features
2. **Error handling enhancement is optional:** Phase 2 can be skipped if basic error handling is sufficient
3. **Future config paths:** Recommend keeping #[allow] attributes until features are designed
4. **Test incrementally:** Run tests after each phase before moving to next

---

## Questions for User

Before proceeding, please clarify:

1. **Phase 3 decision:** Should we remove `#[allow(unused_imports)]` from tools/mod.rs and accept clippy false positives, or keep them with better comments?

2. **Phase 4 decision:** Should we implement skill_docs resource reading from actual files now, or keep it as future work with #[allow(dead_code)]?

3. **Priority:** Would you like to implement all phases, or focus only on Phase 1 (resources)?
