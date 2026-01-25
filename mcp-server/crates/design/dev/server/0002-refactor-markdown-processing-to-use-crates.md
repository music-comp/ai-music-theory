# Refactor Markdown Processing to replace bespoke impls with crates

**Goal:** Replace all bespoke markdown parsing with professional-grade libraries for better maintainability, correctness, and features.

**Date:** 2026-01-25
**Estimated Effort:** 3-4 hours
**Complexity:** Medium

## Executive Summary

The codebase currently has **duplicate, fragile markdown parsing logic** across 4 files:

- `tools/concepts.rs` - Manual frontmatter + header parsing
- `tools/guides.rs` - Manual frontmatter + header parsing (duplicate code)
- `tools/search.rs` - Manual frontmatter + header parsing (duplicate code)
- `tools/sources.rs` - Filename-based title extraction (no markdown parsing)

**New Dependencies Added:**

- `pulldown-cmark = "0.9"` - The markdown parser used by rustdoc
- `serde_yaml = "0.9"` - YAML deserialization for frontmatter

**Key Benefits:**

1. **DRY**: Eliminate 180+ lines of duplicate frontmatter parsing code
2. **Correctness**: Replace fragile string manipulation with proper AST-based parsing
3. **Features**: Unlock full markdown capabilities (heading levels, text extraction, HTML rendering)
4. **Maintainability**: Industry-standard libraries vs custom parsing logic

---

## Current State Analysis

### Locations with Manual Markdown Parsing

#### 1. `crates/server/src/tools/concepts.rs`

**Function:** `extract_title_and_preview()` (lines 127-189)

**Current Issues:**

- Manual frontmatter state machine with `in_frontmatter` boolean
- Hardcoded `---` delimiter detection
- Uses `.strip_prefix("title:")` for YAML parsing
- Uses `.starts_with('#')` and `.trim_start_matches('#')` for headers
- Ignores heading levels - treats all `#`, `##`, `###` the same
- Preview extraction limited to 200 chars via character counting
- Takes only first 20 lines - arbitrary limit

**Code Pattern:**

```rust
let mut in_frontmatter = false;
let mut frontmatter_count = 0;

for line in content.lines().take(20) {
    if line.trim() == "---" {
        frontmatter_count += 1;
        if frontmatter_count == 1 {
            in_frontmatter = true;
            continue;
        } else if frontmatter_count == 2 {
            in_frontmatter = false;
            continue;
        }
    }

    if in_frontmatter {
        if let Some(stripped) = line.strip_prefix("title:") {
            title = stripped.trim().trim_matches('"').to_string();
        }
        continue;
    }

    if title.is_empty() && line.starts_with('#') {
        title = line.trim_start_matches('#').trim().to_string();
        continue;
    }
    // ... preview extraction
}
```

---

#### 2. `crates/server/src/tools/guides.rs`

**Function:** `extract_title_and_description()` (lines 97-168)

**Current Issues:**

- **DUPLICATE CODE**: Nearly identical to concepts.rs
- Same manual frontmatter parsing with state machine
- Extracts both `title:` and `description:` from YAML
- Same header parsing with `.starts_with('#')`
- Description extraction limited to 300 chars
- Takes only first 30 lines - different arbitrary limit than concepts.rs

**Code Pattern:**

```rust
// EXACT SAME pattern as concepts.rs with minor variations
let mut in_frontmatter = false;
let mut frontmatter_count = 0;
let mut found_title = false;

for line in content.lines().take(30) {  // Different limit!
    if line.trim() == "---" {
        // ... identical logic
    }

    if in_frontmatter {
        if let Some(stripped) = line.strip_prefix("title:") {
            title = stripped.trim().trim_matches('"').to_string();
            found_title = true;
        } else if let Some(stripped) = line.strip_prefix("description:") {
            description = stripped.trim().trim_matches('"').to_string();
        }
        continue;
    }
    // ... more duplicate code
}
```

---

#### 3. `crates/server/src/tools/search.rs`

**Function:** `extract_metadata()` (lines 131-164)

