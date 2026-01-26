---
number: 3
title: "Fix Category Extraction and Improve Metadata Architecture"
author: "Option<String>,"
component: All
tags: [change-me]
created: 2026-01-26
updated: 2026-01-26
state: Under Review
supersedes: null
superseded-by: null
version: 1.0
---

# Fix Category Extraction and Improve Metadata Architecture

## Executive Summary

**Problem:** Category is extracted from directory structure, ignoring YAML frontmatter
**Impact:** All 146 concepts show `category: "open-music-theory"` instead of thematic categories like `"fundamentals"`, `"voice-leading"`, `"harmony"`
**Root Cause:** Frontmatter struct missing `category` field; duplicate `extract_category()` functions use directory-based logic
**Solution:** Add category to frontmatter, consolidate metadata extraction, add list_categories tool

---

## Bugs Identified

### Bug 1: Category Field Missing from Frontmatter Struct

**File:** `crates/server/src/markdown/frontmatter.rs:10-23`
**Issue:** Frontmatter only has `title`, `description`, `tags`, `author`, `date` — missing `category`
**Result:** Category in YAML frontmatter is silently ignored

### Bug 2: Duplicate `extract_category()` Functions

**Files:**

- `crates/server/src/tools/concepts.rs:108-117`
- `crates/server/src/tools/search.rs:117-127`

**Issue:** Identical implementations (AP-02: Code Duplication)
**Result:** Violates DRY; changes must be made in two places

### Bug 3: Directory Structure Used Instead of Metadata

**Both `extract_category()` functions:**

```rust
// Takes FIRST directory component after base path
relative.components().next()
```

**Issue:** Assumes directory organization = thematic category
**Result:** All files in `/open-music-theory/` get category `"open-music-theory"` regardless of frontmatter

### Bug 4: No Source Field in Data Models

**Structs:** `ConceptInfo`, `SearchResult`
**Issue:** Category field conflates source name with thematic category
**Result:** Can't distinguish source (Open Music Theory) from topic (fundamentals)

### Bug 5: Inconsistent Metadata Extraction

**Files:**

- `concepts.rs` calls `extract_title_and_preview()` → reads frontmatter for title
- `search.rs` calls `extract_metadata()` → reads frontmatter for title
- Both call `extract_category()` → uses directory structure

**Issue:** Metadata from two different sources (frontmatter + filesystem)
**Result:** No single source of truth for concept metadata

---

## Analysis: Current State vs Requirements

### Current Architecture

```
Concept Card File (.md)
├── Frontmatter (YAML)
│   ├── concept: "Accidental"         ← Not used
│   ├── category: "fundamentals"      ← IGNORED (bug)
│   ├── source: "Open Music Theory"   ← Not used
│   ├── chapter: "Half Steps..."      ← Not used
│   ├── part: 1                       ← Not used
│   └── (standard fields: title, description, tags, author, date)
├── Heading (# Accidental)
└── Content (markdown body)

Metadata Extraction
├── Title: From frontmatter.title OR first heading
├── Description/Preview: From frontmatter.description OR first paragraph
└── Category: From directory structure (WRONG)
```

### Required Architecture

```
Concept Card File (.md)
├── Frontmatter (YAML) - Single source of truth
│   ├── concept: "Accidental"
│   ├── category: "fundamentals"      ← Use this
│   ├── source: "Open Music Theory"   ← Use this
│   ├── chapter: "Half Steps..."      ← Use this
│   ├── part: 1                       ← Expose in API
│   └── (standard fields: title, description, tags, author, date)
├── Heading
└── Content

Metadata Extraction
├── All fields from frontmatter (single read)
├── Fallbacks ONLY if frontmatter missing
└── Directory structure as last resort
```

---

## Proposed Improvements

### 1. Generalize: Extended Frontmatter Struct

**Add music-theory-specific fields to Frontmatter:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    // Standard fields
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub date: Option<String>,

    // Music theory specific fields
    pub concept: Option<String>,      // NEW: canonical concept name
    pub category: Option<String>,     // NEW: thematic category (fundamentals, harmony, etc.)
    pub source: Option<String>,       // NEW: source text name
    pub chapter: Option<String>,      // NEW: chapter/section reference
    pub part: Option<u32>,           // NEW: part number
}
```

**Benefits:**

- Single source of truth for all metadata
- Type-safe deserialization with serde
- Extensible for future fields
- Graceful handling of missing fields (Option types)

---

### 2. Standardize: Centralized Metadata Module

**Create:** `crates/server/src/metadata/mod.rs`

**Purpose:** Consolidate all metadata extraction logic

```rust
pub mod frontmatter;  // Re-export from markdown
pub mod extraction;   // New: standardized extraction

