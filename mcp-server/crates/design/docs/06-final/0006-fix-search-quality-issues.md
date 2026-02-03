---
number: 6
title: "Fix Search Quality Issues"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-01-27
updated: 2026-02-03
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Fix Search Quality Issues

*Multi-word Queries, Stemming, Stopwords*

**Version:** 1.0
**Created:** 2026-01-27
**Task:** Fix critical search quality issues identified in QA testing
**Estimated Time:** 5-7 days over 4 phases

---

## Executive Summary

**Problem:** QA testing revealed that multi-word queries like `"fugue subject answer"` return zero results, and stemming/stopword handling is missing.

**Root Cause:**

- SimpleSearch (current default) uses substring matching instead of word tokenization
- TantivySearch exists and has proper infrastructure but uses OR logic for all terms
- Neither handles stopwords properly

**Solution:** Enhance TantivySearch with:

1. Configurable AND/OR query logic (tiered: 2 words = AND, 3+ = OR with minimum match)
2. Stopword filtering with music theory domain allowlist
3. Phrase search support (quoted strings)
4. Make TantivySearch the default backend

**Impact:** Zero breaking changes to API, only config default change and index rebuild needed.

---

## QA Issues Summary

| Issue | Priority | Current State | Fix Required |
|-------|----------|---------------|--------------|
| Multi-word queries fail | P1 Critical | `"fugue subject answer"` → 0 results | Configurable AND/OR logic |
| No stemming in SimpleSearch | P2 High | `write` ≠ `writing` ≠ `written` | Use TantivySearch (has stemming) |
| No stopword handling | P3 Medium | `"what is a cadence"` → 0 results | Add stopword filter |
| No phrase search | P4 Medium | `"imperfect consonance"` → 0 results | Detect quotes, use PhraseQuery |
| Short terms filtered | P5 Low | Roman numerals may fail | Domain allowlist |

**QA Report Location:** `mcp-server/crates/design/dev/server/0007-music-theory-mcp-server-full-text-search-qa-report.md`

---

## Current State Analysis

### SimpleSearch (`search/document.rs` lines 86-104)

```rust
pub fn matches_query(&self, query: &str) -> bool {
    let query_lower = query.to_lowercase();
    // ...
    title_lower.contains(&query_lower)  // Substring matching!
        || desc_lower.contains(&query_lower)
        || content_lower.contains(&query_lower)
}
```

**Problem:** Searches for entire query as substring, not tokenized words.

### TantivySearch (`search/query.rs` lines 72-189)

```rust
// Line 74: Splits query into terms
let terms: Vec<&str> = query_str.split_whitespace().collect();

// Line 181: Always uses OR logic
query.add_clause(Occur::Should, Box::new(term_query));  // Should = OR
```

**Status:** Has stemming (`en_stem` tokenizer in schema.rs line 76), position indexing, proper tokenization.
**Problem:** Hardcoded OR logic for all multi-word queries.

---

## Implementation Plan

### Phase 1: Fix Multi-word Query Logic (P1 Critical) - 2-3 days

#### Goals

- Implement configurable AND/OR query logic
- Add "smart" tiered defaults: 2 words = AND, 3+ = OR with minimum match
- Allow configuration override per search

#### Critical Files

**1. Add QueryMode enum** (`crates/server/src/config.rs`)

Location: Add to `SearchConfig` struct (lines 106-159)

```rust
/// Query matching mode for multi-word queries
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMode {
    /// Match ANY term (OR logic) - current behavior
    Or,
    /// Match ALL terms (AND logic)
    And,
    /// Match at least N% of terms
    MinimumMatch(f32),
    /// Smart tiered: 2 words = AND, 3+ = OR with 60% minimum
    Smart,
}

pub struct SearchConfig {
    // ... existing fields ...

    #[serde(default = "default_query_mode")]
    pub query_mode: QueryMode,

    #[serde(default = "default_minimum_match")]
    pub minimum_match_percent: f32,
}

fn default_query_mode() -> QueryMode {
    QueryMode::Smart
}

fn default_minimum_match() -> f32 {
    0.6  // 60% of terms
}
```

**2. Update QueryBuilder** (`crates/server/src/search/query.rs`)

Location: Modify `create_field_query()` method (lines 127-189)

Current code uses:

