---
# === CORE IDENTIFICATION ===
concept: Cents
slug: cents

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: measurement
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Microtuning and Cents"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - cent

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semitone-ratio
extends: []
related:
  - n-chromatic-units
  - microtuning
  - converting-ratios-to-cents
  - multiplicative-and-additive-measurements
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a cent in music?"
  - "How many cents are in a semitone? In an octave?"
  - "What ratio corresponds to one cent?"
---

# Quick Definition

A cent is 1/100 of a semitone, or equivalently 1/1200 of an octave. It provides a fine-grained additive unit for measuring musical intervals, particularly useful for microtuning.

# Core Definition

The cent is defined by dividing each semitone into 100 equal intervals, so that 1200 cents equal one octave. The ratio corresponding to one cent is:

c = 2^(1/1200) ~ 1.0005778

The interval of x cents has ratio:

r = 2^(x/1200)    (Formula 4.3)

Cents, like semitones and octaves, is an additive measurement of intervals (Wright, pp. 61-62).

# Prerequisites

- **Semitone Ratio** -- Cents are defined as subdivisions of the semitone

# Key Properties

1. 100 cents = 1 semitone
2. 1200 cents = 1 octave
3. One cent: ratio 2^(1/1200) ~ 1.0005778
4. Cents are an additive measurement (100 + 100 = 200 cents = 2 semitones)
5. The interval of one cent is imperceptible to most listeners
6. Even 10 cents is difficult to perceive
7. Cents are the special case of n-chromatic units with n = 1200

# Construction / Recognition

## To Express an Interval in Cents

1. Given a ratio r, compute x = 1200 * log_2(r)
2. Or given semitones s, compute cents = 100 * s
3. The result is the interval size in cents

# Context & Application

Cents are the standard unit for describing deviations from equal temperament, comparing tuning systems, and specifying microtuning adjustments. They allow precise quantification of small differences that cannot be expressed in whole semitones. The measurement is "fine enough to be quite satisfactory for microtuning" (Wright, pp. 61-62).

# Examples

**Example 1** (p. 62): 17 cents: ratio 2^(17/1200) ~ 1.009868.

**Example 2** (p. 61): 1 cent: ratio 2^(1/1200) ~ 1.0005778 (imperceptible).

**Example 3**: 100 cents = 1 semitone; 1200 cents = 1 octave.

**Example 4**: The just perfect fifth (ratio 3/2) is approximately 702 cents, vs. the equal-tempered fifth at exactly 700 cents -- a difference of about 2 cents.

# Relationships

## Builds Upon

- **Semitone Ratio** -- Cents subdivide the semitone into 100 equal parts

## Enables

- **Converting Ratios to Cents** -- The cent is the unit used in the conversion formula
- **Microtuning** -- Cents provide the resolution needed for microtuning specifications

## Related

- **N-Chromatic Units** -- Cents are n-chromatic units with n = 1200
- **Multiplicative and Additive Measurements** -- Cents are an additive measurement derived from the multiplicative ratio

# Common Errors

- **Error**: Computing cents as a fraction of frequency rather than a fraction of the logarithmic scale
  **Correction**: Cents are equal ratio subdivisions, not equal frequency subdivisions

# Common Confusions

- **Confusion**: Thinking cents are perceptually meaningful units
  **Clarification**: 1 cent is effectively inaudible; cents are a measurement tool, not a perceptual unit
- **Confusion**: Believing cents are "hundredths of a semitone" in the frequency sense
  **Clarification**: Cents are equal logarithmic (ratio) subdivisions of the semitone

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 61-62. Formula 4.3.

# Verification Notes

- Definition source: Direct from pp. 61-62
- Confidence rationale: High -- explicitly defined with formula and examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: n=1200 connection to n-chromatic units, imperceptibility observations, just fifth comparison
