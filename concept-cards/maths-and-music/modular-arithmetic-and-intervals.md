---
# === CORE IDENTIFICATION ===
concept: Modular Arithmetic and Intervals
slug: modular-arithmetic-and-intervals

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: twelve-tone
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Octave identification"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - interval arithmetic mod 12
  - chromatic interval composition

# === TYPED RELATIONSHIPS ===
prerequisites:
  - modular-arithmetic
  - octave-equivalence-formalized
extends:
  - modular-arithmetic
related:
  - modular-chromatic-intervals
  - z-twelve-as-chromatic-interval-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does modular arithmetic apply to musical interval composition?"
  - "How are intervals composed under octave equivalence?"
---

# Quick Definition

The application of Z_12 (or Z_n) arithmetic to compose, iterate, and analyze musical intervals under octave equivalence.

# Core Definition

Under octave equivalence, chromatic intervals measured in semitones compose by addition in Z_12. Going up an octave is the identity ([12] = [0]). The interval of a fourth followed by a fifth yields unison ([5] + [7] = [0]). Two fifths equal a whole step ([7] + [7] = [2]). This identification parameterizes chromatic interval composition by the modular group Z_12 (Wright, p. 82).

# Prerequisites

- **Modular arithmetic** — Interval composition is modular arithmetic
- **Octave equivalence formalized** — The equivalence relation that motivates the modular framework

# Key Properties

1. Interval composition = addition in Z_12
2. The octave = identity: [12] = [0]
3. Fourth + fifth = unison: [5] + [7] = [0]
4. Two fifths = whole step: [7] + [7] = [2]
5. Every chromatic interval has a unique representative in {0, 1, ..., 11}
6. Iteration of intervals = repeated addition in Z_12

# Construction / Recognition

## To Compose Intervals Modularly
1. Express each interval in semitones
2. Add the values
3. Reduce modulo 12
4. Interpret the result as a chromatic interval

# Context & Application

Modular arithmetic provides the computational framework for all chromatic interval analysis under octave equivalence. It is used extensively in twelve-tone composition (row chart construction), pitch-class set theory, and the analysis of interval cycles. The same framework applies to Z_n for non-standard chromatic scales.

# Examples

**Example 1** (p. 82): A fourth + a fifth = [5] + [7] = [12] = [0] (unison).

**Example 2** (p. 82): Two fifths = [7] + [7] = [14] = [2] (a whole step).

**Example 3** (p. 89): Minor third + octave + fourth = [3] + [0] + [5] = [8] (augmented fifth).

**Example 4** (exercise): Six fifths = 6 * [7] = [42] = [6] (tritone).

# Relationships

## Builds Upon
- **Modular arithmetic** — The underlying algebraic framework
- **Octave equivalence formalized** — The musical motivation

## Enables
- **Modular chromatic intervals** — The formal treatment of interval classes
- **Z_12 as chromatic interval group** — The full algebraic analysis

## Related
- **Modular chromatic intervals** — The interval classes that result from this arithmetic

# Common Errors

- **Error**: Computing interval compositions without reducing modulo 12
  **Correction**: Always reduce modulo 12 to get the canonical representative; 14 semitones -> [2], not [14]

# Common Confusions

- **Confusion**: Thinking modular interval composition loses information
  **Clarification**: It intentionally discards octave information, retaining only the interval class; this is the mathematical formalization of octave equivalence

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 82 (Octave identification section) and pp. 88-89 (interval examples).

# Verification Notes

- Definition source: Synthesized from Wright, pp. 82, 88-89
- Confidence rationale: High — concept explicitly demonstrated throughout chapter
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card (which was from Ch. 3); updated provenance to Ch. 7 where the formal treatment appears. Preserved: ninth=second example, interval composition examples
