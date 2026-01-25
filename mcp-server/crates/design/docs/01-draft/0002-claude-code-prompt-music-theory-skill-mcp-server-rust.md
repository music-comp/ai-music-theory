---
number: 2
title: "Claude Code Prompt: Music Theory Skill MCP Server (Rust)"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-01-25
updated: 2026-01-25
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Claude Code Prompt: Music Theory Skill MCP Server (Rust)

## Overview

Build an MCP (Model Context Protocol) server in Rust that provides Claude with direct access to the music theory skill materials. This enables the bootstrapping loop: Claude uses the skill to help build more of the skill.

**Tech Stack:**
- `rmcp` — Rust MCP SDK
- `confyg` — Configuration management
- `twyg` — Logging

## Project Location

```
~/lab/music-comp/ai-music-theory/
├── mcp-server/                      # Server code goes here
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── tools/
│   │   │   ├── mod.rs
│   │   │   ├── sources.rs
│   │   │   ├── concepts.rs
│   │   │   ├── guides.rs
│   │   │   └── search.rs
│   │   └── resources/
│   │       └── mod.rs
│   └── config/
│       └── default.toml
│
├── sources-md/                      # Converted source materials
│   └── open-music-theory/           # 123 chapters extracted
│
├── concept-cards/                   # Concept cards (growing)
│   └── open-music-theory/           # Cards extracted from OMT
│
├── concepts-unified/                # (future) Cross-source unified cards
├── guides/                          # (future) Topic guides
│
├── SKILL.md                         # (to create) Entry point
├── CONVENTIONS.md                   # Notation standards
├── SCOPE.md                         # Project scope
├── SOURCES.md                       # Source inventory
├── PIPELINE.md                      # Process documentation
└── INDEX.md                         # (to create) Cross-reference index
```

## MCP Server Requirements

### Tools to Implement

#### 1. `list_sources`
List available source materials (PDFs, EPUBs, converted markdown).

```rust
// Returns structured list of sources with metadata
{
    "sources": [
        {
            "id": "open-music-theory",
            "title": "Open Music Theory",
            "format": "markdown",
            "path": "sources-md/open-music-theory/",
            "chapters": 123,
            "status": "converted"
        },
        {
            "id": "lewin-gmit",
            "title": "Generalized Musical Intervals and Transformations",
            "format": "pdf",
            "path": "~/Dropbox/Apps/Oxford University Press/[2007] Lewin...",
            "status": "not_converted"
        },
        // ... etc
    ]
}
```

#### 2. `get_source_chapter`
Retrieve a specific chapter from a converted source.

Parameters:
- `source_id`: string (e.g., "open-music-theory")
- `chapter`: string (e.g., "01-16" or "01-16-intervals")

Returns: Full markdown content of the chapter.

#### 3. `list_concepts`
List available concept cards, optionally filtered by category.

Parameters:
- `category` (optional): "fundamentals" | "harmony" | "counterpoint" | "form" | "chromaticism" | "post-tonal" | "serial" | "rhythm"
- `source` (optional): filter by source (e.g., "open-music-theory")

Returns: List of concept cards with metadata.

#### 4. `get_concept`
Retrieve a specific concept card.

Parameters:
- `name`: string (e.g., "pitch-class", "interval", "staff")

Returns: Full markdown content of the concept card.

#### 5. `search_concepts`
Search across concept cards for relevant information.

Parameters:
- `query`: string (search terms)
- `limit` (optional): max results (default 10)

Returns: Ranked list of matching concepts with relevance snippets.

#### 6. `get_conventions`
Retrieve notation conventions, optionally filtered by topic.

Parameters:
- `topic` (optional): "pitch" | "intervals" | "sets" | "transformations" | "groups" | "geometric" | "messiaen"

Returns: Relevant section of CONVENTIONS.md.

#### 7. `get_skill_doc`
Retrieve core skill documentation.

Parameters:
- `doc`: "skill" | "scope" | "conventions" | "sources" | "pipeline" | "debates" | "index"

Returns: Full content of the requested document.

#### 8. `list_guides`
List available topic guides.

Returns: List of guides with titles and descriptions.

#### 9. `get_guide`
Retrieve a specific topic guide.

Parameters:
- `name`: string (e.g., "voice-leading-geometry", "pitch-class-sets")

Returns: Full markdown content of the guide.

#### 10. `get_source_pdf_path`
Get the filesystem path to a source PDF/EPUB for use with PDF tools.

Parameters:
- `source_id`: string (e.g., "lewin-gmit", "tymoczko-tonality")

Returns: Full filesystem path to the source file.

### Resources to Expose

The MCP server should also expose resources (for context injection):

1. **`skill://conventions`** — Full CONVENTIONS.md
2. **`skill://scope`** — Full SCOPE.md  
3. **`skill://sources`** — Full SOURCES.md
4. **`skill://index`** — Cross-reference index (when created)

## Configuration (confyg)

Create `config/default.toml`:

```toml
[server]
name = "music-theory-skill"
version = "0.1.0"

[paths]
# Base path for the skill repository
base = "~/lab/music-comp/ai-music-theory"

# Converted sources
sources_md = "${paths.base}/sources-md"

# Concept cards
concept_cards = "${paths.base}/concept-cards"

# Unified concepts (future)
concepts_unified = "${paths.base}/concepts-unified"

# Topic guides (future)
guides = "${paths.base}/guides"

# Core skill documents
skill_docs = "${paths.base}"

[sources]
# Source file locations (original PDFs/EPUBs)
# These are the raw sources before conversion

[sources.oxford]
path = "~/Dropbox/Apps/Oxford University Press"

[sources.oxford.files]
lewin-gmit = "[2007] Lewin - Generalized Musical Intervals and Transformations - Revised Edition.pdf"
tymoczko-geometry = "[2011] Tymoczko - A Geometry of Music - Harmony and Counterpoint in the Extended Common Practice.pdf"
tymoczko-tonality = "[2023] Tymoczko - Tonality - An Owners Manual.epub"
cohn-audacious = "[2012] Cohn - Audacious Euphony - Chromaticism and the Triad's Second Nature.pdf"
gollin-handbook = "[2012] Gollin - The Oxford Handbook of Neo-Riemannian Music Theories.pdf"

[sources.general]
path = "~/Dropbox/Apps/General Books/Music"

[sources.general.files]
open-music-theory = "[2022] Gotham - Open Music Theory.xml"
straus-post-tonal = "[2016] Straus - Introduction to Post-Tonal Theory - 4th Edition.pdf"
wright-math-music = "[2009] Wright - Mathematics and Music.pdf"
hutchinson-21c = "[2023] Hutchinson - Music Theory for the 21st-Century Classroom.epub"
messiaen-technique = "[1944] Messiaen - The Technique of My Musical Language.pdf"

[sources.papers]
path = "~/Dropbox/Apps/Papers/Music Theory"

[sources.papers.files]
papadopoulos = "[2014] Papadopoulos - Mathematics and group theory in music/[2014] Papadopoulos - Mathematics and group theory in music.pdf"
fiore = "[2009] Fiore - Music and Mathematics.pdf"
acef-sanchez = "[2012] Acef-Sanchez - An Introduction to Group Theory with applications to Mathematical Music Theory.pdf"

[logging]
level = "info"
```

## Implementation Notes

### Search Implementation

For `search_concepts`, implement a simple but effective search:

1. **Index on startup**: Load all concept cards, extract searchable text
2. **Simple ranking**: TF-IDF or even simpler keyword matching
3. **Return snippets**: Show context around matches

Future enhancement: Use a proper search library like `tantivy`.

### File Discovery

The server should discover available content dynamically:
- Scan `sources-md/*/` for converted chapters
- Scan `concept-cards/*/` for concept cards
- Scan `guides/` for topic guides
- Check for existence of core docs (CONVENTIONS.md, etc.)

### Error Handling

- Return helpful errors when files not found
- Indicate when sources exist but aren't converted yet
- Suggest next steps (e.g., "Run marker on this PDF to convert")

## Cargo.toml

```toml
[package]
name = "music-theory-mcp"
version = "0.1.0"
edition = "2021"

[dependencies]
rmcp = { version = "0.1", features = ["server", "transport-stdio"] }
confyg = "0.1"
twyg = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
walkdir = "2"
glob = "0.3"
shellexpand = "3"

# For search (optional, can start simpler)
# tantivy = "0.21"
```

## Testing the Server

Once built, test with:

```bash
# Run server
cd ~/lab/music-comp/ai-music-theory/mcp-server
cargo run

# In another terminal, send test requests via stdio
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | cargo run
```

## Claude Desktop Integration

Add to Claude Desktop config (`~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "cargo",
      "args": ["run", "--manifest-path", "/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/Cargo.toml"]
    }
  }
}
```

Or build a release binary:

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/target/release/music-theory-mcp"
    }
  }
}
```

## Success Criteria

The MCP server is working when Claude can:

1. ✅ List available sources and their conversion status
2. ✅ Retrieve any converted chapter by ID
3. ✅ List and retrieve concept cards
4. ✅ Search across concepts for relevant information
5. ✅ Access CONVENTIONS.md when working on notation
6. ✅ Access SOURCES.md to find original file paths
7. ✅ Get PDF paths to use with PDF Tools for unconverted sources

## Future Enhancements (v0.2+)

- Full-text search with tantivy
- Concept relationship graph traversal
- "Find related concepts" tool
- Integration with abstract-algebra-topology skill (when built)
- Caching for faster startup
- Watch mode for auto-reload when files change

## Let's Build!

Start with:
1. Create the project structure (`cargo new music-theory-mcp`)
2. Set up confyg configuration
3. Implement `list_sources` and `get_source_chapter` first
4. Add `list_concepts` and `get_concept`
5. Implement `search_concepts`
6. Add the skill doc tools
7. Test with Claude Desktop

Focus on getting a working v0.1 that Claude can use immediately, then iterate.
