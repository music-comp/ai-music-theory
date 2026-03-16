---
concept: Nonstandard Chromatic Intervals
slug: nonstandard-chromatic-intervals

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Nonstandard Chromatic Intervals"

extraction_confidence: high

aliases:
  - n-chromatic intervals
  - non-twelve chromatic intervals

prerequisites:
  - modular-chromatic-intervals
  - n-chromatic-scale
extends:
  - modular-chromatic-intervals
related:
  - generating-interval
  - cyclic-group-and-generator
  - non-standard-chromatic-scales
contrasts_with:
  - modular-chromatic-intervals

answers_questions:
  - "How are modular intervals defined in non-standard chromatic scales?"
  - "What group structure governs intervals in an n-chromatic scale?"
---

# Quick Definition

Modular intervals in an n-chromatic scale where n != 12, identified with the group Z_n and providing the interval vocabulary for non-standard equal temperaments.

# Core Definition

If the octave is divided into n equal intervals and intervals are measured in n-chromatic units, the group of intervals modulo octave is identified with Z_n. Each element [k] in Z_n represents an interval of k * (1200/n) cents. The generating intervals are those [m] with gcd(m, n) = 1, and there are phi(n) such generators (Wright, p. 89).

# Prerequisites

- **Modular chromatic intervals** — Nonstandard intervals generalize the Z_12 framework
- **N-chromatic scale** — The underlying scale whose intervals are being classified

# Key Properties

1. The group of modular intervals is Z_n
2. Each [k] represents k * (1200/n) cents
3. Z_n is cyclic with phi(n) generators
4. For prime n, every non-zero element is a generator
5. For composite n, some intervals generate only subgroups

# Construction / Recognition

## To Work with Nonstandard Chromatic Intervals
1. Establish n (the division of the octave)
2. Identify each interval with an element [k] in Z_n
3. Compose intervals by adding in Z_n
4. Determine generators using the gcd criterion

# Context & Application

Non-standard chromatic intervals provide the building blocks for composition in alternative equal temperaments. Each choice of n creates a different palette of available intervals. The structure of Z_n determines which intervals can serve roles analogous to the fifth or fourth in standard harmony.

# Examples

**Example 1** (p. 75): In Z_5: all non-zero elements are generators (phi(5) = 4), since 5 is prime.

**Example 2** (p. 75): In Z_14: six generators [1], [3], [5], [9], [11], [13] (phi(14) = 6).

**Example 3** (implied): In Z_6: generators are [1] and [5] only (phi(6) = 2); [2], [3], [4] generate proper subgroups.

# Relationships

## Builds Upon
- **Modular chromatic intervals** — The n != 12 generalization
- **N-chromatic scale** — Provides the underlying tuning

## Enables
- **N-tone row chart** — Composition using Z_n arithmetic

## Related
- **Generating interval** — The generators of Z_n for non-standard scales
- **Cyclic group and generator** — The algebraic structure underlying nonstandard intervals

## Contrasts With
- **Modular chromatic intervals** — The standard case uses Z_12; nonstandard uses Z_n for n != 12

# Common Errors

- **Error**: Measuring nonstandard chromatic intervals in semitones
  **Correction**: Use n-chromatic units (each = 1200/n cents), not semitones (each = 100 cents)

# Common Confusions

- **Confusion**: Thinking the group structure is the same for all n
  **Clarification**: The number of generators (phi(n)) and the subgroup structure vary with n; prime n gives the richest generator structure

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 89 (Nonstandard Chromatic Intervals section).

# Verification Notes

- Definition source: Direct from Wright, p. 89
- Confidence rationale: High — explicit definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_5, Z_14, Z_6 examples with phi values
