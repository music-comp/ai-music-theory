---
concept: Inversion and Transposition of Rows
slug: inversion-and-transposition-of-rows

category: modular-arithmetic
subcategory: serialism
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Twelve-Tone Music"

extraction_confidence: high

aliases:
  - row inversion
  - row transposition
  - twelve-tone operations

prerequisites:
  - prime-row
  - twelve-tone-technique
  - modular-arithmetic
extends: []
related:
  - retrograde
  - n-tone-row-chart
  - modular-chromatic-intervals
contrasts_with:
  - retrograde

answers_questions:
  - "How are the inversion and transpositions of a twelve-tone row defined?"
  - "How does the formula entry(i,j) = a_j - a_i generate the entire row chart?"
---

# Quick Definition

Operations on a twelve-tone row that generate the full row chart: inversion reverses intervals from the starting note, while transposition shifts the entire row by a fixed interval.

# Core Definition

Given a prime row a_1 = [0], a_2, ..., a_n in Z_n (Wright, pp. 77-78, 91-92):
- The **inversion** is the sequence -a_1, -a_2, ..., -a_n (negation in Z_n), forming the left column of the row chart.
- The **transpositions** are the subsequent rows, each starting with -a_i (from the inversion column) and maintaining the same interval pattern as the prime row.
- The entry at position (i, j) is a_j - a_i in Z_n.
- Columns are transpositions of the inversion, or equivalently, inversions of transpositions.

# Prerequisites

- **Prime row** — The inversion and transpositions are derived from the prime row
- **Twelve-tone technique** — These operations are fundamental to the technique
- **Modular arithmetic** — The operations are defined using Z_n arithmetic

# Key Properties

1. Inversion negates each modular integer: [k] -> [-k] = [n - k]
2. Transposition by [c] shifts each entry: [k] -> [k + c]
3. Entry at (i, j) = a_j - a_i encodes both inversion and transposition
4. Row i is the prime row transposed by -a_i
5. Column j is the inversion transposed by a_j
6. These operations commute: transposing the inversion = inverting the transposition
7. Together with retrograde, these yield 4n distinct sequences (n transpositions x 2 for prime/inversion x 2 for forward/retrograde)

# Construction / Recognition

## To Generate the Row Chart
1. Start with the prime row: a_1 = [0], a_2, ..., a_n
2. Compute the inversion (left column): -a_1 = [0], -a_2, ..., -a_n
3. For each row i, compute entry(i, j) = a_j - a_i for all j
4. Convert modular integers to note names using a modular clock

# Context & Application

The inversion reverses the direction of every interval while preserving interval sizes. If the prime row goes up a minor third then down a semitone, the inversion goes down a minor third then up a semitone. Together with retrograde, these operations provide 48 distinct sequences in the 12-chromatic case (12 transpositions x 2 x 2).

# Examples

**Example 1** (pp. 91-92): Prime row from E: [0], [3], [2], [5], [4], [8], [1], [10], [11], [9], [7], [6]. Inversion: [0], [9], [10], [7], [8], [4], [11], [2], [1], [3], [5], [6].

**Example 2** (p. 92): Entry at position (8, 5) = a_5 - a_8 = [4] - [10] = [6] in Z_12.

**Example 3** (p. 92): Row 2 starts at -a_2 = [9] and transposes the prime row by [9].

# Relationships

## Builds Upon
- **Prime row** — Inversion and transposition derive from the prime row
- **Modular arithmetic** — Operations are defined in Z_n

## Enables
- **N-tone row chart** — The same formula works for any n

## Related
- **Modular chromatic intervals** — The entries of the chart are modular chromatic intervals

## Contrasts With
- **Retrograde** — Retrograde reverses order; inversion reverses interval direction. These are independent operations

# Common Errors

- **Error**: Computing inversion by reversing the order of notes (confusing with retrograde)
  **Correction**: Inversion negates each interval in Z_n; retrograde reverses the sequence order

# Common Confusions

- **Confusion**: Thinking inversion in twelve-tone theory is the same as "melodic inversion" in tonal music
  **Clarification**: Twelve-tone inversion negates each interval modulo octave exactly; tonal melodic inversion may not preserve exact interval sizes

- **Confusion**: Believing each row of the chart is independently constructed
  **Clarification**: Each row is algebraically determined by the formula entry(i,j) = a_j - a_i

# Source Reference

Chapter 6: "Chromatic Scales," pp. 77-78 (Twelve-Tone Music section). Modular arithmetic formulation in Chapter 7, pp. 91-92.

# Verification Notes

- Definition source: Direct from Wright, pp. 77-78, 91-92
- Confidence rationale: High — explicit formulas and worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: specific entry computation example (8,5), commutativity of operations, 48 distinct sequences count
