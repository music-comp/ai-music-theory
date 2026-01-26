---
number: 4
title: "Tantivy Full-Text Search for the MCP Server"
author: "Field,       // STORED (metadata)"
component: All
tags: [change-me]
created: 2026-01-26
updated: 2026-01-26
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Tantivy Full-Text Search for the MCP Server

**Version:** 1.0
**Created:** 2026-01-26
**Task:** Add Tantivy search engine with backward compatibility
**Estimated Time:** 15-22 hours over multiple sessions

---

## Executive Summary

**Problem:** Simple linear search (O(n)) becoming bottleneck with ~200 concept cards and rapid growth ("not even done with first book")
**Impact:** Search latency approaching 100ms; no typo tolerance, boolean queries, or phrase search
**Solution:** Integrate Tantivy full-text search engine with config-driven backend selection
**Benefit:** 10-100x faster search, typo tolerance, advanced queries, faceted filtering, scales to 10,000+ cards

---

## Why Tantivy Now?

### Growth Trajectory

- **Current:** ~200 concept cards (approaching performance threshold)
- **Trend:** Accelerating ("not even done with first book")
- **Threshold:** Simple search degrades at 300-500 cards
- **Proactive:** Better to implement before users experience slowdown

### Feature Requirements

- ✅ **Typo tolerance:** "paralel fifths" → "parallel fifths"
- ✅ **Boolean queries:** `category:harmony AND (triad OR seventh)`
- ✅ **Phrase search:** `"voice leading"` (exact match)
- ✅ **Faceted filtering:** Category + source + tags simultaneously
- ✅ **Scalability:** Ready for 1000+ cards without code changes

### Architecture Readiness

Our recent metadata refactoring makes Tantivy integration straightforward:

- ✅ `SearchDocument` already has perfect schema structure
- ✅ `ConceptMetadata` provides clean extraction pipeline
- ✅ Weighted relevance hints (title 3x, description 2x, content 1x)
- ✅ Facet fields ready (category, source, tags)
- ✅ No refactoring needed

---

## Architecture Overview

### Current State

```
search_concepts() → Linear scan all files → Filter/Score/Sort → Return top N
                    O(n) performance, ~20-50ms for 200 cards
```

### Target State

```
Indexing (background):
  File changes → ConceptMetadata → SearchDocument → Tantivy Index

Querying (realtime):
  Query → Tantivy Index → Retrieve IDs → Return results
  O(log n) performance, sub-millisecond for 1000+ cards
```

### Backend Abstraction

```
search_concepts() → Backend factory (config-driven)
                    ├─ SimpleSearch (fallback, existing code)
                    └─ TantivySearch (new, O(log n))
```

---

## Implementation Phases

### Phase 1: Foundation (2-3 hours)

#### 1.1 Schema Design

**New file:** `crates/server/src/search/schema.rs`

Define Tantivy schema mapping from SearchDocument:

```rust
pub struct SearchSchema {
    pub schema: Schema,
    // Field handles
    pub id: Field,           // STORED (identity)
    pub path: Field,         // STORED (identity)
    pub title: Field,        // TEXT | STORED (3x boost)
    pub description: Field,  // TEXT | STORED (2x boost)
    pub content: Field,      // TEXT | STORED (1x boost)
    pub category: Field,     // FACET | STORED
    pub source: Field,       // FACET | STORED
    pub tags: Field,         // FACET | STORED
    pub chapter: Field,      // STORED (metadata)
    pub part: Field,         // STORED (metadata)
    pub author: Field,       // STORED (metadata)
    pub date: Field,         // STORED (metadata)
}

impl SearchSchema {
    pub fn build() -> Self {
        // Create schema builder
        // Add fields with appropriate options
        // Full-text: en_stem tokenizer, WithFreqsAndPositions
        // Facets: hierarchical facets (e.g., "/harmony")
        // Return SearchSchema with field handles
    }
}
```

**Design Rationale:**

- **TEXT fields:** Enable full-text search with stemming ("harmonic" matches "harmony")
- **FACET fields:** Enable efficient filtering by category/source/tags
- **STORED fields:** Enable retrieval without re-reading markdown files
- **Position indexing:** Enables phrase queries like `"parallel fifths"`

#### 1.2 Configuration