```rust
query.add_clause(Occur::Should, Box::new(term_query));  // Line 181
```

Change to:

```rust
// Determine occur mode based on config and term count
let occur_mode = self.determine_occur_mode(term_count);

for term in terms {
    let term_query = Term::from_field_text(field, &term);
    query.add_clause(occur_mode, Box::new(TermQuery::new(
        term_query,
        IndexRecordOption::WithFreqsAndPositions,
    )));
}

// For OR mode with 3+ terms, wrap in MinimumShouldMatchQuery
if matches!(occur_mode, Occur::Should) && term_count >= 3 {
    let min_match = (term_count as f32 * self.config.minimum_match_percent).ceil() as usize;
    query.set_min_should_match(min_match);
}
```

Add helper method:

```rust
fn determine_occur_mode(&self, term_count: usize) -> Occur {
    match &self.config.query_mode {
        QueryMode::Or => Occur::Should,
        QueryMode::And => Occur::Must,
        QueryMode::MinimumMatch(_) => Occur::Should,  // Use with min_should_match
        QueryMode::Smart => {
            if term_count <= 2 {
                Occur::Must  // AND for 2 words
            } else {
                Occur::Should  // OR for 3+ with minimum match
            }
        }
    }
}
```

**3. Update search tool parameters** (`crates/server/src/tools/search.rs`)

Location: Add optional parameter to `SearchConceptsParams` (line 27)

```rust
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchConceptsParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,

    // NEW: Allow per-query mode override
    #[serde(default)]
    pub query_mode: Option<QueryMode>,
}
```

#### Testing

**Unit Tests** (`crates/server/src/search/query.rs` test module):

```rust
#[tokio::test]
async fn test_query_mode_smart_two_words() {
    // "authentic cadence" → AND logic
}

#[tokio::test]
async fn test_query_mode_smart_three_words() {
    // "fugue subject answer" → OR with min 60% = 2/3 terms
}

#[tokio::test]
async fn test_query_mode_explicit_and() {
    // Override mode to AND
}
```

**Integration Tests** (`crates/server/tests/search_qa_integration.rs` - NEW FILE):

```rust
#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_multiword_fugue_subject_answer() {
    // Test QA failure: "fugue subject answer" should return results
    let results = search("fugue subject answer", None).await.unwrap();
    assert!(results.len() > 0, "Should find fugue-related cards");
    assert!(results.iter().any(|r| r.title.to_lowercase().contains("fugue")));
}

#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_multiword_suspension_dissonance() {
    let results = search("suspension dissonance resolution", None).await.unwrap();
    assert!(results.len() > 0, "Should find suspension cards");
}

// Add tests for all 17 failing queries from QA report
```

#### Verification

- [ ] 2-word queries use AND logic (both terms required)
- [ ] 3+ word queries use OR with minimum match 60%
- [ ] Query mode can be overridden per search
- [ ] All QA multi-word test cases pass
- [ ] Relevance ranking still works (BM25 scores preserved)

---

### Phase 2: Add Stopword Filtering (P3 Medium) - 1 day

#### Goals

- Filter common English stopwords before search
- Preserve music theory domain terms (Roman numerals)
- Make stopword list configurable

#### Critical Files

**1. Add stopword configuration** (`crates/server/src/config.rs`)

Location: Add to `SearchConfig` struct

```rust
pub struct SearchConfig {
    // ... existing fields ...

    #[serde(default = "default_enable_stopwords")]
    pub enable_stopwords: bool,

    #[serde(default)]
    pub custom_stopwords: Vec<String>,

    #[serde(default = "default_stopword_allowlist")]
    pub stopword_allowlist: Vec<String>,
}

fn default_enable_stopwords() -> bool {
    true
}

fn default_stopword_allowlist() -> Vec<String> {
    vec![
        // Music theory Roman numerals and solfège
        "I", "V", "ii", "IV", "vi", "vii", "i", "v", "iv",
        "do", "re", "mi", "fa", "sol", "la", "ti",
    ].into_iter().map(String::from).collect()
}
```

**2. Create stopword filter** (`crates/server/src/search/stopwords.rs` - NEW FILE)

