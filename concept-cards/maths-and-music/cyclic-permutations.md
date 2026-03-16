---
concept: Cyclic Permutations
slug: cyclic-permutations

category: mathematical-foundations
subcategory: sets-and-relations
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Cyclic Permutations"

extraction_confidence: high

aliases:
  - "cyclic permutation"
  - "circular permutation"
  - "cyclic rotation"

prerequisites:
  - sets-and-number-systems
extends: []
related:
  - ecclesiastical-modes
  - diatonic-and-chromatic-scales
  - modality-and-key
  - cyclic-permutations-and-root-identification
  - chord-types-and-interval-sequences
contrasts_with: []

answers_questions:
  - "What is a cyclic permutation of a sequence?"
  - "How do cyclic permutations generate musical modes?"
  - "Can a sequence be a non-trivial cyclic permutation of itself?"
  - "What is a non-trivial cyclic symmetry?"
---

# Quick Definition

A rearrangement of a finite sequence obtained by moving elements from the beginning to the end, equivalent to rotating the sequence on a circular arrangement.

# Core Definition

Given a finite sequence $x_1, x_2, \ldots, x_n$, a cyclic permutation is obtained by choosing an integer $i$ with $1 \leq i \leq n$, taking entries $x_1, \ldots, x_i$ from the beginning and placing them at the end, yielding $x_{i+1}, x_{i+2}, \ldots, x_n, x_1, x_2, \ldots, x_i$. The case $i = n$ returns the original sequence (trivial). The cases $i = 1, \ldots, n-1$ are called *non-trivial* cyclic permutations. A sequence has "no non-trivial cyclic symmetries" if no non-trivial cyclic permutation reproduces the original sequence (Wright, pp. 25-26).

# Prerequisites

- **Sets and Number Systems** — Sequences are indexed by integers

# Key Properties

1. Every sequence is a (trivial) cyclic permutation of itself ($i = n$)
2. Non-trivial cyclic permutations use $i = 1, \ldots, n-1$
3. A sequence can be a non-trivial cyclic permutation of itself if it has internal periodicity
4. The operation can be visualized by arranging elements on a clock and rotating
5. There are exactly $n$ cyclic permutations of an $n$-element sequence (including the trivial one)
6. The number of distinct permutations divides $n$

# Construction / Recognition

## To compute a cyclic permutation:

1. Choose the shift amount $i$ ($1 \leq i \leq n$)
2. Remove the first $i$ elements from the sequence
3. Append them at the end
4. The result is the cyclic permutation for that $i$

# Context & Application

Cyclic permutations are the mathematical foundation for understanding musical modes. The seven ecclesiastical modes are obtained as cyclic permutations of the standard (Ionian) diatonic scale's interval pattern. The fact that the interval sequence $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ has no non-trivial cyclic permutation equal to itself guarantees that all seven modal permutations produce distinct modes. The concept is also applied in Chapter 3 to chord theory, where cyclic permutations of interval sequences determine whether a chord has a discernible root.

# Examples

- The sequence $7, 4, 1, 7$ has cyclic permutations: $4, 1, 7, 7$ ($i=1$), $1, 7, 7, 4$ ($i=2$), $7, 7, 4, 1$ ($i=3$), and $7, 4, 1, 7$ ($i=4$, trivial) — all distinct (p. 25)
- The sequence $3, 5, 3, 3, 5, 3$ is a non-trivial cyclic permutation of itself (using $i = 3$) (p. 26)
- The Dorian mode is the cyclic permutation of the Ionian interval pattern starting on the second note: $1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}, 1$
- The major chord interval sequence $(4, 3, 5)$ has three distinct cyclic permutations, so no non-trivial cyclic symmetries (Ch. 3, p. 46)
- The augmented chord $(4, 4, 4)$ has full cyclic symmetry — all permutations are identical (Ch. 3, p. 47)

# Relationships

## Builds Upon
- **Sets and Number Systems** — Uses integer indexing

## Enables
- **Ecclesiastical Modes** — The seven modes are cyclic permutations of the diatonic scale
- **Modality and Key** — Mode is determined by which cyclic permutation is used
- **Cyclic Permutations and Root Identification** — Applied to chord interval sequences in Ch. 3

## Related
- **Diatonic and Chromatic Scales** — The standard scale whose permutations generate modes
- **Chord Types and Interval Sequences** — Interval sequences analyzed via cyclic permutations

# Common Errors

- **Error**: Confusing cyclic permutation with arbitrary rearrangement
  **Correction**: Cyclic permutations preserve the circular order; only the starting point changes

# Common Confusions

- **Confusion**: Thinking a sequence cannot be a non-trivial cyclic permutation of itself
  **Clarification**: A sequence with internal periodicity can be (e.g., $3, 5, 3, 3, 5, 3$), but the major scale interval sequence does NOT have this property
- **Confusion**: Thinking that repeated values automatically create cyclic symmetries
  **Clarification**: Having repeated intervals does not guarantee cyclic symmetry; the pattern of repetition must align with a cyclic rotation

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Cyclic Permutations" section, pp. 25-26 (PDF). Applied to chords in Chapter 3, pp. 46-49.

# Verification Notes

- Definition source: Direct from source, pp. 25-26
- Confidence rationale: High — explicit definition with worked numerical example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: self-permutation example ($3,5,3,3,5,3$), Exercise 8 reference, modal connection, chord theory applications from Ch. 3
