---
# === CORE IDENTIFICATION ===
concept: Prime Row
slug: prime-row

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
  - original row
  - tone row

# === TYPED RELATIONSHIPS ===
prerequisites:
  - twelve-tone-technique
extends: []
related:
  - retrograde
  - inversion-and-transposition-of-rows
  - n-tone-row-chart
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a prime row in twelve-tone composition?"
  - "How does the prime row determine the entire row chart?"
---

# Quick Definition

The original, top row of a twelve-tone row chart, consisting of an ordered sequence of all 12 (or n) note classes that determines the entire chart through the formula entry(i, j) = a_j - a_i.

# Core Definition

The prime row (or original row) is an ordered sequence a_1, a_2, ..., a_n of all elements of Z_n, with a_1 = [0] (when using modular integer representation). It occupies the first row of the row chart and, through the formula entry(i, j) = a_j - a_i, determines every other entry (Wright, pp. 77, 91-93).

# Prerequisites

- **Twelve-tone technique** — The prime row is the creative input to this compositional method

# Key Properties

1. Contains each of the n note classes exactly once
2. When expressed in modular integers, a_1 = [0]
3. The entire row chart is determined by the prime row alone
4. The number of possible prime rows in the 12-chromatic scale is 12! = 479,001,600
5. The prime row is typically used melodically (horizontally) in composition

# Construction / Recognition

## To Create a Prime Row
1. Choose a designated starting note class (assigned [0])
2. Order all remaining note classes in any desired sequence
3. Express each note as its modular chromatic interval from the first note
4. Verify that each element of Z_n appears exactly once

# Context & Application

The prime row is the creative choice that defines a twelve-tone composition. The composer selects an ordering of all 12 note classes, and this single decision generates the entire row chart through inversion and transposition. The spelling of notes (sharps vs. flats) may vary and does not affect the row chart structure.

# Examples

**Example 1** (p. 77): E, G, F#, A, G#, C, F, D, D#, C#, B, Bb. Expressed as modular integers from E: [0], [3], [2], [5], [4], [8], [1], [10], [11], [9], [7], [6].

**Example 2** (p. 93): In a 7-tone example, C, G, D, B, A, E, F corresponds to [0], [4], [1], [6], [5], [2], [3] in Z_7.

**Example 3** (p. 77): The spelling uses sharp four times and flat once, demonstrating the mixed accidental convention in twelve-tone music.

# Relationships

## Builds Upon
- **Twelve-tone technique** — The prime row is the input to the technique

## Enables
- **Inversion and transposition of rows** — The inversion and transpositions are derived from the prime row
- **Retrograde** — The retrograde is the prime row read in reverse

## Related
- **N-tone row chart** — Prime rows can be defined for any n, not just 12

# Common Errors

- **Error**: Starting the modular integer representation with a value other than [0]
  **Correction**: The first entry must be [0] for the inversion formula (negation) to work correctly

# Common Confusions

- **Confusion**: Thinking "prime" refers to prime numbers
  **Clarification**: "Prime" here means "first" or "original"

- **Confusion**: Believing the composer has creative freedom in filling the remaining rows
  **Clarification**: The prime row determines the entire chart; there is no additional creative input beyond the prime row

# Source Reference

Chapter 6: "Chromatic Scales," pp. 77-78 (Twelve-Tone Music section). See also Chapter 7, pp. 91-93, for the modular arithmetic formulation.

# Verification Notes

- Definition source: Direct from Wright, pp. 77, 91-93
- Confidence rationale: High — explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: E-based example with modular integers, 7-tone example, "prime" etymology note
