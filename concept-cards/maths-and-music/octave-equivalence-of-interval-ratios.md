---
concept: Octave Equivalence of Interval Ratios
slug: octave-equivalence-of-interval-ratios

category: pitch-and-intervals
subcategory: ratios
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Octave Equivalence of Interval Ratios"

extraction_confidence: high

aliases:
  - interval ratio equivalence modulo octave

prerequisites:
  - interval-as-frequency-ratio
  - multiplicative-composition-of-intervals
extends: []
related:
  - chords-as-note-class-collections
contrasts_with: []

answers_questions:
  - "When are two interval ratios equivalent modulo octave?"
  - "How does octave equivalence work with ratios instead of semitones?"
---

# Quick Definition

Two interval ratios r_1 and r_2 are equivalent modulo octave if they differ by some number of octaves, which in ratio terms means r_1 / r_2 = 2^n for some integer n.

# Core Definition

**PROPOSITION**: Two interval ratios r_1 and r_2 are equivalent modulo octave if and only if there exists n in Z such that r_1 * r_2^(-1) = 2^n.

Equivalently, r_1 = r_2 * 2^n. This defines an equivalence relation on R+. The difference of two intervals is the result of juxtaposing the first with the opposite of the second. If the intervals have ratios r_1 and r_2, this difference has ratio r_1 * r_2^(-1). The interval of n octaves has ratio 2^n (Wright, pp. 62-63).

# Prerequisites

- **Interval as Frequency Ratio** -- Must understand intervals as ratios
- **Multiplicative Composition of Intervals** -- The difference of intervals involves multiplication by inverses

# Key Properties

1. r_1 ~ r_2 (mod octave) iff r_1/r_2 = 2^n for some integer n
2. Each equivalence class contains a unique representative in [1, 2)
3. This is the multiplicative analog of congruence modulo 12 for semitones
4. The equivalence classes partition R+ by the action of the group {2^n : n in Z}
5. Taking log_2 transforms this into additive congruence modulo 1 (in octaves) or modulo 12 (in semitones)

# Construction / Recognition

## To Check Octave Equivalence

1. Given ratios r_1 and r_2
2. Compute r_1 / r_2
3. Check if the result is a power of 2 (i.e., 2^n for some integer n)
4. If yes: the ratios are equivalent modulo octave

# Context & Application

This formalizes the musical principle that intervals differing by whole octaves are "the same" in an important sense. A twelfth (octave plus a fifth) and a fifth are equivalent modulo octave. This principle is essential for identifying chord types and note classes across different registers (Wright, pp. 62-63).

# Examples

**Example 1** (p. 63): Ratios 41 and 328 are equivalent modulo octave, since 41/328 = 1/8 = 2^(-3).

**Example 2**: Ratio 3 (approximately an octave + a fifth) is equivalent to 3/2 (a fifth), since 3/(3/2) = 2 = 2^1.

**Example 3** (Exercise 9): Ratios 5 and 20 are equivalent: 5/20 = 1/4 = 2^(-2).

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- Octave equivalence is defined on interval ratios
- **Multiplicative Composition of Intervals** -- Uses multiplicative inverse to compute differences

## Related

- **Chords as Note Class Collections** -- Note classes are pitches identified modulo octave; interval ratios are intervals identified modulo octave

# Common Errors

- **Error**: Testing octave equivalence by subtracting ratios rather than dividing them
  **Correction**: Octave equivalence uses division (r_1/r_2 = 2^n), not subtraction

# Common Confusions

- **Confusion**: Thinking octave equivalence for ratios uses addition/subtraction of 12
  **Clarification**: That's the additive (semitone) version; for ratios, use multiplication/division by powers of 2
- **Confusion**: Believing two ratios must both be powers of 2 to be octave-equivalent
  **Clarification**: Any two ratios can be octave-equivalent as long as their quotient is a power of 2

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 62-63. Includes the formal Proposition.

# Verification Notes

- Definition source: Direct from pp. 62-63, stated as a Proposition
- Confidence rationale: High -- formally stated as a proposition with worked example
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Proposition statement, quotient space description, additive analog via log_2
