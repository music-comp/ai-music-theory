---
# === CORE IDENTIFICATION ===
concept: Irrationality of Equally-Tempered Intervals
slug: irrationality-of-equally-tempered-intervals

# === CLASSIFICATION ===
category: rational-intervals
subcategory: just-intervals
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Irrationality of Equally-Tempered Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - irrationality theorem for equal temperament

# === TYPED RELATIONSHIPS ===
prerequisites:
  - unique-factorization-in-positive-rationals
  - rational-interval
extends: []
related:
  - equal-temperament-versus-just-intonation
  - symmetric-chords-and-irrational-temperament
contrasts_with:
  - just-interval

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Are equally tempered intervals rational?"
  - "Why are equal temperament and just intonation fundamentally incompatible?"
---

# Quick Definition

All intervals in any equally-tempered scale are irrational (have irrational frequency ratios) except for iterations of the octave. This theorem explains why equal temperament and just intonation are inherently incompatible.

# Core Definition

"Theorem: Let I be an interval between two notes in the n-chromatic scale. If I is not the iteration of octaves (i.e., the ratio corresponding to I is not a power of 2), then I is an irrational interval" (Wright, p. 145). The proof uses unique factorization in Q+: if x is rational and x^n = 2^k, then x must itself be a power of 2, making I a multi-octave.

# Prerequisites

- **Unique factorization in positive rationals** -- The proof relies on uniqueness of prime factorization in Q+
- **Rational interval** -- Understanding the distinction between rational and irrational intervals

# Key Properties

1. Applies to all equally-tempered scales (any n-tone), not just 12-tone
2. The only rational equal-tempered intervals are multi-octaves (powers of 2)
3. Every tempered fifth, third, semitone, etc. has an irrational frequency ratio
4. The proof is a direct consequence of unique prime factorization
5. Implies that no equal-tempered interval is exactly just

# Construction / Recognition

## Proof Sketch (pp. 145-146)

1. Suppose interval I has rational ratio x in Q+
2. Since I lies in Z_n (the modular group), it has finite order: x^n = 2^k for some integers n, k
3. Write x = p1^a1 * p2^a2 * ... * pr^ar (unique factorization)
4. Then x^n = p1^(n*a1) * ... * pr^(n*ar) = 2^k
5. Uniqueness forces 2 to be the only prime, so x = 2^(a1)
6. Therefore I is a multi-octave, contradicting the assumption

# Context & Application

This theorem tells us that no equally-tempered interval (other than the octave) can be a just interval. However, many just intervals are closely approximated by tempered ones -- "this likely explains why the 12-chromatic scale gained acceptance" (p. 146). The theorem also implies that augmented triads and diminished seventh chords, which divide the octave equally, can only exist with irrational intervals and may be considered "a result of equal temperament."

# Examples

**Example 1** (p. 146): The tempered fifth 2^(7/12) is irrational (close to but not equal to 3/2).

**Example 2** (p. 146): The tempered major third 2^(1/3) is irrational (not equal to 5/4).

**Example 3** (p. 146): The tempered semitone 2^(1/12) is irrational.

**Example 4** (p. 146): Only multi-octaves (2^k for integer k) are rational among equally-tempered intervals.

# Relationships

## Builds Upon
- **Unique factorization in positive rationals** -- The proof mechanism

## Enables
- **Equal temperament versus just intonation** -- The mathematical basis for their incompatibility
- **Symmetric chords and irrational temperament** -- Symmetric chords require irrational intervals

## Related
- **Comma of Pythagoras** -- Equal temperament resolves this by accepting irrational fifths

## Contrasts With
- **Just interval** -- Just intervals are rational; tempered intervals are irrational (except octaves)

# Common Errors

- **Error**: Thinking the theorem only applies to 12-tone equal temperament
  **Correction**: It applies to any n-tone equally-tempered scale

# Common Confusions

- **Confusion**: Thinking the theorem says equal temperament is "wrong"
  **Clarification**: It says equal temperament and just intonation are mathematically distinct systems that can only approximate each other

- **Confusion**: Thinking irrational intervals cannot sound acceptable
  **Clarification**: Many tempered intervals closely approximate just intervals (e.g., tempered fifth is only ~2 cents off); irrationality does not imply poor sound quality

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 145-146.

# Verification Notes

- Definition source: Direct theorem statement from p. 145
- Confidence rationale: Formal theorem with complete proof
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: proof sketch, symmetric chord observation, generality to n-tone scales
