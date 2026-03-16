---
# === CORE IDENTIFICATION ===
concept: N-Chromatic Scale
slug: n-chromatic-scale

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
  - n-tone equal temperament
  - n-EDO
  - equal division of the octave

# === TYPED RELATIONSHIPS ===
prerequisites:
  - multiplicative-to-additive-conversion
  - exponents-and-exponential-functions
extends: []
related:
  - twelve-chromatic-scale
  - non-standard-chromatic-scales
  - generating-interval
  - modular-integers
contrasts_with:
  - twelve-chromatic-scale

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an n-chromatic scale?"
  - "How is the chromatic unit defined for an arbitrary equal division of the octave?"
---

# Quick Definition

A scale that divides the octave into n equal intervals, generalizing the standard 12-tone chromatic scale to arbitrary equal divisions of the octave.

# Core Definition

For a positive integer n, the n-chromatic scale is the scale obtained by dividing the octave into n equal intervals. The smallest interval (the n-chromatic unit) has ratio 2^(1/n) = nth_root(2) and measures 1200/n cents. The standard chromatic scale is the special case n = 12 (Wright, p. 74).

# Prerequisites

- **Multiplicative-to-additive conversion** — Understanding how ratios and cents relate is needed to define the n-chromatic unit
- **Exponents and exponential functions** — The ratio 2^(1/n) requires understanding of exponential expressions

# Key Properties

1. The n-chromatic unit has interval ratio 2^(1/n) = nth_root(2)
2. The n-chromatic unit measures 1200/n cents
3. The kth note above the starting pitch has frequency f * 2^(k/n)
4. The set of intervals modulo octave is identified with the group Z_n
5. When n divides 12 (n = 1, 2, 3, 4, 6, 12), the scale can be played on a standard keyboard without detuning

# Construction / Recognition

## To Construct an n-Chromatic Scale
1. Choose a starting frequency f and the value n
2. Compute the chromatic unit ratio: 2^(1/n)
3. Successive notes have frequencies f, f * 2^(1/n), f * 2^(2/n), ..., f * 2^((n-1)/n), f * 2 = 2f
4. Each step is 1200/n cents
5. If n does not divide 12, a synthesizer with detuning capability is needed

# Context & Application

The 12-chromatic scale is the foundation of Western equal temperament. Non-standard values of n (such as 5, 7, 14, 19, 24, 48) produce alternative tuning systems explored by microtonal composers. The mathematical framework applies uniformly regardless of n, with the algebraic structure captured by Z_n.

# Examples

**Example 1** (p. 74): n = 12: the standard chromatic scale, with unit = 100 cents (the semitone).

**Example 2** (p. 74): n = 4: unit = 300 cents (minor third); playable on keyboard as G, Bb, Db, E.

**Example 3** (p. 74): n = 3: unit = 400 cents (major third). n = 6: unit = 200 cents (whole step).

**Example 4** (p. 74): n = 5: unit = 240 cents; requires detuning to play on a keyboard.

**Example 5** (p. 75): n = 14: unit ~ 85.714 cents; used extensively in worked examples.

# Relationships

## Builds Upon
- **Multiplicative-to-additive conversion** — The formula x = n * log_2(r) converts ratios to n-chromatic units

## Enables
- **Generating interval** — Generating intervals are defined within n-chromatic scales
- **Non-standard chromatic scales** — Any n != 12 produces a non-standard chromatic scale
- **N-tone row chart** — Composition technique using Z_n arithmetic

## Related
- **Modular integers** — The group Z_n represents the modular intervals of the n-chromatic scale

## Contrasts With
- **Twelve-chromatic scale** — The special case n = 12 that dominates Western music

# Common Errors

- **Error**: Computing the n-chromatic unit as 1200*n cents instead of 1200/n cents
  **Correction**: The unit is 1200/n cents (dividing 1200 cents by n equal parts)

# Common Confusions

- **Confusion**: Believing the n-chromatic scale divides the octave equally by frequency difference
  **Clarification**: It divides equally by frequency ratio; the intervals are equal in logarithmic (cents) measure, not in Hz

- **Confusion**: Thinking any n-chromatic scale can be played on a standard keyboard
  **Clarification**: Only those where n divides 12 (n = 1, 2, 3, 4, 6, 12) use standard tuning; others require detuning

# Source Reference

Chapter 6: "Chromatic Scales," p. 74. See the definition of n-chromatic scale and examples for various values of n.

# Verification Notes

- Definition source: Direct from Wright, p. 74
- Confidence rationale: High — explicit definition with multiple examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: divisibility condition for keyboard playability, all original examples
