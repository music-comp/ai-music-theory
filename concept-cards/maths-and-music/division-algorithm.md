---
# === CORE IDENTIFICATION ===
concept: Division Algorithm
slug: division-algorithm

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: number-systems
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Some Properties of Integers"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Euclidean division"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - sets-and-number-systems
extends: []
related:
  - well-ordering-principle
  - octave-equivalence
  - note-classes
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Division Algorithm?"
  - "How does integer division relate to octave equivalence?"
  - "What are the quotient and remainder in integer division?"
---

# Quick Definition

A fundamental theorem stating that any integer can be divided by a positive integer to produce a unique quotient and non-negative remainder.

# Core Definition

Given $m \in \mathbb{Z}^+$ and $n \in \mathbb{Z}$, there exist $q, r \in \mathbb{Z}$ with $0 \leq r < m$ such that $n = qm + r$ (Wright, p. 14). Despite its name, the Division Algorithm is an existence theorem, not a computational procedure.

# Prerequisites

- **Sets and Number Systems** — The theorem operates on $\mathbb{Z}$ and $\mathbb{Z}^+$

# Key Properties

1. The remainder $r$ always satisfies $0 \leq r < m$, ensuring it is non-negative even when $n$ is negative
2. The quotient $q$ and remainder $r$ are unique for given $m$ and $n$
3. The proof relies on the Well-Ordering Principle
4. The case $r = 0$ means $m$ divides $n$ (written $m \mid n$), i.e., $n = qm$

# Construction / Recognition

## To apply the Division Algorithm:

1. Identify the divisor $m \in \mathbb{Z}^+$ and the dividend $n \in \mathbb{Z}$
2. Find the largest multiple of $m$ that is less than or equal to $n$: this gives $qm$
3. Compute the remainder: $r = n - qm$
4. Verify $0 \leq r < m$

# Context & Application

The Division Algorithm is the mathematical foundation for modular arithmetic in music. When counting semitones modulo 12 for octave equivalence, dividing a semitone count $n$ by $m = 12$ yields a remainder $r$ that identifies the note class. This connection between integer division and musical interval reduction is central to the text's approach.

# Examples

- $m = 9, n = 123$: $123 = 13 \cdot 9 + 6$, so $q = 13, r = 6$ (p. 14)
- $m = 12, n = 17$ (semitones): $17 = 1 \cdot 12 + 5$, so 17 semitones above C is 5 semitones above C (an F)
- Exercise 1(b): $m = 12, n = -37$: $-37 = (-4) \cdot 12 + 11$, so $r = 11$
- Exercise 1(d): $m = 7, n = 14k + 23$: $14k + 23 = (2k + 3) \cdot 7 + 2$, so $r = 2$

# Relationships

## Builds Upon
- **Sets and Number Systems** — Uses $\mathbb{Z}$ and $\mathbb{Z}^+$

## Enables
- **Octave Equivalence** — Semitone reduction modulo 12 uses the Division Algorithm
- **Note Classes** — The 12 note classes correspond to remainders modulo 12

## Related
- **Well-Ordering Principle** — Required for the proof

# Common Errors

- **Error**: Computing a negative remainder when $n$ is negative
  **Correction**: The remainder $r$ is always $0 \leq r < m$; for $n = -37, m = 12$, use $q = -4$ to get $r = 11$, not $q = -3$ giving $r = -1$

# Common Confusions

- **Confusion**: Thinking the Division Algorithm is an algorithm (computational procedure)
  **Clarification**: Despite its name, it is a theorem asserting the existence and uniqueness of $q$ and $r$
- **Confusion**: Believing the remainder can be negative
  **Clarification**: By definition, $0 \leq r < m$ always holds

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Some Properties of Integers" section, p. 14 (PDF).

# Verification Notes

- Definition source: Direct quote from source, p. 14
- Confidence rationale: High — explicit statement with worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical context (semitone counting modulo 12), negative remainder confusion
