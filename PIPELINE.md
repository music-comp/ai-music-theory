# PIPELINE.md — Music Theory Skill Development Process

## Overview

This document describes the complete pipeline for building the music theory skill,
from raw sources to a functioning MCP server that Claude can use to improve itself.

**Key Insight**: We're building a bootstrapping loop:

```
Foundational docs → MCP server → Claude uses skill → Better docs → Better server → ...
```

## Directory Structure

```
/music-theory-skill/
├── SKILL.md                    # Entry point for Claude
├── SCOPE.md                    # Goals, boundaries, target tasks
├── CONVENTIONS.md              # Notation standards
├── SOURCES.md                  # Annotated source inventory
├── DEBATES.md                  # Theoretical disagreements
├── INDEX.md                    # Cross-reference index
├── VALIDATION.md               # Test results
├── PIPELINE.md                 # This document
├── INQUIRY_PATTERNS.md         # Meta-patterns of musical inquiry
│
├── /sources_md/                # Converted source materials
│   ├── open-music-theory/      # Pedagogical foundation
│   ├── lewin-gmit/             # GIS, transformations
│   ├── tymoczko-geometry/      # Voice-leading geometry
│   ├── tymoczko-tonality/      # Crown jewel
│   ├── cohn-audacious/         # Neo-Riemannian
│   └── ...
│
├── /concept_cards/             # Per-source concept extractions
│   ├── open-music-theory/
│   │   ├── interval.md
│   │   ├── chord.md
│   │   └── ...
│   ├── lewin-gmit/
│   │   ├── gis.md
│   │   └── ...
│   └── ...
│
├── /concepts_unified/          # Synthesized cards (cross-source)
│   ├── interval.md
│   ├── pitch-class-set.md
│   ├── transformation.md
│   └── ...
│
├── /guides/                    # AI-optimized topic guides (4K-8K tokens)
│   ├── fundamentals/
│   │   ├── pitch-and-intervals.md
│   │   ├── scales-and-modes.md
│   │   └── ...
│   ├── transformational/
│   │   ├── gis-foundations.md
│   │   ├── neo-riemannian.md
│   │   └── ...
│   ├── geometric/
│   │   ├── voice-leading-spaces.md
│   │   ├── orbifolds.md
│   │   └── ...
│   └── compositional/
│       ├── generator-harmony.md
│       ├── modes-limited-transposition.md
│       └── ...
│
└── /mcp-server/                # MCP server for skill access
    ├── package.json
    ├── src/
    │   └── index.ts
    └── README.md
```

## Phase 0: Foundation Setup

### 0.1 Create Directory Structure

```bash
mkdir -p ~/music-theory-skill/{sources_md,concept_cards,concepts_unified,guides,mcp-server}
mkdir -p ~/music-theory-skill/guides/{fundamentals,transformational,geometric,compositional}
```

### 0.2 Core Documents

Create initial versions of:

- [ ] SCOPE.md (from draft)
- [ ] CONVENTIONS.md (from draft)
- [ ] SOURCES.md (full annotated inventory)
- [ ] PIPELINE.md (this document)

### 0.3 Verify Tooling

- [ ] `marker` installed and working (PDF → Markdown with LaTeX)
- [ ] `pandoc` available (EPUB → Markdown)
- [ ] PDF Tools plugin accessible

**Checkpoint**: Directory exists, core docs in place, tools verified.

---

## Phase 1: Open Music Theory (Pedagogical Foundation)

**Why first?** Without foundational terminology, Claude can't properly understand
Lewin, Tymoczko, or Cohn. This is our grounding document.

### 1.1 Conversion

```bash
# Open Music Theory is XML/HTML-based
# Convert to clean markdown sections
pandoc open-music-theory.xml -o sources_md/open-music-theory/full.md

# Or process chapter by chapter if needed
```

### 1.2 Quality Check

- [ ] Mathematical notation renders correctly
- [ ] Musical examples referenced (even if not embedded)
- [ ] Structure preserved

### 1.3 Concept Extraction

For each major concept in Open Music Theory, create a concept card:

**Card Template:**

```markdown
# [Concept Name]

## Definition
[Precise definition]

## Mathematical Formulation
[If applicable, using CONVENTIONS.md notation]

## Musical Context
[Where this appears, what it sounds like]

## Examples
[Concrete musical examples]

## Connections
- Related to: [other concepts]
- Prerequisite for: [advanced concepts]
- See also: [related cards]

## Source
[Open Music Theory, Chapter X]
```

### 1.4 Concepts to Extract (Priority Order)

