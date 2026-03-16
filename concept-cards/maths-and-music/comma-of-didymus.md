---
# === CORE IDENTIFICATION ===
concept: Comma of Didymus
slug: comma-of-didymus

# === CLASSIFICATION ===
category: rational-intervals
subcategory: commas
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Lesser Whole Tone"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - syntonic comma
  - "ratio 81:80"
  - Ptolemaic comma

# === TYPED RELATIONSHIPS ===
prerequisites:
  - greater-whole-tone
  - lesser-whole-tone
extends: []
related:
  - just-major-third
  - pythagorean-major-third
  - mean-tone-scale
  - mean-tone-fifth
contrasts_with:
  - comma-of-pythagoras

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the comma of Didymus?"
  - "What is the difference between the two just whole tones?"
---

# Quick Definition

The comma of Didymus (syntonic comma) is the small interval 81/80, approximately 21.51 cents, representing the difference between the greater whole tone (9/8) and the lesser whole tone (10/9), or equivalently, the difference between the Pythagorean major third and the just major third.

# Core Definition

The comma of Didymus is the rational interval corresponding to the ratio 81/80 = 3^4 / (2^4 * 5) in Q+. Wright defines it as the interval between the lesser and greater whole tones: "The interval between the lesser and greater whole tones has a ratio of 9/8 / (10/9) = 81/80 which is measured in cents by 1200 * log2(81/80) ~ 21.50. This is called the comma of Didymus" (p. 142).

# Prerequisites

- **Greater whole tone** -- One of the two intervals whose difference defines the comma
- **Lesser whole tone** -- The other interval whose difference defines the comma

# Key Properties

1. Ratio: 81/80
2. Cents: ~21.51
3. Prime factorization: 3^4 * 2^(-4) * 5^(-1) (5-limit interval)
4. Equals (9/8) / (10/9) = 81/80 (greater minus lesser whole tone)
5. Equals (81/64) / (5/4) = 81/80 (Pythagorean major third minus just major third)
6. Roughly one-fifth of a semitone -- clearly audible
7. The mean-tone fifth is flat of the just fifth by one-quarter of the comma of Didymus

# Construction / Recognition

1. Compute the ratio of the greater whole tone to the lesser: (9/8) / (10/9) = 81/80
2. Alternatively, compute the ratio of the Pythagorean major third to the just major third: (81/64) / (5/4) = 81/80
3. Result: 81/80 ~ 21.51 cents

# Context & Application

The comma of Didymus measures the fundamental incompatibility between 3-limit tuning (Pythagorean) and 5-limit tuning (just intonation). It explains why the Pythagorean scale has poor thirds: the Pythagorean major third (81/64) is sharp of the just major third (5/4) by this comma. The mean-tone scale addresses this by flattening each fifth by one-quarter of the comma of Didymus, so that four such fifths yield an exact just major third. The comma is named after Didymus the Musician (1st century BC).

# Examples

**Example 1** (p. 142): (9/8) / (10/9) = 81/80, the difference between the two whole tones.

**Example 2** (Ch. 12, p. 154): (81/64) / (5/4) = 81/80, the Pythagorean major third minus the just major third.

**Example 3** (p. 142): 1200 * log2(81/80) ~ 21.51 cents, roughly one-fifth of a semitone.

**Example 4** (Exercise 6d, p. 150): The comma of Didymus + one octave = two just fifths minus a lesser whole tone: 81/80 * 2 = (3/2)^2 / (10/9).

# Relationships

## Builds Upon
- **Greater whole tone** -- One component of the comma's definition
- **Lesser whole tone** -- The other component

## Enables
- **Mean-tone scale** -- Built by distributing one-quarter of the comma across each fifth
- **Pythagorean major third** -- Identified as being sharp of the just major third by exactly this comma

## Related
- **Just major third** -- The "target" interval that the comma measures distance from

## Contrasts With
- **Comma of Pythagoras** -- A different comma (~23.46 cents) arising from the failure of twelve just fifths to close the circle; involves only primes 2 and 3

# Common Errors

- **Error**: Confusing the comma of Didymus with the comma of Pythagoras
  **Correction**: The comma of Didymus (81/80 ~ 21.51 cents) measures the 3-limit vs. 5-limit gap; the comma of Pythagoras (3^12/2^19 ~ 23.46 cents) measures the twelve-fifths-vs.-seven-octaves gap

# Common Confusions

- **Confusion**: Thinking the comma of Didymus is negligibly small
  **Clarification**: At ~21.5 cents (roughly one-fifth of a semitone), it is clearly audible and constitutes a significant tuning discrepancy

- **Confusion**: Thinking the comma measures a flaw in a single tuning system
  **Clarification**: The comma measures the inherent gap between 3-limit and 5-limit tuning; it is a mathematical property, not a defect of any particular system

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," p. 142. Also referenced in Chapter 12, pp. 154, 160.

# Verification Notes

- Definition source: Direct quote from p. 142
- Confidence rationale: Explicitly named and defined with exact ratio and cent value
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: dual definition (whole-tone gap and third gap), mean-tone relationship, Exercise 6d identity