**Current Issues:**

- **TRIPLE DUPLICATE CODE**: Third copy of frontmatter parsing
- Uses hardcoded index `line[6..]` to extract YAML value (assumes "title:" is 6 chars)
- **UNSAFE**: Will panic if line is shorter than 6 chars
- Same header parsing logic
- Only extracts title, not other metadata
- Takes only first 20 lines

**Code Pattern:**

```rust
let mut in_frontmatter = false;
let mut frontmatter_count = 0;

for line in content.lines().take(20) {
    if line.trim() == "---" {
        // ... identical state machine
    }

    if in_frontmatter && line.starts_with("title:") {
        title = line[6..].trim().trim_matches('"').to_string();  // UNSAFE!
        continue;
    }

    if !in_frontmatter && title.is_empty() && line.starts_with('#') {
        // ... same header extraction
    }
}
```

**Additional Context:** This file also has:

- `extract_snippet()` - Could benefit from proper markdown parsing for better context
- `calculate_relevance()` - Simple word counting, could use markdown structure for better scoring

---

#### 4. `crates/server/src/tools/sources.rs`

**Function:** `extract_title()` (lines 183-196)

**Current State:**

- Simple filename parsing: removes `[YEAR]` prefix and file extension
- No actual markdown parsing
- Could be enhanced to read title from file frontmatter/headers

---

### Problems with Current Approach

**Code Duplication (AP-02):**

- 3 separate implementations of frontmatter parsing (~60 lines each)
- 180+ lines of duplicate code
- Different bugs in each copy (search.rs has unsafe indexing)

**Fragility:**

- String-based parsing breaks on edge cases
- No proper YAML validation
- Assumes specific formatting (quotes, spacing)
- Hardcoded delimiters and field names

**Limited Features:**

- Can't extract heading hierarchy
- Can't process markdown formatting (bold, links, lists)
- No HTML rendering capability
- No proper text extraction from complex structures

**Maintainability:**

- Adding new frontmatter fields requires changes in 3 places
- No type safety for metadata
- Error messages are generic ("parse error")

---

## Proposed Architecture

### New Module: `crates/server/src/markdown/mod.rs`

Create a centralized markdown utilities module with:

```rust
mod frontmatter;
mod parser;

pub use frontmatter::{extract_frontmatter, Frontmatter};
pub use parser::{
    extract_first_heading,
    extract_first_paragraph,
    extract_text_content,
    MarkdownParser,
};
```

---

### Submodule 1: `frontmatter.rs`

**Purpose:** Parse YAML frontmatter using serde_yaml

```rust
use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Standard frontmatter structure for markdown files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

/// Extract frontmatter from markdown content
pub fn extract_frontmatter(content: &str) -> Result<(Option<Frontmatter>, &str)> {
    // Split on --- delimiters
    // Parse YAML section with serde_yaml
    // Return (frontmatter, remaining_markdown)
}

/// Extract just the body content (everything after frontmatter)
pub fn strip_frontmatter(content: &str) -> &str {
    // Return content without frontmatter block
}
```

**Key Functions:**

1. **`extract_frontmatter(content: &str) -> Result<(Option<Frontmatter>, &str)>`**
   - Splits content on `---` delimiters
   - Uses `serde_yaml::from_str()` to deserialize frontmatter
   - Returns both frontmatter struct and remaining content
   - Handles missing frontmatter gracefully (returns None)

2. **`strip_frontmatter(content: &str) -> &str`**
   - Returns markdown body without frontmatter
   - Used when we only need the content, not metadata

**Benefits:**

- Type-safe metadata extraction
- Proper YAML parsing with validation
- Extensible: add new fields by updating struct
- Error handling with proper error messages
- Reusable across all tools

---

### Submodule 2: `parser.rs`

**Purpose:** Extract content from markdown using pulldown-cmark