```rust
/// English stopwords for search query preprocessing
pub const ENGLISH_STOPWORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "but", "by",
    "for", "if", "in", "into", "is", "it", "no", "not", "of",
    "on", "or", "such", "that", "the", "their", "then", "there",
    "these", "they", "this", "to", "was", "will", "with",
    "what", "how", "why", "when", "where", "which", "who",
];

pub struct StopwordFilter {
    stopwords: HashSet<String>,
    allowlist: HashSet<String>,
}

impl StopwordFilter {
    pub fn new(config: &SearchConfig) -> Self {
        let mut stopwords: HashSet<String> = ENGLISH_STOPWORDS
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        // Add custom stopwords
        for word in &config.custom_stopwords {
            stopwords.insert(word.to_lowercase());
        }

        let allowlist: HashSet<String> = config.stopword_allowlist
            .iter()
            .map(|s| s.to_string())
            .collect();

        Self { stopwords, allowlist }
    }

    /// Filter stopwords from query, preserving allowlisted terms
    pub fn filter(&self, query: &str) -> String {
        query
            .split_whitespace()
            .filter(|word| {
                let word_lower = word.to_lowercase();
                // Keep if: not a stopword OR is allowlisted
                !self.stopwords.contains(&word_lower)
                    || self.allowlist.contains(word)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
```

**3. Update QueryBuilder** (`crates/server/src/search/query.rs`)

Location: Add preprocessing in `build_query()` method (line 64)

```rust
pub fn build_query(&self, query_str: &str) -> Result<Box<dyn Query>> {
    if query_str.is_empty() {
        return Err(Error::invalid_query("Query cannot be empty"));
    }

    // NEW: Filter stopwords before processing
    let filtered_query = if self.config.enable_stopwords {
        let filter = StopwordFilter::new(&self.config);
        filter.filter(query_str)
    } else {
        query_str.to_string()
    };

    if filtered_query.is_empty() {
        // All words were stopwords, use original query
        filtered_query = query_str.to_string();
    }

    // Continue with existing tokenization...
    let terms: Vec<&str> = filtered_query.split_whitespace().collect();
    // ...
}
```

**4. Update module exports** (`crates/server/src/search/mod.rs`)

```rust
#[cfg(feature = "fts")]
mod stopwords;

#[cfg(feature = "fts")]
pub use stopwords::{StopwordFilter, ENGLISH_STOPWORDS};
```

#### Testing

**Unit Tests** (`crates/server/src/search/stopwords.rs`):

```rust
#[test]
fn test_stopword_filter_removes_common_words() {
    let filter = StopwordFilter::new(&default_config());
    assert_eq!(filter.filter("what is a cadence"), "cadence");
    assert_eq!(filter.filter("how to write fugue"), "write fugue");
}

#[test]
fn test_stopword_filter_preserves_allowlist() {
    let filter = StopwordFilter::new(&default_config());
    assert_eq!(filter.filter("V I resolution"), "V I resolution");
    assert_eq!(filter.filter("do re mi"), "do re mi");
}

#[test]
fn test_stopword_filter_all_stopwords() {
    let filter = StopwordFilter::new(&default_config());
    // Should preserve original query if all words are stopwords
    assert_eq!(filter.filter("what is this"), "what is this");
}
```

**Integration Tests** (`crates/server/tests/search_qa_integration.rs`):

```rust
#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_natural_language_what_is_cadence() {
    let results = search("what is a cadence", None).await.unwrap();
    assert!(results.len() > 0, "Should find cadence cards after stopword removal");
}

#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_roman_numerals_preserved() {
    let results = search("V I resolution", None).await.unwrap();
    assert!(results.len() > 0, "Should preserve Roman numerals");
}
```

#### Verification

- [ ] Common stopwords filtered: `"what is a cadence"` → `"cadence"`
- [ ] Music theory terms preserved: `"V I resolution"` keeps all terms
- [ ] Empty query fallback works
- [ ] Configurable via `enable_stopwords` flag
- [ ] Custom stopwords and allowlist work

---

### Phase 3: Add Phrase Search Support (P4 Medium) - 1-2 days

#### Goals

- Detect quoted phrases in queries
- Use Tantivy's `PhraseQuery` for exact matching
- Support mixing phrases with regular terms

#### Critical Files

**1. Add phrase detection** (`crates/server/src/search/query.rs`)

Location: Add helper methods before `build_query()`