pub use extraction::{
    extract_concept_metadata,
    ConceptMetadata,
};
```

**New file:** `crates/server/src/metadata/extraction.rs`

```rust
/// Complete metadata for a concept card
#[derive(Debug, Clone)]
pub struct ConceptMetadata {
    pub id: String,
    pub title: String,
    pub category: String,
    pub source: Option<String>,
    pub chapter: Option<String>,
    pub part: Option<u32>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

/// Extract all metadata from a concept card file
/// Strategy:
/// 1. Read frontmatter (primary source)
/// 2. Fallback to markdown structure (heading, etc.)
/// 3. Derive from filesystem (last resort)
pub async fn extract_concept_metadata(
    base_path: &Path,
    file_path: &Path,
) -> Result<ConceptMetadata> {
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Extract ID from filename
    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Title: frontmatter.title OR frontmatter.concept OR heading OR filename
    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| frontmatter.as_ref().and_then(|fm| fm.concept.clone()))
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    // Category: frontmatter.category OR directory structure
    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    // Source: frontmatter.source OR infer from directory
    let source = frontmatter
        .as_ref()
        .and_then(|fm| fm.source.clone());

    // Other fields from frontmatter
    let fm = frontmatter.unwrap_or_default();

    Ok(ConceptMetadata {
        id,
        title,
        category,
        source,
        chapter: fm.chapter,
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
    })
}

/// Fallback: extract category from directory structure
fn extract_category_from_path(base: &Path, file_path: &Path) -> String {
    file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base).ok())
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .unwrap_or("uncategorized")
        .to_string()
}
```

**Benefits:**

- Single function for all metadata extraction
- Clear precedence: frontmatter > markdown > filesystem
- Eliminates duplicate `extract_category()` implementations
- Easy to test and maintain
- Follows AP-06 (Design for extensibility)

---

### 3. Improve: Update Data Models

**Add source field to response structs:**

```rust
// concepts.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptInfo {
    pub id: String,
    pub title: String,
    pub category: String,      // Thematic category (fundamentals, harmony, etc.)
    pub source: Option<String>, // NEW: Source text (Open Music Theory, etc.)
    pub chapter: Option<String>, // NEW: Chapter reference
    pub path: String,
    pub preview: Option<String>,
}

// search.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub category: String,      // Thematic category
    pub source: Option<String>, // NEW: Source text
    pub path: String,
    pub snippet: String,
    pub relevance: f32,
}
```

**Benefits:**

- Separate concerns: category (topic) vs source (origin)
- Enables filtering by source AND category
- Richer API responses
- Backward compatible (Option types)

---

### 4. Future-Proof: Tantivy-Ready Document Model

**Create:** `crates/server/src/search/document.rs` (future)

```rust
/// Searchable document for Tantivy indexing
/// This struct defines the schema for full-text search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    // Identity
    pub id: String,              // Unique ID (indexed, stored)
    pub path: String,            // File path (stored)

    // Metadata fields (indexed as facets)
    pub category: String,        // Facet: filter by category
    pub source: Option<String>,  // Facet: filter by source
    pub tags: Vec<String>,       // Facet: filter by tags

    // Text fields (full-text indexed)
    pub title: String,           // Indexed with boost=3.0
    pub content: String,         // Indexed with boost=1.0
    pub description: Option<String>, // Indexed with boost=2.0

    // Structured fields (stored, not indexed)
    pub chapter: Option<String>,
    pub part: Option<u32>,
    pub author: Option<String>,
    pub date: Option<String>,
}

