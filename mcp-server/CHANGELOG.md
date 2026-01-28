# Changelog

All notable changes to the Music Theory AI Skill MCP Server will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-01-27

### Added

#### Universal Content Search & Deep Source Material Access

- **Multi-content-type indexing** - Search across ALL content types:
  - `concept_cards/` - Per-source concept extractions
  - `sources_md/` - Converted source chapters (PDFs/EPUBs → markdown)
  - `concepts_unified/` - Cross-source synthesized concepts
  - `guides/` - AI-optimized topic guides
- **Content type filtering** - Filter search results by document type:
  - `content_types: ["concept_card"]` - Search only concept cards
  - `content_types: ["source_chapter", "guide"]` - Multiple types
  - No filter searches all content types
- **Section/page tracking** - Fine-grained location information:
  - Source chapters: "pp. 23-28" from frontmatter
  - Guides: "Section 2.3" for navigation
- **Per-type statistics** - Index metadata now tracks document counts by type
- **Graceful degradation** - Missing content directories don't cause failures

#### Source Material Diagnostic Tools

- **`check_source_availability` tool** - Check if source is indexed/converted/exists
  - Returns detailed status: `indexed`, `converted`, `file_exists`, or `unavailable`
  - Human-readable messages for graceful error handling
  - Chapter counts for available sources
- **`list_source_chapters` tool** - List all chapters for a source
  - Queries index if available, falls back to filesystem
  - Returns chapter metadata: id, title, section, path
- **Enhanced `get_source_chapter`** - Added optional `section` parameter
  - Better error messages when chapter not found
  - Fine-grained retrieval support
- **Enhanced `health` tool** - Exposes search configuration for transparency
  - Field boost multipliers (title: 3.0x, description: 2.0x, content: 1.0x)
  - Enabled content types
  - Per-type document counts in index stats
  - Search settings (stopwords, fuzzy, query mode)

#### Configuration Enhancements

- **Configurable field boosts** - Tune relevance without code changes:
  - `field_boost_title = 3.0` (default)
  - `field_boost_description = 2.0` (default)
  - `field_boost_content = 1.0` (default)
- **Frontmatter section field** - Added `section` field for page/section tracking

### Changed

- **Schema extended** from 12 to 14 fields:
  - Added `content_type` (STRING | FAST | STORED) for filtering
  - Added `section` (STORED) for fine-grained location
- **Search backend** now indexes all 4 content directories
- **Index metadata** includes per-type document counts
- **Search results** include `content_type` and `section` fields

### Technical Details

- **100% backward compatible** - Existing queries work unchanged
- **409 tests passing** (396 library + 13 integration tests)
- **Minimal schema changes** - Only 2 new fields added
- **Type-specific metadata extraction** - Each content type has optimized extractor
- **Comprehensive test coverage** - ≥95% coverage maintained

## [0.2.0] - 2026-01-27

### Added

#### Full-Text Search with Tantivy
- **Tantivy-based full-text search backend** with BM25 ranking algorithm
- **Multi-field search** across title (3x boost), description (2x boost), and content (1x boost)
- **Multi-word query support** with Smart AND/OR mode:
  - 2-word queries use AND for precision
  - 3+ word queries use OR with 60% minimum match for flexibility
- **Stopword filtering** using industry-standard `stop-words` crate (500+ words)
  - Custom stopword support in configuration
  - Allowlist for domain-specific terms (Roman numerals, solfège syllables)
- **Phrase search support** with quoted strings for exact phrase matching
  - Order-sensitive phrase matching
  - Example: `"perfect authentic cadence"` matches the exact phrase
- **Category filtering parameter** to scope searches to specific categories
  - Example: `{"query": "suspension", "category": "voice-leading"}`
- **Improved snippet generation** with comprehensive fallback chain:
  - Attempts to find query terms in description first
  - Falls back to content if not in description
  - Always returns non-empty snippets with relevant context
  - Handles multi-word queries by searching for individual terms
- **CLI commands for index management**:
  - `music-theory-mcp index` - Build the search index
  - `music-theory-mcp index --force` - Force rebuild index
  - `music-theory-mcp serve` - Start MCP server with Tantivy backend

