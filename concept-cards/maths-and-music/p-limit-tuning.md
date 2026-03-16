---
concept: p-Limit Tuning
slug: p-limit-tuning

category: rational-intervals
subcategory: p-limit
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
section: "p-Limit Tuning"

extraction_confidence: high

aliases:
  - prime-limit tuning
  - p-limit intonation

prerequisites:
  - unique-factorization-in-positive-rationals
  - rational-interval
extends: []
related:
  - pythagorean-scale
  - just-intonation-scale
  - septimal-intervals
contrasts_with: []

answers_questions:
  - "What is p-limit tuning?"
  - "What is 3-limit tuning?"
  - "What is 5-limit tuning?"
---

# Quick Definition

p-limit tuning restricts all interval ratios to positive rationals whose prime factorizations involve only primes less than or equal to p. Pythagorean tuning is 3-limit; just intonation is 5-limit.

# Core Definition

"Given a prime number p, the subset of Q+ consisting of those rational numbers x whose prime factorization has the form x = p1^a1 * p2^a2 * ... * pr^ar with p1, ..., pr <= p forms a subgroup of (Q+, *). We say that a scale or system of tuning uses p-limit tuning if all interval ratios between pitches lie in this subgroup" (Wright, p. 152). The concept also appears in Chapter 11, Exercise 5, where students verify the subgroup property.

# Prerequisites

- **Unique factorization in positive rationals** -- The definition depends on prime factorization of rationals
- **Rational interval** -- p-limit tuning classifies rational intervals by their prime content

# Key Properties

1. The p-limit rationals form a subgroup of (Q+, *) for each prime p
2. Subgroups are nested: 2-limit < 3-limit < 5-limit < 7-limit < ...
3. 2-limit: only octaves (ratios 2^a)
4. 3-limit: Pythagorean tuning (ratios 2^a * 3^b) -- fifths, fourths, whole tones
5. 5-limit: just intonation (adds major thirds, minor thirds, sixths, lesser whole tone)
6. 7-limit: septimal tuning (adds "blue" seventh, septimal thirds)
7. Lower limits = simpler, more consonant intervals; higher limits = richer variety

# Construction / Recognition

## To Determine the p-Limit of an Interval

1. Write the interval ratio as a fraction in lowest terms
2. Find the prime factorization of numerator and denominator
3. Identify the largest prime appearing
4. That largest prime is the interval's limit

## Examples of Classification

- 9/8 = 3^2/2^3: largest prime is 3, so this is a 3-limit interval
- 5/4 = 5/2^2: largest prime is 5, so this is a 5-limit interval
- 7/4: largest prime is 7, so this is a 7-limit interval
- 81/80 = 3^4/(2^4 * 5): largest prime is 5, so this is a 5-limit interval

# Context & Application

The p-limit concept organizes the historical development of Western tuning: Pythagorean tuning (3-limit) uses only octaves and fifths; just intonation (5-limit) adds thirds; septimal tuning (7-limit) adds the "blue" seventh. Each expansion brings new intervals that are progressively less consonant but harmonically richer. The concept also explains the different strengths and weaknesses of tuning systems: 3-limit gives perfect fifths but poor thirds; 5-limit gives perfect thirds and fifths but cannot serve all keys equally.

# Examples

**Example 1** (p. 152): Pythagorean tuning is 3-limit: all intervals have form 2^a * 3^b.

**Example 2** (p. 155): Just intonation is 5-limit: intervals involve primes 2, 3, and 5.

**Example 3** (p. 143): The septimal minor seventh 7/4 is a 7-limit interval, not 5-limit.

**Example 4** (p. 142): The comma of Didymus 81/80 = 3^4/(2^4 * 5) is a 5-limit interval; the comma of Pythagoras 3^12/2^19 is a 3-limit interval.

# Relationships

## Builds Upon
- **Unique factorization in positive rationals** -- The definition depends on identifying primes in the factorization

## Enables
- **Pythagorean scale** -- Classified as 3-limit tuning
- **Just intonation scale** -- Classified as 5-limit tuning

## Related
- **Septimal intervals** -- 7-limit intervals extending beyond 5-limit

# Common Errors

- **Error**: Thinking p-limit means "intervals up to the value p"
  **Correction**: p-limit means the primes in the factorization are at most p; 81/64 is 3-limit despite involving numbers much larger than 3

# Common Confusions

- **Confusion**: Thinking the mean-tone scale has a p-limit classification
  **Clarification**: The mean-tone scale uses irrational intervals (e.g., 5^(1/4)), so it is not purely p-limit for any p

- **Confusion**: Assuming higher p-limit means "better tuning"
  **Clarification**: Higher limits provide more intervals but each system has trade-offs; 3-limit has perfect fifths, 5-limit has perfect thirds, and so on

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," p. 152. Also Chapter 11, Exercise 5, p. 150.

# Verification Notes

- Definition source: Direct quote from p. 152
- Confidence rationale: Formal definition with clear terminology
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: nested subgroup chain, mean-tone caveat, historical progression