impl From<ConceptMetadata> for SearchDocument {
    fn from(meta: ConceptMetadata) -> Self {
        SearchDocument {
            id: meta.id,
            path: /* from file_path */,
            category: meta.category,
            source: meta.source,
            tags: meta.tags,
            title: meta.title,
            content: /* full markdown content */,
            description: meta.description,
            chapter: meta.chapter,
            part: meta.part,
            author: meta.author,
            date: meta.date,
        }
    }
}
```

**Benefits:**

- Defines the search schema upfront
- Clean conversion from ConceptMetadata
- Tantivy can use this directly for indexing
- Current simple search can use it too
- Decouples domain model (ConceptMetadata) from search model (SearchDocument)

---

## Rust Best Practices Applied

### From SKILL.md and Guides

**AP-02: Avoid Code Duplication**

- ✅ Eliminate duplicate `extract_category()` functions
- ✅ Consolidate metadata extraction into single module

**AP-06: Design for Extensibility**

- ✅ Extended Frontmatter with new fields using Option types
- ✅ ConceptMetadata captures all fields in one place
- ✅ SearchDocument prepares for Tantivy migration

**AP-09: Prefer Explicit Over Implicit**

- ✅ Clear precedence: frontmatter > markdown > filesystem
- ✅ Separate category (topic) from source (origin)
- ✅ Document fallback behavior

**01-core-idioms.md: Type-Driven Design**

- ✅ Use strong types (ConceptMetadata) instead of tuples
- ✅ Option types for optional fields
- ✅ Serde derives for serialization
- ✅ From trait for conversions

**03-error-handling.md: Result and Context**

- ✅ extract_concept_metadata returns Result<ConceptMetadata>
- ✅ Propagate errors with ?
- ✅ Graceful fallbacks for missing fields

**06-testing.md: Testability**

- ✅ Pure function: extract_concept_metadata(paths) → metadata
- ✅ Easy to test with temp files
- ✅ Mock frontmatter variations

---

## Implementation Plan

### Phase 1: Extend Frontmatter (15 min)

**File:** `crates/server/src/markdown/frontmatter.rs`

1. Add new fields to Frontmatter struct:

   ```rust
   pub concept: Option<String>,
   pub category: Option<String>,
   pub source: Option<String>,
   pub chapter: Option<String>,
   pub part: Option<u32>,
   ```

2. Remove `#[allow(dead_code)]` from `strip_frontmatter()` function (line 99)
   - We'll use it in SearchDocument to get content without frontmatter

3. Update tests to verify new fields deserialize correctly

---

### Phase 2: Create Metadata Module (30 min)

**Files to create:**

- `crates/server/src/metadata/mod.rs`
- `crates/server/src/metadata/extraction.rs`

1. Implement `ConceptMetadata` struct
2. Implement `extract_concept_metadata()` with precedence logic
3. Implement `extract_category_from_path()` (move from concepts.rs/search.rs)
4. Add comprehensive tests (10+ test cases)

**Files to modify:**

- `crates/server/src/lib.rs` or `main.rs` - add `mod metadata;`

---

### Phase 3: Create SearchDocument Module (25 min)

**Files to create:**

- `crates/server/src/search/mod.rs`
- `crates/server/src/search/document.rs`

**File:** `crates/server/src/search/document.rs`

