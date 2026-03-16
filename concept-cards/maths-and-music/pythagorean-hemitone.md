---
concept: Pythagorean Hemitone
slug: pythagorean-hemitone

category: tuning-systems
subcategory: pythagorean
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
section: "The Pythagorean Scale"

extraction_confidence: high

aliases:
  - Pythagorean limma
  - "ratio 256:243"

prerequisites:
  - pythagorean-scale
  - greater-whole-tone
extends: []
related:
  - just-semitone
contrasts_with:
  - just-semitone

answers_questions:
  - "What is the half-step interval in the Pythagorean scale?"
---

# Quick Definition

The Pythagorean hemitone is the half-step interval in the Pythagorean scale with ratio 256/243 (= 2^8/3^5), approximately 90.22 cents, about 10 cents flat of the equal-tempered semitone.

# Core Definition

The Pythagorean hemitone is the rational interval 256/243 = 2^8/3^5, appearing as both half-step intervals in the Pythagorean scale (between scale degrees 3-4 and 7-8). Wright notes that "Pythagoras called [it] a hemitone" (p. 154). It is a 3-limit interval. Measured in cents: 1200 * log2(256/243) ~ 90.22.

# Prerequisites

- **Pythagorean scale** -- The hemitone is the half-step interval of this scale
- **Greater whole tone** -- The hemitone complements five greater whole tones to fill the octave

# Key Properties

1. Ratio: 256/243 = 2^8/3^5
2. Cents: ~90.22
3. Deviation from equal temperament: ~9.78 cents flat
4. Prime factorization: 2^8 * 3^(-5) (3-limit interval)
5. Narrower than half a Pythagorean whole tone: (9/8)^(1/2) ~ 101.96 cents
6. Narrower than the just semitone (16/15 ~ 111.73 cents) by about 22 cents
7. Appears at intervals 3-to-4 and 7-to-8 in the Pythagorean scale

# Construction / Recognition

1. From the Pythagorean scale table, compute the interval 3-hat to 4-hat: (4/3) / (81/64) = 256/243
2. Alternatively, the octave minus five greater whole tones divided by 2: (2 / (9/8)^5) / 2 ... is not the simplest path
3. More directly: 256/243 is what remains after fitting five 9/8 whole tones into the octave

# Context & Application

Pythagoras named this interval despite it not being exactly half of any whole tone. It gives the Pythagorean scale a distinctive character with wide whole steps (~204 cents) and narrow half steps (~90 cents) compared to equal temperament. The narrow half steps contribute to the melodic character of medieval music tuned in this system.

# Examples

**Example 1** (p. 154): 256/243 = 2^8/3^5 ~ 90.22 cents.

**Example 2** (p. 154): Compare with the equal-tempered semitone (100 cents) and the just semitone (16/15 ~ 111.73 cents).

**Example 3** (p. 154): Half of the Pythagorean whole tone: (9/8)^(1/2) ~ 101.96 cents -- the hemitone is smaller.

**Example 4** (p. 154): Appears at both half-step positions in the Pythagorean scale: 3-to-4 and 7-to-8.

# Relationships

## Builds Upon
- **Pythagorean scale** -- The hemitone is defined within this scale
- **Greater whole tone** -- Five whole tones plus two hemitones fill the octave

## Related
- **Just semitone** -- A different half-step interval (16/15) from 5-limit tuning

## Contrasts With
- **Just semitone** -- 16/15 ~ 111.73 cents is about 22 cents wider than the hemitone

# Common Errors

- **Error**: Assuming the hemitone is exactly half the Pythagorean whole tone
  **Correction**: The hemitone (~90 cents) is narrower than half a whole tone (~102 cents); it is called "hemitone" by convention

# Common Confusions

- **Confusion**: Confusing the Pythagorean hemitone with the just semitone
  **Clarification**: The hemitone (256/243 ~ 90 cents) is 3-limit; the just semitone (16/15 ~ 112 cents) is 5-limit. They differ by ~22 cents

- **Confusion**: Thinking the complex ratio 256/243 implies a complex interval
  **Clarification**: Despite the large numbers, it involves only the primes 2 and 3

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," pp. 153-154.

# Verification Notes

- Definition source: Direct from pp. 153-154
- Confidence rationale: Explicitly named with ratio and context
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: half-whole-tone comparison, just semitone contrast, naming convention note
