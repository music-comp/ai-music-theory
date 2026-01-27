# Plan: v0.2.0 Polish & Enhancement - External Crates + QA Improvements

**Version:** 3.0
**Created:** 2026-01-27
**Target:** v0.2.0 (pre-tag polish)
**Scope:** Implementation work using ecosystem crates

---

## Executive Summary

Before tagging v0.2.0, implement polish items from QA Report 2 and migrate to external crates for better maintainability.

**Phase 1-4 Status:** ✅ Complete and validated - all critical issues fixed!

**This Plan:** Enhance v0.2.0 with:
1. **Crate migration** - Replace custom implementations with ecosystem crates
2. **Polish fixes** - Fix snippets, add category filtering, verify phrase search
3. **Final release** - Tag v0.2.0 with all enhancements included

**Strategy:** Leverage high-quality, popular Rust crates for better code quality and maintainability.

---

## Task #1: Migrate to External Crates

### Goal
Replace custom implementations with well-maintained ecosystem crates for better quality and maintainability.

### 1A: Replace Custom Stopwords with `stop-words` Crate

**Current Implementation:**
- File: `crates/server/src/search/stopwords.rs`
- ~100 hardcoded English stopwords
- Custom `StopwordFilter` with `HashSet`

**Migration to `stop-words` Crate:**

**Why:**
- Professional, comprehensive stopword lists
- Multiple language support (future: Italian, German music terms)
- Well-maintained (active 2025-2026)
- Sources from Stopwords ISO and NLTK
- 470K+ downloads

**Changes Required:**

1. **Add dependency** (`Cargo.toml` workspace):
```toml
[workspace.dependencies]
stop-words = "0.8"
```

2. **Update feature** (`crates/server/Cargo.toml`):
```toml
[dependencies]
stop-words = { workspace = true, optional = true }

[features]
fts = ["dep:tantivy", "dep:regex", "dep:stop-words"]
```

3. **Refactor StopwordFilter** (`stopwords.rs`):
```rust
use stop_words::{get, LANGUAGE};

pub struct StopwordFilter {
    stopwords: HashSet<String>,
    allowlist: HashSet<String>,
}

impl StopwordFilter {
    pub fn new(config: &SearchConfig) -> Self {
        // Get English stopwords from crate
        let base_stopwords = get(LANGUAGE::English);
        let mut stopwords: HashSet<String> = base_stopwords
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect();

        // Add custom stopwords from config
        for word in &config.custom_stopwords {
            stopwords.insert(word.to_lowercase());
        }

        // Build allowlist
        let allowlist: HashSet<String> = config.stopword_allowlist
            .iter()
            .map(|s| s.to_string())
            .collect();

        Self { stopwords, allowlist }
    }

    // filter() and is_stopword() methods unchanged
}
```

4. **Remove hardcoded constant:**
- Delete `ENGLISH_STOPWORDS` constant (no longer needed)
- Update tests to not reference the constant

5. **Update tests:**
- Tests should still pass with new implementation
- May need minor adjustments for expanded stopword list

**Benefits:**
- More comprehensive stopword list (~500 vs ~100)
- Easy to add languages later: `get(LANGUAGE::Italian)`
- Better maintained
- Industry standard lists

### 1B: Evaluate `tantivy-stemmers` (Optional)

**Current:**
- Using Tantivy's built-in `Stemmer::default()` (English only)

**Decision:** DEFER to post-v0.2.0
- Current stemmer works well
- No multilingual needs yet
- Would require schema changes and index rebuild
- Can evaluate for v0.3.0 when we add Italian/German support

**Document for future:** Add note in design doc about potential upgrade path.

---

## Task #2: Fix Polish Items from QA Report 2

### Goal
Address remaining usability issues identified in QA testing.

### 2A: Fix Snippet Generation (CRITICAL)

**Problem:** Many results return empty `"snippet": ""` - inconsistent and unhelpful.

**Current Implementation:** `crates/server/src/search/tantivy_search.rs` lines ~168-210

**Investigation Needed:**
1. Read current `generate_snippet()` method
2. Identify why snippets are sometimes empty
3. Likely issues:
   - Query terms not found in description/content
   - Empty fields in some documents
   - Snippet extraction logic failing silently

**Fix Approach:**
1. Ensure fallback chain:
   - Try description first
   - Fall back to content if description empty/no match
   - Fall back to first N chars of content if no match anywhere
   - **Never return empty string**

2. Improve context extraction:
   - Find first occurrence of any query term
   - Extract surrounding context (configurable window)
   - Handle edge cases (term at start/end of field)

3. Consider using Tantivy's built-in snippet generator:
   - `tantivy::snippet::SnippetGenerator`
   - Handles highlighting, context extraction
   - May be better than custom implementation

**Test:**
- Verify all QA test queries return non-empty snippets
- Test edge cases (no match, term at boundaries)

### 2B: Verify/Improve Phrase Search

**Problem:** QA report unclear if `"perfect authentic cadence"` matches exact phrase or just all words.

