---
# === CORE IDENTIFICATION ===
concept: Non-Standard Chromatic Scales
slug: non-standard-chromatic-scales

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Non-standard chromatic scales"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - alternative equal temperaments
  - non-twelve equal divisions

# === TYPED RELATIONSHIPS ===
prerequisites:
  - n-chromatic-scale
extends:
  - n-chromatic-scale
related:
  - detuning
  - approximating-standard-keyboard-intervals
  - twelve-chromatic-scale
contrasts_with:
  - twelve-chromatic-scale

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens when the octave is divided into something other than 12 equal parts?"
  - "How can non-standard chromatic scales be realized on a synthesizer?"
---

# Quick Definition

Chromatic scales that divide the octave into a number of equal intervals other than 12, producing alternative tuning systems with unfamiliar interval sizes.

# Core Definition

A non-standard chromatic scale is an n-chromatic scale where n != 12. The chromatic unit has ratio 2^(1/n) and measures 1200/n cents. Standard keyboard intervals can be approximated by finding the nearest whole number of n-chromatic units (Wright, p. 74).

# Prerequisites

- **N-chromatic scale** — Non-standard chromatic scales are the cases where n != 12

# Key Properties

1. The chromatic unit is 1200/n cents (differs from 100 cents when n != 12)
2. To convert l semitones to n-chromatic units: l * (n/12) n-chromatic units
3. To convert any ratio r to n-chromatic units: x = n * log_2(r) = n * ln(r)/ln(2)
4. Some standard keyboard intervals may be well-approximated; others may not
5. The tritone is exactly n/2 units in any even-n scale

# Construction / Recognition

## To Experience a Non-Standard Scale
1. Choose n and compute the chromatic unit: 1200/n cents
2. If n divides 12, select the appropriate subset of keyboard keys (no detuning needed)
3. If n does not divide 12, detune synthesizer keys to achieve equal spacing of 1200/n cents
4. Use only the detuned keys to play in the n-chromatic scale

# Context & Application

Non-standard chromatic scales can be experienced using synthesizer detuning. When n < 12, one can detune existing keys. The mathematical framework (conversion formulas, group structure) applies uniformly regardless of n, making it straightforward to analyze interval relationships in any equal temperament.

# Examples

**Example 1** (p. 74): n = 5: detuning required. Starting on G, detune A (+40 cents), B (+80 cents), C (+220 cents), D (+260 cents) to create equal 240-cent intervals.

**Example 2** (pp. 75-76): n = 14: the chromatic unit is ~85.714 cents. A fourth (5 semitones) ~ 5.833 14-chromatic units, best approximated by 6 units (~514.29 cents, about 14.29 cents sharp).

**Example 3** (p. 76): The ratio 0.75 in 14-chromatic units: 14 * ln(0.75)/ln(2) ~ -5.81 units (5.81 units downward).

# Relationships

## Builds Upon
- **N-chromatic scale** — Non-standard scales are specific instances with n != 12

## Enables
- **Detuning** — The technique for realizing non-standard scales on a keyboard
- **Approximating standard keyboard intervals** — Comparing non-standard and standard intervals

## Related
- **Twelve-chromatic scale** — The standard case from which non-standard scales depart

## Contrasts With
- **Twelve-chromatic scale** — Non-standard scales use different interval sizes and may poorly approximate familiar intervals

# Common Errors

- **Error**: Assuming familiar chord qualities will transfer to non-standard scales
  **Correction**: Approximation quality varies by interval; a scale that well-approximates fifths may poorly approximate thirds

# Common Confusions

- **Confusion**: Thinking non-standard chromatic scales are non-equal temperaments
  **Clarification**: They still divide the octave equally, just into a different number of parts; non-equal temperaments are a different concept entirely

# Source Reference

Chapter 6: "Chromatic Scales," pp. 74-76. See the detuning examples for n = 5 and the approximation calculations for n = 14.

# Verification Notes

- Definition source: Direct from Wright, pp. 74-76
- Confidence rationale: High — explicit definition with worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: n=5 detuning example, n=14 approximation calculations