```rust
/// Parse query into phrases and terms
/// Returns: (phrases, remaining_terms)
fn parse_phrases(query: &str) -> (Vec<String>, String) {
    let phrase_regex = regex::Regex::new(r#""([^"]+)""#).unwrap();
    let mut phrases = Vec::new();

    for capture in phrase_regex.captures_iter(query) {
        if let Some(phrase) = capture.get(1) {
            phrases.push(phrase.as_str().to_string());
        }
    }

    // Remove phrases from query to get remaining terms
    let remaining = phrase_regex.replace_all(query, "").to_string();

    (phrases, remaining)
}

/// Create a PhraseQuery for exact phrase matching
fn create_phrase_query(
    field: Field,
    phrase: &str,
    tokenizer: &TextAnalyzer,
) -> Result<Box<dyn Query>> {
    let mut token_stream = tokenizer.token_stream(phrase);
    let mut terms = Vec::new();

    while let Some(token) = token_stream.next() {
        let term = Term::from_field_text(field, &token.text);
        terms.push(term);
    }

    if terms.is_empty() {
        return Err(Error::invalid_query("Phrase contains no valid terms"));
    }

    Ok(Box::new(PhraseQuery::new(terms)))
}
```

**2. Update build_query()** (`crates/server/src/search/query.rs`)

Location: Modify `build_query()` method (line 64)

```rust
pub fn build_query(&self, query_str: &str) -> Result<Box<dyn Query>> {
    if query_str.is_empty() {
        return Err(Error::invalid_query("Query cannot be empty"));
    }

    // Parse phrases and terms
    let (phrases, remaining_query) = parse_phrases(query_str);

    // Filter stopwords from remaining terms
    let filtered_query = if self.config.enable_stopwords {
        let filter = StopwordFilter::new(&self.config);
        filter.filter(&remaining_query)
    } else {
        remaining_query
    };

    // Create main query
    let mut main_query = BooleanQuery::new(vec![]);

    // Add phrase queries
    if !phrases.is_empty() {
        for phrase in phrases {
            let phrase_clause = self.create_phrase_query_for_fields(&phrase)?;
            main_query.add_clause(Occur::Should, phrase_clause);  // OR across fields
        }
    }

    // Add term queries
    if !filtered_query.trim().is_empty() {
        let terms_clause = self.create_terms_query(&filtered_query)?;
        main_query.add_clause(Occur::Should, terms_clause);
    }

    if main_query.clauses().is_empty() {
        return Err(Error::invalid_query("Query contains no searchable terms"));
    }

    Ok(Box::new(main_query))
}

fn create_phrase_query_for_fields(&self, phrase: &str) -> Result<Box<dyn Query>> {
    let mut field_queries = BooleanQuery::new(vec![]);
    let tokenizer = self.get_tokenizer();

    // Search phrase in all fields with boosting
    let phrase_title = self.create_phrase_query(self.schema.title_field, phrase, &tokenizer)?;
    field_queries.add_clause(Occur::Should, Box::new(Boost::new(phrase_title, 3.0)));

    let phrase_desc = self.create_phrase_query(self.schema.description_field, phrase, &tokenizer)?;
    field_queries.add_clause(Occur::Should, Box::new(Boost::new(phrase_desc, 2.0)));

    let phrase_content = self.create_phrase_query(self.schema.content_field, phrase, &tokenizer)?;
    field_queries.add_clause(Occur::Should, phrase_content);

    Ok(Box::new(field_queries))
}
```

**3. Add regex dependency** (`crates/server/Cargo.toml`)

```toml
[dependencies]
regex = { workspace = true }
```

#### Testing

**Unit Tests** (`crates/server/src/search/query.rs`):

```rust
#[test]
fn test_parse_phrases_single() {
    let (phrases, remaining) = parse_phrases(r#""imperfect consonance""#);
    assert_eq!(phrases, vec!["imperfect consonance"]);
    assert_eq!(remaining.trim(), "");
}

#[test]
fn test_parse_phrases_mixed() {
    let (phrases, remaining) = parse_phrases(r#""perfect fifth" and octave"#);
    assert_eq!(phrases, vec!["perfect fifth"]);
    assert_eq!(remaining.trim(), "and octave");
}

#[tokio::test]
async fn test_create_phrase_query() {
    // Test phrase query creation with tokenizer
}
```

**Integration Tests** (`crates/server/tests/search_qa_integration.rs`):