**Update:** `crates/server/src/config.rs`

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub sources: SourcesConfig,
    pub logging: LoggingConfig,
    pub search: SearchConfig,  // NEW
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchConfig {
    /// Backend: "simple" or "tantivy"
    #[serde(default = "default_backend")]
    pub backend: String,

    /// Tantivy index directory
    #[serde(default = "default_index_path")]
    pub index_path: String,

    /// Rebuild on startup (dev mode)
    #[serde(default)]
    pub rebuild_on_startup: bool,

    /// Snippet size (characters)
    #[serde(default = "default_snippet_size")]
    pub snippet_size: usize,

    /// Enable fuzzy search (typo tolerance)
    #[serde(default)]
    pub fuzzy_search: bool,

    /// Fuzzy edit distance (1-2)
    #[serde(default = "default_fuzzy_distance")]
    pub fuzzy_distance: u8,
}

fn default_backend() -> String { "simple".to_string() }
fn default_index_path() -> String { ".tantivy-index".to_string() }
fn default_snippet_size() -> usize { 200 }
fn default_fuzzy_distance() -> u8 { 2 }
```

**Update:** `config/default.toml`

```toml
[search]
# Backend selection: "simple" or "tantivy"
backend = "simple"  # Start with simple for backward compatibility

# Tantivy index storage (relative to skill root)
index_path = ".tantivy-index"

# Rebuild index on startup (useful for development)
rebuild_on_startup = false

# Snippet context size in characters
snippet_size = 200

# Enable fuzzy search (typo tolerance)
fuzzy_search = false

# Maximum edit distance for fuzzy matching (1-2)
fuzzy_distance = 2
```

#### 1.3 Dependencies

**Update:** `Cargo.toml`

```toml
[dependencies]
tantivy = "0.22"  # Full-text search engine
```

---

### Phase 2: Indexing Pipeline (3-4 hours)

#### 2.1 Indexer Core

**New file:** `crates/server/src/search/indexer.rs`

```rust
pub struct Indexer {
    schema: SearchSchema,
    index: Index,
    writer: IndexWriter,
}

impl Indexer {
    /// Create or open index at path
    pub fn new(index_path: &Path) -> Result<Self> {
        // Create directory if needed
        // Open existing or create new index
        // Create writer with 50MB buffer
    }

    /// Add document to index
    pub fn add_document(&mut self, doc: &SearchDocument) -> Result<()> {
        // Convert SearchDocument → Tantivy Document
        // Add to writer
    }

    /// Commit pending changes
    pub fn commit(&mut self) -> Result<()> {
        // Flush writer to disk
    }

    /// Clear index for rebuild
    pub fn clear(&mut self) -> Result<()> {
        // Delete all documents
        // Commit
    }

    /// Get Index for creating searchers
    pub fn index(&self) -> &Index

