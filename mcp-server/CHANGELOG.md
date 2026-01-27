# Changelog

All notable changes to the Music Theory AI Skill MCP Server will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