```rust
#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_phrase_search_quoted() {
    let results = search(r#""imperfect consonance""#, None).await.unwrap();
    assert!(results.len() > 0, "Should find exact phrase");
    // Verify exact phrase appears in results
    assert!(results.iter().any(|r|
        r.content.to_lowercase().contains("imperfect consonance")
    ));
}

#[tokio::test]
#[serial]
#[cfg(feature = "fts")]
async fn test_qa_phrase_with_terms() {
    let results = search(r#""perfect cadence" dominant"#, None).await.unwrap();
    assert!(results.len() > 0, "Should find phrase + terms");
}
```

#### Verification

- [ ] Quoted phrases detected and parsed correctly
- [ ] PhraseQuery created with proper tokenization
- [ ] Phrase queries respect field boosting (title > description > content)
- [ ] Mixed phrase + term queries work
- [ ] Empty/invalid phrases handled gracefully

---

### Phase 4: Configuration & Migration (1 day)

#### Goals

- Update default configuration to use TantivySearch
- Document migration path
- Add deprecation warning for SimpleSearch

#### Critical Files

**1. Update default config** (`crates/server/config/default.toml`)

```toml
[search]
# Backend selection: "simple" or "tantivy"
# CHANGED: tantivy is now default for better search quality
# Note: "tantivy" requires building with --features fts
backend = "tantivy"

# Tantivy index storage directory
index_path = ".tantivy-index"

# Rebuild index on startup
# Set to true for initial setup, then false
rebuild_on_startup = false

# Snippet size for search results
snippet_size = 200

# Fuzzy search (typo tolerance)
fuzzy_search = false
fuzzy_distance = 2

# NEW: Query mode (smart, and, or, minimum_match)
query_mode = "smart"

# Minimum match percentage for OR queries with 3+ terms
minimum_match_percent = 0.6

# Enable stopword filtering
enable_stopwords = true

# Custom stopwords (in addition to English defaults)
custom_stopwords = []

# Domain-specific terms to preserve (not filtered as stopwords)
stopword_allowlist = ["I", "V", "ii", "IV", "vi", "vii", "do", "re", "mi", "fa", "sol", "la", "ti"]
```

**2. Add migration documentation** (`crates/server/MIGRATION.md` - NEW FILE or section in README)

```markdown
# Search Backend Migration Guide

## Version 0.2.0: TantivySearch Becomes Default

### What Changed

- Default search backend changed from `simple` to `tantivy`
- Added multi-word query support with smart AND/OR logic
- Added stopword filtering for natural language queries
- Added phrase search support with quoted strings

### Migration Steps

#### 1. Build Index (First Time)

```bash
# Build with FTS feature enabled
cargo build --release --features fts

# Build the initial index
./target/release/music-theory-mcp index
```

#### 2. Update Configuration (Optional)

If you have a custom config file, you can:

- Keep `backend = "simple"` to use old behavior (not recommended)
- Set `backend = "tantivy"` to use new backend (recommended)
- Customize query mode: `query_mode = "and"` for strict matching

#### 3. Test Search Quality

```bash
# Test multi-word queries
curl -X POST http://localhost:3000/search \
  -d '{"query": "fugue subject answer", "limit": 5}'

# Should now return results!
```

### Rollback Plan

If you encounter issues, you can temporarily revert:

```toml
[search]
backend = "simple"  # Use old substring matching
```

### Breaking Changes

**None** - The API remains unchanged. Only the config default changed.

```

**3. Add deprecation warning** (`crates/server/src/search/simple_search.rs`)

Location: Add at top of `SimpleSearch::new()` method

```rust
pub fn new(config: Config) -> Self {
    // Deprecation warning
    log::warn!(
        "SimpleSearch backend is deprecated and will be removed in version 0.3.0. \
        Please migrate to 'backend = \"tantivy\"' for better search quality. \
        See MIGRATION.md for instructions."
    );

    Self { config }
}
```

**4. Update README** (`crates/server/README.md`)

Add section about search configuration:

```markdown
## Search Configuration

The server supports two search backends:

### TantivySearch (Recommended, Default)

Full-text search with:
- ✅ Multi-word query support (smart AND/OR logic)
- ✅ Stemming (write/writing/written → same results)
- ✅ Stopword filtering (natural language queries work)
- ✅ Phrase search ("exact phrases")
- ✅ Relevance ranking (BM25 algorithm)

**Requirements:**
- Build with `--features fts`
- Run index build: `music-theory-mcp index`

### SimpleSearch (Deprecated)

Basic substring matching:
- ❌ Limited multi-word support
- ❌ No stemming
- ❌ No stopword filtering

**When to use:** Development only, or <100 documents.

### Query Modes

Configure in `config/default.toml`:

```toml
query_mode = "smart"  # 2 words = AND, 3+ = OR with 60% match
# query_mode = "and"    # All terms required (strict)
# query_mode = "or"     # Any term matches
```

### Query Syntax

```
cadence                    → Single term
authentic cadence          → Smart mode: both terms required (AND)
fugue subject answer       → Smart mode: 2 of 3 terms required (OR + min match)
"perfect authentic"        → Exact phrase
"leading tone" resolution  → Mix phrase + terms
```

```

#### Verification
- [ ] Default config uses `backend = "tantivy"`
- [ ] Deprecation warning logged when SimpleSearch used
- [ ] Migration documentation complete
- [ ] README updated with search configuration
- [ ] Index build command documented

---

## Testing Strategy

### Unit Test Coverage Target: 95%+

Following `CLAUDE-CODE-COVERAGE.md`:

**Per file:**
- `query.rs`: Test all new functions (phrase parsing, stopword filtering, query mode logic)
- `stopwords.rs`: Test filter with various inputs, edge cases
- `config.rs`: Test QueryMode serialization/deserialization

**Naming convention:** `test_<fn>_<scenario>_<expectation>`

### Integration Test Matrix

**Create:** `crates/server/tests/search_qa_integration.rs`

Test all 17 failing queries from QA report (lines 26-47):

```rust
// Multi-word failures (P1)
test_qa_multiword_suspension_dissonance_resolution()
test_qa_multiword_dominant_seventh_resolution()
test_qa_multiword_fugue_subject_answer()
test_qa_multiword_parallel_fifths_forbidden()
test_qa_multiword_raised_sixth_minor()
test_qa_multiword_sonata_form_exposition()
test_qa_multiword_thirds_sixths()
test_qa_multiword_leading_tone_tonic()
test_qa_multiword_prinner_romanesca()
test_qa_multiword_schema_opening()
test_qa_multiword_common_chord_pivot()
test_qa_multiword_species_first_second()

// Natural language (P3 stopwords)
test_qa_natural_language_what_is_cadence()
test_qa_natural_language_how_to_write_counterpoint()

// Phrase search (P4)
test_qa_phrase_search_imperfect_consonance()

// Verb forms (P2 stemming) - already works with Tantivy!
test_qa_stemming_writing_melodies()

// Short terms (P5)
test_qa_short_terms_v_i_resolution()
```

### Manual Validation

After each phase:

1. **Build and run server:**

   ```bash
   cargo build --features fts
   cargo run --features fts -- serve
   ```

2. **Test via MCP client:**
   - Call `search_concepts` tool with QA test queries
   - Verify results are returned (not zero)
   - Check relevance scores are reasonable

3. **Smoke test:**

   ```bash
   cargo run --features fts -- index  # Build index
   cargo run --features fts -- status # Verify index
   ```

---

## Backward Compatibility

### Breaking Changes: NONE

**API unchanged:**

- `SearchConceptsParams` interface same
- `SearchConceptsResponse` interface same
- Optional `query_mode` parameter is backward compatible

**Configuration change:**

- Default `backend` changes from `"simple"` to `"tantivy"`
- Users can explicitly set `backend = "simple"` to preserve old behavior
- All existing config files continue to work

### Migration Requirements

**For existing deployments:**

1. Build with `--features fts` (or keep `backend = "simple"`)
2. Run index build: `music-theory-mcp index`
3. Restart server

**Rollback plan:**

- Set `backend = "simple"` in config
- No data loss (concept cards unchanged)

---

## Critical Files Summary

### Files to Modify

| File | Lines | Changes | Phase |
|------|-------|---------|-------|
| `crates/server/src/config.rs` | 106-159 | Add QueryMode, stopword config | 1, 2 |
| `crates/server/src/search/query.rs` | 64-189 | Multi-word logic, phrase detection | 1, 3 |
| `crates/server/src/tools/search.rs` | 27 | Add query_mode param | 1 |
| `crates/server/config/default.toml` | 102-115 | Update defaults | 4 |
| `crates/server/src/search/simple_search.rs` | 36 | Add deprecation warning | 4 |
| `crates/server/README.md` | - | Document search config | 4 |