**Investigation:**
1. Read `parse_phrases()` in `query.rs`
2. Read `create_phrase_query()` in `query.rs`
3. Test with reversed phrase: `"authentic cadence perfect"` should return different results

**Fix (if needed):**
- Ensure `PhraseQuery` is actually used (not just `TermQuery` for each word)
- Verify tokenization preserves order
- Add integration test for phrase order sensitivity

### 2C: Add Category Filtering Parameter

**Problem:** No way to scope searches to specific categories.

**Desired:** `search_concepts(query: "suspension", category: "voice-leading")`

**Implementation:**

1. **Update SearchConceptsParams** (`tools/search.rs`):
```rust
pub struct SearchConceptsParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    #[cfg_attr(not(feature = "fts"), allow(dead_code))]
    pub query_mode: Option<QueryMode>,

    // NEW: Category filter
    #[serde(default)]
    pub category: Option<String>,
}
```

2. **Update TantivySearch** (`tantivy_search.rs`):
```rust
async fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>> {
    // ... existing query building ...

    // Add category filter if specified
    if let Some(ref cat) = params.category {
        let category_query = TermQuery::new(
            Term::from_field_text(self.schema.category, cat),
            IndexRecordOption::Basic,
        );
        // Combine with main query using AND
        let mut combined = BooleanQuery::new(vec![]);
        combined.add_clause(Occur::Must, Box::new(category_query));
        combined.add_clause(Occur::Must, query);
        query = Box::new(combined);
    }

    // ... rest of search ...
}
```

3. **Update SimpleSearch** (`simple_search.rs`):
```rust
async fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>> {
    // ... existing code ...

    // Filter by category if specified
    if let Some(ref cat) = params.category {
        results.retain(|r| r.category == *cat);
    }

    // ... rest of search ...
}
```

4. **Add tests:**
- Test category filtering returns only matching categories
- Test combined query + category filter
- Test invalid category (returns empty)

### 2D: Expose Result Count Parameter (Easy)

**Problem:** `limit` is already configurable but defaults to 10. QA wants flexibility.

**Current:** `default_limit()` returns 10

**Change:** Document that users can pass any `limit` value:
```json
{
  "query": "suspension",
  "limit": 20,  // Custom limit
  "category": "voice-leading"  // Optional category
}
```

**Action:** Just document in MCP tool description - already works!

---

## Task #3: Complete v0.2.0 Release

### Goal
Tag v0.2.0 with all enhancements included.

### 3A: Update Config Version String

**File:** `crates/server/config/default.toml`
**Change:** Line 3: `version = "0.1.0"` → `version = "0.2.0"`

### 3B: Create CHANGELOG.md

**Location:** `CHANGELOG.md` (root directory)

**Content:** (from earlier plan, with additions)

Add to v0.2.0 section:
```markdown
### Added (cont'd)
- **Category filtering** - Scope searches to specific categories
- **Improved snippet generation** - Consistent, non-empty snippets for all results
- **Better stopwords** - Migrated to `stop-words` crate with comprehensive lists

### Changed (cont'd)
- Stopword implementation now uses industry-standard lists (500+ words)
```

### 3C: Update Documentation

**Files to update:**
1. **MIGRATION.md** - Note new `category` parameter
2. **README.md** - Document category filtering in search section
3. **Tool descriptions** - Update `search_concepts` MCP tool description

### 3D: Create Git Tag

```bash
git tag -a 0.2.0 -m "Version 0.2.0: Tantivy full-text search with polish

Major features:
- Tantivy full-text search with BM25 ranking
- Multi-word query support (Smart AND/OR mode)
- Stopword filtering (stop-words crate)
- Phrase search support
- Category filtering parameter
- Consistent snippet generation
- CLI commands for index management
- 444+ tests passing

QA Validated: All critical issues fixed
Breaking changes: None
Migration: See MIGRATION.md"
```

---

## Implementation Order

### Phase 1: Crate Migration (1-2 hours)
1. Add `stop-words` dependency
2. Refactor `StopwordFilter` to use crate
3. Remove hardcoded stopwords constant
4. Update tests
5. Verify all tests still pass

### Phase 2: Fix Snippets (2-3 hours)
1. Investigate snippet generation code
2. Implement fix (fallback chain or Tantivy SnippetGenerator)
3. Test with QA queries
4. Verify all results have non-empty snippets

### Phase 3: Category Filtering (1-2 hours)
1. Add `category` parameter to `SearchConceptsParams`
2. Implement filtering in TantivySearch
3. Implement filtering in SimpleSearch
4. Add tests for category filtering

### Phase 4: Verify Phrase Search (30 min)
1. Test phrase order sensitivity
2. Fix if needed (likely already works)
3. Add integration test

### Phase 5: Documentation & Release (1 hour)
1. Update config version string
2. Create/update CHANGELOG.md
3. Update MIGRATION.md and README.md
4. Create git tag
5. Commit all changes

**Total Estimated Time:** 6-8 hours of implementation work

---

## Files to Create/Modify

### Modified Files

