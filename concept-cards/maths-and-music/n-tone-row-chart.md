---
# === CORE IDENTIFICATION ===
concept: N-Tone Row Chart
slug: n-tone-row-chart

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: serialism
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Creating an n-Tone Row Chart Using Modular Arithmetic"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - generalized row chart
  - n-tone serial chart

# === TYPED RELATIONSHIPS ===
prerequisites:
  - modular-arithmetic
  - prime-row
  - inversion-and-transposition-of-rows
extends:
  - twelve-tone-technique
related:
  - detuning
  - non-standard-chromatic-scales
  - modular-clock
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is a row chart constructed for an arbitrary n-chromatic scale?"
  - "How does the formula entry(i,j) = a_j - a_i generalize to any n?"
---

# Quick Definition

A generalization of the twelve-tone row chart to any n-chromatic scale, using Z_n arithmetic to construct an n x n array for composition in non-standard equal temperaments.

# Core Definition

Given an original row a_1 = [0], a_2, ..., a_n from Z_n, the n x n row chart is constructed by setting entry(i, j) = a_j - a_i in Z_n. The arithmetic takes place in Z_n. The method requires detuning a synthesizer to play in n-tone equal temperament, where the chromatic unit is 1200/n cents (Wright, pp. 92-94).

# Prerequisites

- **Modular arithmetic** — The chart is constructed using Z_n arithmetic
- **Prime row** — The original row determines the chart
- **Inversion and transposition of rows** — The chart encodes these operations

# Key Properties

1. The formula entry(i,j) = a_j - a_i works identically in Z_n for any n
2. The top row is the original row
3. The left column is the inversion (negation of original row entries)
4. Each row is a transposition of the original
5. Each column is a transposition of the inversion
6. A modular clock labeled with note classes aids in converting between Z_n and note names

# Construction / Recognition

## To Build an n-Tone Row Chart
1. Choose n and detune the synthesizer for n-tone equal temperament
2. Assign note classes to elements of Z_n using a modular clock
3. Choose an original row a_1 = [0], a_2, ..., a_n (a permutation of Z_n)
4. Compute each entry: entry(i, j) = a_j - a_i in Z_n
5. Convert from Z_n to note classes using the clock

# Context & Application

The technique extends serialist composition to any equal division of the octave. A 7-tone row chart uses detuned white keys with intervals of ~171.43 cents. The resulting music has distinctive qualities with intervals unfamiliar to ears trained on 12-tone temperament.

# Examples

**Example 1** (pp. 93-94): 7-tone row chart with original row [0], [4], [1], [6], [5], [2], [3] in Z_7, using detuned white keys C, D, E, F, G, A, B.

**Example 2** (p. 93): Detuning for 7-tone from C: D = -28.57, E = -57.14, F = +114.29, G = +85.71, A = +57.14, B = +28.57 cents.

**Example 3** (pp. 91-92): The 12-tone row chart for E, G, F#, A, G#, C, F, D, D#, C#, B, Bb is constructed using the same formula in Z_12.

**Example 4** (p. 94): A 7-tone composition uses the inversion of the original row (left column) melodically and harmonically.

# Relationships

## Builds Upon
- **Twelve-tone technique** — The n-tone chart generalizes the 12-tone chart
- **Modular arithmetic** — Z_n arithmetic constructs the chart
- **Prime row** — The original row input

## Enables
Composition in non-standard equal temperaments using serialist methods.

## Related
- **Detuning** — Required to realize non-standard scales on a keyboard
- **Modular clock** — Aids conversion between Z_n and note names

# Common Errors

- **Error**: Using 12-tone arithmetic (mod 12) when working with an n-tone chart
  **Correction**: All arithmetic must be performed in Z_n (mod n), not Z_12

# Common Confusions

- **Confusion**: Thinking n-tone composition requires different formulas than 12-tone
  **Clarification**: The formula entry(i,j) = a_j - a_i is the same for any n; only the modulus changes

- **Confusion**: Believing n-tone music sounds like 12-tone music transposed
  **Clarification**: The interval sizes are different (1200/n cents per unit), producing fundamentally different sonorities

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 92-94 (Creating an n-Tone Row Chart section). See the 7-tone row chart and composition example.

# Verification Notes

- Definition source: Direct from Wright, pp. 92-94
- Confidence rationale: High — explicit formula with complete worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: 7-tone example with detuning values, composition example description