    /// Convert SearchDocument → Tantivy Document
    fn convert_to_tantivy_doc(&self, doc: &SearchDocument) -> Result<Document> {
        // Map all SearchDocument fields to Tantivy fields
        // Handle Option types gracefully
        // Convert facets to hierarchical format ("/category")
    }
}
```

**Key Conversions:**

- `doc.id` → `schema.id` (text field)
- `doc.title` → `schema.title` (text, indexed + stored)
- `doc.category` → `schema.category` (facet, format: `/harmony`)
- `doc.source` → `schema.source` (facet, optional)
- `doc.tags` → `schema.tags` (facet, multiple)

#### 2.2 Index Builder

**New file:** `crates/server/src/search/builder.rs`

```rust
/// Build complete Tantivy index from all concept cards
pub async fn build_index(config: &Config) -> Result<()> {
    let index_path = config.search.index_path()?;
    let concept_cards_path = config.paths.concept_cards_path()?;

    info!("Building Tantivy index at: {}", index_path.display());

    // Create indexer
    let mut indexer = Indexer::new(&index_path)?;

    // Clear existing
    indexer.clear()?;

    // Find all markdown files
    let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;
    info!("Found {} concept card files", files.len());

    let mut indexed_count = 0;
    let mut error_count = 0;

    // Index each file
    for file_info in files {
        match extract_concept_metadata(&concept_cards_path, &file_info.path).await {
            Ok(meta) => {
                match SearchDocument::from_metadata(meta, &file_info.path).await {
                    Ok(doc) => {
                        if let Err(e) = indexer.add_document(&doc) {
                            warn!("Failed to index {}: {}", file_info.path.display(), e);
                            error_count += 1;
                        } else {
                            indexed_count += 1;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to create SearchDocument: {}", e);
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                warn!("Failed to extract metadata: {}", e);
                error_count += 1;
            }
        }
    }

    // Commit
    info!("Committing index...");
    indexer.commit()?;

    info!("Index complete: {} docs, {} errors", indexed_count, error_count);
    Ok(())
}
```

**Error Handling:**

- Graceful: Log warnings for individual file failures
- Continue indexing remaining files
- Return overall success with stats

---

### Phase 3: Query Translation (3-4 hours)

#### 3.1 Query Builder

**New file:** `crates/server/src/search/query.rs`

```rust
pub struct QueryBuilder<'a> {
    schema: &'a SearchSchema,
    fuzzy_enabled: bool,
    fuzzy_distance: u8,
}

impl<'a> QueryBuilder<'a> {
    /// Build weighted multi-field query
    /// Implements same relevance as simple search:
    /// - Title: 3.0x boost
    /// - Description: 2.0x boost
    /// - Content: 1.0x boost
    pub fn build_query(&self, query_str: &str) -> Result<Box<dyn Query>> {
        // Create BooleanQuery with Should clauses
        // One clause per field with appropriate boost
        // Handle fuzzy matching if enabled
    }

    /// Create field-specific query with boosting
    fn create_field_query(&self, field: Field, query_str: &str, boost: f32)
        -> Result<Box<dyn Query>> {
        // Parse terms from query string
        // For single term: TermQuery or FuzzyTermQuery
        // For multiple terms: BooleanQuery with Should
        // Wrap in Boost query with specified weight
    }
}
```

**Query Types:**

- **Simple term:** `"harmony"` → TermQuery on all fields with weights
- **Multi-word:** `"voice leading"` → BooleanQuery (Should) on all fields
- **Fuzzy:** `"haromny"` → FuzzyTermQuery (edit distance 2) finds "harmony"

#### 3.2 Tantivy Search Backend

**New file:** `crates/server/src/search/tantivy_search.rs`

```rust
pub struct TantivySearch {
    index: Index,
    schema: SearchSchema,
    searcher: Searcher,
    config: SearchConfig,
}

impl TantivySearch {
    pub fn new(index_path: &Path, config: SearchConfig) -> Result<Self> {
        // Open index
        // Create reader
        // Get searcher
    }

    pub fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>> {
        // Build query with QueryBuilder
        // Execute search with TopDocs collector
        // For each result:
        //   - Retrieve document
        //   - Extract fields
        //   - Generate snippet with Tantivy's SnippetGenerator
        //   - Convert to SearchResult
        // Return sorted by relevance
    }

    fn generate_snippet(&self, doc: &Document, query: &str) -> Result<String> {
        // Use Tantivy's SnippetGenerator
        // Highlight matching terms
        // Respect config.snippet_size
    }

    fn get_text_field(&self, doc: &Document, field: Field) -> Result<String>
    fn get_facet_field(&self, doc: &Document, field: Field) -> Result<String>
    fn get_optional_facet_field(&self, doc: &Document, field: Field) -> Option<String>
}
```

**Snippet Generation:**

- Use Tantivy's built-in SnippetGenerator
- Highlights matching terms in context
- Fallback to description if no content match

---

### Phase 4: Backend Abstraction (2-3 hours)

#### 4.1 Search Backend Trait

**Update:** `crates/server/src/search/mod.rs`

```rust
mod document;      // Existing
mod schema;        // New
mod indexer;       // New
mod builder;       // New
mod query;         // New
mod tantivy_search; // New
mod simple_search;  // New (adapter)

pub use document::SearchDocument;
pub use schema::SearchSchema;
pub use indexer::Indexer;
pub use builder::build_index;

/// Abstract search backend
pub trait SearchBackend: Send + Sync {
    fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>>;
}

/// Factory: create backend based on config
pub fn create_search_backend(config: &Config) -> Result<Box<dyn SearchBackend>> {
    match config.search.backend.as_str() {
        "tantivy" => {
            let index_path = config.search.index_path()?;
            let backend = tantivy_search::TantivySearch::new(&index_path, config.search.clone())?;
            Ok(Box::new(backend))
        }
        "simple" | _ => {
            let backend = simple_search::SimpleSearch::new(config.clone());
            Ok(Box::new(backend))
        }
    }
}
```

#### 4.2 Simple Search Adapter

**New file:** `crates/server/src/search/simple_search.rs`

```rust
/// Adapter for existing simple search (backward compatibility)
pub struct SimpleSearch {
    config: Config,
}

impl SimpleSearch {
    pub fn new(config: Config) -> Self {
        SimpleSearch { config }
    }
}

impl SearchBackend for SimpleSearch {
    fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>> {
        // COPY existing search_concepts implementation here
        // This becomes the fallback backend
    }
}
```

**Rationale:**

- Preserve existing simple search logic
- No breaking changes
- Instant rollback if Tantivy has issues

#### 4.3 Updated search_concepts Tool

**Update:** `crates/server/src/tools/search.rs`

```rust
use crate::search::create_search_backend;

pub async fn search_concepts(
    config: &Config,
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    // Create backend based on config
    let backend = create_search_backend(config)?;

    // Execute search (polymorphic dispatch)
    let results = backend.search(&params)?;

    let total = results.len();
    let limited = results.into_iter().take(params.limit).collect();

    Ok(SearchConceptsResponse {
        results: limited,
        total,
        query: params.query,
    })
}
```

**Changes:**

- Replace direct search logic with backend factory
- No changes to SearchResult, SearchConceptsResponse, or SearchConceptsParams
- **Fully backward compatible**

---

### Phase 5: Index Lifecycle (1-2 hours)

#### 5.1 Server Initialization

**Update:** `crates/server/src/main.rs` or `server.rs`

```rust
use crate::search::build_index;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    // Initialize logging
    twyg::setup(config.logging.to_twyg()?)?;

    // Build Tantivy index if requested
    if config.search.backend == "tantivy" && config.search.rebuild_on_startup {
        log::info!("Rebuilding Tantivy index on startup");
        build_index(&config).await?;
    }

    // Create and run server
    let service = MusicTheoryServer::new(config).serve(stdio()).await?;
    // ... rest of main
}
```

**Startup Behavior:**

- `backend = "simple"`: No index building, instant startup
- `backend = "tantivy"`, `rebuild_on_startup = false`: Use existing index (fast)
- `backend = "tantivy"`, `rebuild_on_startup = true`: Rebuild index (1-2 seconds for 200 docs)

#### 5.2 Index Persistence

**Index Location:**

- Default: `.tantivy-index` in skill root
- Configurable via `search.index_path`
- Gitignored (add to `.gitignore`)

**Index Lifecycle:**

1. **First run:** No index exists, created on first rebuild
2. **Subsequent runs:** Existing index reused (fast startup)
3. **Manual rebuild:** Set `rebuild_on_startup = true`, restart server
4. **Future:** File watcher for incremental updates (Phase 8)

---

### Phase 6: Testing Strategy (3-4 hours)

#### 6.1 Unit Tests

**Schema Tests** (`search/schema.rs`):

```rust
#[test]
fn test_schema_has_all_required_fields()
fn test_schema_field_types_correct()
fn test_facet_fields_configured()
```

**Query Builder Tests** (`search/query.rs`):

```rust
#[test]
fn test_query_builder_single_term()
fn test_query_builder_multi_word()
fn test_query_builder_fuzzy_enabled()
fn test_query_builder_empty_query_error()
fn test_query_builder_applies_field_boosts()
```

**Indexer Tests** (`search/indexer.rs`):

```rust
#[test]
fn test_indexer_creates_directory()
fn test_indexer_adds_document()
fn test_indexer_commit_persists()
fn test_indexer_clear_removes_all()
fn test_convert_search_document_to_tantivy()
```

#### 6.2 Integration Tests

**New file:** `crates/server/tests/tantivy_integration.rs`

```rust
#[tokio::test]
async fn test_build_index_from_test_data() {
    // Create temp directory with test markdown files
    // Build index
    // Verify documents are indexed
    // Verify facets are correct
}

#[tokio::test]
async fn test_search_returns_relevant_results() {
    // Build test index with known documents
    // Search for term present in title
    // Verify result ranking (title > description > content)
}

#[tokio::test]
async fn test_fuzzy_search_finds_typos() {
    // Index documents with "harmony"
    // Search for "haromny" (typo)
    // Verify fuzzy search finds correct documents
}

#[tokio::test]
async fn test_backend_switching() {
    // Create config with backend = "simple"
    // Execute search
    // Change to backend = "tantivy"
    // Execute same search
    // Verify both return results (may differ slightly in ranking)
}

#[tokio::test]
async fn test_snippet_generation() {
    // Index document with query term in middle
    // Search and get snippet
    // Verify snippet contains query with context
    // Verify snippet respects size limit
}
```

#### 6.3 Benchmark Tests

**New file:** `crates/server/benches/search_bench.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn search_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");

    for size in [50, 100, 200, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::new("simple", size),
            &size,
            |b, &size| {
                // Benchmark simple search with size documents
            }
        );

        group.bench_with_input(
            BenchmarkId::new("tantivy", size),
            &size,
            |b, &size| {
                // Benchmark Tantivy search with size documents
            }
        );
    }

    group.finish();
}

