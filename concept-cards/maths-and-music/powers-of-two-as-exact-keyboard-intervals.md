---
concept: Powers of Two as Exact Keyboard Intervals
slug: powers-of-two-as-exact-keyboard-intervals

category: pitch-and-intervals
subcategory: integer-ratios
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: "Non-Chromatic Nature of Intervals Other Than Multiple Octaves"

extraction_confidence: high

aliases:
  - "non-chromatic nature of integer intervals"

prerequisites:
  - integral-intervals
  - unique-prime-factorization
extends: []
related:
  - keyboard-approximation-of-integer-ratios
  - error-calculation-in-cents
  - irrationality-of-equally-tempered-intervals
contrasts_with: []

answers_questions:
  - "Which integer ratios can be played exactly on an equally tempered keyboard?"
  - "Why can only octaves be played precisely in equal temperament?"
---

# Quick Definition

The only positive integers that correspond to exact keyboard intervals are powers of 2 (1, 2, 4, 8, 16, ...). All other integer ratios are necessarily approximated with some error. This applies to any equally tempered scale, not just 12-tone.

# Core Definition

"**Theorem:** The only keyboard intervals which have integer ratios are the powers of 2" (Wright, Ch. 9, p. 117). Proof: If $n \in \mathbb{Z}^+$ is a keyboard interval, then $n = (2^{1/12})^k = 2^{k/12}$ for some integer $k \geq 0$. Raising to the 12th power: $n^{12} = 2^k$. By the Unique Factorization Theorem, $n$ can have only 2 in its prime factorization, so $n$ is a power of 2.

# Prerequisites

- **Integral Intervals** -- The theorem is about which integral intervals are exact
- **Unique Prime Factorization** -- The proof relies on uniqueness of factorization

# Key Properties

1. Only $n = 2^j$ for integer $j \geq 0$ can be exact keyboard intervals
2. The proof works for any $m$-chromatic equally tempered scale, not just $m = 12$
3. The fifth, fourth, major third, and all other consonant intervals are inherently approximations
4. This is the fundamental trade-off of equal temperament

# Construction / Recognition

## The proof generalized to m-chromatic scales:
1. In an $m$-chromatic scale, keyboard intervals have ratio $2^{k/m}$
2. If $n = 2^{k/m}$ for integer $n$, then $n^m = 2^k$
3. By unique factorization, $n$ must be a power of 2
4. This holds for any $m$, not just $m = 12$

# Context & Application

This theorem is profound: the equally tempered keyboard can never perfectly render any interval other than multiple octaves. The fifth, fourth, major third, and all other consonant intervals based on primes other than 2 are inherently approximated. This is the fundamental trade-off -- gaining the ability to play in all keys at the cost of pure intervals.

# Examples

**Example 1** (p. 111): $2^0 = 1$: unison (0 semitones), exact.

**Example 2** (p. 111): $2^1 = 2$: one octave (12 semitones), exact.

**Example 3** (p. 111): $2^2 = 4$: two octaves (24 semitones), exact.

**Example 4** (p. 113): $2^3 = 8$: three octaves (36 semitones), exact.

**Example 5** (p. 111): 3 is NOT a power of 2, so requires approximation (~2 cents error).

# Relationships

## Builds Upon
- **Unique Prime Factorization** -- The proof depends on this theorem

## Enables
- **Understanding of equal temperament limitations**

## Related
- **Keyboard Approximation of Integer Ratios** -- How non-power-of-2 integers are approximated
- **Error Calculation in Cents** -- Quantifies the approximation errors

# Common Errors

- **Error**: Thinking the perfect fifth (700 cents) is exact on the keyboard
  **Correction**: The tempered fifth $2^{7/12}$ is irrational; the just fifth 3/2 requires ratio 3, which is not a power of 2

# Common Confusions

- **Confusion**: Thinking this result is specific to 12-tone equal temperament
  **Clarification**: The theorem applies to any $m$-chromatic equal temperament -- only powers of 2 are exact in any equal division of the octave

# Source Reference

Chapter 9: "The Integers as Intervals," p. 117. Theorem and proof under "Non-Chromatic Nature of Intervals Other Than Multiple Octaves."

# Verification Notes

- Definition source: Direct quote of theorem from p. 117
- Confidence rationale: Explicit theorem with proof
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: generalization to m-chromatic scales, proof structure
