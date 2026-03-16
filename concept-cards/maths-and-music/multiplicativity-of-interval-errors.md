---
concept: Multiplicativity of Interval Errors
slug: multiplicativity-of-interval-errors

category: pitch-and-intervals
subcategory: temperament-and-tuning
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
  - "additivity of cent errors"
  - "logarithmic additivity of approximation errors"

prerequisites:
  - error-calculation-in-cents
  - keyboard-approximation-of-integer-ratios
  - converting-ratios-to-cents
extends:
  - integral-intervals
related:
  - prime-interval-personality
  - powers-of-two-as-exact-keyboard-intervals
  - monoid-of-integral-intervals
contrasts_with: []

answers_questions:
  - "Why does the keyboard approximation error for a composite integer equal the sum of errors of its factors?"
  - "Why does multiplying by a power of 2 not change the approximation error?"
  - "How can you predict the keyboard error for any composite integer from its prime factorization?"
---

# Quick Definition

When approximating composite integer ratios on the equally tempered keyboard, the cent errors add: the error for the product n * m equals the error for n plus the error for m, and multiplying by powers of 2 contributes zero error since the octave is rendered exactly.

# Core Definition

For a positive integer n, let E(n) denote the difference in cents between the true interval ratio n and its closest equally tempered keyboard approximation. The logarithmic property of cents gives 1200 * log_2(nm) = 1200 * log_2(n) + 1200 * log_2(m), which means the cent value of a composite interval is the sum of the cent values of its factors. Since the keyboard rounds each to the nearest semitone, the errors are approximately additive: E(nm) = E(n) + E(m). In particular, E(2^k * n) = E(n) for all k, because E(2^k) = 0 -- the octave is rendered exactly by equal temperament (Wright, pp. 112-115).

# Prerequisites

- **Error calculation in cents** -- Must understand how to compute the cent difference between an integer ratio and its nearest keyboard approximation
- **Keyboard approximation of integer ratios** -- Must know that each integer ratio is approximated by rounding 1200 * log_2(n) to the nearest multiple of 100
- **Converting ratios to cents** -- Must know the formula: cents = 1200 * log_2(r)

# Key Properties

1. Cent values are additive under composition: cents(nm) = cents(n) + cents(m)
2. This follows directly from the logarithmic identity: log_2(nm) = log_2(n) + log_2(m)
3. The error function E is approximately additive: E(nm) = E(n) + E(m)
4. Powers of 2 have zero error: E(2^k) = 0 for all k >= 0
5. Consequently, octave transposition does not change approximation quality: E(2^k * n) = E(n)
6. The error of any composite integer can be predicted from the errors of its prime factors

# Construction / Recognition

## To Compute E(n) for a Composite Integer
1. Factor n into primes: n = p_1^{a_1} * p_2^{a_2} * ... * p_k^{a_k}
2. Look up or compute E(p_i) for each prime factor
3. Sum the errors with multiplicities: E(n) = a_1 * E(p_1) + a_2 * E(p_2) + ... + a_k * E(p_k)
4. Note that all factors of 2 contribute zero error

## To Verify
1. Compute 1200 * log_2(n) directly
2. Round to the nearest multiple of 100 (semitone)
3. The difference is E(n); confirm it matches the sum from step 3 above

# Context & Application

This principle provides a systematic way to predict how well the equally tempered keyboard approximates any composite integer ratio, using only knowledge of the prime errors. Since the approximation quality of the primes 2, 3, 5, 7, 11, and 13 is computed directly by Wright, the error for any integer built from these primes follows immediately.

The principle also explains why octave equivalence is special in equal temperament: multiplying by 2 (adding an octave) never degrades the approximation. This is because equal temperament is defined by dividing the octave into 12 equal parts, making the octave the one integer ratio that is rendered exactly.

# Examples

**Example 1** (p. 112): E(6) = E(2) + E(3) = 0 + (-2) = -2 cents. Verified: 1200 * log_2(6) = 1200 * (log_2(2) + log_2(3)) = 1200 + 1901.96 = 3101.96, which is about 2 cents above 3100 (31 semitones).

**Example 2** (p. 113): E(9) = E(3) + E(3) = (-2) + (-2) = -4 cents. Since 9 = 3^2, the error doubles. Verified: three octaves plus a step approximation is about 4 cents flat.

**Example 3** (p. 114): E(10) = E(2) + E(5) = 0 + (+14) = +14 cents. Since 10 = 2 * 5, the octave factor contributes nothing, and the error matches that of 5. Verified: three octaves plus a major third is about 14 cents sharp.

**Example 4** (p. 114): E(12) = E(4) + E(3) = E(2^2) + E(3) = 0 + (-2) = -2 cents. Verified: three octaves plus a fifth is about 2 cents flat.

# Relationships

## Builds Upon
- **Error calculation in cents** -- The error function E(n) is the core quantity
- **Keyboard approximation of integer ratios** -- Provides the framework for measuring approximation quality
- **Integral intervals** -- The domain over which this property operates

## Enables
- **Prime interval personality** -- Knowing that composite errors derive from prime errors focuses attention on the primes as the fundamental building blocks of interval quality

## Related
- **Powers of two as exact keyboard intervals** -- The special case E(2^k) = 0 that makes octave transposition error-free
- **Monoid of integral intervals** -- The multiplicative structure of integers underlies the additivity of errors

# Common Errors

- **Error**: Computing E(nm) by finding the keyboard approximation of nm directly without checking additivity
  **Correction**: While direct computation works, the additive property E(nm) = E(n) + E(m) provides a faster method and deeper understanding

- **Error**: Forgetting to account for multiplicity when a prime appears more than once
  **Correction**: E(p^k) = k * E(p); for example, E(9) = E(3^2) = 2 * E(3), not just E(3)

# Common Confusions

- **Confusion**: Thinking the errors multiply rather than add
  **Clarification**: The interval ratios multiply, but the cent errors (being logarithmic) add. This is the key insight from the logarithmic nature of the cent scale

- **Confusion**: Believing additivity of errors is exact
  **Clarification**: The additivity is approximate because rounding to the nearest semitone can introduce small discrepancies. For the integers 1-13 in the text, it holds exactly because individual errors are small relative to 50 cents

# Source Reference

Chapter 9: "The Integers as Intervals," pp. 112-115 (PDF page 110). The principle is demonstrated through the worked examples of integers 6, 9, 10, and 12, with explicit logarithmic calculations for 6 (pp. 112-113).

# Verification Notes

- Definition: Synthesized from Wright's worked examples and the logarithmic calculation on pp. 112-113; the term "multiplicativity" is from the old card, not explicitly named in the source
- Key Properties: Items 1-2 explicit in the logarithmic derivation; items 3-6 demonstrated through examples
- Examples: All four examples drawn directly from the source with verified calculations
- Confidence: HIGH -- the principle is explicitly demonstrated with full calculation for E(6), and applied consistently for E(9), E(10), E(12)
- Re-extraction notes: Re-extracted from v2 card to v3.1 format; preserved: all four worked examples with error values, the formal notation E(n), the observation about E(2^k * n) = E(n)
