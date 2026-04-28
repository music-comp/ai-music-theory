# AI Music Theory

[![][build-badge]][build]
[![][tag-badge]][tag]

[![][logo]][logo-large]

*A comprehensive, machine-readable music theory knowledge base — fourteen
canonical sources synthesised into a single typed concept graph, served to AI
assistants over MCP with full-text, graph-traversal, and semantic search.*

## Overview

This project assembles modern Western music theory into a queryable knowledge
graph that an AI assistant (or any MCP client) can use to verify claims, trace
prerequisites, find cross-source bridges, and reason about even the trickiest
mathematical territory — Lewin's transformations, Tymoczko's voice-leading
geometry, Cohn's neo-Riemannian systems, Caplin's formal functions, Straus's
post-tonal pedagogy.

The knowledge base spans:

- **Fundamentals** — notation, scales, intervals, chords, rhythm, meter
- **Tonal harmony** — diatonic progressions, tonicization, modulation, schemata
- **Counterpoint & voice leading** — species, fugue, parsimonious motion
- **Form** — phrase structure, cadences, sonata, rondo, formal functions
- **Chromaticism** — modal mixture, augmented sixths, neo-Riemannian theory
- **Post-tonal & set theory** — pitch-class sets, twelve-tone, collections
- **Transformational theory** — GIS, transformation networks, K-nets
- **Geometric voice leading** — orbifolds, chord spaces, the *extended common practice*
- **Open Tone Harmony (OTH)** — the [6, 8] metric space of 228 quintal chords
- **Mathematical foundations** — group theory, modular arithmetic, tuning systems

## What's in the Box

### Scale of the Knowledge Graph

| | |
|---|---|
| **Source texts** | 14 (textbooks, handbooks, and pedagogical references) |
| **Converted chapters** | 432 markdown chapters across all sources |
| **Concept cards on disk** | 4,315 atomic concept cards (v3.1 format) |
| **Canonical graph nodes** | 3,742 |
| **Typed edges** | 18,618 |
| **Average degree** | ~9.95 |
| **Concept categories** | 64+ (harmony, voice-leading, form, sonata-form, …) |

Edges are typed: `prerequisite` (6,471), `relates_to` (9,095), `extends`
(1,615), and `contrasts_with` (1,437). The graph supports prerequisite
ordering, learning-path generation, neighbourhood exploration, bridge
detection between categories, and centrality analysis — all served as
first-class MCP tools.

### Source Library

Every source listed here is converted to markdown, chapter-split, and mined
for v3.1 concept cards with frontmatter relationships. *Card counts are
on-disk; some get merged into canonical nodes during graph build.*

| # | Source | Author / Year | Chapters | Cards |
|---|--------|---------------|----------|-------|
|  1 | *Open Music Theory* | Gotham et al., 2022 | 123 | 435 |
|  2 | *Tonality: An Owner's Manual* | Tymoczko, 2023 | 29 | 244 |
|  3 | *A Geometry of Music* | Tymoczko, 2011 | 24 | 233 |
|  4 | *Generalized Musical Intervals & Transformations* | Lewin, 2007 | 15 | 301 |
|  5 | *Audacious Euphony* | Cohn, 2012 | 14 | 269 |
|  6 | *The Oxford Handbook of Neo-Riemannian Music Theories* | Gollin & Rehding, 2012 | 22 | 178 |
|  7 | *Introduction to Post-Tonal Theory* | Straus, 2016 | 8 | 250 |
|  8 | *Classical Form* | Caplin, 1998 | 26 | 432 |
|  9 | *Analyzing Classical Form* | Caplin, 2013 | 28 | 584 |
| 10 | *The Complete Musician* | Laitz | 42 | 390 |
| 11 | *Fundamentals of Musical Composition* | Schoenberg | 27 | 210 |
| 12 | *Twentieth-Century Harmony* | Persichetti | 18 | 261 |
| 13 | *Music Theory for the 21st-Century Classroom* | Hutchinson, 2023 | 40 | 312 |
| 14 | *Mathematics and Music* | Wright, 2009 | 16 | 216 |

`SOURCES.md` carries the full annotated bibliography, including reference
papers (Papadopoulos on Messiaen, Fiore, Acef-Sanchez), explicit exclusions
(Mazzola), and the integration strategy by topic.

### Search — Three Complementary Modes

The MCP server exposes the graph through three coordinated retrieval surfaces:

1. **Full-text search (Tantivy)** — BM25-ranked keyword search across concept
   cards, source chapters, unified concepts, and guides. Multi-field boosts
   (title 3×, description 2×, content 1×), smart AND/OR query mode, stopword
   filtering with a domain-aware allowlist (Roman numerals, solfège), phrase
   queries, and category/source filtering. Index lives in `.tantivy-index/`
   and rebuilds automatically when content changes.