| File | Changes | Lines |
|------|---------|-------|
| `Cargo.toml` (workspace) | Add stop-words dependency | 1 |
| `crates/server/Cargo.toml` | Add stop-words to fts feature | 1 |
| `crates/server/src/search/stopwords.rs` | Use stop-words crate | -50, +30 |
| `crates/server/src/search/tantivy_search.rs` | Fix snippets, add category filter | +50 |
| `crates/server/src/search/simple_search.rs` | Add category filter | +5 |
| `crates/server/src/tools/search.rs` | Add category parameter | +5 |
| `crates/server/config/default.toml` | Update version to 0.2.0 | 1 |
| `crates/server/README.md` | Document category filtering | +20 |
| `crates/server/MIGRATION.md` | Update with new features | +15 |
| `CHANGELOG.md` | Full changelog for 0.1.0 and 0.2.0 | NEW, ~250 |

### New Tests

| Test File | New Tests | Purpose |
|-----------|-----------|---------|
| `stopwords.rs` | Update existing | Verify stop-words crate usage |
| `search_qa_integration.rs` | +5 tests | Category filtering, snippet verification |

---

## Success Criteria

### Task #1: Crate Migration
- [ ] `stop-words` dependency added and compiling
- [ ] `StopwordFilter` uses crate instead of hardcoded list
- [ ] All existing stopword tests pass
- [ ] Stopword list is more comprehensive (500+ words)
- [ ] No functionality regression

### Task #2: Polish Fixes
- [ ] **Snippets:** All search results return non-empty snippets
- [ ] **Phrase search:** Order sensitivity verified/working
- [ ] **Category filtering:** Can filter by category, tests pass
- [ ] **Result count:** Documented (already works)
- [ ] All QA test queries work with new features

### Task #3: Release
- [ ] Config version updated to 0.2.0
- [ ] CHANGELOG.md created with complete history
- [ ] Documentation updated (README, MIGRATION.md)
- [ ] Git tag 0.2.0 created with detailed message
- [ ] All 444+ tests passing
- [ ] `make check` passes
- [ ] Working tree clean

---

## Verification Steps

### After Crate Migration
```bash
# Build with new dependency
cargo build --features fts

# Run tests
cargo test --features fts stopwords

# Verify stopwords work
# (manual test via MCP or CLI)
```

### After Snippet Fix
```bash
# Run QA queries and verify snippets
cargo run --features fts -- serve &

# Test queries from QA report, check all have snippets
# Example: search("fugue subject answer") should have snippet
```

### After Category Filtering
```bash
# Test category filtering
cargo test --features fts category

# Manual test
# search(query="suspension", category="voice-leading")
```

### Final Release Verification
```bash
# All tests pass
make check

# Build succeeds
cargo build --release --features fts

# Index builds
./target/release/music-theory-mcp index --force

# Server starts
./target/release/music-theory-mcp serve

# Version correct
grep "version.*0.2.0" crates/server/config/default.toml

# Tag exists
git tag -l | grep 0.2.0
```

---

## Risk Mitigation

### Risk 1: stop-words Crate Breaking Changes
**Mitigation:**
- Pin to specific version (0.8)
- Test thoroughly before committing
- Keep custom stopwords as backup

### Risk 2: Snippet Fix More Complex Than Expected
**Mitigation:**
- Start with simple fallback chain
- Use Tantivy's SnippetGenerator if custom approach fails
- Allow empty snippets as last resort (log warning)

### Risk 3: Category Filtering Performance
**Mitigation:**
- Category is indexed STRING field (fast)
- Use TermQuery (exact match, efficient)
- Test with large result sets

### Risk 4: Scope Creep
**Mitigation:**
- Stick to defined tasks only
- Defer fuzzy matching, synonyms, etc. to v0.3.0
- Focus on QA-identified issues

---

## Out of Scope (Defer to Future)

These items from QA report are intentionally deferred:

**v0.3.0+ Features:**
- Fuzzy matching (typo tolerance)
- Synonym expansion (V7 ↔ dominant seventh)
- Highlighted snippets (bold matching terms)
- Search suggestions ("Did you mean...?")
- Field boosting tuning (already implemented, minor tweaks only)
- tantivy-stemmers migration (multilingual support)

**Reason:** Focus on core usability (snippets, category filtering) for v0.2.0. Advanced features can wait.

---

## Critical Files Reference

**Stopwords:**
- `crates/server/src/search/stopwords.rs` (lines 1-270)

**Snippets:**
- `crates/server/src/search/tantivy_search.rs` (lines 168-210, `generate_snippet()` method)

**Search Parameters:**
- `crates/server/src/tools/search.rs` (lines 33-40, `SearchConceptsParams`)

**Search Implementation:**
- `crates/server/src/search/tantivy_search.rs` (lines 103-164, `search()` method)
- `crates/server/src/search/simple_search.rs` (lines 37-87, `search()` method)

**Schema:**
- `crates/server/src/search/schema.rs` (category field definition)

---

## Notes

- This is all pre-tag polish for v0.2.0
- QA validation confirmed Phase 1-4 success
- Focus on highest-value improvements (snippets, category filtering)
- Use ecosystem crates for better maintainability
- Keep changes focused and testable
- All work should complete in 6-8 hours