```rust
use pulldown_cmark::{Event, Parser, Tag, TagEnd, HeadingLevel};
use crate::error::Result;

/// Extract the first heading from markdown content
pub fn extract_first_heading(content: &str) -> Option<(HeadingLevel, String)> {
    // Parse markdown into events
    // Find first Heading tag
    // Collect text content and return with level
}

/// Extract the first paragraph after headings
pub fn extract_first_paragraph(content: &str, max_chars: usize) -> Option<String> {
    // Parse markdown into events
    // Skip headers, find first paragraph
    // Collect text up to max_chars
}

/// Extract all text content (strip markdown formatting)
pub fn extract_text_content(content: &str) -> String {
    // Parse markdown
    // Collect all Text events
    // Return plain text
}

/// Extract snippet around a search query match
pub fn extract_snippet(content: &str, query: &str, context_chars: usize) -> Option<String> {
    // Parse markdown to plain text
    // Find query position
    // Extract context before/after
}
```

**Key Functions:**

1. **`extract_first_heading(content: &str) -> Option<(HeadingLevel, String)>`**
   - Uses `Parser::new(content)` to create event iterator
   - Pattern matches on `Event::Start(Tag::Heading { level, .. })`
   - Collects following `Event::Text` events until `Event::End(TagEnd::Heading(level))`
   - Returns heading text and level (H1, H2, etc.)

2. **`extract_first_paragraph(content: &str, max_chars: usize) -> Option<String>`**
   - Skips heading events
   - Finds first `Event::Start(Tag::Paragraph)`
   - Collects text events until `Event::End(TagEnd::Paragraph)` or max_chars reached
   - Properly handles inline formatting (preserves text from bold, italic, links)

3. **`extract_text_content(content: &str) -> String`**
   - Iterates all events, collecting only `Event::Text` and `Event::Code`
   - Strips all formatting, returns plain text
   - Useful for search and indexing

**Benefits:**

- Proper markdown AST parsing
- Handles all markdown syntax correctly
- Extracts heading hierarchy information
- Respects inline formatting in text extraction
- Can be extended for HTML rendering, TOC generation, etc.

---

## Implementation Plan

### Phase 1: Create Markdown Module (NEW CODE)

**Estimated Time:** 60 minutes

#### Step 1.1: Add Dependencies to Binary Crate

**File:** `crates/server/Cargo.toml`

**Add:**

```toml
[dependencies]
# ... existing deps ...
pulldown-cmark = { workspace = true }
serde_yaml = { workspace = true }
```

#### Step 1.2: Create Module Structure

**Files to Create:**

```
crates/server/src/
  markdown/
    mod.rs          (module declaration and re-exports)
    frontmatter.rs  (YAML frontmatter parsing)
    parser.rs       (pulldown-cmark utilities)
```

#### Step 1.3: Implement `frontmatter.rs`

**Key Implementation Details:**

```rust
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

pub fn extract_frontmatter(content: &str) -> Result<(Option<Frontmatter>, &str)> {
    // Check if content starts with ---
    if !content.trim_start().starts_with("---") {
        return Ok((None, content));
    }

    // Find frontmatter boundaries
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        // No valid frontmatter
        return Ok((None, content));
    }

    // parts[0] is empty (before first ---), parts[1] is YAML, parts[2] is body
    let yaml_text = parts[1].trim();
    let body = parts[2];

    // Parse YAML
    let frontmatter = if yaml_text.is_empty() {
        None
    } else {
        match serde_yaml::from_str::<Frontmatter>(yaml_text) {
            Ok(fm) => Some(fm),
            Err(e) => {
                // Log warning but don't fail - treat as no frontmatter
                log::warn!("Failed to parse frontmatter: {}", e);
                None
            }
        }
    };

    Ok((frontmatter, body))
}

pub fn strip_frontmatter(content: &str) -> &str {
    match extract_frontmatter(content) {
        Ok((_, body)) => body,
        Err(_) => content,
    }
}
```

**Testing Strategy:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = r#"---
title: "Test Title"
description: "Test Description"
tags: ["tag1", "tag2"]
---
# Heading