### New Files to Create

| File | Size | Purpose | Phase |
|------|------|---------|-------|
| `crates/server/src/search/stopwords.rs` | ~150 lines | Stopword filtering | 2 |
| `crates/server/tests/search_qa_integration.rs` | ~500 lines | QA validation tests | 1-3 |
| `crates/server/MIGRATION.md` | ~100 lines | Migration guide | 4 |

---

## Success Criteria

### Functional Requirements

- [ ] Multi-word queries return results (not zero)
- [ ] 2-word queries use AND logic by default
- [ ] 3+ word queries use OR with 60% minimum match
- [ ] Stopwords filtered from queries
- [ ] Music theory terms preserved in allowlist
- [ ] Phrase search works with quoted strings
- [ ] All 17 QA failure cases pass
- [ ] Query mode configurable per search and in config

### Quality Requirements

- [ ] Test coverage ≥95% for new code
- [ ] All existing tests still pass
- [ ] No breaking changes to API
- [ ] Deprecation warnings logged appropriately
- [ ] Documentation complete and accurate

### Performance Requirements

- [ ] Query latency <100ms for typical searches
- [ ] Index size <50MB for ~200 concept cards
- [ ] Memory usage <200MB during search operations
- [ ] No degradation in relevance quality

---

## Risk Mitigation

### Identified Risks

**1. Tantivy Index Required**

- **Risk:** Users must rebuild index, adds deployment step
- **Mitigation:**
  - Provide clear migration docs
  - Add index build to setup instructions
  - Keep SimpleSearch as fallback option

**2. Query Mode Confusion**

- **Risk:** Users may not understand AND vs OR behavior
- **Mitigation:**
  - Smart defaults handle most cases
  - Document query syntax with examples
  - Add `query_mode` to response for transparency

**3. Stopword Over-filtering**

- **Risk:** Domain-specific terms accidentally filtered
- **Mitigation:**
  - Comprehensive music theory allowlist
  - Configurable stopword list
  - Fallback to original query if all terms filtered

**4. Phrase Query Performance**

- **Risk:** Complex phrase queries may be slow
- **Mitigation:**
  - Tantivy has optimized phrase query implementation
  - Monitor query latency in tests
  - Add timeout if needed

---

## Verification Steps (End-to-End)

### After Phase 1 (Multi-word Logic)

```bash
# Build and index
cargo build --features fts
cargo run --features fts -- index

# Test queries
cargo run --features fts -- serve &
curl -X POST localhost:3000/search -d '{"query": "fugue subject answer"}'
# Should return results!

curl -X POST localhost:3000/search -d '{"query": "authentic cadence"}'
# Should return fewer, more precise results (AND logic)
```

### After Phase 2 (Stopwords)

```bash
curl -X POST localhost:3000/search -d '{"query": "what is a cadence"}'
# Should return cadence results (stopwords filtered)

curl -X POST localhost:3000/search -d '{"query": "V I resolution"}'
# Should preserve Roman numerals
```

### After Phase 3 (Phrases)

```bash
curl -X POST localhost:3000/search -d '{"query": "\"imperfect consonance\""}'
# Should find exact phrase

curl -X POST localhost:3000/search -d '{"query": "\"leading tone\" resolution"}'
# Should find phrase + additional term
```

### Run Full Test Suite

```bash
cargo test --features fts
cargo test --features fts search_qa_integration
make coverage  # Should be ≥95%
```

---

## Implementation Notes

1. **Follow CLAUDE.md guidelines** throughout
2. **Use CLAUDE-CODE-COVERAGE.md** for test development
3. **Test continuously** - run tests after each function added
4. **Commit incrementally** - one commit per logical unit of work
5. **Document as you go** - update doc comments for modified functions

**Key Insight:** Tantivy already has 90% of what we need (stemming, tokenization, position indexing). We just need to:

- Fix the hardcoded `Occur::Should` to be configurable
- Add stopword preprocessing
- Add phrase query parsing

This is a 5-7 day fix, not a rewrite!
