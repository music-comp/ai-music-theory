---
number: 1
title: "Open Music Theory Processing Plan"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-01-24
updated: 2026-01-24
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Open Music Theory Processing Plan

## Overview

Open Music Theory (OMT) is our foundational source — 1297 pages of CC-licensed 
pedagogical content. This document outlines how to process it efficiently.

## Source Information

- **File**: `[2022] Gotham - Open Music Theory.pdf`
- **Original**: Also available as XML (website source)
- **License**: CC-BY-SA
- **Online**: https://viva.pressbooks.pub/openmusictheory/

## Strategy: Section-Based Processing

Given the size, we process in **priority sections**, not sequentially.

### Priority 1: Fundamentals (Required Before Anything Else)

These establish the vocabulary everything else depends on.

| Topic | Chapters | Why Critical |
|-------|----------|--------------|
| Pitch & Intervals | Fundamentals Part I | Everything builds on this |
| Scales & Keys | Fundamentals Part II | Mode/scale terminology |
| Triads & Chords | Fundamentals Part III | Harmonic vocabulary |
| Basic Voice Leading | Part of Harmony | Foundation for Tymoczko |

**Target**: ~15-20 concept cards from this section

### Priority 2: Harmony & Analysis

| Topic | Chapters | Why Important |
|-------|----------|---------------|
| Roman Numerals | Harmony section | Standard analytical notation |
| Cadences | Harmony section | Structural vocabulary |
| Sequences | Harmony section | Common patterns |
| Form basics | Form section | Phrase, period, etc. |

**Target**: ~10-15 concept cards

### Priority 3: Post-Tonal Introduction

| Topic | Chapters | Why Important |
|-------|----------|---------------|
| Pitch-class sets | Atonality section | Bridges to Straus/Lewin |
| Set class | Atonality section | Foundation for transformations |
| Twelve-tone basics | Serial section | Context for historical development |

**Target**: ~8-10 concept cards

### Priority 4: Later or As-Needed

- Species counterpoint (reference when relevant)
- Pop/rock analysis (style-specific)
- Jazz harmony (style-specific)
- Worksheets/exercises (skip)

## Processing Steps

### Step 1: Conversion (Run Locally)

```bash
# Option A: marker for PDF
marker_single --output_dir ~/music-theory-skill/sources_md/open-music-theory/ \
  "[2022] Gotham - Open Music Theory.pdf"

# Option B: If XML available, pandoc may be cleaner
pandoc open-music-theory.xml -o full.md
```

### Step 2: Section Extraction

After conversion, split into manageable sections:
```bash
# Create section files from full conversion
# (Manual or scripted based on chapter markers)
```

### Step 3: Concept Card Creation

For each concept, create a card following this template:

```markdown
# [Concept Name]

## Quick Definition
[1-2 sentence definition a musician would give]

## Precise Definition  
[Mathematical/formal definition using CONVENTIONS.md notation]

## In Musical Terms
[What it sounds like, where you hear it]

## Examples
### Simple
[Basic example]

### From Repertoire
[Real music example if available]

## Common Confusions
[What this is NOT, common mistakes]

## Connections
- **Prerequisite for**: [more advanced concepts]
- **Builds on**: [simpler concepts]
- **Related to**: [parallel concepts]
- **In different contexts**: [style-period variations]

## Source
Open Music Theory, [Chapter/Section]
```

### Step 4: Validation

For each concept card:
- [ ] Definition matches OMT
- [ ] Notation follows CONVENTIONS.md
- [ ] Musical examples are clear
- [ ] Connections are accurate

## Expected Output

After processing OMT Priority 1-3:

```
/concept_cards/open-music-theory/
├── fundamentals/
│   ├── pitch.md
│   ├── pitch-class.md
│   ├── octave-equivalence.md
│   ├── interval-melodic.md
│   ├── interval-harmonic.md
│   ├── interval-class.md
│   ├── scale-major.md
│   ├── scale-minor.md
│   ├── mode.md
│   ├── triad.md
│   ├── seventh-chord.md
│   ├── inversion.md
│   └── ...
├── harmony/
│   ├── roman-numeral.md
│   ├── figured-bass.md
│   ├── cadence.md
│   ├── voice-leading-basic.md
│   ├── parallel-motion.md
│   ├── contrary-motion.md
│   └── ...
├── form/
│   ├── phrase.md
│   ├── period.md
│   ├── sentence.md
│   └── ...
└── post-tonal/
    ├── pitch-class-set.md
    ├── set-class.md
    ├── normal-form.md
    ├── prime-form.md
    ├── interval-vector.md
    └── ...
```

## Milestone: MCP Server v0.1

Once we have ~40 concept cards from OMT, we can build MCP server v0.1:

```typescript
// Tools available in v0.1:
// - list_concepts() → all available concept cards
// - get_concept(name) → returns card content
// - search_concepts(query) → basic text search
```

This enables the **bootstrapping loop** — Claude can start using the skill
to help create more of the skill.

## Timeline Estimate

| Step | Effort | Notes |
|------|--------|-------|
| Conversion | 30 min | marker run + spot check |
| Section split | 1 hour | Manual organization |
| Priority 1 cards | 3-4 hours | ~20 cards |
| Priority 2 cards | 2-3 hours | ~15 cards |
| Priority 3 cards | 1-2 hours | ~10 cards |
| Validation | Ongoing | Human spot-checks |
| MCP v0.1 | 1-2 hours | Basic TypeScript server |

**Total to v0.1**: ~10-12 hours of focused work

## Notes

### What We're NOT Doing
- Complete coverage of all 1297 pages
- Preserving OMT's exact organization
- Copying text verbatim (we synthesize)

### What We ARE Doing
- Extracting essential concepts
- Standardizing notation (CONVENTIONS.md)
- Building toward unified cards
- Creating foundation for Lewin/Tymoczko

---

*Ready to begin when you run marker on the PDF locally.*
