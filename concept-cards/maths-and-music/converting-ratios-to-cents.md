---
# === CORE IDENTIFICATION ===
concept: Converting Ratios to Cents
slug: converting-ratios-to-cents

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
section: "Conversion of Cents to a Ratio"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - ratio to cent conversion
  - cents conversion formula

# === TYPED RELATIONSHIPS ===
prerequisites:
  - cents
extends:
  - cents
related:
  - converting-ratios-to-semitones
  - converting-ratios-to-octaves
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I convert a frequency ratio to cents?"
  - "How do I convert cents to a frequency ratio?"
---

# Quick Definition

The interval of x cents has ratio r = 2^(x/1200) (Formula 4.3). Inversely, a ratio r corresponds to x = 1200 * log_2(r) cents.

# Core Definition

The forward conversion (cents to ratio) is given by:

r = 2^(x/1200)    (Formula 4.3)

This follows from the same reasoning as the semitone formula, with the cent defined as 1/1200 of an octave. The inverse (ratio to cents) requires the logarithm:

x = 1200 * log_2(r)

The forward direction is established in Chapter 4. The inverse is developed in Chapter 5, but is the natural algebraic complement of Formula 4.3 (Wright, pp. 61-62).

# Prerequisites

- **Cents** -- Must understand the cent as a unit (1/1200 of an octave) to use the conversion

# Key Properties

1. Forward: r = 2^(x/1200) converts cents to ratio (Formula 4.3)
2. Inverse: x = 1200 * log_2(r) converts ratio to cents
3. Using natural logarithm: x = 1200 * ln(r) / ln(2)
4. 1200 cents = 1 octave (ratio 2)
5. The conversion is a group homomorphism between (R+, *) and (R, +)
6. Negative cent values indicate downward intervals

# Construction / Recognition

## To Convert Cents to a Ratio

1. Given x cents
2. Compute r = 2^(x/1200)

## To Convert a Ratio to Cents

1. Given ratio r
2. Compute x = 1200 * log_2(r) = 1200 * ln(r)/ln(2)

# Context & Application

This conversion is the workhorse formula for comparing intervals across different tuning systems, measuring deviations from equal temperament, and expressing the sizes of just intervals. It answers questions like "how many cents is a just fifth?" (about 702) or "how far off is the equal-tempered major third from the just major third?" (about 14 cents) (Wright, pp. 61-62).

# Examples

**Example 1** (p. 62): 17 cents corresponds to ratio 2^(17/1200) ~ 1.009868.

**Example 2**: One cent: ratio 2^(1/1200) ~ 1.0005778 (imperceptible).

**Example 3**: Ratio 3/2: x = 1200 * log_2(3/2) ~ 701.96 cents (a just fifth, about 2 cents wider than the tempered fifth at 700 cents).

**Example 4**: Ratio 2: x = 1200 * log_2(2) = 1200 cents (one octave, by definition).

# Relationships

## Builds Upon

- **Cents** -- The cent is the unit being converted to/from

## Related

- **Converting Ratios to Semitones** -- x_semitones = x_cents / 100
- **Converting Ratios to Octaves** -- x_octaves = x_cents / 1200

# Common Errors

- **Error**: Using 1200 * log_10(r) instead of 1200 * log_2(r)
  **Correction**: The formula requires base-2 logarithm; use the change of base formula if needed

# Common Confusions

- **Confusion**: Thinking 1200 is an arbitrary number
  **Clarification**: 1200 = 12 semitones * 100 cents/semitone; it is the number of cents per octave by definition
- **Confusion**: Interpreting negative cent values as errors
  **Clarification**: Negative cents correctly indicate downward intervals (r < 1)

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 61-62 (Formula 4.3, forward conversion). Chapter 5: "Logarithms and Musical Intervals," pp. 70-72 (inverse formula).

# Verification Notes

- Definition source: Forward formula (4.3) from Ch. 4, pp. 61-62; inverse from Ch. 5
- Confidence rationale: High -- explicit formula with worked example
- Uncertainties: Inverse formula is technically from Ch. 5, but the concept is introduced in Ch. 4
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card (which had Ch. 5 provenance); preserved: homomorphism note, worked examples, comparison of just and tempered intervals