Content here"#;

        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Test Title".to_string()));
        assert!(body.contains("# Heading"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\n\nSome content";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    #[test]
    fn test_invalid_yaml() {
        let content = "---\ninvalid: yaml: structure\n---\nBody";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none()); // Gracefully handles invalid YAML
        assert!(body.contains("Body"));
    }
}
```

---

#### Step 1.4: Implement `parser.rs`

**Key Implementation Details:**

```rust
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

/// Extract first heading with its level
pub fn extract_first_heading(content: &str) -> Option<(HeadingLevel, String)> {
    let parser = Parser::new(content);

    let mut in_heading = false;
    let mut heading_level = None;
    let mut heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = Some(level);
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(&text);
            }
            Event::End(TagEnd::Heading(_)) if in_heading => {
                return heading_level.map(|level| (level, heading_text));
            }
            _ => {}
        }
    }

    None
}

/// Extract first paragraph, up to max_chars
pub fn extract_first_paragraph(content: &str, max_chars: usize) -> Option<String> {
    let parser = Parser::new(content);

    let mut in_paragraph = false;
    let mut paragraph_text = String::new();
    let mut skip_headings = true;

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) if !skip_headings => {
                in_paragraph = true;
            }
            Event::Text(text) | Event::Code(text) if in_paragraph => {
                // Add text respecting max_chars
                let remaining = max_chars.saturating_sub(paragraph_text.len());
                if remaining > 0 {
                    let to_add = if text.len() > remaining {
                        &text[..remaining]
                    } else {
                        &text
                    };
                    paragraph_text.push_str(to_add);
                    paragraph_text.push(' ');
                }
            }
            Event::End(TagEnd::Paragraph) if in_paragraph => {
                return Some(paragraph_text.trim().to_string());
            }
            Event::End(TagEnd::Heading(_)) => {
                skip_headings = false; // Start looking for paragraphs after first heading
            }
            _ => {}
        }
    }

    if paragraph_text.is_empty() {
        None
    } else {
        Some(paragraph_text.trim().to_string())
    }
}

/// Extract all text content (plain text, no formatting)
pub fn extract_text_content(content: &str) -> String {
    let parser = Parser::new(content);
    let mut text = String::new();

    for event in parser {
        if let Event::Text(t) | Event::Code(t) = event {
            text.push_str(&t);
            text.push(' ');
        }
    }

    text.trim().to_string()
}
```

**Testing Strategy:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::HeadingLevel;

    #[test]
    fn test_extract_first_heading() {
        let content = "# Main Title\n\n## Subtitle\n\nContent";
        let (level, text) = extract_first_heading(content).unwrap();
        assert_eq!(level, HeadingLevel::H1);
        assert_eq!(text, "Main Title");
    }

    #[test]
    fn test_extract_first_paragraph() {
        let content = "# Title\n\nThis is the first paragraph.\n\nSecond paragraph.";
        let para = extract_first_paragraph(content, 200).unwrap();
        assert_eq!(para, "This is the first paragraph.");
    }

    #[test]
    fn test_extract_text_content() {
        let content = "# Heading\n\n**Bold** and *italic* text with [link](url).";
        let text = extract_text_content(content);
        assert!(text.contains("Bold"));
        assert!(text.contains("italic"));
        assert!(!text.contains("**"));
        assert!(!text.contains("["));
    }

    #[test]
    fn test_max_chars_respected() {
        let content = "# Title\n\nThis is a very long paragraph that should be truncated.";
        let para = extract_first_paragraph(content, 20).unwrap();
        assert!(para.len() <= 20);
    }
}
```

---

#### Step 1.5: Create Module Declaration

**File:** `crates/server/src/markdown/mod.rs`

```rust
//! Markdown parsing utilities using pulldown-cmark and serde_yaml.
//!
//! This module provides centralized markdown processing functionality:
//! - YAML frontmatter extraction and deserialization
//! - Heading and paragraph extraction
//! - Text content extraction for search and indexing
//!
//! All markdown files in the server use these utilities for consistency.

mod frontmatter;
mod parser;

pub use frontmatter::{extract_frontmatter, strip_frontmatter, Frontmatter};
pub use parser::{extract_first_heading, extract_first_paragraph, extract_text_content};
```

