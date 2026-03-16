---
concept: Rational Interval
slug: rational-interval

category: rational-intervals
subcategory: just-intervals
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: null

extraction_confidence: high

aliases:
  - rational frequency ratio

prerequisites: []
extends:
  - interval-as-frequency-ratio
related:
  - just-interval
  - unique-factorization-in-positive-rationals
  - string-fretting-and-rational-intervals
contrasts_with:
  - irrationality-of-equally-tempered-intervals

answers_questions:
  - "What is a rational interval?"
  - "How do rational intervals differ from equally tempered intervals?"
---

# Quick Definition

A rational interval is a musical interval whose frequency ratio can be expressed as a ratio of positive integers, i.e., as an element of Q+.

# Core Definition

"An interval I will be called rational if its corresponding ratio lies in Q+. Otherwise we say I is an irrational interval" (Wright, p. 138). Since elements of R+ are in one-to-one correspondence with musical intervals via the group isomorphism from the group of intervals to (R+, *), rational intervals correspond precisely to the subgroup (Q+, *) within this correspondence.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. A rational interval has a frequency ratio expressible as n/m where n, m are positive integers
2. Every positive rational number has a unique prime factorization x = p1^a1 * p2^a2 * ... * pr^ar with nonzero integer exponents
3. Rational intervals can be created physically using string fretting: ratio n/m is obtained by fretting at distance (m/n)*L
4. The set of rational intervals forms a subgroup of all intervals under composition (multiplication of ratios)
5. Consonance of a rational interval depends on the size of the integers in its ratio -- smaller integers yield more consonant intervals

# Construction / Recognition

## To Construct a Rational Interval

1. Choose a ratio n/m of positive integers (n >= m for an ascending interval)
2. On a string of length L, fret at distance (m/n)*L from one end
3. The resulting pitch will be in the ratio n:m with the open string fundamental

## To Recognize a Rational Interval

1. Determine the frequency ratio of the interval
2. Check whether the ratio can be expressed as a fraction of positive integers
3. If yes, the interval is rational; if the ratio involves irrational numbers (like 2^(1/12)), it is irrational

# Context & Application

Rational intervals have been fundamental to music since antiquity. Ancient mathematicians could construct any rational interval precisely using compass and straightedge to divide a string, whereas irrational intervals like the equal-tempered semitone (2^(1/12)) were inaccessible with classical geometric tools. The classification of intervals as rational or irrational provides the mathematical framework for understanding tuning systems, consonance hierarchies, and the fundamental tension between just intonation and equal temperament.

# Examples

**Example 1** (p. 139): The just fifth 3/2 is a rational interval whose frequency ratio involves the small integers 3 and 2.

**Example 2** (p. 139): The just major third 5/4 is a rational interval.

**Example 3** (p. 139): The tempered semitone 2^(1/12) is an irrational interval, not accessible to ancient construction techniques.

**Example 4** (p. 139): The fraction 1,222,452/11,180,400 factors as (7^2 * 3^3)/(11^2 * 5^2 * 2^2) after prime factorization and cancellation, demonstrating that even complex-looking ratios can be analyzed through their prime factors.

# Relationships

## Builds Upon
- **Interval as frequency ratio** -- Rational intervals are the subset of all frequency ratios that lie in Q+

## Enables
- **Just interval** -- Just intervals are rational intervals with specifically small integers
- **p-limit tuning** -- Tuning systems classified by which primes appear in their rational interval ratios
- **Unique factorization in positive rationals** -- The analytical tool for studying rational intervals

## Related
- **String fretting and rational intervals** -- Physical realization of rational intervals on strings
- **Consonance and small integer ratios** -- Consonance correlates with small integers in rational ratios

## Contrasts With
- **Irrationality of equally tempered intervals** -- All non-octave equal-tempered intervals are irrational

# Common Errors

- **Error**: Assuming any fraction-like expression makes an interval rational
  **Correction**: The ratio must be expressible as a ratio of integers; 2^(1/12) is not rational even though it can be written as a fraction-like expression

- **Error**: Confusing the ratio n/m with the string length fraction m/n
  **Correction**: To produce the rational interval n/m on a string of length L, fret at (m/n)*L, not (n/m)*L

# Common Confusions

- **Confusion**: Believing all rational intervals sound consonant
  **Clarification**: Consonance depends on the size of the integers involved, not merely on rationality; the ratio 1,222,452/11,180,400 is rational but would not sound consonant

- **Confusion**: Thinking a rational interval's cent measurement is also rational
  **Clarification**: The cent measurement of a rational interval is always irrational, except for multi-octaves (this is proved in Exercise 8, p. 151)

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 138-139.

# Verification Notes

- Definition source: Direct quote from p. 138
- Confidence rationale: Explicit formal definition with DEFINITION label in source
- Uncertainties: None
- Cross-reference status: Verified against planned extractions
- Re-extraction notes: Re-extracted from v2 card; preserved: confusion about cent measurement irrationality, factorization example