```rust
//! Search document representation for full-text search.
//!
//! SearchDocument is the search-ready representation of a concept card.
//! It's used by both the current simple search and future Tantivy indexing.

use serde::{Deserialize, Serialize};
use crate::metadata::ConceptMetadata;
use crate::util::files::read_file;
use crate::error::Result;
use std::path::Path;

/// A search-ready document containing all searchable fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    // Identity
    pub id: String,
    pub path: String,

    // Metadata (facets for filtering)
    pub category: String,
    pub source: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

    // Full-text fields (with implicit weights for relevance)
    pub title: String,           // Weight: 3.0 (title matches are most important)
    pub description: String,      // Weight: 2.0 (description is secondary)
    pub content: String,          // Weight: 1.0 (body content is baseline)

    // Structured metadata (stored but not full-text indexed)
    pub chapter: Option<String>,
    pub part: Option<u32>,
    pub author: Option<String>,
    pub date: Option<String>,
}

impl SearchDocument {
    /// Create a SearchDocument from ConceptMetadata and file content
    pub async fn from_metadata(meta: ConceptMetadata, file_path: &Path) -> Result<Self> {
        // Read full file content
        let full_content = read_file(file_path).await?;

        // Strip frontmatter from content (we already have metadata)
        let content = crate::markdown::strip_frontmatter(&full_content);

        Ok(SearchDocument {
            id: meta.id,
            path: file_path.to_string_lossy().to_string(),
            category: meta.category,
            source: meta.source.clone(),
            tags: meta.tags.clone(),
            title: meta.title,
            description: meta.description.unwrap_or_default(),
            content: content.to_string(),
            chapter: meta.chapter,
            part: meta.part,
            author: meta.author,
            date: meta.date,
        })
    }

    /// Check if this document matches a query (case-insensitive)
    pub fn matches_query(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        let title_lower = self.title.to_lowercase();
        let desc_lower = self.description.to_lowercase();
        let content_lower = self.content.to_lowercase();

        title_lower.contains(&query_lower)
            || desc_lower.contains(&query_lower)
            || content_lower.contains(&query_lower)
    }

    /// Calculate relevance score for a query
    pub fn relevance(&self, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let mut score = 0.0;

        // Title matches (weight: 3.0)
        let title_lower = self.title.to_lowercase();
        if title_lower.contains(&query_lower) {
            score += 10.0 * 3.0;
        }
        let title_occurrences = title_lower.matches(&query_lower).count() as f32;
        score += title_occurrences * 3.0;

        // Description matches (weight: 2.0)
        let desc_lower = self.description.to_lowercase();
        let desc_occurrences = desc_lower.matches(&query_lower).count() as f32;
        score += desc_occurrences * 2.0;

        // Content matches (weight: 1.0)
        let content_lower = self.content.to_lowercase();
        let content_occurrences = content_lower.matches(&query_lower).count() as f32;
        score += content_occurrences;

        // Exact word match bonus (across all fields)
        let all_text = format!("{} {} {}", title_lower, desc_lower, content_lower);
        let words: Vec<&str> = all_text.split_whitespace().collect();
        let exact_matches = words.iter().filter(|&&w| w == query_lower).count() as f32;
        score += exact_matches * 2.0;

        // Normalize by content length
        let total_len = (self.title.len() + self.description.len() + self.content.len()) as f32;
        if total_len > 0.0 {
            score = score * 1000.0 / total_len;
        }

        score
    }

    /// Extract a snippet around the first query match
    pub fn extract_snippet(&self, query: &str, context_chars: usize) -> String {
        let query_lower = query.to_lowercase();

        // Try to find match in description first (more relevant)
        if let Some(snippet) = Self::find_snippet(&self.description, query, context_chars) {
            return snippet;
        }

        // Fall back to content
        if let Some(snippet) = Self::find_snippet(&self.content, query, context_chars) {
            return snippet;
        }

        // No match found - return first part of description or content
        if !self.description.is_empty() {
            self.description.chars().take(context_chars).collect()
        } else {
            self.content.chars().take(context_chars).collect()
        }
    }

    fn find_snippet(text: &str, query: &str, context_chars: usize) -> Option<String> {
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();

        if let Some(pos) = text_lower.find(&query_lower) {
            let start = pos.saturating_sub(context_chars / 2);
            let end = (pos + query.len() + context_chars / 2).min(text.len());

            let mut snippet = String::with_capacity(end - start + 6);

            if start > 0 {
                snippet.push_str("...");
            }

            let slice = &text[start..end];
            for ch in slice.chars() {
                if ch == '\n' {
                    snippet.push(' ');
                } else {
                    snippet.push(ch);
                }
            }

            if end < text.len() {
                snippet.push_str("...");
            }

            Some(snippet.trim().to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_doc() -> SearchDocument {
        SearchDocument {
            id: "test-doc".to_string(),
            path: "/test.md".to_string(),
            category: "harmony".to_string(),
            source: Some("Test Source".to_string()),
            tags: vec!["test".to_string()],
            title: "Test Document".to_string(),
            description: "A test document about harmony".to_string(),
            content: "This document discusses harmonic concepts in detail.".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
        }
    }

    #[test]
    fn test_matches_query_title() {
        let doc = sample_doc();
        assert!(doc.matches_query("document"));
        assert!(doc.matches_query("DOCUMENT"));
    }

    #[test]
    fn test_matches_query_content() {
        let doc = sample_doc();
        assert!(doc.matches_query("harmonic"));
    }

    #[test]
    fn test_matches_query_no_match() {
        let doc = sample_doc();
        assert!(!doc.matches_query("nonexistent"));
    }

    #[test]
    fn test_relevance_title_boost() {
        let doc = sample_doc();
        let score = doc.relevance("document");
        assert!(score > 10.0); // Title match should give high score
    }

    #[test]
    fn test_relevance_content_match() {
        let doc = sample_doc();
        let score = doc.relevance("harmonic");
        assert!(score > 0.0);
    }

    #[test]
    fn test_extract_snippet_from_description() {
        let doc = sample_doc();
        let snippet = doc.extract_snippet("harmony", 20);
        assert!(snippet.contains("harmony"));
    }

    #[test]
    fn test_extract_snippet_from_content() {
        let doc = sample_doc();
        let snippet = doc.extract_snippet("discusses", 30);
        assert!(snippet.contains("discusses"));
    }
}
```

**File:** `crates/server/src/search/mod.rs`