1. Pitch, pitch class, octave equivalence
2. Intervals (melodic, harmonic, ordered, unordered)
3. Scales (major, minor, modes)
4. Chords (triads, seventh chords, extensions)
5. Roman numeral analysis
6. Voice leading (basic)
7. Cadences
8. Form (phrase, period, binary, ternary, sonata)
9. Counterpoint fundamentals
10. Set theory basics (if covered)

**Checkpoint**: Open Music Theory converted, ~20-30 concept cards created.
**Milestone**: MCP server v0.1 can serve these foundational concepts.

---

## Phase 2: Lewin (GIS & Transformations)

### The Lewin Protocol

Lewin is mathematically dense and foundational. Errors here propagate everywhere.

### 2.1 Conversion

```bash
marker_single --output_dir ~/music-theory-skill/sources_md/lewin-gmit/ \
  "[2007] Lewin - Generalized Musical Intervals and Transformations - Revised Edition.pdf"
```

### 2.2 Chapter-by-Chapter Validation

For each chapter:

1. Convert via marker
2. Human spot-check against physical copy
3. Verify all GIS definitions correct
4. Verify all transformation definitions correct
5. Note any OCR errors for manual fix

### 2.3 Critical Definitions (Must Be Perfect)

- [ ] Generalized Interval System (GIS) definition
- [ ] IVLS function
- [ ] Interval-preserving operations
- [ ] Simply transitive group action
- [ ] Transformation graphs
- [ ] Networks

### 2.4 Concept Cards

- gis.md
- transformation-group.md
- interval-function.md
- transformation-network.md
- simply-transitive.md

**Checkpoint**: Lewin fully converted, validated, concept cards created.

---

## Phase 3: Tymoczko - A Geometry of Music

### 3.1 Conversion

```bash
marker_single --output_dir ~/music-theory-skill/sources_md/tymoczko-geometry/ \
  "[2011] Tymoczko - A Geometry of Music.pdf"
```

### 3.2 Key Concepts

- Voice-leading as path in quotient space
- Chord space as orbifold
- T^n/S_n construction
- Efficient voice leading
- Scalar collections
- Extended common practice

### 3.3 Concept Cards

- voice-leading-space.md
- orbifold.md
- efficient-voice-leading.md
- scalar-collection.md
- extended-common-practice.md

**Checkpoint**: Geometry of Music converted, concept cards created.

---

## Phase 4: Tymoczko - Tonality: An Owner's Manual

**Crown jewel. Most mature synthesis. EPUB format.**

### 4.1 Conversion

```bash
pandoc "[2023] Tymoczko - Tonality - An Owners Manual.epub" \
  -o ~/music-theory-skill/sources_md/tymoczko-tonality/full.md
```

### 4.2 Quality Check

- [ ] EPUB conversion preserves structure
- [ ] Mathematical notation (may need post-processing)
- [ ] Cross-references intact

### 4.3 Key Additions Beyond Geometry

- Refined voice-leading theory
- Better treatment of scales
- More compositional applications
- Category-theoretic hints

**Checkpoint**: Tonality converted, integrated with Geometry concepts.

---

## Phase 5: Cohn & Neo-Riemannian

### 5.1 Conversion

```bash
marker_single --output_dir ~/music-theory-skill/sources_md/cohn-audacious/ \
  "[2012] Cohn - Audacious Euphony.pdf"
```

### 5.2 Key Concepts

- P, L, R operations
- Hexatonic systems
- Hyper-hexatonic systems
- Parsimonious voice leading
- Tonnetz

### 5.3 Concept Cards

- plr-operations.md
- hexatonic-system.md
- tonnetz.md
- parsimonious-voice-leading.md

**Checkpoint**: Cohn converted, Neo-Riemannian concepts integrated.

---

## Phase 6: Synthesis & Unification

### 6.1 Concept Unification

Merge per-source cards into unified cards:

- Compare definitions across sources
- Resolve notational differences (using CONVENTIONS.md)
- Note disagreements in DEBATES.md
- Create unified card with best synthesis

### 6.2 DEBATES.md

Document theoretical disagreements:

- Lewin vs. Tymoczko on certain constructions
- Different definitions of "efficient" voice leading
- Mazzola critique (why we exclude it)
- Notation wars (T_n vs. τ_n, etc.)

### 6.3 INDEX.md

Build comprehensive cross-reference:

- Concept → Concept cards
- Concept → Guides
- Concept → Source locations
- Style period → Applicable concepts

**Checkpoint**: Unified concept cards, DEBATES.md, INDEX.md complete.

---

## Phase 7: Guide Generation

### 7.1 Guide Principles

- 4K-8K tokens per guide (fits in context efficiently)
- Self-contained but cross-referenced
- Mathematical precision + musical grounding
- Include "what can I do with this?" section