criterion_group!(benches, search_comparison);
criterion_main!(benches);
```

**Expected Results:**

- 50-200 docs: Simple and Tantivy similar (~20-50ms)
- 500+ docs: Tantivy significantly faster (sub-ms vs 100ms+)
- Crossover point: ~300-400 documents

---

### Phase 7: Migration & Documentation (1-2 hours)

#### 7.1 Migration Steps

**For Development:**

1. Keep `backend = "simple"` initially
2. Run tests to ensure no breaking changes
3. Set `backend = "tantivy"`, `rebuild_on_startup = true`
4. Verify search results quality (compare to simple)
5. Tune relevance if needed (adjust boosts in query.rs)

**For Production:**

1. Deploy with `backend = "tantivy"`, `rebuild_on_startup = true` (one-time)
2. Verify index builds successfully
3. Set `rebuild_on_startup = false`
4. Restart server (uses persisted index, fast startup)

#### 7.2 Rollback Strategy

If Tantivy search has issues:

1. Edit `config/default.toml`: `backend = "simple"`
2. Restart server
3. Immediately back to working simple search (zero downtime)

#### 7.3 Documentation Updates

**Update:** `README.md`

```markdown
## Search Configuration

The server supports two search backends:
- **Simple:** Linear scan (suitable for <500 concepts)
- **Tantivy:** Full-text search engine (recommended for 500+ concepts)