```rust
//! Search functionality for concept cards.

mod document;

pub use document::SearchDocument;
```

**Register module:**

- `crates/server/src/lib.rs` or `main.rs` - add `mod search;`

**Why implement SearchDocument now instead of waiting for Tantivy?**

**Benefits for current simple search:**

1. **Better relevance scoring:** Weighted fields (title 3x > description 2x > content 1x) instead of flat scoring
2. **Cleaner code:** Search logic encapsulated in SearchDocument methods, not scattered across search.rs
3. **Easier testing:** Test `doc.relevance("query")` in isolation, not the whole search flow
4. **Maintainability:** One place to improve search algorithm
5. **Separation of concerns:** Document preparation (SearchDocument) vs search logic (tools/search.rs)

**Benefits for future Tantivy migration:**
6. **Zero tool changes:** Tools already use SearchDocument, just swap indexer
7. **Schema defined:** Tantivy index schema directly from SearchDocument fields
8. **Gradual migration:** Can keep simple search as fallback while testing Tantivy
9. **Consistent behavior:** Same relevance weighting in simple and Tantivy search

**Cost:**

- 250 lines of well-tested code
- Minimal performance impact (building SearchDocument vs ad-hoc parsing)
- Small memory overhead (SearchDocument struct vs loose variables)

**Verdict:** Benefits far outweigh costs. SearchDocument is useful TODAY and essential for TOMORROW.

---

### Phase 4: Update Tools to Use Metadata + SearchDocument (30 min)

**File:** `crates/server/src/tools/concepts.rs`

1. Import `use crate::metadata::extract_concept_metadata;`
2. Add `source` and `chapter` fields to `ConceptInfo`
3. Replace:

   ```rust
   // OLD
   let (title, preview) = extract_title_and_preview(&file_info.path)?;
   let category = extract_category(&concept_cards_path, &file_info.path);

   // NEW
   let meta = extract_concept_metadata(&concept_cards_path, &file_info.path).await?;
   ```

4. Update ConceptInfo construction to use meta fields
5. Remove old `extract_category()` and `extract_title_and_preview()` functions
6. Update tests

**File:** `crates/server/src/tools/search.rs`

1. Import `use crate::metadata::extract_concept_metadata;`
2. Import `use crate::search::SearchDocument;`
3. Add `source` field to `SearchResult`
4. Replace search logic:

   ```rust
   // OLD: Read content, extract metadata, extract category separately
   let content = read_file(path).await?;
   let content_lower = content.to_lowercase();
   if content_lower.contains(&query_lower) {
       let (title, _) = extract_metadata(&content);
       let category = extract_category(&concept_cards_path, path);
       let snippet = extract_snippet(&content, &params.query);
       let relevance = calculate_relevance(&content_lower, &title.to_lowercase(), &query_lower);
       // ...
   }

   // NEW: Build SearchDocument, use its methods
   let meta = extract_concept_metadata(&concept_cards_path, &file_info.path).await?;
   let doc = SearchDocument::from_metadata(meta, &file_info.path).await?;

   if doc.matches_query(&query_lower) {
       let snippet = doc.extract_snippet(&params.query, 200);
       let relevance = doc.relevance(&params.query);

       results.push(SearchResult {
           id: doc.id,
           title: doc.title,
           category: doc.category,
           source: doc.source,
           path: doc.path,
           snippet,
           relevance,
       });
   }
   ```

5. Remove `extract_category()`, `extract_metadata()`, `extract_snippet()`, `calculate_relevance()` functions
6. Update tests

**Benefits of SearchDocument for current search:**

- **Better relevance**: Weighted scoring (title 3x, description 2x, content 1x)
- **Cleaner code**: Document knows how to match/score/extract itself
- **Easier testing**: Test SearchDocument independently
- **Tantivy-ready**: Same document used for future indexing
- **Performance**: Could cache SearchDocuments if needed (future optimization)

---

### Phase 5: Add list_categories Tool (30 min)

**File:** `crates/server/src/tools/concepts.rs`

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCategoriesResponse {
    pub categories: Vec<CategoryInfo>,
    pub total: usize,
}

