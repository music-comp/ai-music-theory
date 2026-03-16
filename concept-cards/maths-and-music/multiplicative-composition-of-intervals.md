---
# === CORE IDENTIFICATION ===
concept: Multiplicative Composition of Intervals
slug: multiplicative-composition-of-intervals

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: ratios
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Multiplicativity"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - multiplicativity of intervals
  - interval multiplication

# === TYPED RELATIONSHIPS ===
prerequisites:
  - interval-as-frequency-ratio
extends: []
related:
  - semitone-ratio
  - multiplicative-and-additive-measurements
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do intervals combine when one follows another?"
  - "Why does combining two intervals involve multiplying their ratios?"
---

# Quick Definition

When two intervals are juxtaposed (one followed by another), the resulting interval's ratio is the product of the two individual ratios. This multiplicative property is the fundamental arithmetic of interval ratios.

# Core Definition

If x_1 = f_2/f_1 represents the interval from f_1 to f_2, and x_2 = f_3/f_2 represents the interval from f_2 to f_3, then the composed interval from f_1 to f_3 has ratio x_1 * x_2 = (f_2/f_1)(f_3/f_2) = f_3/f_1. "Thus the result of juxtaposing two intervals, i.e., following one interval by another, is given by multiplying the two corresponding real numbers" (Wright, p. 59).

# Prerequisites

- **Interval as Frequency Ratio** -- Must understand that intervals are ratios to see why composition is multiplicative

# Key Properties

1. Juxtaposing intervals = multiplying ratios
2. The cancellation of the common frequency (f_2) makes the multiplication work
3. (R+, *) is the natural group for interval arithmetic
4. Identity element: 1 (unison)
5. Inverse of ratio r: r^(-1) (opposite interval)
6. The measurement of intervals by ratio is called "multiplicative" because of this property

# Construction / Recognition

## To Compose Two Intervals

1. Identify the ratio r_1 of the first interval
2. Identify the ratio r_2 of the second interval
3. Multiply: r_1 * r_2 = combined interval ratio
4. This is equivalent to going from f_1 to f_2 (by r_1) then from f_2 to f_3 (by r_2), yielding f_3/f_1

# Context & Application

This property explains why intervals "add" in the usual musical sense when their ratios are multiplied. It is also the principle that enables the derivation of the semitone ratio: since twelve iterations of the semitone give the octave, s^12 = 2, hence s = 2^(1/12). The multiplicative framework is more fundamental than the additive one; the additive framework is derived from it via logarithms (Wright, pp. 59-60).

# Examples

**Example 1** (p. 60): Twelve semitones compose to one octave: (2^(1/12))^12 = 2.

**Example 2**: Two octaves: 2 * 2 = 4 (the ratio of a double octave).

**Example 3**: Major third + minor third = fifth: 2^(4/12) * 2^(3/12) = 2^(7/12).

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- Multiplicativity follows from the ratio definition of intervals

## Enables

- **Semitone Ratio** -- Derived by requiring s^12 = 2
- **Multiplicative and Additive Measurements** -- The distinction between multiplicative and additive measurement rests on this property

## Related

- **Converting Ratios to Semitones** -- The logarithm converts multiplicative to additive
- **Converting Ratios to Cents** -- Same principle applied to the cent scale

# Common Errors

- **Error**: Adding ratios instead of multiplying them when combining intervals
  **Correction**: A major third (ratio ~1.26) followed by a minor third (ratio ~1.19) gives ~1.26 * 1.19 = ~1.50, not 1.26 + 1.19

# Common Confusions

- **Confusion**: Thinking "a third plus a third equals a fifth" involves addition of ratios
  **Clarification**: The "plus" is additive language for a multiplicative operation; the ratios are multiplied, and the semitone counts add because semitones are logarithmic units
- **Confusion**: Wondering why equal temperament uses 2^(1/12) rather than 2/12
  **Clarification**: Because intervals compose multiplicatively, equal subdivision requires the 12th root, not the 12th fraction

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 59-60. The derivation of the semitone ratio follows directly from this principle.

# Verification Notes

- Definition source: Direct from p. 59, "Multiplicativity" section
- Confidence rationale: High -- explicitly stated property with clear derivation
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: group structure (R+, *), logarithm as bridge between frameworks, 2^(1/12) vs 2/12 clarification