### Configuring Search Backend

Edit `config/default.toml`:

```toml
[search]
backend = "tantivy"  # or "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
fuzzy_search = true
fuzzy_distance = 2
```

### Building the Index

First time with Tantivy:

1. Set `rebuild_on_startup = true`
2. Restart server (builds index on startup)
3. Set `rebuild_on_startup = false`
4. Restart again (uses cached index)

Manual rebuild: `cargo run -- build-index`

```

**New file:** `crates/design/tantivy-integration.md`

- Architecture overview
- Schema design rationale
- Query translation strategy
- Performance benchmarks
- Future enhancements (incremental updates, faceted search)

---

## Critical Files to Modify/Create

### New Files (Create)
- `crates/server/src/search/schema.rs` (~150 lines)
- `crates/server/src/search/indexer.rs` (~200 lines)
- `crates/server/src/search/builder.rs` (~100 lines)
- `crates/server/src/search/query.rs` (~150 lines)
- `crates/server/src/search/tantivy_search.rs` (~200 lines)
- `crates/server/src/search/simple_search.rs` (~100 lines, adapter)
- `crates/server/tests/tantivy_integration.rs` (~300 lines)
- `crates/server/benches/search_bench.rs` (~100 lines)
- `crates/design/tantivy-integration.md` (documentation)

### Modified Files
- `crates/server/src/search/mod.rs` (~50 lines added)
- `crates/server/src/config.rs` (~50 lines added for SearchConfig)
- `crates/server/src/tools/search.rs` (~20 lines changed for backend factory)
- `crates/server/src/main.rs` (~10 lines added for index building)
- `config/default.toml` (~15 lines added for [search])
- `Cargo.toml` (~5 lines for tantivy dependency)
- `README.md` (~30 lines for search configuration)

**Total:** ~1,500 new lines, ~85 modified lines

---

## Rust Best Practices Applied

### From CLAUDE.md Checklists

**Before Starting:**
- ✅ Loaded `11-anti-patterns.md` (avoid common pitfalls)
- ✅ Loaded `01-core-idioms.md` (type-driven design)
- ✅ Loaded `03-error-handling.md` (Result types, context)
- ✅ Understood existing patterns (SearchDocument, Config structure)

**When Writing Code:**
- ✅ **AP-02 (Avoid Duplication):** Backend trait eliminates duplicate search logic
- ✅ **AP-06 (Extensibility):** SearchBackend trait allows future backends (Meilisearch, etc.)
- ✅ **AP-09 (Explicit Over Implicit):** Clear config-driven backend selection
- ✅ **Type-Driven Design:** Strong types (SearchSchema, Indexer, QueryBuilder)
- ✅ **Error Handling:** All Tantivy operations use Result with context
- ✅ **Graceful Degradation:** Fallback to simple search on errors

**Testing:**
- ✅ Test naming: `test_<fn>_<scenario>_<expectation>`
- ✅ Coverage target: ≥95%
- ✅ Happy path + error paths + edge cases
- ✅ Integration tests with real data
- ✅ Benchmarks for performance validation

---

## Error Handling Strategy

All Tantivy operations map to existing `Error::SearchError`:

```rust
// Index operations
Index::open_in_dir(path)
    .map_err(|e| Error::search_error(format!("Failed to open index: {}", e)))?;