2. **Graph traversal (rkyv-cached)** — Direct queries over the typed concept
   graph: prerequisites in topological order, shortest paths between concepts,
   N-hop neighbourhoods, dependents, bridge concepts spanning categories,
   degree-centrality "hubs". Loads via memory-mapped rkyv cache in a few
   milliseconds; rebuilds on demand from concept-card frontmatter.
3. **Semantic search (LanceDB + fastembed)** — Vector and hybrid retrieval
   over locally-computed embeddings. `semantic_search` accepts `mode`:
   `vector` (pure embedding similarity), `keyword` (FTS fallback), or
   `hybrid` (default — fused ranking). `search_by_question` is tuned for
   natural-language queries phrased as questions.

All three share the same `SearchDocument` schema, so a query result reliably
points back to a concept ID, source ID, chapter, page, and section.

### MCP Tools — 53 in Total

Tools are grouped by domain and registered via a composable `ToolRegistry`
pattern (`fabryk_mcp::ToolRegistry`). The `mt_directory` tool returns a live
manifest with use-when guidance for every tool.

**Content access (10 tools)**
`list_sources`, `list_source_chapters`, `get_source_chapter`,
`check_source_availability`, `get_source_pdf_path`, `list_concepts`,
`list_categories`, `get_concept`, `list_guides`, `get_guide`.

**Search (4 tools)**
`search_concepts` (FTS), `semantic_search` (vector / keyword / hybrid),
`search_by_question` (NL question matcher), `search_status`.

**Music-theory computation (9 tools, via `music-comp-mt`)**
`get_scale_notes`, `get_chord_notes`, `get_interval`, `transpose`,
`get_diatonic_chords`, `identify_chord`, `identify_scale`,
`check_enharmonic`, `analyze_roman_numerals`.

**Graph (17 tools, requires `--features graph`)**
`graph_status`, `graph_stats`, `graph_validate`, `get_node`,
`get_node_edges`, `get_related_concepts`, `find_concept_path`,
`get_prerequisites`, `get_concept_neighborhood`, `get_dependents`,
`get_central_concepts`, `graph_bridges`, `find_bridge_concepts`,
`get_concept_sources`, `get_concept_variants`, `get_source_coverage`,
`get_learning_path`.

**Open Tone Harmony — OTH (12 tools)**
Built on the [6, 8] metric base space *B*: 228 voiced quintal/quartal
chords across 14 T/I orbits, with 52 distinct modes. Tools cover orbit
analysis (`get_oth_orbit_info`, `list_oth_orbits`,
`get_oth_parent_scales`, `get_oth_chord_info`), mode enumeration
(`list_oth_modes`, `find_oth_modes_by_opening`), interscalar
inversion / chord scales (`get_oth_chord_scale`), graph geometry
(`get_oth_distance`, `get_oth_neighbors`, `get_oth_geodesics`,
`get_oth_crossroads`), and mathematical verification
(`verify_oth_properties` — fiber-mode connection, multiset uniqueness,
the Universal L1 Law, quartal/quintal duality).

**Meta (1 tool, plus the `mt_directory` catalogue)**
`health` returns service / backend status. `mt_directory` is the live
self-describing tool registry — call it first in any new session.

## Architecture

The MCP server is a Rust workspace built on the **Fabryk** knowledge-fabric
framework (extracted to its own ecosystem; see `mcp-server/Cargo.toml`).
Domain-specific logic lives here; generic concerns — content layer, FTS
integration, graph cache, vector backend, MCP transport, CLI scaffolding —
are delegated to fabryk crates (`fabryk`, `fabryk-mcp`, `fabryk-cli`,
`fabryk-fts`, `fabryk-graph`).

```
ai-music-theory/
├── concept-cards/          # 4,315 v3.1 concept cards (per-source extractions)
│   ├── 20th-century-harmony/        (261)
│   ├── 21st-century-classroom/      (312)
│   ├── analyzing-classical-form/    (584)
│   ├── audacious-euphony/           (269)
│   ├── classical-form/              (432)
│   ├── complete-musician/           (390)
│   ├── fundamentals-music-comp/     (210)
│   ├── gen-intervals-xforms/        (301)
│   ├── geometry-of-music/           (233)
│   ├── maths-and-music/             (216)
│   ├── neo-riemannian-handbook/     (178)
│   ├── open-music-theory/           (435)
│   ├── post-tonal-theory/           (250)
│   └── tonality-owners-manual/      (244)
├── concepts-unified/       # Cross-source synthesised canonical concepts
├── sources-md/             # Markdown-converted source texts (14 sources)
├── extraction-metadata/    # Per-source extraction logs and provenance
├── guides/                 # Topic deep-dives
├── mcp-server/             # Rust workspace — the MCP server
│   ├── crates/server/      # Server crate (binary: music-theory-mcp)
│   ├── crates/design/      # Design docs and ADRs
│   └── data/               # Built graph + caches (generated)
├── workbench/              # Working notes, design docs, migration audits
├── scripts/                # Source conversion / extraction utilities
├── CONVENTIONS.md          # Notation standards (pitch classes, intervals, …)
├── SCOPE.md                # What's in / out of scope, validation tests
├── SOURCES.md              # Annotated bibliography
└── PIPELINE.md             # Source → markdown → concept-card workflow
```

