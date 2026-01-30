# Claude Code Prompt: OMT Concept Card Extraction

## Context

You are helping build a comprehensive music theory skill for AI assistants. We've extracted 123 chapters from Open Music Theory into markdown files. Your job is to create **concept cards** — concise, structured reference documents for individual concepts.

## Project Location

```
~/lab/music-comp/ai-music-theory/
├── sources-md/open-music-theory/    # Source chapters (123 files)
├── concept-cards/                    # Output location for cards
│   └── open-music-theory/           # Create this subdirectory
├── CONVENTIONS.md                    # Notation standards (READ THIS FIRST)
├── SCOPE.md                         # Project scope and goals
└── PIPELINE.md                      # Overall process documentation
```

## Your Task

Process OMT chapters sequentially, extracting concept cards. Start with Part 01 (Fundamentals).

### Step 1: Read the conventions

```bash
cat ~/lab/music-comp/ai-music-theory/CONVENTIONS.md
```

This establishes notation standards you MUST follow (pitch class integers, interval notation, etc.)

### Step 2: Create output directory

```bash
mkdir -p ~/lab/music-comp/ai-music-theory/concept-cards/open-music-theory
```

### Step 3: Process chapters in order

Start with `01-01-introduction-to-western-musical-notation.md` and work through Part 01.

For each chapter:

1. Read the chapter
2. Identify extractable concepts (some chapters may have multiple, some may have none)
3. Create a concept card for each concept

### Concept Card Template

```markdown
---
concept: [Concept Name]
category: [fundamentals|harmony|counterpoint|form|chromaticism|post-tonal|serial|rhythm]
source: Open Music Theory
chapter: "[Chapter Title]"
part: [Part Number]
---

# [Concept Name]

## Quick Definition
[1-2 sentence definition a working musician would give]

## Formal Definition
[Precise definition, using CONVENTIONS.md notation where applicable]

## Musical Context
[Where this appears in real music, what it sounds like, why it matters]

## Examples

### Basic
[Simple example]

### From Repertoire
[Real music example if mentioned in source]

## Related Concepts
- **Prerequisite**: [concepts needed to understand this one]
- **Leads to**: [more advanced concepts this enables]
- **See also**: [related concepts at same level]

## Common Confusions
[What this is NOT, typical mistakes]

## Source Reference
Open Music Theory, Part [X], Chapter [Y]: "[Chapter Title]"
```

### Filename Convention

```
{concept-name-slugified}.md
```

Examples:

- `pitch.md`
- `interval.md`
- `major-scale.md`
- `pitch-class.md`
- `interval-class.md`

### What to Extract

**DO extract** as separate concept cards:

- Core definitions (pitch, interval, scale, chord, etc.)
- Named structures (major scale, minor scale, triad types, etc.)
- Operations (transposition, inversion, etc.)
- Analytical concepts (roman numerals, figured bass, etc.)

**DON'T extract** as separate cards:

- Procedural instructions ("how to sight-sing")
- Historical context (unless it's a named concept)
- Exercise instructions
- Navigation/UI artifacts from conversion

### Handling Multiple Concepts per Chapter

Some chapters cover multiple concepts. For example, `01-16-intervals.md` covers:

- Interval (general concept)
- Melodic vs harmonic intervals
- Interval size
- Interval quality (perfect, major, minor, augmented, diminished)
- Simple vs compound intervals
- Interval inversion
- Consonance and dissonance

Create separate cards for each substantial concept. Use judgment — some distinctions (melodic vs harmonic) might be subsections within `interval.md` rather than separate cards.

### Cleaning Up Source Content

The source files contain some artifacts:

- `[pb_glossary id="..."]term[/pb_glossary]` — strip the shortcode, keep the term
- `Example 1.` references — note as `[Example: description]`
- Broken links — just note the intended reference

### Progress Tracking

After processing each chapter, output:

```
Processed: 01-01-introduction-to-western-musical-notation.md
  Created: [list of concept cards created, or "No extractable concepts"]
```

## Starting Point

Begin with Part 01, Chapters 01-01 through 01-21. These establish the foundational vocabulary.

Key concepts to expect in Part 01:

- Staff, clef, ledger lines
- Note names, octave designation (ASPN)
- Accidentals (sharp, flat, natural)
- Half step, whole step
- Rhythm values (whole, half, quarter, etc.)
- Meter (simple, compound)
- Scale (major, minor)
- Scale degree
- Key signature
- Mode
- Interval (size, quality)
- Triad (major, minor, augmented, diminished)
- Seventh chord (types)
- Inversion
- Figured bass
- Roman numeral
- Texture (monophonic, homophonic, polyphonic)

## Quality Checks

Before saving each card, verify:

1. Uses CONVENTIONS.md notation correctly
2. Formal definition is precise
3. Has at least one example
4. Related concepts are accurate
5. No leftover HTML/shortcode artifacts

## Let's Begin

Start by reading CONVENTIONS.md, then process 01-01. Show me the first few concept cards you create so we can calibrate before you continue through the rest of Part 01.