#[tool(
    name = "list_categories",
    description = "List all distinct concept categories with counts"
)]
pub async fn list_categories(config: &Config) -> Result<ListCategoriesResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !exists(&concept_cards_path).await {
        return Ok(ListCategoriesResponse {
            categories: Vec::new(),
            total: 0,
        });
    }

    // Scan all concept files
    let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;

    // Count by category
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for file_info in files {
        if let Ok(meta) = extract_concept_metadata(&concept_cards_path, &file_info.path).await {
            *category_counts.entry(meta.category).or_insert(0) += 1;
        }
    }

    // Convert to sorted list
    let mut categories: Vec<CategoryInfo> = category_counts
        .into_iter()
        .map(|(name, count)| CategoryInfo { name, count })
        .collect();

    categories.sort_by(|a, b| a.name.cmp(&b.name));
    let total = categories.len();

    Ok(ListCategoriesResponse { categories, total })
}
```

**Register tool in server.rs:**

```rust
use crate::tools::list_categories;  // Add to imports
```

The #[tool] macro will automatically register it.

---

### Phase 6: Add Category Filtering (15 min)

**File:** `crates/server/src/tools/concepts.rs`

Update ListConceptsParams:

```rust
#[derive(Debug, Deserialize)]
pub struct ListConceptsParams {
    #[serde(default)]
    pub category: Option<String>,  // NEW: filter by category
    #[serde(default = "default_limit")]
    pub limit: usize,
}
```

Update list_concepts to filter:

```rust
// After extracting metadata
if let Some(filter_cat) = &params.category {
    if meta.category != *filter_cat {
        continue;  // Skip concepts not in requested category
    }
}
```

---

### Phase 7: Update Tests (20 min)

**Update test files for:**

- `crates/server/src/metadata/extraction.rs` - NEW tests
- `crates/server/src/tools/concepts.rs` - Update existing tests
- `crates/server/src/tools/search.rs` - Update existing tests

**Test cases to add:**

1. Metadata extraction with all frontmatter fields present
2. Metadata extraction with missing frontmatter (fallbacks)
3. Category from frontmatter vs directory structure
4. list_categories returns correct counts
5. list_concepts with category filter
6. SearchResult includes source field

---

## Critical Files to Modify

| File | Lines Changed | Purpose |
|------|--------------|---------|
| `crates/server/src/markdown/frontmatter.rs` | +5 fields, +1 export | Add music-theory fields, export strip_frontmatter |
| `crates/server/src/metadata/mod.rs` | +10 (new) | Module declaration |
| `crates/server/src/metadata/extraction.rs` | +150 (new) | Consolidated metadata extraction |
| `crates/server/src/search/mod.rs` | +5 (new) | Module declaration |
| `crates/server/src/search/document.rs` | +250 (new) | SearchDocument with methods |
| `crates/server/src/tools/concepts.rs` | -40, +60 | Use metadata module, add list_categories, add filtering |
| `crates/server/src/tools/search.rs` | -80, +30 | Use SearchDocument for cleaner search logic |
| `crates/server/src/main.rs` or `lib.rs` | +2 | Declare metadata and search modules |

**Total:** ~465 new lines, ~120 deleted lines, **net +345 lines** of well-structured, maintainable code

---

## Future-Proofing for Tantivy

### Current Simple Search Architecture (After This Plan)

```
Query → Scan files → Build SearchDocument for each → Filter/Score/Sort → Return top N
```

**Key improvement:** SearchDocument provides structured representation with weighted fields

### Future Tantivy Architecture

```
Indexing (background):
  File changes → ConceptMetadata → SearchDocument → Tantivy Index

Querying (realtime):
  Query → Tantivy Index → Retrieve IDs → Load SearchDocument → Return
