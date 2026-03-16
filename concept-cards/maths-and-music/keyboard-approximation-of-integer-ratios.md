---
concept: Keyboard Approximation of Integer Ratios
slug: keyboard-approximation-of-integer-ratios

category: pitch-and-intervals
subcategory: approximation
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

extraction_confidence: high

aliases: []

prerequisites:
  - integral-intervals
  - error-calculation-in-cents
extends: []
related:
  - powers-of-two-as-exact-keyboard-intervals
  - in-the-cracks-intervals
  - consonance-and-dissonance-from-integer-ratios
contrasts_with: []

answers_questions:
  - "How closely does the equally tempered keyboard approximate each integer ratio?"
  - "Which integer ratios are well represented on the keyboard?"
---

# Quick Definition

The process of finding the closest equally tempered chromatic interval to each positive integer ratio and measuring the approximation error in cents. Some integers (powers of 2) are rendered exactly; others have varying degrees of error.

# Core Definition

For a positive integer $n$, its interval in cents is $1200 \log_2 n$. The best keyboard approximation is the nearest multiple of 100 cents (nearest semitone), and the error is the difference. The source systematically analyzes integers 1 through 13, revealing which are well-approximated and which are not (Wright, Ch. 9, pp. 111-116).

# Prerequisites

- **Integral Intervals** -- Understanding what an integer ratio interval is
- **Error Calculation in Cents** -- The method for quantifying approximation quality

# Key Properties

1. The error for a composite $mn$ equals the sum of errors of $m$ and $n$ (modulo exact powers of 2)
2. Powers of 2 are rendered exactly (0 cents error)
3. Primes 2, 3 are well-approximated (0 and ~2 cents)
4. Prime 5 has noticeable error (~14 cents)
5. Prime 7 has significant error (~31 cents)
6. Primes 11, 13 have extreme error (~49, ~41 cents)

# Construction / Recognition

## To find the keyboard approximation of integer n:
1. Compute $c = 1200 \log_2 n$ (cent value)
2. Round to nearest multiple of 100: $k = \text{round}(c / 100)$
3. The approximation is $k$ semitones
4. Error = $c - 100k$ (positive = keyboard is sharp, negative = flat)

# Context & Application

This analysis reveals the fundamental trade-off of equal temperament: gaining the ability to play in all keys at the cost of pure integer-ratio intervals. The tempered scale was historically controversial because of these discrepancies, particularly the 14-cent error on the ratio 5 (major third).

# Examples

**Summary** (pp. 111-116): Keyboard approximations for integers 1-13:
- 1: unison, exact (0 cents error)
- 2: octave, exact
- 3: octave + fifth, ~2 cents flat
- 4: two octaves, exact
- 5: two octaves + major third, ~14 cents sharp
- 6: two octaves + fifth, ~2 cents flat
- 7: two octaves + minor seventh, ~31 cents sharp
- 8: three octaves, exact
- 9: three octaves + major second, ~4 cents flat
- 10: three octaves + major third, ~14 cents sharp
- 11: three octaves + tritone, ~49 cents sharp
- 12: three octaves + fifth, ~2 cents flat
- 13: three octaves + minor sixth, ~41 cents flat

# Relationships

## Builds Upon
- **Integral Intervals** -- The intervals being approximated
- **Error Calculation in Cents** -- The measurement method

## Enables
- **Consonance and Dissonance from Integer Ratios** -- Approximation quality correlates with consonance

## Related
- **Powers of Two as Exact Keyboard Intervals** -- Only powers of 2 are exact
- **In-the-Cracks Intervals** -- Primes 11 and 13 are poorly approximated

# Common Errors

- **Error**: Expecting each integer to have an independent error
  **Correction**: The error for a composite equals the sum of its prime factors' errors (since cents are logarithmic); e.g., error of 6 = error of 2 + error of 3

# Common Confusions

- **Confusion**: Thinking some non-power-of-2 integers might be exact on the keyboard
  **Clarification**: No integer other than a power of 2 can be rendered exactly (proved in the text)

# Source Reference

Chapter 9: "The Integers as Intervals," pp. 111-116.

# Verification Notes

- Definition source: Synthesized from systematic treatment in Ch. 9
- Confidence rationale: Each integer 1-13 is explicitly computed in the source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete summary table, error additivity insight
