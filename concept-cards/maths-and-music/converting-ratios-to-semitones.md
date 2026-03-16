---
# === CORE IDENTIFICATION ===
concept: Converting Ratios to Semitones
slug: converting-ratios-to-semitones

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: measurement
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Semitones"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - ratio to semitone conversion

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semitone-ratio
extends:
  - semitone-ratio
related:
  - converting-ratios-to-cents
  - converting-ratios-to-octaves
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I convert a frequency ratio to semitones?"
  - "What is the formula for converting a ratio to semitones?"
---

# Quick Definition

An interval ratio r is converted to semitones by the formula x = 12 * log_2(r). This is the inverse of the forward conversion r = 2^(x/12) introduced in Formula 4.2.

# Core Definition

Given an interval ratio r in R+, its measurement in semitones is:

x = 12 * log_2(r)

This follows from solving r = 2^(x/12) (Formula 4.2) for x. The forward direction (semitones to ratio, r = 2^(x/12)) is established in Chapter 4. The inverse (ratio to semitones, requiring logarithms) is developed in Chapter 5, but the relationship is implicit from the moment Formula 4.2 is introduced (Wright, pp. 60-61, inverse in Ch. 5, p. 71).

# Prerequisites

- **Semitone Ratio** -- Must understand the forward conversion r = 2^(x/12) to understand its inverse

# Key Properties

1. x = 12 * log_2(r) converts ratio to semitones
2. Equivalent to x = 1200 * log_2(r) / 100 (i.e., cents divided by 100)
3. The result is generally not an integer; only equal-tempered intervals give exact integers
4. Negative results indicate downward intervals
5. The formula is a group homomorphism from (R+, *) to (R, +)
6. The nearest integer gives the best chromatic approximation

# Construction / Recognition

## To Convert a Ratio to Semitones

1. Given an interval ratio r
2. Compute x = 12 * log_2(r)
3. Using natural logarithm: x = 12 * ln(r) / ln(2)
4. The integer part gives the nearest chromatic interval
5. The fractional part (times 100) gives the deviation in cents

# Context & Application

Converting to semitones tells a musician which chromatic interval best approximates a given ratio. For non-integer results, the nearest integer gives the best chromatic approximation, and the fractional part (converted to cents by multiplying by 100) gives the deviation from equal temperament. This is essential for comparing just intervals to their equal-tempered approximations (Wright, Ch. 5, p. 71).

# Examples

**Example 1**: Ratio 3/2: x = 12 * log_2(3/2) ~ 7.02 semitones (close to 7 semitones = a fifth).

**Example 2**: Ratio 2: x = 12 * log_2(2) = 12 semitones (one octave, exactly).

**Example 3**: Ratio 5/4: x = 12 * log_2(5/4) ~ 3.86 semitones (close to 4 semitones = a major third; about 14 cents flat).

# Relationships

## Builds Upon

- **Semitone Ratio** -- This is the inverse of the r = 2^(x/12) formula

## Related

- **Converting Ratios to Cents** -- x_cents = 100 * x_semitones
- **Converting Ratios to Octaves** -- x_octaves = x_semitones / 12

# Common Errors

- **Error**: Using log_10 instead of log_2 in the formula
  **Correction**: The formula requires log_2; use the change of base: log_2(r) = ln(r)/ln(2)

# Common Confusions

- **Confusion**: Expecting the result to always be a whole number
  **Clarification**: Only equal-tempered intervals give exact integer semitone values; just intervals typically give non-integer results
- **Confusion**: Interpreting a negative result as an error
  **Clarification**: Negative semitone values correctly indicate downward intervals (r < 1)

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 60-61 (forward formula). Chapter 5: "Logarithms and Musical Intervals," p. 71 (inverse formula, Formula 5.2).

# Verification Notes

- Definition source: Forward formula from Ch. 4, p. 60 (Formula 4.2); inverse from Ch. 5, p. 71
- Confidence rationale: High -- the forward conversion is explicit in Ch. 4; the inverse is the natural algebraic complement
- Uncertainties: The inverse formula is technically developed in Ch. 5, but is listed as a Ch. 4 concept for this extraction
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card (which had Ch. 5 provenance); preserved: homomorphism observation, nearest-integer approximation technique, examples with 3/2 and 5/4
