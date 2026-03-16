---
# === CORE IDENTIFICATION ===
concept: Modular Arithmetic
slug: modular-arithmetic

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
section: "Modular Arithmetic"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - clock arithmetic
  - arithmetic modulo m

# === TYPED RELATIONSHIPS ===
prerequisites:
  - modular-integers
  - group
extends:
  - modular-integers
related:
  - modular-chromatic-intervals
  - modular-clock
  - n-tone-row-chart
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is modular arithmetic?"
  - "How does modular arithmetic model interval composition under octave equivalence?"
---

# Quick Definition

The arithmetic of equivalence classes in Z_m, where operations are performed on representatives and the result is reduced modulo m, providing the computational framework for interval arithmetic under octave equivalence.

# Core Definition

The group Z_m is called a modular group, and operations involving its law of composition are called modular arithmetic. Addition in Z_m is defined by [k] + [l] = [k + l], and subtraction by [k] - [l] = [k - l]. Both are well-defined. Modular arithmetic can be visualized as rotations on the m-hour clock (Wright, p. 88).

# Prerequisites

- **Modular integers** — Modular arithmetic operates on elements of Z_m
- **Group** — Z_m is a group under addition

# Key Properties

1. Addition: [k] + [l] = [k + l]
2. Subtraction: [k] - [l] = [k - l] = [k] + [-l]
3. Both operations are well-defined (independent of representative choice)
4. Results have a unique representative in {0, 1, ..., m-1}
5. Computation can be performed on any representatives, then reduced mod m

# Construction / Recognition

## To Perform Modular Arithmetic
1. Choose any representatives for the operands
2. Perform the integer operation (addition or subtraction)
3. Reduce the result modulo m to get the canonical representative
4. Alternatively, use the modular clock for visualization

# Context & Application

Modular arithmetic is the algebra of intervals under octave equivalence. Every chromatic interval computation reduces to modular arithmetic in Z_12. Creating twelve-tone row charts uses modular arithmetic extensively: the entry at position (i, j) is a_j - a_i in Z_12 (or Z_n for n-tone charts).

# Examples

**Example 1** (p. 88): [6] + [13] = [19] = [1] in Z_9 (since 19 = 2 * 9 + 1).

**Example 2** (p. 82): [7] + [7] = [14] = [2] in Z_12 (two fifths = whole step).

**Example 3** (p. 82): [5] + [7] = [12] = [0] in Z_12 (fourth + fifth = unison).

**Example 4** (p. 92): [4] - [10] = [-6] = [6] in Z_12 (used to compute row chart entries).

# Relationships

## Builds Upon
- **Modular integers** — The elements on which modular arithmetic operates
- **Group** — The group structure of Z_m underlies the arithmetic

## Enables
- **Modular chromatic intervals** — Interval composition is modular arithmetic
- **N-tone row chart** — Row charts are constructed using modular arithmetic

## Related
- **Modular clock** — The geometric computation device for modular arithmetic

# Common Errors

- **Error**: Forgetting to reduce the result modulo m
  **Correction**: Always reduce to a representative in {0, 1, ..., m-1} for the canonical form

# Common Confusions

- **Confusion**: Thinking modular arithmetic is "rounding" or "approximation"
  **Clarification**: Modular arithmetic is exact arithmetic on equivalence classes, not approximation

- **Confusion**: Believing the result depends on which representatives are chosen
  **Clarification**: The result is the same regardless of representative choice; this is the well-definedness property

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 88 (Modular Arithmetic section).

# Verification Notes

- Definition source: Direct from Wright, p. 88
- Confidence rationale: High — explicitly defined
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_9 example, row chart entry computation example
