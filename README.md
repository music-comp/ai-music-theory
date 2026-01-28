# AI Music Theory Skill

[![][build-badge]][build]
[![][tag-badge]][tag]

[![][logo]][logo-large]

*A comprehensive music theory knowledge base designed for AI assistants, covering Western music theory from fundamentals through advanced mathematical and transformational approaches*

## Overview

This skill provides structured access to music theory concepts spanning:

- **Fundamentals** — notation, scales, intervals, chords, rhythm
- **Harmony** — diatonic progressions, tonicization, modulation
- **Counterpoint** — species counterpoint, voice leading, fugue
- **Form** — phrase structure, binary, ternary, sonata, rondo
- **Chromaticism** — modal mixture, augmented sixths, neo-Riemannian theory
- **Post-tonal** — pitch-class sets, twelve-tone technique, collections
- **Mathematical foundations** — group theory, geometric voice-leading spaces

### Intellectual Lineage

The skill synthesizes insights from:

- **David Lewin** — Generalized intervals and transformations
- **Dmitri Tymoczko** — Geometric voice-leading theory
- **Richard Cohn** — Neo-Riemannian theory and hexatonic systems
- **Joseph Straus** — Post-tonal theory pedagogy
- **Open Music Theory** — Comprehensive open-source textbook

## Project Structure

```
ai-music-theory/
├── concept-cards/           # Atomic concept definitions
│   └── open-music-theory/   # 76 cards from OMT
├── sources-md/              # Converted source materials
│   └── open-music-theory/   # 123 chapters
├── guides/                  # Topic deep-dives (coming soon)
├── mcp-server/              # MCP server for AI access
├── SKILL.md                 # Entry point for AI assistants
├── CONVENTIONS.md           # Notation standards
├── SCOPE.md                 # Coverage and boundaries
├── SOURCES.md               # Bibliography and priorities
└── PIPELINE.md              # Content creation workflow
```

## MCP Server

The `mcp-server/` directory contains a Rust implementation of an MCP (Model Context Protocol) server that provides AI assistants with direct access to the skill materials.

### Features

- **Source browsing** — List and retrieve converted textbook chapters
- **Concept lookup** — Access atomic concept cards by ID
- **Full-text search** — Find relevant concepts by query (with Tantivy backend)
- **Graph database** — Explore concept relationships and learning paths
- **PDF path resolution** — Locate original source files for deeper reading
- **Convention access** — Retrieve notation standards on demand

### Available Tools (25 total)

**Content Access (10 tools):**

| Tool | Description |
|------|-------------|
| `list_sources` | List all source materials with conversion status |
| `get_source_chapter` | Retrieve a specific chapter from a source |
| `get_source_pdf_path` | Get filesystem path to source PDF/EPUB |
| `list_concepts` | List concept cards (optionally by category) |
| `list_categories` | List all concept categories with counts |
| `get_concept` | Retrieve a specific concept card |
| `search_concepts` | Full-text search across concepts with ranking |
| `list_guides` | List available topic guides |
| `get_guide` | Retrieve a specific topic guide |
| `health` | Get server health and search backend status |

**Graph Database (15 tools, requires `--features graph`):**

*Inspection:*

| Tool | Description |
|------|-------------|
| `graph_status` | Get graph loading state and statistics |
| `graph_stats` | Detailed graph statistics (categories, relationships) |
| `graph_validate` | Check graph integrity (orphans, self-loops) |
| `get_node` | Get node information by ID |
| `get_node_edges` | Get all edges for a node with direction filtering |

*Relationship Exploration:*

| Tool | Description |
|------|-------------|
| `get_related_concepts` | Find related concepts with relationship filtering |
| `find_concept_path` | Find shortest path between concepts |
| `get_prerequisites` | Get prerequisites in topological learning order |
| `get_concept_neighborhood` | Get local subgraph around a concept |
| `get_dependents` | Find concepts that depend on this as prerequisite |
| `get_central_concepts` | Find most connected concepts by degree centrality |
| `get_concept_sources` | Get all sources that introduce/cover a concept |
| `get_concept_variants` | Get source-specific variants of canonical concept |
| `find_bridge_concepts` | Find concepts bridging two categories |
| `get_source_coverage` | Get all concepts introduced/covered by a source |

### Building

```bash
cd mcp-server

# Minimal build (10 content tools only)
cargo build --release

# With full-text search (adds Tantivy backend)
cargo build --release --features fts

# With graph database (adds 15 relationship tools)
cargo build --release --features graph

# Full build - recommended (all 25 tools)
cargo build --release --features fts,graph
```

See [`mcp-server/README.md`](mcp-server/crates/server/README.md) for complete documentation on features, CLI tools, and configuration.

### Claude Desktop Integration

**1. Build the server with all features:**

```bash
cd mcp-server
cargo build --release --features fts,graph

# Initialize FTS index (optional but recommended)
./target/release/music-theory-mcp index

# Initialize graph database (if using graph feature)
./target/release/music-theory-mcp graph build
```

**2. Configure Claude Desktop:**

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "/path/to/ai-music-theory/mcp-server/target/release/music-theory-mcp"
    }
  }
}
```

**3. Restart Claude Desktop** and start exploring music theory!

See the [MCP server documentation](mcp-server/crates/server/README.md) for detailed setup, configuration options, and usage examples.

## Content Status

### Available Now

- ✅ **Open Music Theory** — 123 chapters converted, 200+ concept cards
- ✅ **Core documentation** — CONVENTIONS.md, SCOPE.md, SOURCES.md
- ✅ **MCP Server** — 25 tools (content + search + graph database)
- ✅ **Full-text search** — Tantivy backend with advanced query support
- ✅ **Graph database** — Concept relationships and learning path exploration

### In Progress

- 🔄 Additional concept cards from OMT (covering Parts V-X)
- 🔄 Topic guides for key subjects

### Planned

- 📋 Lewin GMIT conversion and concept extraction
- 📋 Tymoczko Geometry/Tonality integration
- 📋 Straus Post-Tonal Theory coverage
- 📋 Unified cross-source concept cards

## Notation Conventions

The skill uses consistent notation (see `CONVENTIONS.md`):

- **Pitch classes**: Integers 0-11 (C=0)
- **Pitch with octave**: Scientific notation (C4 = middle C)
- **Intervals**: Ordered i(a,b), unordered ic(a,b)
- **Transformations**: Tₙ (transposition), Iₙ (inversion), P/R/L (neo-Riemannian)
- **Sets**: Curly braces {0, 4, 7}

## License

Content derived from Open Music Theory is CC-BY-SA. Original concept cards and tooling are available under the same license.

MCP server is covered by the MIT license.

[//]: ---Named-Links---

[logo]: https://raw.githubusercontent.com/music-comp/ai-music-theory/main/assets/images/logo/v1-y250.png
[logo-large]: https://raw.githubusercontent.com/music-comp/ai-music-theory/main/assets/images/logo/v1.png
[build]: https://github.com/music-comp/ai-music-theory/actions/workflows/cicd.yml
[build-badge]: https://github.com/music-comp/ai-music-theory/actions/workflows/cicd.yml/badge.svg
[tag-badge]: https://img.shields.io/github/tag/music-comp/ai-music-theory.svg
[tag]: https://github.com/music-comp/ai-music-theory/tags