// Query parsing
query_builder.build_query(query_str)
    .map_err(|e| Error::search_error(format!("Invalid query: {}", e)))?;

// Document retrieval
searcher.doc(doc_address)
    .map_err(|e| Error::search_error(format!("Failed to retrieve doc: {}", e)))?;
```

**Graceful Degradation:**

- If Tantivy backend fails to initialize → log error, suggest `backend = "simple"`
- If index is corrupted → return error suggesting rebuild
- If query is invalid → return helpful error message
- Indexing errors (individual files) → log warning, continue with remaining files

---

## Performance Expectations

### Index Size

- **200 docs @ ~5KB each:** ~1MB index (negligible disk space)
- **1000 docs @ ~5KB each:** ~5MB index

### Startup Time

- **Simple backend:** <100ms (no indexing)
- **Tantivy with existing index:** ~200-300ms (open + warm cache)
- **Tantivy with rebuild:** ~1-2 seconds for 200 docs, ~5-10 seconds for 1000 docs

### Query Time (200 docs)

- **Simple:** ~20-50ms (linear scan)
- **Tantivy:** ~1-5ms (indexed search)

### Query Time (1000 docs)

- **Simple:** ~100-200ms (becomes bottleneck)
- **Tantivy:** ~1-5ms (logarithmic scaling)

### Memory

- **Index writer buffer:** 50MB (configurable)
- **Index reader:** Memory-mapped (efficient, OS handles caching)
- **Per-query memory:** Minimal (Tantivy reuses buffers)

---

## Future Enhancements (Not in This Plan)

### Phase 8: Advanced Features (Future Work)

1. **Incremental Index Updates:**
   - File system watcher (notify crate)
   - Update index on file changes (not rebuild)
   - Hot reloading without restart

2. **Advanced Query Syntax:**
   - Boolean operators: `"harmony AND voice-leading"`
   - Phrase queries: `"parallel fifths"`
   - Field-specific: `"title:cadence"`
   - Negation: `"harmony NOT jazz"`

3. **Faceted Search API:**
   - New tool: `search_concepts_faceted(query, facets: {category, source})`
   - Return facet counts with results
   - Enable multi-dimensional filtering

4. **Search Analytics:**
   - Log popular queries
   - Track slow searches
   - Report index statistics (size, doc count, etc.)

5. **Typo Tolerance Tuning:**
   - Configurable edit distance per field
   - Phonetic matching (Soundex, Metaphone)
   - Synonym support

---

## Testing & Validation

### Unit Test Coverage Target

- ✅ Schema: 100% (all fields, types, options)
- ✅ Indexer: ≥95% (creation, add, commit, clear, conversion)
- ✅ Query builder: ≥95% (single term, multi-term, fuzzy, boosts)
- ✅ Search: ≥90% (query execution, snippet generation, result mapping)

### Integration Test Scenarios

1. Build index from scratch with test data
2. Search returns results ranked by relevance
3. Fuzzy search finds typos
4. Backend switching works (simple ↔ tantivy)
5. Snippet generation includes query with context
6. Facet filtering works (category, source, tags)
7. Empty query returns error
8. Index rebuild clears old data

### Benchmark Validation

- Simple vs Tantivy at 50, 100, 200, 500, 1000 docs
- Verify Tantivy faster at 500+ docs
- Confirm sub-millisecond query time with Tantivy

### Manual Verification

1. Start server with `backend = "tantivy"`, `rebuild_on_startup = true`
2. Verify index builds (check logs)
3. Test MCP tool: `search_concepts("harmony")` → returns results
4. Test typo: `search_concepts("haromny")` → finds "harmony" (if fuzzy enabled)
5. Compare results: simple vs tantivy (should be similar quality)
6. Verify persistence: restart server with `rebuild_on_startup = false` (fast startup)

---

## Implementation Timeline

### Session 1: Foundation (2-3 hours)

- [ ] Create SearchSchema (schema.rs)
- [ ] Add SearchConfig to config.rs
- [ ] Update config/default.toml
- [ ] Add tantivy dependency
- [ ] Tests: Schema structure
- [ ] **Checkpoint:** cargo build succeeds, tests pass

### Session 2: Indexing (3-4 hours)

- [ ] Create Indexer (indexer.rs)
- [ ] Create build_index (builder.rs)
- [ ] Implement SearchDocument → Tantivy conversion
- [ ] Tests: Indexing pipeline
- [ ] **Checkpoint:** Can build index from test data

### Session 3: Querying (3-4 hours)

- [ ] Create QueryBuilder (query.rs)
- [ ] Create TantivySearch (tantivy_search.rs)
- [ ] Implement weighted query boosting
- [ ] Implement snippet generation
- [ ] Tests: Query building, search execution
- [ ] **Checkpoint:** Can execute searches, get results

### Session 4: Integration (2-3 hours)

- [ ] Create SearchBackend trait
- [ ] Implement SimpleSearch adapter
- [ ] Update search_concepts to use factory
- [ ] Update server initialization for index building
- [ ] Tests: Backend switching
- [ ] **Checkpoint:** search_concepts works with both backends

### Session 5: Testing & Polish (3-4 hours)

- [ ] Integration tests (tantivy_integration.rs)
- [ ] Benchmark tests (search_bench.rs)
- [ ] Coverage verification (≥95%)
- [ ] Performance validation
- [ ] **Checkpoint:** All tests pass, benchmarks confirm improvement

### Session 6: Documentation (1-2 hours)

- [ ] Update README.md
- [ ] Create design doc (crates/design/tantivy-integration.md)
- [ ] Add inline documentation (doc comments)
- [ ] Test manual verification steps
- [ ] **Checkpoint:** Documentation complete, ready for production

**Total:** 15-22 hours over 6 sessions

---

## Success Criteria

### Phase Completion

- ✅ All unit tests pass (≥95% coverage)
- ✅ All integration tests pass
- ✅ Benchmarks show expected performance (Tantivy faster at 500+ docs)
- ✅ No clippy warnings
- ✅ Code formatted with rustfmt
- ✅ Documentation updated

### Feature Validation

- ✅ Config-driven backend selection works
- ✅ Simple backend unchanged (backward compatible)
- ✅ Tantivy backend returns relevant results
- ✅ Fuzzy search finds typos (if enabled)
- ✅ Snippets include query with context
- ✅ Index persists across restarts

### Production Readiness

- ✅ Zero breaking changes to MCP API
- ✅ Instant rollback possible (change config)
- ✅ Error messages are helpful
- ✅ Startup time acceptable (<2 seconds with rebuild)
- ✅ Memory usage reasonable (<100MB)

---

## Rollback Plan

If any issues arise during implementation:

1. **Code issues:** Git commits are incremental per phase
2. **Runtime issues:** Change `backend = "simple"` in config
3. **Index corruption:** Delete `.tantivy-index`, rebuild
4. **Performance issues:** Benchmark to identify bottleneck, tune or rollback

**Minimal Working State:**

- Keep SearchConfig (harmless if not used)
- Keep SimpleSearch adapter (maintains existing behavior)
- Revert backend factory if integration fails

---

## Conclusion

This plan provides a complete, incremental path to integrating Tantivy full-text search with:

- ✅ **Backward compatibility:** No breaking changes, instant rollback
- ✅ **Rust best practices:** Follows CLAUDE.md guidelines, anti-patterns avoided
- ✅ **Comprehensive testing:** Unit, integration, benchmark tests
- ✅ **Clear migration path:** Phased approach with checkpoints
- ✅ **Future-proof:** Extensible architecture for advanced features

The existing SearchDocument architecture makes this integration straightforward - no refactoring needed, just add Tantivy as an alternative backend.