---

### Phase 2: Refactor `concepts.rs` (MODIFY EXISTING)

**Estimated Time:** 30 minutes

#### Step 2.1: Add Markdown Module Import

**File:** `crates/server/src/tools/concepts.rs`

**Add at top:**

```rust
use crate::markdown::{extract_frontmatter, extract_first_heading, extract_first_paragraph};
```

#### Step 2.2: Replace `extract_title_and_preview()`

**Lines to Replace:** 127-189 (entire function)

**New Implementation:**

```rust
fn extract_title_and_preview(path: &Path) -> Result<(String, Option<String>)> {
    let content = fs::read_to_string(path)?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Get title from frontmatter or first heading
    let title = frontmatter
        .and_then(|fm| fm.title)
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| {
            // Fallback to filename
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .replace('-', " ")
        });

    // Extract first paragraph as preview
    let preview = extract_first_paragraph(body, 200);

    Ok((title, preview))
}
```

**Changes:**

- **Before:** 62 lines of manual parsing
- **After:** 20 lines using markdown utilities
- **Eliminated:** Frontmatter state machine, manual header parsing, character counting
- **Improved:** Proper YAML parsing, correct heading extraction, better text extraction

---

### Phase 3: Refactor `guides.rs` (MODIFY EXISTING)

**Estimated Time:** 30 minutes

#### Step 3.1: Add Markdown Module Import

**File:** `crates/server/src/tools/guides.rs`

**Add at top:**

```rust
use crate::markdown::{extract_frontmatter, extract_first_heading, extract_first_paragraph};
```

#### Step 3.2: Replace `extract_title_and_description()`

**Lines to Replace:** 97-168 (entire function)

**New Implementation:**

```rust
fn extract_title_and_description(path: &Path) -> Result<(String, Option<String>)> {
    let content = fs::read_to_string(path)?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Get title from frontmatter or first heading
    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| {
            // Fallback to filename
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled Guide")
                .replace(['-', '_'], " ")
        });

    // Get description from frontmatter or first paragraph
    let description = frontmatter
        .and_then(|fm| fm.description)
        .or_else(|| extract_first_paragraph(body, 300));

    Ok((title, description))
}
```

**Changes:**

- **Before:** 71 lines of manual parsing
- **After:** 25 lines using markdown utilities
- **Eliminated:** Duplicate frontmatter parsing, manual header extraction
- **Improved:** Can now get description from frontmatter OR paragraph

---

### Phase 4: Refactor `search.rs` (MODIFY EXISTING)

**Estimated Time:** 30 minutes

#### Step 4.1: Add Markdown Module Import

**File:** `crates/server/src/tools/search.rs`

**Add at top:**

```rust
use crate::markdown::{extract_frontmatter, extract_first_heading};
```

#### Step 4.2: Replace `extract_metadata()`

**Lines to Replace:** 131-164 (entire function)

**New Implementation:**

```rust
fn extract_metadata(content: &str) -> (String, Option<String>) {
    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(content).unwrap_or((None, content));

    // Get title from frontmatter or first heading
    let title = frontmatter
        .and_then(|fm| fm.title)
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| "Untitled".to_string());

    (title, None)
}
```

**Changes:**

- **Before:** 33 lines with UNSAFE indexing `line[6..]`
- **After:** 12 lines using markdown utilities
- **Fixed:** Removed unsafe hardcoded index that could panic
- **Improved:** Proper YAML parsing, better error handling

**Note:** The second tuple element (Option<String>) is currently unused in this function. Could be removed or used for description in future.

---

### Phase 5: Add Markdown Module to Server (WIRE IT UP)

**Estimated Time:** 10 minutes

#### Step 5.1: Declare Markdown Module

**File:** `crates/server/src/main.rs` or `crates/server/src/lib.rs`

**Add:**

```rust
mod markdown;  // Add this line
```

**Context:**
The server binary needs to know about the new markdown module. Add this declaration where other modules are declared (config, error, resources, server, tools).