```

### Migration Path

**This Plan (Completed Here):**

- ✅ ConceptMetadata captures all fields
- ✅ SearchDocument with all searchable fields
- ✅ Separate category/source/tags for faceted search
- ✅ Centralized extraction logic
- ✅ Simple search using SearchDocument

**Future Phase 1: Add Tantivy Indexing**

- Add `tantivy` dependency
- Create index schema from SearchDocument fields:
  - `id`: STORED, STRING (primary key)
  - `category`: FACET (for filtering)
  - `source`: FACET (for filtering)
  - `tags`: FACET (for filtering)
  - `title`: TEXT (indexed with weight 3.0)
  - `description`: TEXT (indexed with weight 2.0)
  - `content`: TEXT (indexed with weight 1.0)
- Background indexer reads SearchDocument, writes to Tantivy
- File watcher triggers re-indexing on changes

**Future Phase 2: Switch Search Implementation**

- Implement `search_tantivy()` function
- Keep `search_simple()` as fallback
- Feature flag or config to choose implementation
- Same SearchResult API (tools don't change)

**Key Benefit:** SearchDocument is the abstraction layer. Current simple search and future Tantivy both consume it. Tools only know about SearchResult.

---

## Impact on Full-Text Search Plans

### Current Simple Search: Significantly Improved ✅

**For 100-200 concept cards:**

- Scan-based search: ~20-50ms latency (unchanged)
- Low memory footprint (minimal increase for SearchDocument)
- Zero infrastructure (no index files)
- Simple to maintain (cleaner code with SearchDocument)

**This plan improves simple search:**

- **More accurate metadata:** Category from frontmatter, not directory
- **Richer results:** Source, chapter fields included
- **Better relevance scoring:** Weighted fields (title 3x, description 2x, content 1x)
- **Cleaner code:** SearchDocument encapsulates matching/scoring/snippet logic
- **Easier to extend:** Add new fields to SearchDocument without changing tools
- **Testable:** Search logic isolated in SearchDocument methods

**Performance impact:**

- Slightly slower per-file (builds SearchDocument instead of simple string search)
- But more accurate results (better relevance ranking)
- Net neutral: ~20-50ms still acceptable for 100-200 cards

### Future Tantivy Migration: Trivial ✅

**This plan sets up:**

1. **SearchDocument exists:** Already defines the search schema
2. **Clean domain model:** ConceptMetadata → SearchDocument → SearchResult pipeline
3. **Separation of concerns:** Extraction, search prep, and tools are decoupled
4. **Extensible schema:** SearchDocument has all Tantivy-ready fields
5. **Facet-ready:** Category, source, tags ready for Tantivy faceting
6. **Stable API:** Tools return same SearchResult before/after Tantivy
7. **Drop-in replacement:** Just change search implementation, keep SearchDocument interface

### When to Migrate to Tantivy

**Triggers:**

- Collection grows >500 concept cards
- Search latency >100ms
- Need typo tolerance
- Need boolean queries (AND/OR/NOT)
- Need phrase search
- Need faceted navigation UI

**Current:** Stay with simple search
**After this plan:** Can migrate incrementally without breaking tools

---

## Testing Strategy

### Unit Tests (cargo test)

**New tests in `metadata/extraction.rs`:**

1. extract_concept_metadata with complete frontmatter
2. extract_concept_metadata with partial frontmatter (fallbacks)
3. extract_concept_metadata with no frontmatter (all fallbacks)
4. extract_category_from_path for various directory structures
5. Title extraction precedence (frontmatter.title > concept > heading > filename)
6. Category extraction precedence (frontmatter.category > directory)

**Updated tests in `tools/concepts.rs`:**

1. list_concepts returns correct categories from frontmatter
2. list_concepts with category filter
3. list_categories returns accurate counts
4. ConceptInfo serialization with new fields

**Updated tests in `tools/search.rs`:**

1. search_concepts returns correct categories from frontmatter
2. SearchResult serialization with new source field

### Integration Tests (with MCP client)

**Test 1: Category correctness**

```
list_concepts() → verify categories match frontmatter, not directory
```

**Test 2: Category filtering**

```
list_concepts(category: "fundamentals") → only fundamentals concepts
```

**Test 3: List categories**

```
list_categories() → verify counts and category names
```

**Test 4: Search with category**

```
search_concepts("interval") → results include category from frontmatter
```

**Test 5: Source field populated**

```
list_concepts() → verify source field shows "Open Music Theory"
get_concept("accidental") → verify source in response
```

### Manual Verification

**Step 1: Check actual concept files**

```bash
cd ~/lab/music-comp/ai-music-theory/concept-cards/open-music-theory
head -20 accidental.md
# Should show: category: fundamentals
```

**Step 2: Start server and query**

```bash
cargo run --release
```

**Step 3: Use MCP client to verify**

```
list_concepts(limit: 5)
→ Should show category: "fundamentals" not "open-music-theory"

list_categories()
→ Should show: fundamentals, harmony, voice-leading, etc.
→ NOT: open-music-theory