### Backends and feature flags

The server crate exposes optional features so you can build a minimal binary
or the full kit:

| Feature | Adds |
|---------|------|
| *(none)* | Content + computation tools, simple linear search fallback |
| `fts` | Tantivy full-text search backend, BM25 ranking, advanced query syntax |
| `graph` | rkyv-cached concept graph, 16 traversal tools |
| `vector` | LanceDB store + fastembed embeddings, semantic & hybrid search |
| `http` | HTTP transport (in addition to the default stdio) |
| `full` | All of the above (recommended) |

All backends initialise asynchronously: the server starts in under a second
and tools become available as their backends finish loading. Pre-built caches
for graph, FTS, and vector indexes can be downloaded
(`music-theory-mcp cache download <kind>`) instead of built locally.

## Quick Start

### 1. Build

```bash
cd mcp-server

# Recommended — all features
make build-release-full

# Or build a slimmer binary
cargo build --release                                # content + computation only
cargo build --release --features fts                 # + Tantivy FTS
cargo build --release --features fts,graph           # + graph traversal
cargo build --release --features fts,graph,vector    # + semantic search
```

The binary lands in `mcp-server/bin/music-theory-mcp`.

### 2. Build (or download) the indexes

```bash
# Build locally
./bin/music-theory-mcp index                  # Tantivy FTS
./bin/music-theory-mcp graph build            # Concept graph + rkyv cache
./bin/music-theory-mcp vectordb build         # Embeddings + LanceDB index

# Or pull pre-built caches
./bin/music-theory-mcp cache download graph
./bin/music-theory-mcp cache download fts
./bin/music-theory-mcp cache download vector
```

### 3. Wire up Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "music-theory": {
      "command": "/absolute/path/to/ai-music-theory/mcp-server/bin/music-theory-mcp"
    }
  }
}
```

Restart Claude Desktop. On first use, call `mt_directory` for a guided tour
of the 53 tools, or `health` to confirm which backends are live.

### Other transports

`music-theory-mcp serve --transport http --bind 127.0.0.1:8000` runs the
server over HTTP for non-Claude clients (build with `--features http`).

## Notation Conventions

Consistent notation across the corpus (see `CONVENTIONS.md` for the full
spec):

- **Pitch classes** — integers 0–11 (C = 0)
- **Pitches with octave** — scientific pitch notation (C4 = middle C)
- **Intervals** — ordered `i(a, b) = (b − a) mod 12`; unordered `ic(a, b)`
- **Sets** — `{0, 4, 7}` (C major triad)
- **Transformations** — Tₙ (transposition), Iₙ (inversion), P / R / L
  (neo-Riemannian), and standard GIS notation for transformation networks

## Validation

The skill is graded against three structured tests in `SCOPE.md`:

- **Quartal/quintal test** — can the materials support a deep dive from
  generator-based collection construction through the [6, 8] metric space to
  compositional application? (The OTH tools were built around this.)
- **Messiaen test** — can the corpus reason about modes of limited
  transposition as Z₁₂ subgroups, non-retrogradable rhythms as palindromes,
  and symmetrical permutations as group actions?
- **Novel query test** — does it answer questions not explicitly in any
  source, e.g. "how does Cohn's hexatonic system relate to Tymoczko's
  orbifolds?"

## Development

The Rust workspace (`mcp-server/`) follows the canonical guidelines bundled
in `assets/ai/ai-rust/`. Some highlights:

- **EH-17 error handling** — structured error kind + always-captured backtrace
- **1,087+ tests** across the workspace; 95%+ coverage target
- **`make ci`** runs the full local CI pipeline
- **`make test-full`** runs every test under all feature combinations

See `mcp-server/crates/server/README.md` for the deep-dive on configuration,
backends, and CLI commands.

## License

- Original concept cards, schemas, tooling — **CC0** (see `SCOPE.md`)
- The MCP server crate — **Apache-2.0**
- Source materials retain their original licenses (Open Music Theory is
  CC-BY-SA; commercial textbooks are referenced under fair-use research and
  not redistributed)

## Author

Duncan McGreggor — [oubiwann@gmail.com](mailto:oubiwann@gmail.com)

[//]: ---Named-Links---

[logo]: https://raw.githubusercontent.com/music-comp/ai-music-theory/main/assets/images/logo/v1-y250.png
[logo-large]: https://raw.githubusercontent.com/music-comp/ai-music-theory/main/assets/images/logo/v1.png
[build]: https://github.com/music-comp/ai-music-theory/actions/workflows/cicd.yml
[build-badge]: https://github.com/music-comp/ai-music-theory/actions/workflows/cicd.yml/badge.svg
[tag-badge]: https://img.shields.io/github/tag/music-comp/ai-music-theory.svg
[tag]: https://github.com/music-comp/ai-music-theory/tags
