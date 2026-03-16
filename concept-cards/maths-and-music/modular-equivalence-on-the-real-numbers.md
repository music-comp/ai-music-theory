---
# === CORE IDENTIFICATION ===
concept: Modular Equivalence on the Real Numbers
slug: modular-equivalence-on-the-real-numbers

# === CLASSIFICATION ===
category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Modular Equivalence on the Real Numbers"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - real modular equivalence
  - equivalence modulo m on R

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-division-algorithm
extends: []
related:
  - modular-equivalence-on-the-integers
  - wrapping-real-line-around-circle
  - group-of-modular-intervals
  - octave-equivalence-formalized
contrasts_with:
  - modular-equivalence-on-the-integers

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is modular equivalence on the real numbers?"
  - "How does modular equivalence partition R into equivalence classes?"
---

# Quick Definition

An equivalence relation on real numbers where two numbers are equivalent if they differ by an integer multiple of a fixed modulus m, modeling the wrapping of the number line around a circle.

# Core Definition

For a fixed positive integer m, two real numbers x and y are equivalent, written x ~ y, if x - y = qm for some q in Z. This defines an equivalence relation on R, partitioning it into equivalence classes. The equivalence class of x is denoted x-bar. The set of equivalence classes is denoted R/~ (Wright, pp. 84-85).

# Prerequisites

- **Generalized division algorithm** — Guarantees each class has a unique representative in [0, m)

# Key Properties

1. x ~ y iff x - y is a multiple of m in Z
2. The relation partitions R into equivalence classes
3. Each equivalence class has exactly one representative r with 0 <= r < m
4. The set R/~ inherits addition: x-bar + y-bar = (x+y)-bar
5. The addition is well-defined (independent of representative choice)
6. (R/~, +) is a group, and the map w(x) = x-bar is a surjective homomorphism

# Construction / Recognition

## To Determine if x ~ y (mod m)
1. Compute x - y
2. Check if the result is an integer multiple of m
3. If (x - y)/m is an integer, then x ~ y; otherwise x is not equivalent to y

## To Find the Canonical Representative
1. Apply the Generalized Division Algorithm: x = qm + r with 0 <= r < m
2. The canonical representative is r

# Context & Application

When m = 12 (measuring intervals in semitones), modular equivalence on R models octave equivalence for all intervals, not just chromatic ones. This captures microtonal intervals and continuous pitch spaces. The wrapping of R around a circle of circumference m is the geometric visualization.

# Examples

**Example 1** (p. 85): With m = 8: 13-bar = 53-bar = (-11)-bar, since 13 - 53 = -40 = -5 * 8 and 13 - (-11) = 24 = 3 * 8.

**Example 2** (p. 85): With m = 8: 6.5-bar = (-1.5)-bar, since 6.5 - (-1.5) = 8 = 1 * 8.

**Example 3** (p. 85): Each equivalence class has a unique representative in [0, 8): the representative of 13-bar is 5, since 13 = 1 * 8 + 5.

# Relationships

## Builds Upon
- **Generalized division algorithm** — Provides unique representatives for each class

## Enables
- **Wrapping real line around circle** — The geometric visualization of R/~
- **Group of modular intervals** — (R/~, +) is the group of interval classes modulo octave

## Related
- **Octave equivalence formalized** — Modular equivalence with m = 12 formalizes octave equivalence

## Contrasts With
- **Modular equivalence on the integers** — The integer version is the restriction of ~ to Z, producing the finite set Z_m

# Common Errors

- **Error**: Assuming two equivalent numbers must both be integers
  **Correction**: The equivalence applies to ALL real numbers; 3.7 ~ 11.7 when m = 8

# Common Confusions

- **Confusion**: Thinking ~ is the same relation regardless of m
  **Clarification**: The equivalence relation depends on the choice of m, which must always be established in context

- **Confusion**: Believing the wrapping function w is an isomorphism
  **Clarification**: w: R -> R/~ is a surjective homomorphism but NOT an isomorphism, since infinitely many real numbers map to each class

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 84-85 (Modular Equivalence on the Real Numbers section).

# Verification Notes

- Definition source: Direct from Wright, pp. 84-85
- Confidence rationale: High — explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: m=8 examples, non-integer equivalence example (6.5 ~ -1.5), homomorphism characterization
