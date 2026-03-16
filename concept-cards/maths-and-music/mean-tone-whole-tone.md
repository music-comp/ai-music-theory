---
concept: Mean-Tone Whole Tone
slug: mean-tone-whole-tone

category: tuning-systems
subcategory: mean-tone
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
section: "The Classical Mean-Tone Scale"

extraction_confidence: high

aliases:
  - meantone whole step

prerequisites:
  - mean-tone-fifth
extends: []
related:
  - mean-tone-scale
  - greater-whole-tone
  - lesser-whole-tone
  - just-major-third
contrasts_with:
  - greater-whole-tone
  - lesser-whole-tone

answers_questions:
  - "What is the mean-tone whole tone?"
---

# Quick Definition

The mean-tone whole tone has ratio sqrt(5)/2, approximately 193.16 cents, equal to two mean-tone fifths minus one octave, about 7 cents flat of the equal-tempered whole step.

# Core Definition

Wright computes: "each diatonic whole tone equals two mean-tone fifths minus an octave, which has ratio (5^(1/4))^2/2 = sqrt(5)/2, calculated in cents by 1200 * log2(sqrt(5)/2) ~ 193.157, about 7 cents flat of the tempered step" (p. 157). It is an irrational interval.

# Prerequisites

- **Mean-tone fifth** -- The mean-tone whole tone is derived as two mean-tone fifths minus an octave

# Key Properties

1. Ratio: sqrt(5)/2 ~ 1.11803
2. Cents: ~193.16
3. Deviation from equal temperament: ~6.84 cents flat
4. Irrational (involves sqrt(5))
5. Two mean-tone whole tones = just major third: (sqrt(5)/2)^2 = 5/4
6. Lies between the lesser (10/9 ~ 182 cents) and greater (9/8 ~ 204 cents) whole tones
7. All diatonic whole steps in the mean-tone scale are equal (unlike just intonation)

# Construction / Recognition

1. Take two mean-tone fifths: (5^(1/4))^2 = 5^(1/2) = sqrt(5)
2. Subtract one octave: sqrt(5)/2
3. Result: ~193.16 cents

# Context & Application

The mean-tone whole tone unifies the two different whole-tone sizes of just intonation into a single size. Two mean-tone whole tones equal one just major third: (sqrt(5)/2)^2 = 5/4. This contrasts with the Pythagorean scale (where two greater whole tones give 81/64) and just intonation (where one greater plus one lesser gives 5/4). The uniformity simplifies melodic relationships while maintaining just thirds.

# Examples

**Example 1** (p. 157): sqrt(5)/2 ~ 1.11803; 1200 * log2(sqrt(5)/2) ~ 193.16 cents.

**Example 2** (p. 157): Two mean-tone whole tones = just major third: (sqrt(5)/2)^2 = 5/4.

**Example 3**: Compare: greater whole tone (9/8 ~ 204 cents), lesser whole tone (10/9 ~ 182 cents), mean-tone whole tone (~193 cents).

# Relationships

## Builds Upon
- **Mean-tone fifth** -- Derived as two mean-tone fifths minus an octave

## Related
- **Mean-tone scale** -- The whole-step interval of this scale
- **Just major third** -- Two mean-tone whole tones compose to exactly 5/4

## Contrasts With
- **Greater whole tone** -- 9/8 ~ 204 cents, the Pythagorean whole tone
- **Lesser whole tone** -- 10/9 ~ 182 cents, the other just whole tone

# Common Errors

- **Error**: Assuming the mean-tone whole tone is the arithmetic mean of the two just whole tones
  **Correction**: It is determined by the requirement that two of them equal a just major third; it is not a simple average

# Common Confusions

- **Confusion**: Thinking the mean-tone whole tone is rational
  **Clarification**: sqrt(5)/2 is irrational, unlike 9/8 and 10/9 which are rational

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," p. 157.

# Verification Notes

- Definition source: Direct from p. 157
- Confidence rationale: Explicitly calculated with derivation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: two-whole-tones-to-third property, three-way comparison, geometric mean clarification