search_concepts("accidental")
→ Should show category: "fundamentals"
→ Should show source: "Open Music Theory"
```

---

## Success Criteria

**Phase 1-2 (Foundation):**

- ✅ Frontmatter supports all music-theory fields
- ✅ ConceptMetadata captures all metadata from frontmatter
- ✅ Metadata extraction has clear precedence (frontmatter > markdown > filesystem)
- ✅ All metadata module tests pass

**Phase 3 (SearchDocument):**

- ✅ SearchDocument created with all searchable fields
- ✅ Weighted relevance scoring (title 3x, description 2x, content 1x)
- ✅ SearchDocument.matches_query() and .relevance() work correctly
- ✅ All SearchDocument tests pass

**Phase 4 (Core Fix):**

- ✅ list_concepts returns frontmatter category values
- ✅ search_concepts uses SearchDocument for better results
- ✅ No duplicate extract_category functions (AP-02 compliance)
- ✅ ConceptInfo and SearchResult have source field
- ✅ All tool tests pass

**Phase 5-6 (New Features):**

- ✅ list_categories tool exists and returns accurate counts
- ✅ list_concepts(category: "X") filters correctly
- ✅ MCP client can discover categories and filter by them

**Phase 7 (Quality):**

- ✅ 240+ tests passing (new tests added for metadata, search, tools)
- ✅ No clippy warnings
- ✅ Code coverage ≥95% on new modules

**Overall:**

- ✅ Feedback items 1-3 from music-theory-mcp-feedback.md addressed
- ✅ Metadata extraction follows Rust best practices (AP-02, AP-06, AP-09)
- ✅ SearchDocument ready for Tantivy migration
- ✅ Better search relevance with weighted fields
- ✅ No breaking changes to existing MCP API

---

## Rollback Plan

**If issues arise:**

1. Git commits are incremental (one per phase)
2. Can revert any phase independently
3. Metadata module is additive (doesn't break existing code)
4. Tests validate backward compatibility

**Minimal working state:**

- Keep Phase 1 (extended frontmatter)
- Keep Phase 2 (metadata module)
- Revert Phase 3 if tool updates have issues
- Phases 4-5 are purely additive (can skip)

---

## Timeline Estimate

| Phase | Time | Cumulative |
|-------|------|-----------|
| Phase 1: Extend frontmatter | 15 min | 15 min |
| Phase 2: Metadata module | 30 min | 45 min |
| Phase 3: SearchDocument module | 25 min | 70 min |
| Phase 4: Update tools | 30 min | 100 min |
| Phase 5: list_categories | 30 min | 130 min |
| Phase 6: Category filtering | 15 min | 145 min |
| Phase 7: Update tests | 25 min | 170 min |

**Total: ~3 hours** (with buffer for debugging)

---

## Alignment with Feedback

### From music-theory-mcp-feedback.md

**✅ Issue 1: Category Indexing Mismatch**

- Fixed: Read category from frontmatter, not directory structure

**✅ Suggestion 1: Fix Category Indexing (High Priority)**

- Implemented: Category from frontmatter with directory fallback
- Implemented: Separate category and source fields

**✅ Suggestion 2: Add list_categories Tool (High Priority)**

- Implemented: New tool with category counts

**✅ Suggestion 3: Separate category and source Fields (Medium Priority)**

- Implemented: ConceptInfo and SearchResult have both fields

**✅ Suggestion 4: Add Category Parameter to search_concepts (Low Priority)**

- Deferred: Can add later if needed (search already returns category)
- Note: list_concepts has category filtering (more useful for discovery)

**✅ Question 1: Canonical category list**

- Answered: Categories are in frontmatter, not hardcoded
- Server returns actual categories from files

**⚪ Question 2: Hierarchical categories**

- Deferred: Use flat categories for now (simpler)
- Frontmatter can store `category: "counterpoint/species"` if desired
- Can split on `/` later if hierarchy needed

**⚪ Question 3: Multiple categories**

- Deferred: Use single category for now
- Can use tags field for additional categorization
- Future: change category to `Vec<String>` if multi-category needed

---

## Overall Analysis

### Current State: Functional but Incorrect

- Server works, but returns wrong category values
- Directory structure drives categorization (accidental)
- Frontmatter ignored (intentional metadata lost)
- Code duplication (maintenance burden)

### After This Plan: Correct and Extensible

- Frontmatter as single source of truth
- Proper separation of concerns (category vs source)
- No code duplication (DRY)
- Rich metadata available to tools
- Future-proof for Tantivy

### Impact on Full-Text Search

**Short Term (Current Simple Search):**

- More accurate search results (correct categories)
- Better filtering (by category)
- Richer metadata (source, chapter fields)
- Foundation for relevance improvements

**Long Term (Potential Tantivy Migration):**

- Clean domain model (ConceptMetadata)
- Search schema prepared (SearchDocument pattern)
- Faceted search ready (category, source, tags)
- Migration path clear and incremental

**Conclusion:** This plan fixes immediate bugs while setting up proper architecture for future enhancements. It follows Rust best practices and makes the codebase more maintainable and extensible.