---

### Phase 6: Testing and Validation

**Estimated Time:** 30 minutes

#### Step 6.1: Unit Tests

**Run all tests:**

```bash
make test
```

**Expected Results:**

- All 16 existing tests should pass
- 6+ new tests in markdown module should pass
- Total: ~22 tests passing

#### Step 6.2: Manual Testing

**Test markdown files with various formats:**

1. **Test frontmatter parsing:**

   ```bash
   # Create test file
   cat > /tmp/test-frontmatter.md <<'EOF'
   ---
   title: "Test Concept"
   description: "Test description"
   tags: ["harmony", "theory"]
   ---
   # Fallback Title

   First paragraph of content.
   EOF

   # Copy to concepts directory and test
   ```

2. **Test header fallback:**

   ```bash
   # Create test file without frontmatter
   cat > /tmp/test-header.md <<'EOF'
   # Main Heading

   Content here.
   EOF
   ```

3. **Test filename fallback:**

   ```bash
   # Create test file without frontmatter or headers
   echo "Just plain content" > /tmp/test-plain.md
   ```

#### Step 6.3: Integration Tests

**Test through MCP tools:**

```bash
# Start server
cargo run

# In MCP client:
# 1. list_concepts - should return concepts with proper titles
# 2. get_concept("harmony-basics") - should return concept
# 3. search_concepts("interval") - should find and rank results
# 4. list_guides - should return guides with descriptions
# 5. get_guide("neo-riemannian") - should return guide
```

**Verify:**

- Titles are extracted correctly from frontmatter
- Fallback to headers works when no frontmatter
- Previews/descriptions are meaningful
- Search results have proper snippets
- No panics or errors

#### Step 6.4: Lint Check

```bash
make lint
```

**Expected:**

- No clippy warnings
- All code formatted correctly

---

## Critical Files Modified

| File | Lines Changed | Purpose |
|------|--------------|---------|
| `crates/server/Cargo.toml` | +2 | Add pulldown-cmark and serde_yaml |
| `crates/server/src/markdown/mod.rs` | +15 (new) | Module declaration |
| `crates/server/src/markdown/frontmatter.rs` | +80 (new) | YAML frontmatter parsing |
| `crates/server/src/markdown/parser.rs` | +120 (new) | pulldown-cmark utilities |
| `crates/server/src/tools/concepts.rs` | -42, +20 | Use markdown module |
| `crates/server/src/tools/guides.rs` | -51, +25 | Use markdown module |
| `crates/server/src/tools/search.rs` | -21, +12 | Use markdown module |
| `crates/server/src/main.rs` or `lib.rs` | +1 | Declare markdown module |

**Total:**

- **New code:** ~215 lines (reusable utilities with tests)
- **Deleted code:** ~114 lines (duplicate parsing logic)
- **Net change:** +101 lines (but massive improvement in quality)

**Code Duplication Eliminated:** ~180 lines of duplicate frontmatter parsing

---

## Success Criteria

**Phase 1 (Markdown Module):**

- ✅ `frontmatter.rs` parses YAML correctly
- ✅ `parser.rs` extracts headings and paragraphs
- ✅ All unit tests pass (6+ new tests)
- ✅ No clippy warnings

**Phase 2 (Concepts Refactor):**

- ✅ `extract_title_and_preview()` uses markdown module
- ✅ Existing tests pass
- ✅ Manual testing with various concept files works

**Phase 3 (Guides Refactor):**

- ✅ `extract_title_and_description()` uses markdown module
- ✅ Can extract description from frontmatter OR paragraph
- ✅ Existing tests pass

**Phase 4 (Search Refactor):**

- ✅ `extract_metadata()` uses markdown module
- ✅ Unsafe indexing removed
- ✅ Search functionality unchanged

**Phase 5 (Integration):**

- ✅ Module properly declared in lib/main
- ✅ All imports resolve correctly

**Phase 6 (Testing):**

- ✅ All tests pass (~22 total)
- ✅ MCP tools work correctly
- ✅ No regressions in functionality
- ✅ Clippy clean

