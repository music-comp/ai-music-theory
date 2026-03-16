---
concept: Prime Intervals
slug: prime-intervals

category: pitch-and-intervals
subcategory: integer-ratios
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

extraction_confidence: high

aliases:
  - "prime interval"

prerequisites:
  - integral-intervals
  - prime-numbers
extends:
  - integral-intervals
related:
  - unique-prime-factorization
  - prime-interval-personality
  - keyboard-approximation-of-integer-ratios
contrasts_with: []

answers_questions:
  - "What is a prime interval in the mathematical sense?"
  - "How do prime intervals relate to prime factorization?"
---

# Quick Definition

A prime interval is an integral interval whose frequency ratio is a prime number. By unique factorization, every integral interval decomposes uniquely as a composition of prime intervals, and every rational interval decomposes into prime intervals and their inverses.

# Core Definition

"We call [an integral interval] a *prime interval* if its ratio is a prime" (Wright, Ch. 9, p. 110). By the Unique Factorization Theorem, every positive integer $n = p_1^{\alpha_1} \cdots p_r^{\alpha_r}$ factors uniquely into primes, so every integral interval decomposes uniquely into a composition of prime intervals. The set of rational intervals ($\mathbb{Q}^+$) forms a group, and every rational interval can be written as a composition of prime intervals and their opposites (Ch. 8, Exercise 3).

# Prerequisites

- **Integral Intervals** -- Prime intervals are a subclass of integral intervals
- **Prime Numbers** -- A prime interval has a prime-number ratio

# Key Properties

1. Prime intervals are musically irreducible -- they cannot be decomposed further
2. The set of prime intervals does NOT form a monoid (composition gives composite ratios)
3. The set of rational intervals forms a group under composition
4. Every rational interval decomposes uniquely into prime intervals and their inverses
5. The musical character of each integer ratio is determined by its prime factorization

# Construction / Recognition

## To identify a prime interval:
1. Determine the frequency ratio $n$
2. Check if $n$ is a prime number
3. If so, it is a prime interval
4. Its cent value is $1200 \log_2 n$

# Context & Application

The musically most important prime intervals correspond to the first several primes. Each introduces a distinct harmonic quality. The octave (2), the perfect fifth (derived from 3), and the major third (derived from 5) form the foundation of Western harmony. Higher primes (7, 11, 13) introduce intervals that lie increasingly outside the Western tonal system.

# Examples

**Example 1** (pp. 111-115): Ratio 2 (prime): the octave, $1200$ cents exactly.

**Example 2** (p. 111): Ratio 3 (prime): approximately octave-and-a-fifth, $\approx 1902$ cents.

**Example 3** (p. 112): Ratio 5 (prime): approximately two octaves plus a major third, $\approx 2786$ cents.

**Example 4** (p. 113): Ratio 6 = $2 \times 3$ is NOT a prime interval; it decomposes as octave composed with the interval of ratio 3.

**Example 5** (Ch. 8, Exercise 3): The just fifth (ratio 3/2) = prime interval 3 up, then prime interval 2 down.

# Relationships

## Builds Upon
- **Integral Intervals** -- Prime intervals are a special case
- **Prime Numbers** -- The mathematical foundation

## Enables
- **Prime Interval Personality** -- Each prime introduces a distinct musical character

## Related
- **Unique Prime Factorization** -- Guarantees unique decomposition into prime intervals
- **Keyboard Approximation of Integer Ratios** -- Prime intervals have characteristic approximation errors

# Common Errors

- **Error**: Assuming prime intervals form a group or monoid
  **Correction**: $2 \times 3 = 6$ is composite, so the composition of two prime intervals is not prime

# Common Confusions

- **Confusion**: Confusing "prime interval" (interval with prime ratio) with "prime form" in twelve-tone theory
  **Clarification**: These are entirely different uses of the word "prime"

- **Confusion**: Thinking the just fifth (3/2) is a prime interval
  **Clarification**: The ratio 3/2 is rational but not an integer; it decomposes into prime interval 3 up and prime interval 2 down

# Source Reference

Chapter 9: "The Integers as Intervals," p. 110. Also Chapter 8, Exercises 3-4, p. 108.

# Verification Notes

- Definition source: Direct quote from p. 110
- Confidence rationale: Explicit definition in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: just fifth decomposition example, Exercise 3-4 references
