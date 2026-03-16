---
concept: Lesser Diesis
slug: lesser-diesis

category: rational-intervals
subcategory: commas
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
  - "ratio 128:125"
  - enharmonic diesis

prerequisites:
  - just-major-third
extends: []
related:
  - mean-tone-scale
  - wolf-fifth
  - comma-of-pythagoras
  - comma-of-didymus
contrasts_with:
  - comma-of-pythagoras
  - comma-of-didymus

answers_questions:
  - "What is the lesser diesis?"
  - "Why don't three just major thirds equal an octave?"
---

# Quick Definition

The lesser diesis is the interval 128/125, approximately 41.06 cents, representing the amount by which three just major thirds fall short of one octave.

# Core Definition

The lesser diesis is the rational interval 2 / (5/4)^3 = 2 / (125/64) = 128/125 = 2^7 / 5^3 in Q+. Wright demonstrates: "(5/4)^3 = 125/64 < 128/64 = 2. Hence if we tune fifths around the clock so that every four consecutive clock positions equals (modulo octave) a just major third, then the twelfth position does not coincide with the starting point, being flat by the interval ratio 2/(125/64) = 128/125, which is about 41 cents" (p. 157). Measured in cents: 1200 * log2(128/125) ~ 41.06.

# Prerequisites

- **Just major third** -- The lesser diesis arises from three just major thirds failing to close the octave

# Key Properties

1. Ratio: 128/125 = 2^7 / 5^3
2. Cents: ~41.06
3. Prime factorization: 2^7 * 5^(-3) (5-limit, involving only primes 2 and 5)
4. Measures the deficit of three just major thirds below one octave
5. Nearly twice the comma of Pythagoras (~23.46 cents)
6. The comma absorbed by the wolf fifth in mean-tone temperament
7. About two-fifths of a semitone -- clearly audible

# Construction / Recognition

1. Stack three just major thirds: (5/4)^3 = 125/64 ~ 1.953125
2. Compare with one octave: 2 = 128/64
3. Compute the deficit: 128/64 / (125/64) = 128/125
4. Convert to cents: 1200 * log2(128/125) ~ 41.06

# Context & Application

The lesser diesis is the comma that must be accommodated in the mean-tone scale. Since the mean-tone system achieves exact just major thirds by design, twelve mean-tone fifths fall short of seven octaves by exactly one lesser diesis, forcing the placement of a wolf fifth that is sharp by ~41 cents. At about two-fifths of a semitone, it is clearly audible and "unacceptable as a musical interval" in a fifth context.

# Examples

**Example 1** (p. 157): (5/4)^3 = 125/64 ~ 1.953125, short of 2 by the factor 128/125.

**Example 2** (p. 157): 1200 * log2(128/125) ~ 41.06 cents.

**Example 3**: Compare commas: Pythagorean ~ 23.46 cents, Didymus ~ 21.51 cents, lesser diesis ~ 41.06 cents.

**Example 4** (p. 157): In mean-tone: the wolf fifth absorbs the entire lesser diesis, producing ~737 cents vs. ~697 for the normal mean-tone fifth.

# Relationships

## Builds Upon
- **Just major third** -- Three of these fail to reach the octave by the lesser diesis

## Enables
- **Wolf fifth** -- The wolf fifth absorbs the lesser diesis
- **Mean-tone scale** -- The lesser diesis is the structural comma of this system

## Contrasts With
- **Comma of Pythagoras** -- ~23.46 cents, the 3-limit comma; the lesser diesis is nearly twice as large
- **Comma of Didymus** -- ~21.51 cents, the 5-limit comma measuring the gap between whole tones

# Common Errors

- **Error**: Confusing the lesser diesis with the comma of Didymus
  **Correction**: The lesser diesis (128/125 ~ 41 cents) measures three-thirds-to-octave deficit; the comma of Didymus (81/80 ~ 21.5 cents) measures the whole-tone gap

# Common Confusions

- **Confusion**: Thinking the lesser diesis involves the prime 3
  **Clarification**: It involves only primes 2 and 5 (128/125 = 2^7/5^3), unlike the comma of Pythagoras (primes 2 and 3) or Didymus (primes 2, 3, and 5)

- **Confusion**: Thinking "lesser" diesis implies a small interval
  **Clarification**: At ~41 cents, it is the largest of the three main commas; "lesser" distinguishes it from the "greater diesis" (648/625)

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," pp. 157-158.

# Verification Notes

- Definition source: Direct from p. 157
- Confidence rationale: Explicitly calculated with derivation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: three-comma comparison, wolf fifth absorption, greater diesis mention, prime content analysis