---

## Benefits Summary

**Before:**

- 180+ lines of duplicate frontmatter parsing
- 3 different implementations with different bugs
- Unsafe indexing in search.rs
- No type safety for metadata
- Limited to title extraction
- Fragile string-based parsing

**After:**

- Single source of truth for markdown parsing
- Type-safe frontmatter with serde
- Proper AST-based parsing with pulldown-cmark
- Extensible for new features (HTML rendering, TOC, etc.)
- Industry-standard libraries
- Better error handling
- Foundation for future improvements

---

## Future Enhancements (Out of Scope)

**With pulldown-cmark foundation, we can easily add:**

1. **HTML Rendering:**

   ```rust
   pub fn markdown_to_html(content: &str) -> String {
       use pulldown_cmark::html::push_html;
       let parser = Parser::new(content);
       let mut html_output = String::new();
       push_html(&mut html_output, parser);
       html_output
   }
   ```

2. **Table of Contents Generation:**
   - Extract all headings with levels
   - Build hierarchical TOC

3. **Structured Search:**
   - Use heading hierarchy for better relevance
   - Search within specific sections
   - Weight matches by heading level

4. **Link Extraction:**
   - Find all markdown links
   - Validate internal links
   - Build graph of related content

5. **Syntax Highlighting:**
   - Extract code blocks with language tags
   - Render with syntax highlighting

6. **Extended Frontmatter:**
   - Add more metadata fields
   - Support custom frontmatter schemas per tool
   - Validation rules

---

## Rollback Plan

If issues arise:

1. **Git Restore:**

   ```bash
   git restore crates/server/src/tools/concepts.rs
   git restore crates/server/src/tools/guides.rs
   git restore crates/server/src/tools/search.rs
   rm -rf crates/server/src/markdown/
   ```

2. **Staged Commits:**
   - Commit Phase 1 (markdown module) first
   - Commit each refactor separately
   - Easy to revert individual changes

3. **Feature Flag (if needed):**
   - Could add `use_new_parser` config option
   - Fall back to old parsing if new parser has issues

---

## Dependencies

**Existing:**

- `serde = "1"` with derive features ✅
- `serde_json = "1"` ✅

**New (already added to workspace):**

- `pulldown-cmark = "0.9"` ✅ (Added to workspace Cargo.toml)
- `serde_yaml = "0.9"` ✅ (Added to workspace Cargo.toml)

**Just need to add to server crate Cargo.toml:**

```toml
pulldown-cmark = { workspace = true }
serde_yaml = { workspace = true }
```

---

## Timeline

**Total Estimated Time:** 3-4 hours

| Phase | Time | Description |
|-------|------|-------------|
| Phase 1 | 60 min | Create markdown module with tests |
| Phase 2 | 30 min | Refactor concepts.rs |
| Phase 3 | 30 min | Refactor guides.rs |
| Phase 4 | 30 min | Refactor search.rs |
| Phase 5 | 10 min | Wire up module |
| Phase 6 | 30 min | Testing and validation |

**Buffer:** 30 minutes for unexpected issues

---

## Notes

**Important Considerations:**

1. **Backwards Compatibility:**
   - Existing markdown files should work without changes
   - Gracefully handle missing frontmatter
   - Gracefully handle invalid YAML
   - Maintain existing title/description extraction behavior

2. **Error Handling:**
   - Don't fail on parse errors - log warnings
   - Fall back to previous behavior when parsing fails
   - Return sensible defaults (empty strings, None values)

3. **Performance:**
   - pulldown-cmark is very fast (used by rustdoc)
   - serde_yaml is production-grade
   - Should be faster than current line-by-line parsing
   - Consider caching parsed results if needed later

4. **Testing:**
   - Test with real concept cards
   - Test with edge cases (empty files, malformed YAML)
   - Test with various markdown styles
   - Ensure search relevance is maintained or improved

---

## Questions for User

None - the plan is comprehensive and self-contained. Ready to proceed with implementation.
