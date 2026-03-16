---
concept: Greater Whole Tone
slug: greater-whole-tone

category: rational-intervals
subcategory: just-intervals
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Greater Whole Tone (Pythagorean Whole Tone)"

extraction_confidence: high

aliases:
  - Pythagorean whole tone
  - "ratio 9:8"

prerequisites:
  - just-fifth
extends:
  - just-interval
related:
  - lesser-whole-tone
  - comma-of-didymus
  - pythagorean-major-third
  - pythagorean-scale
contrasts_with:
  - lesser-whole-tone
  - mean-tone-whole-tone

answers_questions:
  - "What is the greater whole tone?"
  - "Why is it called the Pythagorean whole tone?"
---

# Quick Definition

The greater whole tone (also called the Pythagorean whole tone) is the interval with frequency ratio 9/8, approximately 203.91 cents, about 4 cents sharp of the equal-tempered whole step.

# Core Definition

The greater whole tone is the rational interval corresponding to the ratio 9/8 = 3^2/2^3 in Q+. "Since 3 is approximately one octave plus a fifth, the interval 9/8 is twice that, lowered by three octaves: 9/8 = (3/2)^2 * (1/2)" (Wright, p. 141). It is a 3-limit interval. Wright refrains from calling it "the just whole tone" because of the lesser whole tone (10/9). It is called the Pythagorean whole tone "for a reason that will be given in Chapter 12" -- namely, it is the whole step in the Pythagorean scale.

# Prerequisites

- **Just fifth** -- The greater whole tone is derived as two just fifths minus an octave

# Key Properties

1. Ratio: 9/8
2. Cents: ~203.91
3. Deviation from equal temperament: ~3.91 cents sharp
4. Prime factorization: 2^(-3) * 3^2 (3-limit interval)
5. Equals two just fifths minus one octave: (3/2)^2 / 2 = 9/8
6. Exceeds the lesser whole tone (10/9) by the comma of Didymus: 9/8 / (10/9) = 81/80
7. Two greater whole tones = Pythagorean major third: (9/8)^2 = 81/64

# Construction / Recognition

1. Start with the just fifth (3/2)
2. Iterate once more: (3/2)^2 = 9/4
3. Bring down one octave: 9/4 / 2 = 9/8
4. Result: the greater whole tone at ~203.91 cents

# Context & Application

The greater whole tone is the whole-step interval in the Pythagorean scale, where it appears between all five whole-step pairs of adjacent scale tones. In the just intonation scale, it appears at intervals 1-to-2, 4-to-5, and 6-to-7 (the other whole steps being the lesser whole tone). The tempered whole step (200 cents) lies between the lesser (~182 cents) and greater (~204 cents) whole tones, closer to the latter.

# Examples

**Example 1** (p. 141): 1200 * log2(9/8) ~ 203.91 cents; tempered step = 200 cents, difference ~ 3.91 cents.

**Example 2** (p. 141): Two just fifths minus an octave: (3/2)^2 / 2 = 9/4 / 2 = 9/8.

**Example 3** (p. 142): Greater whole tone + lesser whole tone = just major third: 9/8 * 10/9 = 5/4.

**Example 4** (p. 142): The cents scale shows: lesser whole tone (182) < tempered step (200) < greater whole tone (204).

# Relationships

## Builds Upon
- **Just fifth** -- The greater whole tone equals two just fifths minus an octave

## Enables
- **Pythagorean scale** -- Uses the greater whole tone for all five whole-step intervals
- **Pythagorean major third** -- Two greater whole tones compose to 81/64

## Related
- **Just intonation scale** -- Uses the greater whole tone at three of its five whole-step positions

## Contrasts With
- **Lesser whole tone** -- At 10/9 ~ 182.40 cents, the other just whole tone; differs by the comma of Didymus
- **Mean-tone whole tone** -- At sqrt(5)/2 ~ 193.16 cents, a compromise between the two

# Common Errors

- **Error**: Calling 9/8 simply "the just whole tone" without qualification
  **Correction**: There are two just whole tones; 9/8 is specifically the greater (Pythagorean) whole tone

# Common Confusions

- **Confusion**: Thinking equal temperament has a single "correct" whole tone size
  **Clarification**: The tempered step (200 cents) merges two distinct just intervals: the greater whole tone (204 cents) and lesser whole tone (182 cents)

- **Confusion**: Assuming two greater whole tones make a just major third
  **Clarification**: Two greater whole tones make the Pythagorean major third (81/64), not the just major third (5/4); these differ by the comma of Didymus

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," p. 141.

# Verification Notes

- Definition source: Direct from p. 141
- Confidence rationale: Explicitly named and defined with cent calculation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: cents scale comparison, composition relationships, Pythagorean naming explanation
