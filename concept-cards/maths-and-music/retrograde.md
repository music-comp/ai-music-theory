---
concept: Retrograde
slug: retrograde

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
  - retrograde row
  - row reversal

prerequisites:
  - prime-row
  - twelve-tone-technique
extends: []
related:
  - inversion-and-transposition-of-rows
  - retrogression
  - n-tone-row-chart
contrasts_with:
  - inversion-and-transposition-of-rows

answers_questions:
  - "What is a retrograde in twelve-tone music?"
  - "How is the retrograde of a row obtained?"
---

# Quick Definition

The reversal of a sequence of note classes from a row chart, reading a row from right to left or a column from bottom to top.

# Core Definition

Given a sequence a_1, a_2, ..., a_n from a row or column of an n-tone row chart, its retrograde is the sequence a_n, a_(n-1), ..., a_1. The retrograde of a row is obtained by reading right to left; the retrograde of a column by reading bottom to top (Wright, p. 78).

# Prerequisites

- **Prime row** — The retrograde is defined relative to a row in the chart
- **Twelve-tone technique** — The retrograde is one of the fundamental row operations

# Key Properties

1. The retrograde reverses the order of note classes
2. It is an involution: applying retrograde twice returns the original
3. The retrograde preserves the set of notes but reverses their ordering
4. Combined with inversion, it produces the retrograde-inversion (a fourth transformation)

# Construction / Recognition

## To Compute the Retrograde
1. Take any row or column of the row chart
2. Write its elements in reverse order
3. The retrograde of a row: read right to left
4. The retrograde of a column: read bottom to top

# Context & Application

Retrograde is one of the four fundamental row transformations in twelve-tone music (prime, inversion, retrograde, retrograde-inversion). Composers use retrogrades to create musical material that maintains the structural integrity of the row while introducing variety. The rows, columns, and their retrogrades form the complete palette of melodic and harmonic resources.

# Examples

**Example 1** (pp. 77-78): If the original row is E, G, F#, A, G#, C, F, D, D#, C#, B, Bb, its retrograde is Bb, B, C#, D#, D, F, C, G#, A, F#, G, E.

**Example 2** (p. 78): In the musical example, the top treble clef line uses the retrograde of the original row.

**Example 3** (p. 78): The bottom treble clef line uses a column of the row chart (a transposition of the inversion), not a retrograde.

# Relationships

## Builds Upon
- **Prime row** — The retrograde reverses the prime row or other chart sequences
- **Twelve-tone technique** — Retrograde is one of the four fundamental operations

## Enables
- Retrograde-inversion: the retrograde of the inversion, a fourth distinct transformation

## Related
- **Retrogression** — A related concept from tonal music (time reversal of melodic material)
- **N-tone row chart** — Retrogrades apply equally to n-tone charts

## Contrasts With
- **Inversion and transposition of rows** — Inversion reverses interval direction; retrograde reverses note order. These are distinct operations

# Common Errors

- **Error**: Confusing retrograde with inversion
  **Correction**: Retrograde reverses the order of note classes; inversion reverses the direction of intervals. They operate in different dimensions

# Common Confusions

- **Confusion**: Thinking retrograde changes the identity of note classes
  **Clarification**: Retrograde preserves the set of notes; it only changes their order

- **Confusion**: Believing "retrograde" in twelve-tone music is the same as "retrogression" in tonal contexts
  **Clarification**: In twelve-tone theory, retrograde has a precise meaning (reversal of a chart sequence)

# Source Reference

Chapter 6: "Chromatic Scales," pp. 78-79 (Twelve-Tone Music section). See the retrogrades in the musical examples.

# Verification Notes

- Definition source: Direct from Wright, p. 78
- Confidence rationale: High — explicitly defined with musical examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: E-row retrograde example, distinction from inversion
