---
concept: Unique Factorization in Positive Rationals
slug: unique-factorization-in-positive-rationals

category: rational-intervals
subcategory: just-intervals
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Unique Factorization of Positive Rational Numbers"

extraction_confidence: high

aliases:
  - fundamental theorem of arithmetic for rationals
  - "unique factorization in Q+"

prerequisites:
  - unique-prime-factorization
extends:
  - unique-prime-factorization
related:
  - rational-interval
  - p-limit-tuning
contrasts_with: []

answers_questions:
  - "How do you factor a positive rational number into primes?"
  - "How does unique factorization in Q+ differ from unique factorization in Z+?"
---

# Quick Definition

Every positive rational number can be uniquely factored as a product of distinct primes raised to nonzero integer powers (positive or negative), extending the fundamental theorem of arithmetic from positive integers to positive rationals.

# Core Definition

"Let x in Q+. Then x can be factored as x = p1^a1 * p2^a2 * ... * pr^ar where r >= 0, p1, p2, ..., pr are distinct primes, and a1, a2, ..., ar are nonzero integers. Moreover, this factorization is unique" (Wright, p. 139). This differs from the analogous theorem about Z+ in that the exponents may be any nonzero integers, not just positive ones. Negative exponents correspond to primes appearing in the denominator.

# Prerequisites

- **Unique prime factorization** -- The theorem for Q+ is derived from the fundamental theorem of arithmetic for Z+

# Key Properties

1. Exponents may be any nonzero integer (positive or negative), unlike Z+ where they must be positive
2. The factorization is unique up to rearrangement of factors
3. Positive exponents contribute to the numerator; negative exponents contribute to the denominator
4. When written as a fraction, numerator and denominator share no common prime factors
5. The set of primes appearing in the factorization determines the "type" of rational interval (e.g., 5-limit uses only primes <= 5)
6. A rational number x is an integer if and only if all exponents in its prime factorization are nonnegative

# Construction / Recognition

## To Factor a Positive Rational

1. Write x as a fraction n/m in lowest terms
2. Find the prime factorization of the numerator n
3. Find the prime factorization of the denominator m
4. Combine: primes from the numerator get positive exponents, primes from the denominator get negative exponents
5. Result: x = p1^a1 * p2^a2 * ... * pr^ar with all primes distinct

## Worked Example (p. 139)

The fraction 1,222,452/11,180,400 factors as:
- Numerator: 1,222,452 = 11 * 7^3 * 3^4 * 2^2
- Denominator: 11,180,400 = 11^3 * 7 * 5^2 * 3 * 2^4
- After cancellation: x = (7^2 * 3^3) / (11^2 * 5^2 * 2^2)

# Context & Application

Unique factorization in Q+ is the fundamental analytical tool for studying rational intervals. It reveals which primes are involved in an interval's ratio, which determines the interval's classification in p-limit tuning. It also provides the key to proving that equal-tempered intervals (except octaves) are irrational: if x^n = 2^k, then x can only involve the prime 2, making x itself a power of 2.

# Examples

**Example 1** (p. 139): The just fifth 3/2 = 2^(-1) * 3^1 involves only primes 2 and 3.

**Example 2** (p. 139): The just major third 5/4 = 2^(-2) * 5^1 involves primes 2 and 5.

**Example 3** (p. 139): The comma of Didymus 81/80 = 2^(-4) * 3^4 * 5^(-1) involves primes 2, 3, and 5.

**Example 4** (p. 139): The fraction 1,222,452/11,180,400 reduces to (7^2 * 3^3)/(11^2 * 5^2 * 2^2), demonstrating the factorization method for complex ratios.

# Relationships

## Builds Upon
- **Unique prime factorization** -- Extends the Z+ theorem to Q+ by allowing negative exponents

## Enables
- **Rational interval** -- Prime factorization is the key tool for analyzing rational intervals
- **p-limit tuning** -- Classification depends on which primes appear in factorizations
- **Irrationality of equally tempered intervals** -- The proof relies on uniqueness of factorization

## Related
- **Comma of Pythagoras** -- Its factorization 3^12/2^19 reveals its 3-limit nature
- **Comma of Didymus** -- Its factorization 3^4/(2^4 * 5) reveals its 5-limit nature

# Common Errors

- **Error**: Forgetting that exponents can be negative in Q+ factorization
  **Correction**: Unlike Z+, elements of Q+ have both positive exponents (numerator primes) and negative exponents (denominator primes)

- **Error**: Reading off prime structure before fully reducing the fraction
  **Correction**: The factorization applies after full cancellation; first reduce the fraction before identifying the prime structure

# Common Confusions

- **Confusion**: Thinking the factorization applies only to integers
  **Clarification**: Any positive rational number has a unique prime factorization; the extension to Q+ simply allows negative exponents

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 138-140. Theorem stated on p. 139.

# Verification Notes

- Definition source: Direct quote of theorem from p. 139
- Confidence rationale: Formal theorem with complete statement
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: worked example, integer criterion (nonneg exponents), note about reducing fractions first