#### Configuration & Migration
- **Automatic index freshness checking** - detects when content changes and rebuilds index
- **Fuzzy search support** (optional) for typo tolerance
- **Configurable query modes**: Smart (default), AND, OR, Minimum Match
- **Migration guide** (MIGRATION.md) with step-by-step instructions for upgrading from simple search

#### Testing & Quality
- **444+ tests passing** including comprehensive integration tests
- **QA test suite** with real-world query scenarios
- **Category filtering tests** to verify scoped searches
- **Phrase search tests** to verify order sensitivity
- **Performance benchmarks** showing significant improvements over simple search

### Changed
- **Stopword implementation** now uses industry-standard `stop-words` crate (500+ words vs ~100)
- **Search backend architecture** refactored to support pluggable backends
- **Default search backend** changed from "simple" to "tantivy" (configurable)
- **Snippet generation** improved to handle multi-word queries and always return non-empty results

### Improved
- **Search quality** with BM25 ranking vs simple word-counting relevance
- **Search performance** with indexed search vs linear scan
- **Search precision** with stopword filtering and phrase support
- **User experience** with consistent, non-empty snippets and category filtering

### Technical Details
- Uses `tantivy` 0.22 for full-text search indexing and query execution
- Uses `stop-words` 0.8 for comprehensive English stopword lists
- Index stored in `.tantivy-index/` (configurable)
- Automatic index rebuilding when content changes detected
- Backward compatible with simple search backend (remains available as fallback)

### Breaking Changes
- **None** - Version 0.2.0 is fully backward compatible with 0.1.0
- Simple search backend remains available as fallback option for users who don't want FTS
- See MIGRATION.md for migration instructions

### Documentation
- Added comprehensive MIGRATION.md guide
- Updated README.md with Tantivy search documentation
- Added design documents (crates/design/dev/server/) documenting implementation phases
- Updated MCP tool descriptions with new parameters

---

## [0.1.0] - 2025-12-31

### Added
- **Initial MCP server implementation** for music theory educational materials
- **Simple search backend** using linear scan across concept cards
- **Core MCP tools**:
  - `search_concepts` - Search for music theory concepts
  - `get_concept` - Retrieve a specific concept by ID
  - `list_concepts` - List all available concepts by category
  - `get_guide` - Retrieve a topic guide by ID
  - `list_guides` - List all available topic guides
  - `get_health` - Check server health and backend status
- **Concept card system** with frontmatter metadata (title, category, description, source)
- **Category organization** (harmony, rhythm, form, melody, counterpoint, analysis, voice-leading, notation)
- **Multi-source support** with source attribution
- **Configuration system** using TOML with path resolution
- **Logging system** with configurable levels and output
- **Basic snippet generation** for search results
- **Relevance scoring** based on word counts in title, description, and content

### Technical Details
- Built with Rust using `rmcp` SDK
- Async I/O with `tokio` runtime
- Markdown parsing with `pulldown-cmark`
- YAML frontmatter parsing with `serde_yaml`
- Configuration via `confyg` with default.toml

### Documentation
- README.md with setup and usage instructions
- Tool descriptions for MCP client integration
- Inline API documentation

---

## Version Numbering

**Current Version:** 0.2.0

- **0.1.0** - Initial release with simple search
- **0.2.0** - Tantivy full-text search with polish
- **0.3.0** (Planned) - Multilingual support, synonym expansion, fuzzy matching enhancements
- **1.0.0** (Planned) - Production-ready with comprehensive content library

---

## Migration Path

**From 0.1.0 to 0.2.0:**
- See MIGRATION.md for detailed instructions
- Update `backend = "tantivy"` in config/default.toml
- Run `music-theory-mcp index` to build search index
- No code changes required - fully backward compatible

**From simple to tantivy backend:**
- Simple backend remains available as fallback option
- Tantivy recommended for better search quality and performance
- Simple backend suitable for small collections without FTS setup

---

## Links

- [GitHub Repository](https://github.com/oxur/ai-music-theory)
- [Migration Guide](MIGRATION.md)
- [README](README.md)

---

## Notes

- All versions maintain backward compatibility within the same major version
- Breaking changes are clearly documented and follow semantic versioning
- Deprecated features receive advance notice (one minor version minimum)
