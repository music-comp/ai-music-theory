---
# === CORE IDENTIFICATION ===
concept: Modular Chromatic Intervals
slug: modular-chromatic-intervals

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
section: "The Group of Modular Chromatic Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - chromatic interval classes
  - interval classes mod octave

# === TYPED RELATIONSHIPS ===
prerequisites:
  - modular-arithmetic
  - octave-equivalence-formalized
extends:
  - modular-arithmetic
related:
  - z-twelve-as-chromatic-interval-group
  - modular-clock
  - generating-interval
  - group-of-modular-intervals
contrasts_with:
  - group-of-modular-intervals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are modular chromatic intervals?"
  - "How does Z_12 represent chromatic interval classes?"
---

# Quick Definition

Chromatic intervals considered modulo octave, forming equivalence classes identified with elements of Z_12, with composition given by addition in the group.

# Core Definition

A modular chromatic interval is an equivalence class of keyboard intervals under octave equivalence. Since keyboard intervals are measured in whole semitones, the set of modular chromatic intervals is identified with Z_12. Each has a unique representative n semitones with 0 <= n <= 11. The law of composition is addition in Z_12, and iteration of intervals corresponds to a sequence of rotations on the modular clock (Wright, pp. 88-89).

# Prerequisites

- **Modular arithmetic** — Interval composition is addition in Z_12
- **Octave equivalence formalized** — Modular chromatic intervals are defined by octave equivalence

# Key Properties

1. There are exactly 12 distinct modular chromatic intervals
2. Each has a unique representative in {0, 1, ..., 11} semitones
3. Composition = addition in Z_12
4. The group is cyclic with generators [1], [5], [7], [11]
5. The modular clock provides visualization
6. For non-standard scales, modular chromatic intervals form Z_n

# Construction / Recognition

## To Compose Modular Chromatic Intervals
1. Express each interval in semitones
2. Add the semitone values
3. Reduce modulo 12
4. The result is the composed modular interval

# Context & Application

Under octave equivalence, every chromatic interval computation reduces to Z_12 arithmetic. The modular clock labeled with note names allows quick conversion between numerical and musical representations, essential for twelve-tone composition and analysis.

# Examples

**Example 1** (p. 89): Minor third + octave + fourth = [3] + [12] + [5] = [3] + [0] + [5] = [8] (augmented fifth), since 20 = 8 (mod 12).

**Example 2** (p. 89): The interval names: [0]=unison, [1]=semitone, [2]=whole step, [3]=minor third, [4]=major third, [5]=fourth, [6]=tritone, [7]=fifth, [8]=minor sixth, [9]=major sixth, [10]=minor seventh, [11]=major seventh.

**Example 3** (exercises): Six fifths: 6 * [7] = [42] = [6] (a tritone). Up three minor thirds, down six steps: 3 * [3] + (-6) * [2] = [9] + [-12] = [9].

# Relationships

## Builds Upon
- **Modular arithmetic** — Composition is addition in Z_12
- **Octave equivalence formalized** — The defining equivalence relation

## Enables
- **Z_12 as chromatic interval group** — The detailed algebraic analysis of Z_12 as the interval group

## Related
- **Modular clock** — The visualization tool
- **Generating interval** — The generators of Z_12 are the musically significant modular intervals

## Contrasts With
- **Group of modular intervals** — That group includes all intervals (including microtonal) mod octave; modular chromatic intervals are the discrete subgroup Z_12

# Common Errors

- **Error**: Treating modular chromatic intervals as specific numbers rather than equivalence classes
  **Correction**: [5] includes 5, 17, 29, -7, etc.; it is a class, not a single number

# Common Confusions

- **Confusion**: Thinking "going up 14 semitones" and "going up 2 semitones" are different modular intervals
  **Clarification**: They are the same modular chromatic interval [2], since 14 = 2 (mod 12)

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 88-89 (The Group of Modular Chromatic Intervals section).

# Verification Notes

- Definition source: Direct from Wright, pp. 88-89
- Confidence rationale: High — explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete interval name table, composition examples, six-fifths calculation
