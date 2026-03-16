---
# === CORE IDENTIFICATION ===
concept: Mean-Tone Fifth
slug: mean-tone-fifth

# === CLASSIFICATION ===
category: tuning-systems
subcategory: mean-tone
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
section: "The Classical Mean-Tone Scale"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - quarter-comma meantone fifth

# === TYPED RELATIONSHIPS ===
prerequisites:
  - just-major-third
  - just-fifth
extends: []
related:
  - mean-tone-scale
  - comma-of-didymus
  - wolf-fifth
contrasts_with:
  - just-fifth

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the mean-tone fifth?"
  - "How is the mean-tone fifth related to the just major third?"
---

# Quick Definition

The mean-tone fifth is the irrational interval with ratio 5^(1/4) (~1.49535), approximately 696.58 cents, deliberately flattened from the just fifth so that four such fifths yield an exact just major third.

# Core Definition

Wright determines the mean-tone fifth by requiring that four iterations equal two octaves plus a just major third: "x^4 = 4 * (5/4) = 5, therefore x = 5^(1/4) ~ 1.49535" (p. 157). This is an irrational interval whose measurement in cents is "1200 * log2(5^(1/4)) = 300 * log2(5) ~ 696.58" (p. 157). It lies about 3 cents flat of the tempered fifth and about 5 cents flat of the just fifth -- "tolerably close" (p. 157).

# Prerequisites

- **Just major third** -- The target that four mean-tone fifths must achieve
- **Just fifth** -- The interval the mean-tone fifth is derived from by flattening

# Key Properties

1. Ratio: 5^(1/4) ~ 1.49535
2. Cents: ~696.58
3. Flat of tempered fifth (700 cents) by ~3.42 cents
4. Flat of just fifth (701.96 cents) by ~5.38 cents
5. Irrational (provable via unique factorization)
6. Four mean-tone fifths = 5 = 4 * (5/4) (two octaves + just major third)
7. Flat of just fifth by exactly one-quarter of the comma of Didymus

# Construction / Recognition

1. Require four fifths to span two octaves plus a just major third: x^4 = 5
2. Solve: x = 5^(1/4) ~ 1.49535
3. Convert to cents: 300 * log2(5) ~ 696.58
4. Note closeness to 3/2 = 1.5: the flatness is barely perceptible

# Context & Application

The mean-tone fifth is "tolerably close" to the just fifth -- the ~5 cent flatness is at the threshold of perceptibility for most listeners. This makes it an acceptable compromise for achieving pure major thirds. However, twelve such fifths fall short of seven octaves by the lesser diesis (128/125 ~ 41 cents), creating the wolf fifth.

# Examples

**Example 1** (p. 157): 5^(1/4) ~ 1.49535 (compare: 3/2 = 1.5, 2^(7/12) ~ 1.49831).

**Example 2** (p. 157): Four mean-tone fifths: (5^(1/4))^4 = 5 = 4 * (5/4).

**Example 3** (p. 157): Flat of just fifth by (81/80)^(1/4) ~ 5.38 cents (one-quarter comma of Didymus).

# Relationships

## Builds Upon
- **Just major third** -- Four mean-tone fifths must yield 5/4 (modulo octaves)

## Enables
- **Mean-tone scale** -- Built entirely from this fifth
- **Wolf fifth** -- The residual from twelve mean-tone fifths not closing the circle

## Related
- **Comma of Didymus** -- The mean-tone fifth is flat by 1/4 of this comma

## Contrasts With
- **Just fifth** -- At 3/2 ~ 701.96 cents, the pure rational fifth that mean-tone deliberately flattens

# Common Errors

- **Error**: Thinking the mean-tone fifth is rational
  **Correction**: 5^(1/4) is irrational; its fourth power 5 is prime, so it cannot be expressed as a ratio of integers

# Common Confusions

- **Confusion**: Thinking the mean-tone fifth is flat by one-quarter of the Pythagorean comma
  **Clarification**: It is flat by one-quarter of the comma of Didymus (~5.38 cents), not the Pythagorean comma (~1.96 cents per fifth)

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," pp. 156-157.

# Verification Notes

- Definition source: Direct calculation from p. 157
- Confidence rationale: Explicitly derived with equation and cent value
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: closeness to 3/2, quarter-comma relationship, irrationality proof sketch
