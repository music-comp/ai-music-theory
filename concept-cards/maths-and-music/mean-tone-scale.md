---
concept: Classical Mean-Tone Scale
slug: mean-tone-scale

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
  - mean-tone temperament
  - quarter-comma meantone
  - classical meantone

prerequisites:
  - just-major-third
  - comma-of-didymus
  - mean-tone-fifth
extends: []
related:
  - mean-tone-whole-tone
  - wolf-fifth
  - lesser-diesis
contrasts_with:
  - pythagorean-scale
  - just-intonation-scale

answers_questions:
  - "What is mean-tone temperament?"
  - "What distinguishes mean-tone temperament from Pythagorean tuning?"
---

# Quick Definition

The classical mean-tone scale shrinks fifths equally around the circle so that four consecutive fifths (modulo octave) produce an exact just major third (5/4), using a fifth of ratio 5^(1/4), approximately 696.58 cents.

# Core Definition

Wright explains: "The idea of the mean-tone scale is to shrink the fifths around the clock equally so that the major third spanning four clock positions (modulo octave) is the just major third, having ratio 5:4" (p. 157). The fifth ratio x is determined by x^4 = 4 * (5/4) = 5, giving x = 5^(1/4) ~ 1.49535. Unlike Pythagorean and just intonation scales, the mean-tone scale allows irrational intervals. Its rational intervals involve only primes 2 and 5.

# Prerequisites

- **Just major third** -- The target interval that the scale is designed to achieve
- **Comma of Didymus** -- The mean-tone fifth is flat of the just fifth by one-quarter of this comma
- **Mean-tone fifth** -- The fundamental interval of the scale

# Key Properties

1. Mean-tone fifth: 5^(1/4) ~ 696.58 cents (~3 cents flat of tempered, ~5 cents flat of just)
2. All major thirds are exactly 5/4 (just major third, 386.31 cents)
3. Whole tone: sqrt(5)/2 ~ 193.16 cents (one size, unlike just intonation)
4. Contains irrational intervals (unlike Pythagorean and just intonation)
5. Rational intervals involve only primes 2 and 5
6. Wolf fifth absorbs the lesser diesis (128/125 ~ 41 cents)
7. With wolf placed away from diatonic tones, all diatonic major triads except VII have good tuning

# Construction / Recognition

## To Build a Mean-Tone Scale

1. Set the fifth ratio to x = 5^(1/4) ~ 1.49535
2. Tune consecutive fifths around the circle at this ratio
3. The 12th position falls short of 7 octaves by the lesser diesis (128/125 ~ 41 cents)
4. Place the wolf fifth at a musically inconvenient position (e.g., between 8 and 9 o'clock)
5. Each whole tone = two mean-tone fifths minus an octave = sqrt(5)/2

## Scale Ratios to Tonic

| 1-hat | 2-hat | 3-hat | 4-hat | 5-hat | 6-hat | 7-hat | 8-hat |
|---|---|---|---|---|---|---|---|
| 1/1 | sqrt(5)/2 | 5/4 | 2/5^(1/4) | 5^(1/4) | 5^(3/4)/2 | 5^(5/4)/4 | 2/1 |

# Context & Application

The mean-tone scale emerged in the 14th-15th centuries when thirds became accepted in Western music. Wright notes: "Certain compromises were introduced which detuned fifths in order to improve the sound of thirds. Such scales are called mean-tone scales" (p. 156). The classical mean-tone scale represents the most common compromise, achieving perfect major thirds at the cost of slightly flat fifths and one very bad wolf fifth (~737 cents).

# Examples

**Example 1** (p. 157): Mean-tone fifth: 5^(1/4) ~ 1.49535 (close to 3/2 = 1.5).

**Example 2** (p. 157): 1200 * log2(5^(1/4)) = 300 * log2(5) ~ 696.58 cents.

**Example 3** (p. 157): Three just major thirds: (5/4)^3 = 125/64 < 2, falling short by 128/125 ~ 41 cents.

**Example 4** (p. 157): Mean-tone whole tone: sqrt(5)/2 ~ 193.16 cents, about 7 cents flat of tempered step.

# Relationships

## Builds Upon
- **Just major third** -- The target interval: four mean-tone fifths yield exactly 5/4
- **Comma of Didymus** -- Each fifth is flattened by one-quarter of this comma
- **Mean-tone fifth** -- The fundamental interval of the scale

## Enables
- **Wolf fifth** -- The residual discrepancy that must be placed on the circle

## Related
- **Lesser diesis** -- The comma (128/125 ~ 41 cents) that the wolf fifth absorbs
- **Mean-tone whole tone** -- The single whole-step size of the scale

## Contrasts With
- **Pythagorean scale** -- Optimizes fifths at the expense of thirds; mean-tone does the opposite
- **Just intonation scale** -- Uses only rational intervals and has two whole-tone sizes

# Common Errors

- **Error**: Thinking the mean-tone scale is purely rational
  **Correction**: The mean-tone fifth (5^(1/4)) is irrational; the scale mixes rational and irrational intervals

# Common Confusions

- **Confusion**: Thinking "mean tone" means the arithmetic average of two intervals
  **Clarification**: "Mean tone" refers to the whole step being the geometric mean in a specific sense -- two mean-tone whole tones compose to an exact just major third

- **Confusion**: Thinking the wolf fifth is similar in size to the Pythagorean comma
  **Clarification**: The wolf fifth absorbs ~41 cents (the lesser diesis), nearly twice the Pythagorean comma (~23 cents), making it more disruptive

- **Confusion**: Thinking the mean-tone fifth is flat of the just fifth by the Pythagorean comma
  **Clarification**: It is flat by one-quarter of the comma of Didymus (~5.38 cents), not the Pythagorean comma

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," pp. 156-158.

# Verification Notes

- Definition source: Direct from pp. 156-157
- Confidence rationale: Explicit definition with full calculation and scale table
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete scale table, lesser diesis calculation, quarter-comma relationship, historical context
