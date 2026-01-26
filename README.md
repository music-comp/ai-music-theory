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
- **Full-text search** — Find relevant concepts by query
- **PDF path resolution** — Locate original source files for deeper reading
- **Convention access** — Retrieve notation standards on demand

### Available Tools

| Tool | Description |
|------|-------------|
| `list_sources` | List all source materials with conversion status |
| `get_source_chapter` | Retrieve a specific chapter from a source |
| `list_concepts` | List concept cards (optionally by category) |
| `get_concept` | Retrieve a specific concept card |
| `search_concepts` | Full-text search across concepts |
| `list_guides` | List available topic guides |
| `get_guide` | Retrieve a specific topic guide |
| `get_source_pdf_path` | Get filesystem path to source PDF/EPUB |

### Building

```bash
cd mcp-server
cargo build --release
```

### Claude Desktop Integration

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "/path/to/ai-music-theory/mcp-server/bin/music-theory-mcp"
    }
  }
}
```

## Content Status

### Available Now

- ✅ **Open Music Theory** — 123 chapters converted, 76 concept cards
- ✅ **Core documentation** — CONVENTIONS.md, SCOPE.md, SOURCES.md

### In Progress

- 🔄 Additional concept cards from OMT Parts II-X
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
