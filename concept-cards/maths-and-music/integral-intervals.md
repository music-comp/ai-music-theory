---
concept: Integral Intervals
slug: integral-intervals

category: pitch-and-intervals
subcategory: integer-ratios
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

extraction_confidence: high

aliases:
  - "integral interval"
  - "integer ratio interval"

prerequisites: []
extends: []
related:
  - prime-intervals
  - keyboard-approximation-of-integer-ratios
  - overtone-series
  - unique-prime-factorization
contrasts_with: []

answers_questions:
  - "What is an integral interval?"
  - "How do positive integers correspond to musical intervals?"
---

# Quick Definition

An integral interval is a musical interval whose frequency ratio is a positive integer. The set of integral intervals forms a monoid under composition, identifiable with $(\mathbb{Z}^+, \cdot)$, and each integer's musical character is determined by its prime factorization.

# Core Definition

"We will occasionally employ the slightly awkward term *integral interval* to refer to a musical interval whose ratio is an integer. We call such an interval a *prime interval* if its ratio is a prime" (Wright, Ch. 9, p. 110). The set of integral intervals forms a monoid under composition of intervals, identifiable with $(\mathbb{Z}^+, \cdot)$. Since $\mathbb{Z}^+ \subset \mathbb{R}^+$ and intervals are identified with positive real numbers, each positive integer $n$ defines a musical interval.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Each positive integer $n$ gives an interval measured by $1200 \log_2 n$ cents
2. The set of integral intervals is closed under composition (multiplying ratios)
3. Composition is associative with identity element $1$ (unison)
4. Integral intervals form a monoid, NOT a group (no inverses: $1/n \notin \mathbb{Z}^+$ for $n > 1$)
5. Each integer's musical "personality" is determined by its prime factorization

# Construction / Recognition

## To identify an integral interval:
1. Determine if the frequency ratio is a positive integer
2. Compute its cent value: $1200 \log_2 n$
3. Find the closest keyboard approximation
4. Determine the prime factorization to understand its musical character

# Context & Application

The study of integral intervals connects number theory to the physics of sound. The first several positive integers correspond to the most fundamental musical intervals: $2$ is the octave, $3$ is approximately an octave-and-a-fifth, $4$ is two octaves, $5$ is approximately two octaves plus a major third. These small-integer ratios correspond to the harmonic series and are perceived as consonant.

# Examples

**Example 1** (p. 110): Ratio $1$ = unison, the identity element.

**Example 2** (p. 110): Ratio $2$ = octave ($1200$ cents exactly).

**Example 3** (p. 111): Ratio $3$ $\approx$ octave-and-a-fifth ($1200 \log_2 3 \approx 1901.96$ cents, about 2 cents above 1900).

**Example 4** (p. 111): Ratio $4 = 2^2$ = two octaves ($2400$ cents exactly).

**Example 5** (p. 112): Ratio $6 = 2 \cdot 3$: composition of octave and ratio-3 interval, giving approximately two octaves and a fifth.

# Relationships

## Enables
- **Prime Intervals** -- Prime intervals are the irreducible integral intervals
- **Keyboard Approximation of Integer Ratios** -- Integral intervals are approximated on the keyboard
- **Overtone Series** -- The overtone series consists of integral intervals from a fundamental

## Related
- **Unique Prime Factorization** -- Determines how integral intervals decompose into prime intervals

# Common Errors

- **Error**: Assuming integral intervals form a group
  **Correction**: They form only a monoid -- there are no inverses since $1/n$ is not a positive integer for $n > 1$

# Common Confusions

- **Confusion**: Confusing "integral interval" (ratio is an integer) with "interval measured in integers" (like semitone counts)
  **Clarification**: The term specifically refers to intervals whose frequency ratios are positive integers

- **Confusion**: Thinking ratio $3$ is exactly an octave-plus-a-fifth
  **Clarification**: The ratio $3$ differs from the tempered octave-plus-a-fifth by about 2 cents

# Source Reference

Chapter 9: "The Integers as Intervals," pp. 110-111. Also introduced in Chapter 8, p. 100.

# Verification Notes

- Definition source: Direct quote from p. 110
- Confidence rationale: Explicit definition in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 cards; merged integral-interval.md and integral-intervals.md into one card. Preserved: monoid structure note, musical examples from both cards
