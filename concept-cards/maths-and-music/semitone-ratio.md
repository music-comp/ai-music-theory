---
concept: Semitone Ratio
slug: semitone-ratio

category: pitch-and-intervals
subcategory: measurement
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Semitones"

extraction_confidence: high

aliases:
  - equal-tempered semitone
  - twelfth root of 2

prerequisites:
  - multiplicative-composition-of-intervals
extends: []
related:
  - cents
  - n-chromatic-units
  - frequencies-of-keyboard-notes
contrasts_with: []

answers_questions:
  - "What is a semitone ratio?"
  - "What ratio corresponds to one semitone in equal temperament?"
  - "How do I convert a number of semitones to a frequency ratio?"
---

# Quick Definition

In equal temperament, the semitone has the ratio s = 2^(1/12) = the twelfth root of 2, approximately 1.05946. An interval of x semitones has ratio 2^(x/12).

# Core Definition

Let s denote the ratio of one semitone. Since twelve iterations of this interval gives the octave (ratio 2), we require s^12 = 2, giving s = 2^(1/12). More generally, the interval of x semitones (for any x in R, not necessarily an integer) has ratio:

r = 2^(x/12)    (Formula 4.2)

This extends naturally from the integer case (2^(1/12))^n = 2^(n/12) to all real x by continuity of the exponential function (Wright, pp. 60-61).

# Prerequisites

- **Multiplicative Composition of Intervals** -- The semitone ratio is derived from the requirement that twelve semitones multiply to give the octave

# Key Properties

1. s = 2^(1/12) ~ 1.05946
2. n semitones: ratio = 2^(n/12)
3. x semitones (any real x): ratio = 2^(x/12) -- Formula 4.2
4. The formula works for non-integer and negative values of x
5. Equal temperament means all semitones have the same ratio
6. 12 semitones = 1 octave (ratio 2)

# Construction / Recognition

## To Convert Semitones to a Ratio

1. Given x semitones
2. Compute r = 2^(x/12)
3. If x is negative, the result is the ratio of a downward interval (r < 1)

# Context & Application

Equal temperament divides the octave into 12 equal semitones, each with the same ratio 2^(1/12). This system allows free modulation between keys because all semitones are identical. The semitone ratio enables calculation of the frequency of any keyboard note given a reference pitch (Wright, pp. 60-61).

# Examples

**Example 1** (p. 60): Semitone ratio: s = 2^(1/12) ~ 1.05946.

**Example 2** (p. 60): Major third (4 semitones): 2^(4/12) = 2^(1/3) = cube root of 2 ~ 1.25992.

**Example 3** (p. 60): Down a minor third (-3 semitones): 2^(-3/12) = 2^(-1/4) = 1/fourth root of 2 ~ 0.840896.

# Relationships

## Builds Upon

- **Multiplicative Composition of Intervals** -- Derived from s^12 = 2

## Enables

- **Frequencies of Keyboard Notes** -- The semitone ratio allows computing any keyboard frequency
- **Converting Ratios to Semitones** -- The inverse operation uses logarithms

## Related

- **Cents** -- Cents subdivide the semitone into 100 equal parts
- **N-Chromatic Units** -- The semitone is the special case n = 12

# Common Errors

- **Error**: Computing the semitone ratio as 2/12 or 1/12 instead of 2^(1/12)
  **Correction**: The semitone ratio is the 12th root of 2, not a fraction of 2

# Common Confusions

- **Confusion**: Thinking "equal temperament" means equal frequency differences between adjacent notes
  **Clarification**: Equal temperament means equal ratios, not equal differences; higher notes have larger frequency gaps between adjacent semitones
- **Confusion**: Assuming the formula only works for whole semitones
  **Clarification**: Formula 4.2 works for any real x, allowing measurement of intervals that are not whole semitones

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 60-61. Formula 4.2.

# Verification Notes

- Definition source: Direct from pp. 60-61
- Confidence rationale: High -- explicit derivation from s^12 = 2 with examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Formula 4.2 reference, examples with major third and minor third calculations, extension to non-integer x
