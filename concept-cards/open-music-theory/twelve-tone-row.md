---
concept: Twelve-Tone Row
slug: twelve-tone-row

category: analysis
subcategory: twelve-tone-theory
tier: advanced

source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Basics of Twelve-Tone Theory"
chapter_number: 9
pdf_page: null
section: "IX.1"

extraction_confidence: high

aliases:
  - "tone row"
  - "series"
  - "row"

prerequisites:
  - pitch-class
  - integer-notation
extends: []
related:
  - row-operations
  - row-class
  - row-matrix
  - serialism
contrasts_with:
  - pitch-class-set

answers_questions:
  - "What is a twelve-tone row?"
  - "How many possible twelve-tone rows exist?"
  - "What is the difference between a row and a set?"
---

# Quick Definition
A twelve-tone row (also called a series) is an ordered arrangement of all twelve pitch classes, each appearing exactly once, used as the foundational material for twelve-tone composition. Each row generates up to 48 related forms through transposition, inversion, retrograde, and retrograde inversion.

# Core Definition
Twelve-tone composition is based on a series (row) containing all twelve pitch classes in a particular order. There are 12! = 479,001,600 possible rows. The basic constraints: pitch classes are played in the order specified by the row, and once played, a pitch class is not repeated until the next row statement. Four operations transform a row: Prime (P, the original), Retrograde (R, reversed order), Inversion (I, reversed interval directions), and Retrograde Inversion (RI, both reversed). Each can be transposed to 12 pitch levels, yielding up to 48 forms in a row class. The row does not specify rhythm, register, or duration -- only pitch-class ordering. In practice, composers vary widely in how strictly they follow the ordering.

# Prerequisites
- Pitch class and integer notation (the units of the row)

# Key Properties
1. Contains all 12 pitch classes exactly once
2. Order is fixed (distinguishes a row from an unordered set)
3. 12! = 479,001,600 possible orderings
4. Generates up to 48 forms: P, R, I, RI each at 12 transpositions
5. Rows with symmetry properties may have fewer than 48 distinct forms
6. Does not specify rhythm, register, duration, or dynamics
7. Row forms are labeled by type and starting pitch class (e.g., P7, I3, R10, RI0)

# Context & Application
The twelve-tone technique emerged in the 1920s, associated with Schoenberg (developer), Webern (explored symmetry), and Berg (combined with tonal elements). Rows construct themes, motives, and chords. Not all serial music uses twelve-tone rows, and not all twelve-tone music is strictly serial. Composers like Dallapiccola used rows freely with pitch repetition, while others like Webern adhered strictly to row ordering.

# Examples
**Example 1** (Lutyens, Motet Op. 27): P0: 0-11-3-7-8-4-2-6-5-1-9-10 (C-B-E-flat-G-A-flat-E-D-F-sharp-F-D-flat-A-B-flat).

**Example 2** (Webern, Symphonie Op. 21): Row divides into two symmetric hexachords, both set class 6-1 (chromatic hexachord). The row is retrograde-equivalent (R6 = transposed P0), yielding only 24 distinct forms.

**Example 3** (Webern, Konzert Op. 24): P0: 11-8-2-3-7-6-8-4-5-0-1-9. Four trichords all set class (014), related by P, I, R, RI. Only 12 distinct row forms due to symmetry.

# Relationships
## Builds Upon
- **pitch-class** -- Rows are ordered sequences of pitch classes
- **integer-notation** -- Rows are expressed using integers 0-11
## Related
- **row-operations** -- P, R, I, RI transformations
- **row-class** -- The collection of all 48 (or fewer) forms
- **row-matrix** -- Grid displaying all row forms
## Contrasts With
- **pitch-class-set** -- A set is unordered; a row is ordered

# Common Confusions
- **Confusion**: A twelve-tone row is the same as a pitch-class set
  **Clarification**: A row is ordered; a set is unordered
- **Confusion**: The row must be presented as a twelve-note melody
  **Clarification**: Row pitches can be distributed across voices, presented as chords, etc.
- **Confusion**: Twelve-tone music always sounds "atonal"
  **Clarification**: Composers like Berg and Britten created tonal-sounding twelve-tone music

# Source Reference
Open Music Theory, Part IX, Chapter 1: "Basics of Twelve-Tone Theory."

# Verification Notes
- Definition source: Directly from 09-01 source chapter
- Confidence rationale: High -- comprehensive treatment in source
- Preserved from v2: Lutyens, Webern Op. 21, Webern Op. 24 examples, 48-form calculation
- Cross-reference status: Verified against row properties and naming conventions chapters
