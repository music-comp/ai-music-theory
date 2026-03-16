---
# === CORE IDENTIFICATION ===
concept: Row Chart
slug: row-chart

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: serialism
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Twelve-Tone Music"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "twelve-tone row chart"
  - "twelve-tone matrix"
  - "tone row matrix"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - twelve-tone-technique
  - modular-chromatic-intervals
  - octave-equivalence-of-interval-ratios
extends: []
related:
  - n-tone-row-chart
  - cyclic-permutations
  - generating-interval
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a row chart in twelve-tone composition?"
  - "How is a twelve-tone row chart constructed from an original row?"
  - "What are the structural properties that every row chart satisfies?"
  - "How are retrogrades, inversions, and transpositions encoded in a row chart?"
---

# Quick Definition

A 12 x 12 array of note classes used as the structural foundation for twelve-tone composition, in which each row and column contains every note class exactly once, and all entries are determined by the original (prime) row.

# Core Definition

Wright defines a row chart as "a 12 by 12 array" with the following properties: "Each entry is one of the 12 note classes, modulo octave. Each row and each column contains each note class precisely once." The top row is the **original row** (or **prime row**). The leftmost column is the **inversion** of the top row: "the interval (modulo octave) from the top left note class to the nth entry in the left column is the opposite of the interval from the top left note class to the nth entry in the top row." Subsequent rows are **transpositions** of the top row, filled in by preserving the intervals of the first row (Wright, pp. 77-78). When expressed in modular arithmetic, the entry at position (i, j) is a_j - a_i in Z_12, where the original row is a_1, a_2, ..., a_12 (formalized in Chapter 7).

# Prerequisites

- **Twelve-tone technique** -- Must understand the compositional method that uses all 12 note classes systematically
- **Modular chromatic intervals** -- Must understand intervals computed modulo 12 (or modulo octave)
- **Octave equivalence of interval ratios** -- Must understand that note classes are identified modulo octave

# Key Properties

1. The chart is a 12 x 12 array; each entry is one of the 12 note classes (modulo octave)
2. Each row contains every note class exactly once
3. Each column contains every note class exactly once (the chart is a Latin square)
4. The top row is the original (prime) row
5. The left column is the inversion of the original row
6. Each row is a transposition of the original row
7. Each column is a transposition of the inversion, equivalently an inversion of a transposition
8. All entries are determined by the original row via the formula entry(i,j) = a_j - a_i in Z_12
9. The number of possible original rows is 12! = 479,001,600

# Construction / Recognition

## To Construct a Row Chart
1. Choose an original row: a sequence of all 12 note classes, each appearing exactly once
2. Write this as the top row of the 12 x 12 array
3. Compute the left column (inversion): for each entry n in the top row, the corresponding left-column entry has the opposite interval from the top-left note class
4. Fill in each subsequent row by transposing the original row so that it starts on the note class already placed in the left column
5. Verify that each column also contains all 12 note classes exactly once

## Using Modular Arithmetic
1. Assign each note class a number in Z_12 (e.g., C=0, C#=1, ..., B=11)
2. Express the original row as a_1=[0], a_2, ..., a_12 in Z_12 (transposing so the first entry is [0])
3. Compute entry(i,j) = a_j - a_i in Z_12 for all i, j
4. Convert back to note class names

# Context & Application

The row chart was developed in the 1920s by Arnold Schoenberg as part of the twelve-tone technique, continued by Anton Webern, Alban Berg, and Milton Babbitt. In twelve-tone composition, consonance is "largely abandoned in favor of combinatorics," and the row chart provides the complete catalog of permissible sequences (Wright, p. 77).

**Compositional use:** The goal is to create a composition using the note-class sequences from the rows and/or columns of the chart, or their **retrogrades** (sequences read in reverse). Retrogrades are obtained by reading rows right-to-left or columns bottom-to-top. Sequences may be applied horizontally (melodically) or vertically (harmonically) in the music.

**Notation conventions:** Twelve-tone music is often written in the key of C due to the absence of tonal center. The spelling of accidentals may vary from the row chart and may change during the composition (Wright, p. 79).

# Examples

**Example 1** (pp. 77-78): Original row E, G, F#, A, G#, C, F, D, D#, C#, B, Bb generates a complete 12 x 12 row chart. Wright provides the full chart showing how the inversion (left column: E, C#, D, B, C, G#, D#, F#, F, G, A, Bb) and all transpositions are derived.

**Example 2** (p. 78): A composition based on the chart above uses the original row in the bass clef, the retrograde of the original row in the upper treble, and the second column (a transposition of the inversion) in the lower treble. The sequences are applied horizontally, "often producing the clashing effect of dissonant chords."

**Example 3** (p. 79): A different original row demonstrates vertical use of row sequences, where "groups of note classes" from the row are assembled as chords rather than melodies.

# Relationships

## Builds Upon
- **Twelve-tone technique** -- The row chart is the central organizational tool of twelve-tone composition
- **Modular chromatic intervals** -- The chart's arithmetic operates in Z_12

## Enables
- **N-tone row chart** -- The row chart concept generalizes to n x n charts for any n-chromatic scale using Z_n arithmetic (Chapter 7)

## Related
- **Cyclic permutations** -- Row chart rows are related by transposition, a form of cyclic shift in modular arithmetic
- **Generating interval** -- The generating intervals of Z_n relate to which intervals can produce all note classes

# Common Errors

- **Error**: Constructing the inversion column by simply reversing the original row
  **Correction**: The inversion negates the intervals from the first note, not the order. For original row intervals [0, +3, +2, ...], the inversion has intervals [0, -3, -2, ...] (modulo 12)

- **Error**: Filling in rows without maintaining the interval pattern of the original row
  **Correction**: Each row must preserve the exact interval sequence of the original row; only the starting note changes

# Common Confusions

- **Confusion**: Thinking the row chart entries are arbitrary or freely chosen
  **Clarification**: Every entry is completely determined by the original row through the formula entry(i,j) = a_j - a_i. Only the original row is freely chosen

- **Confusion**: Confusing "retrograde" with "inversion"
  **Clarification**: The retrograde reverses the order of a sequence (read right-to-left or bottom-to-top); the inversion negates the intervals. These are distinct operations that can also be combined (retrograde inversion)

- **Confusion**: Thinking the row chart applies only to 12-tone music
  **Clarification**: The same construction works for any n-chromatic scale using Z_n arithmetic (see n-tone-row-chart)

# Source Reference

Chapter 6: "Chromatic Scales," section "Twelve-Tone Music," pp. 77-80 (PDF page 74). The row chart is defined on p. 77, a complete 12 x 12 example is given on p. 78, and two compositional examples follow on pp. 78-79. The modular arithmetic formalization (entry(i,j) = a_j - a_i) appears in Chapter 7, pp. 91-92.

# Verification Notes

- Definition: Direct adaptation from Wright, p. 77, paragraphs 2-3
- Key Properties: Items 1-7 explicit in the source text; item 8 (formula) from Chapter 7; item 9 (12! count) from p. 77
- Examples: All three drawn directly from the source with page citations
- Confidence: HIGH -- explicit definition with complete worked example and full 12 x 12 chart in the source
- Cross-references: row-chart and n-tone-row-chart confirmed as distinct cards -- row-chart covers the 12-tone concept from Ch. 6, n-tone-row-chart covers the generalization from Ch. 7
- Re-extraction notes: Re-extracted from v2 card to v3.1 format; preserved: formula entry(i,j) = a_j - a_i, example row E-G-F#-A-G#-C-F-D-D#-C#-B-Bb, note about generalization to n-tone