### 7.2 Guide Template

```markdown
# [Topic Name]

## Overview
[2-3 sentence summary]

## Prerequisites
- [Required concepts with links]

## Core Content
### [Subtopic 1]
...
### [Subtopic 2]
...

## Mathematical Foundations
[Formal definitions, using CONVENTIONS.md]

## Musical Applications
[Concrete examples from repertoire]

## Compositional Uses
[How to use this in composition]

## Common Pitfalls
[Mistakes to avoid]

## Connections
- Builds on: [prerequisite guides]
- Leads to: [advanced guides]
- Related: [parallel concepts]

## References
[Source citations]
```

### 7.3 Dual-Track Organization

**Concept-oriented** (what is X?):

- fundamentals/pitch-and-intervals.md
- transformational/gis-foundations.md
- geometric/voice-leading-spaces.md

**Task-oriented** (how do I do X?):

- compositional/finding-voice-leadings.md
- compositional/working-with-modes.md
- compositional/generator-harmony.md

**Checkpoint**: Initial guide set complete.

---

## Phase 8: SKILL.md & MCP Server

### 8.1 SKILL.md

The entry point. Tells Claude:

- What this skill is for
- How to navigate the guides
- When to use which guide
- How to search for concepts

### 8.2 MCP Server v1.0

```typescript
// Minimal tools:
// - list_guides() → returns available guides
// - get_guide(name) → returns guide content
// - search_concepts(query) → searches INDEX.md
// - get_convention(topic) → returns relevant CONVENTIONS.md section
```

### 8.3 Self-Improvement Loop

Once MCP server is live:

1. Claude uses skill to answer music theory questions
2. Gaps identified → new cards/guides needed
3. Claude helps draft new content using existing skill
4. Human validates
5. Skill improves

**Checkpoint**: SKILL.md complete, MCP server functional.

---

## Phase 9: Validation

### 9.1 The Quartal/Quintal Test

Can the skill support recreating the journey from the validation conversations?

- Generator-based harmony construction
- Quartal-quintal duality
- Supporting scales
- Group-theoretic foundations
- Compositional applications

### 9.2 The Messiaen Test

Can the skill support analysis of:

- Modes of limited transposition
- Non-retrogradable rhythms
- Symmetrical permutations

### 9.3 Novel Query Test

Test with questions not explicitly covered:

- "What are the voice-leading options from this chord to that chord?"
- "How does Cohn's hexatonic system relate to Tymoczko's orbifolds?"
- "Generate a chord progression using only parsimonious voice leading"

**Checkpoint**: All validation tests pass.

---

## Phase 10: Polish & Package

### 10.1 Documentation

- README.md for repository
- CONTRIBUTING.md for future improvements
- LICENSE (CC0)

### 10.2 Final Review

- All cross-references work
- Notation consistent throughout
- No orphan concepts
- MCP server stable

### 10.3 Release

- v1.0 tag
- Public repository
- Announcement

---

## Version Milestones

| Version | State |
|---------|-------|
| v0.1 | Open Music Theory concepts, basic MCP server |
| v0.2 | + Lewin concepts, improved MCP |
| v0.3 | + Tymoczko (both books), Cohn |
| v0.4 | Unified concepts, DEBATES.md |
| v0.5 | Initial guide set |
| v0.6 | Full guide coverage |
| v0.7 | SKILL.md, full MCP server |
| v0.8 | Validation tests pass |
| v0.9 | Polish, documentation |
| v1.0 | Release |

---

## Critical Notes

### The "Claude Can Help" Threshold

After v0.1, Claude can start helping with subsequent phases using the skill.
This is the bootstrapping moment.

### Human Validation Points

These steps REQUIRE human verification:

- Lewin definitions (2.2, 2.3)
- EPUB conversion quality (4.2)
- Debate documentation accuracy (6.2)
- Final validation (9.x)

### What NOT to Automate

- Musical judgment calls
- Aesthetic evaluations
- "Is this actually useful?" decisions
- Resolving genuine theoretical disputes

---

## Appendix: Tool Commands

### marker (PDF → Markdown)

```bash
# Single file
marker_single --output_dir ./output/ input.pdf

# Batch (if needed)
marker --input_dir ./pdfs/ --output_dir ./markdown/
```

### pandoc (EPUB → Markdown)

```bash
pandoc input.epub -o output.md
pandoc input.epub -t markdown --wrap=none -o output.md  # no line wrapping
```

### PDF content extraction (via PDF Tools)

Used when marker isn't available or for quick checks.

---

*This pipeline is a living document. Update as we learn.*
