---
# === CORE IDENTIFICATION ===
concept: Twelve-Tone Technique
slug: twelve-tone-technique

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: serialism
tier: advanced

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
  - dodecaphony
  - twelve-tone music
  - twelve-tone method

# === TYPED RELATIONSHIPS ===
prerequisites:
  - twelve-chromatic-scale
  - modular-chromatic-intervals
extends: []
related:
  - prime-row
  - retrograde
  - inversion-and-transposition-of-rows
  - n-tone-row-chart
  - z-twelve-as-chromatic-interval-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is twelve-tone technique?"
  - "How is a twelve-tone row chart constructed?"
  - "What is the relationship between twelve-tone technique and modular arithmetic?"
---

# Quick Definition

A method of musical composition developed by Arnold Schoenberg that uses ordered arrangements of all twelve chromatic note classes, largely abandoning traditional consonance in favor of combinatorics.

# Core Definition

Twelve-tone technique constructs music from ordered sets of all 12 pitch classes modulo octave. A composition is based on a row chart: a 12 x 12 array where each row and column contains each of the 12 note classes exactly once. All entries derive from the original (prime) row through inversion, transposition, and retrograde operations. The technique is a form of serial music (Wright, pp. 76-79).

# Prerequisites

- **Twelve-chromatic scale** — The technique requires the 12-tone equal temperament framework
- **Modular chromatic intervals** — Row chart construction uses Z_12 arithmetic

# Key Properties

1. Based on a row chart: a 12 x 12 array
2. Each row and column contains each note class exactly once
3. The original (prime) row determines the entire chart
4. The left column is the inversion of the top row
5. Subsequent rows are transpositions of the top row
6. The number of possible original rows is 12! = 479,001,600
7. Musical material uses rows, columns, and their retrogrades

# Construction / Recognition

## To Construct a Row Chart
1. Choose an original (prime) row: an ordering of all 12 note classes
2. Express each note as a modular interval from the first note ([0], [a_2], ..., [a_12])
3. The inversion (left column) is obtained by negating each entry: [0], [-a_2], ..., [-a_12]
4. Fill each row using the formula: entry(i, j) = a_j - a_i in Z_12
5. Convert modular integers back to note names using a modular clock

# Context & Application

Developed in the 1920s by Arnold Schoenberg (1874-1951) and continued by Anton Webern (1883-1945), Alban Berg (1885-1935), and Milton Babbitt (b. 1916). Since there is little feeling of tonal center, twelve-tone music is often written in the key of C. The spelling of notes may differ from the row chart and may change during the composition.

# Examples

**Example 1** (pp. 77-78): The prime row E, G, F#, A, G#, C, F, D, D#, C#, B, Bb generates a complete 12 x 12 row chart. The spelling mixes sharps and flats with no apparent pattern.

**Example 2** (p. 78): In a composition based on this chart, the bass clef uses the original row, the top treble line uses the retrograde of the original row, and the bottom treble line uses a column (transposition of the inversion).

**Example 3** (p. 79): A second example shows notes from a sequence assembled vertically (as chords) rather than horizontally (melodically).

# Relationships

## Builds Upon
- **Twelve-chromatic scale** — The technique operates within 12-tone equal temperament
- **Modular chromatic intervals** — Z_12 arithmetic governs row chart construction

## Enables
- **N-tone row chart** — Generalizes the technique to any n-chromatic scale

## Related
- **Prime row** — The original row that determines the entire chart
- **Retrograde** — Reversal of row/column sequences
- **Inversion and transposition of rows** — The operations that generate the chart
- **Z_12 as chromatic interval group** — The algebraic structure underlying the technique

# Common Errors

- **Error**: Assuming each row of the chart is independently chosen
  **Correction**: The prime row alone determines the entire chart via the formula entry(i,j) = a_j - a_i

# Common Confusions

- **Confusion**: Thinking twelve-tone music is random or atonal without structure
  **Clarification**: It follows strict combinatorial rules; the structure comes from the row chart rather than from tonal harmony

- **Confusion**: Believing the technique is limited to 12 tones
  **Clarification**: The method generalizes to n-tone row charts for any positive integer n

# Source Reference

Chapter 6: "Chromatic Scales," pp. 76-79 (Twelve-Tone Music section). See the row chart example and musical compositions.

# Verification Notes

- Definition source: Direct from Wright, pp. 76-79
- Confidence rationale: High — explicit definition with detailed examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: historical attribution, 12! count, vertical vs. horizontal usage distinction
