---
concept: Generating Interval
slug: generating-interval

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Generating intervals"

extraction_confidence: high

aliases:
  - generator of Z_n
  - chromatic generator

prerequisites:
  - n-chromatic-scale
  - modular-integers
extends: []
related:
  - cyclic-group-and-generator
  - gcd-condition-for-generators
  - circle-of-intervals
  - euler-phi-function
  - relatively-prime-integers
contrasts_with: []

answers_questions:
  - "What is a generating interval?"
  - "Which chromatic intervals generate all note classes in a given scale?"
  - "How many generating intervals does an n-chromatic scale have?"
---

# Quick Definition

A modular chromatic interval whose successive iterations produce all intervals in the n-chromatic scale, corresponding to a generator of the cyclic group Z_n.

# Core Definition

In the n-chromatic scale, a modular interval [m] in Z_n is a generating interval if every element of Z_n can be expressed as a multiple of [m]. Equivalently, [m] is a generating interval if and only if gcd(m, n) = 1 (m and n are relatively prime). The number of generating intervals is phi(n), where phi is the Euler phi function (Wright, pp. 75-76, developed further in Ch. 7).

# Prerequisites

- **N-chromatic scale** — Generating intervals are defined within an n-chromatic scale
- **Modular integers** — Generating intervals are generators of the group Z_n

# Key Properties

1. [m] is a generating interval iff gcd(m, n) = 1
2. The number of generating intervals is phi(n) (Euler phi function)
3. A generating interval [m] has order n in Z_n
4. The "circle" based on a generating interval visits all n chromatic intervals
5. Generating intervals come in pairs: [m] and [n - m] = -[m] traverse the circle in opposite directions
6. For prime n, every non-zero element is a generator (phi(n) = n - 1)

# Construction / Recognition

## To Determine if [m] is a Generating Interval in Z_n
1. Compute gcd(m, n)
2. If gcd(m, n) = 1, then [m] is a generating interval
3. If gcd(m, n) > 1, then [m] generates only a proper subgroup

## To List All Generating Intervals
1. For each m from 1 to n-1, check if gcd(m, n) = 1
2. Count the results to verify phi(n) generators found

# Context & Application

In the standard 12-chromatic scale, the generating intervals are those whose iterations cycle through all 12 note classes. The circle of fifths (iterating by 7 semitones) is the most musically significant example. There are exactly phi(12) = 4 generating intervals in the 12-chromatic scale: [1], [5], [7], [11].

# Examples

**Example 1** (pp. 75-76): In the 14-chromatic scale, the numbers 1, 3, 5, 9, 11, 13 are relatively prime to 14, so phi(14) = 6 generating intervals.

**Example 2** (p. 76): The circle of intervals based on [5] in Z_14 visits all 14 elements: [0], [5], [10], [1], [6], [11], [2], [7], [12], [3], [8], [13], [4], [9].

**Example 3** (Ch. 7, p. 95): In the 12-chromatic scale, [1] (semitone), [5] (fourth), [7] (fifth), [11] (major seventh) are the generators. Non-generators like [2], [3], [4], [6] produce only subsets of note classes.

# Relationships

## Builds Upon
- **N-chromatic scale** — Generating intervals are defined within n-chromatic scales
- **Modular integers** — Generators correspond to group generators of Z_n

## Enables
- **Circle of intervals** — The circle is constructed by iterating a generating interval

## Related
- **Cyclic group and generator** — Generating intervals are the musical interpretation of cyclic group generators
- **GCD condition for generators** — The number-theoretic criterion for being a generator
- **Euler phi function** — Counts the number of generating intervals

# Common Errors

- **Error**: Assuming every non-zero interval is a generating interval
  **Correction**: Only those [m] with gcd(m, n) = 1 are generators; for composite n, many intervals generate proper subgroups

# Common Confusions

- **Confusion**: Thinking generating intervals are the same as "generated scales" (scales built by stacking)
  **Clarification**: Generating intervals specifically generate all elements of Z_n through iteration modulo octave

- **Confusion**: Believing the number of generating intervals relates to interval quality
  **Clarification**: The count phi(n) depends only on the number-theoretic properties of n, not on which intervals "sound good"

# Source Reference

Chapter 6: "Chromatic Scales," pp. 75-76 (generating intervals section). Developed further in Chapter 7 (cyclic groups) and Chapter 8 (GCD).

# Verification Notes

- Definition source: Direct from Wright, pp. 75-76
- Confidence rationale: High — explicit definition with GCD criterion
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: n=14 example with phi(14)=6, Z_12 generators list
