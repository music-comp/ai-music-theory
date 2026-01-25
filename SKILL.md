# SKILL.md — Music Theory Skill

## What This Skill Provides

This skill gives Claude comprehensive knowledge of modern mathematical music theory, enabling support for:

- **Analysis** — Understanding musical structure across Western style periods
- **Composition** — Generating possibilities, exploring voice leadings, working with collections
- **Theory exploration** — Deep dives into mathematical foundations
- **Pedagogy** — Explaining concepts at appropriate levels

## How to Use This Skill

### Finding Information

1. **Start with concept cards** for definitions and quick reference
2. **Use guides** for deeper exploration of topics
3. **Consult CONVENTIONS.md** for notation standards
4. **Check SOURCES.md** for original references

### MCP Tools Available

| Tool | Use When |
|------|----------|
| `list_concepts` | Finding what concepts are documented |
| `get_concept` | Looking up a specific concept |
| `search_concepts` | Finding concepts related to a query |
| `get_conventions` | Checking notation standards |
| `list_sources` | Seeing what source materials exist |
| `get_source_chapter` | Reading original source material |
| `get_guide` | Deep dive into a topic |
| `get_skill_doc` | Accessing core documentation |

### Workflow for Answering Questions

```
1. Identify the concept(s) involved
2. Check if concept cards exist → get_concept
3. If not, check source chapters → get_source_chapter
4. Verify notation with → get_conventions
5. For deeper context → get_guide (if available)
```

### When Sources Disagree

Different sources sometimes use different terminology or approaches. Check `DEBATES.md` for documented disagreements. When in doubt:

1. Prefer Tymoczko for voice-leading geometry
2. Prefer Lewin for transformational theory
3. Prefer Cohn for neo-Riemannian
4. Prefer Straus for set-class terminology
5. Note disagreements to the user when relevant

## Intellectual Orientation

### Core Lineage

- **Lewin** — GIS, transformations, the "transformational attitude"
- **Tymoczko** — Geometric voice-leading, orbifolds, extended common practice
- **Cohn** — Neo-Riemannian theory, hexatonic systems

### Out of Scope

- **Mazzola's topos-theoretic approach** — excluded per Tymoczko's critique

### The Inquiry Pattern

This skill supports a particular style of musical inquiry:

```
Musical phenomenon → Pattern recognition → Mathematical structure → 
Generalization → Return to music
```

Not just "what is X?" but "what does X connect to?" and "how do I use X?"

## Quick Reference

### Pitch Notation (from CONVENTIONS.md)

- **Pitch classes**: Integers 0-11 (C=0, C♯/D♭=1, ..., B=11)
- **Pitches with octave**: Scientific notation (C4 = middle C)
- **Pitch-class sets**: Curly braces {0, 4, 7}

### Interval Notation

- **Ordered**: i(a,b) = (b - a) mod 12
- **Unordered (interval class)**: ic(a,b) = min(i, 12-i)

### Transformations

- **Transposition**: Tₙ(x) = (x + n) mod 12
- **Inversion**: Iₙ(x) = (n - x) mod 12
- **Neo-Riemannian**: P (parallel), R (relative), L (leading-tone)

### Style-Period Tags

When tools have limited applicability, they're tagged:
- `[Renaissance]`, `[Baroque]`, `[Classical]`, `[Romantic]`
- `[Romantic-chromatic]`, `[Atonal]`, `[Serial]`
- `[Universal]` — applies across periods

## Content Status

### Available Now
- **Open Music Theory** — 123 chapters converted to markdown
- **Concept Cards** — Fundamentals in progress
- **Core Docs** — CONVENTIONS.md, SCOPE.md, SOURCES.md, PIPELINE.md

### Coming Soon
- Lewin GMIT conversion
- Tymoczko Geometry/Tonality conversion
- Unified concept cards
- Topic guides

### Source PDFs Available (unconverted)
Use `get_source_pdf_path` to get paths for PDF Tools access:
- Lewin - GMIT
- Tymoczko - A Geometry of Music
- Tymoczko - Tonality (EPUB)
- Cohn - Audacious Euphony
- Straus - Post-Tonal Theory
- Wright - Mathematics and Music
- Papadopoulos - Mathematics and Group Theory in Music

## Version

Current: v0.1 (foundational)

See PIPELINE.md for version roadmap.
