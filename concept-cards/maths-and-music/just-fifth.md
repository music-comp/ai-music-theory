---
concept: Just Fifth
slug: just-fifth

category: rational-intervals
subcategory: just-intervals
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Just Fifth"

extraction_confidence: high

aliases:
  - pure fifth
  - "3:2 fifth"
  - "ratio 3:2"

prerequisites:
  - just-interval
  - rational-interval
extends:
  - just-interval
related:
  - just-fourth
  - greater-whole-tone
  - comma-of-pythagoras
  - pythagorean-scale
contrasts_with:
  - mean-tone-fifth

answers_questions:
  - "What is a just fifth?"
  - "How does the just fifth compare to the tempered fifth?"
---

# Quick Definition

The just fifth is the interval with frequency ratio 3/2, approximately 701.96 cents, about 2 cents sharp of the equal-tempered fifth.

# Core Definition

The just fifth is the rational interval corresponding to the ratio 3/2 in Q+. "This is the integral interval 3 lowered by 1 octave" (Wright, p. 140). Its prime factorization is 2^(-1) * 3^1, making it the simplest interval involving the prime 3. Measured in cents: 1200 * log2(3/2) ~ 701.96. The keyboard's fifth is 700 cents, so the just fifth is approximately 1.96 cents sharp.

# Prerequisites

- **Just interval** -- The just fifth is a specific just interval defined by its small-integer ratio
- **Rational interval** -- Understanding that the ratio 3/2 belongs to Q+ and what that means musically

# Key Properties

1. Ratio: 3/2
2. Cents: ~701.96
3. Deviation from equal temperament: ~1.96 cents sharp
4. Prime factorization: 2^(-1) * 3^1 (3-limit interval)
5. Complement: just fourth (4/3), since 3/2 * 4/3 = 2
6. Two just fifths minus one octave = greater whole tone (9/8)
7. Twelve just fifths overshoot seven octaves by the comma of Pythagoras

# Construction / Recognition

1. Start with the 3rd harmonic of a fundamental tone (integral interval 3)
2. Lower by one octave (divide by 2)
3. Result: frequency ratio 3/2
4. On a string of length L, fret at (2/3)*L to produce the just fifth above the open string

# Context & Application

Pythagoras regarded the 3:2 ratio as representing the perfection of the universe. The just fifth is the most consonant interval after the octave and is the foundation of Pythagorean tuning (3-limit). The equal-tempered fifth at 700 cents provides an excellent approximation, being only about 2 cents flat -- close enough that most listeners cannot distinguish them in isolation. The just fifth is the interval between the 2nd and 3rd harmonics of any sustained tone.

# Examples

**Example 1** (p. 140): 1200 * log2(3/2) ~ 701.96 cents; the keyboard's fifth is 700 cents.

**Example 2** (p. 140): The just fifth composed with the just fourth gives the octave: 3/2 * 4/3 = 2.

**Example 3** (p. 141): Two just fifths minus an octave give the greater whole tone: (3/2)^2 / 2 = 9/8.

**Example 4** (pp. 144-145): Twelve just fifths: (3/2)^12 = 531441/4096 ~ 129.75, versus seven octaves = 128.

# Relationships

## Builds Upon
- **Just interval** -- The just fifth is a paradigmatic just interval

## Enables
- **Greater whole tone** -- Two just fifths minus an octave produce the greater whole tone (9/8)
- **Pythagorean scale** -- Built entirely from iterated just fifths
- **Comma of Pythagoras** -- Arises from the failure of twelve just fifths to equal seven octaves

## Related
- **Just fourth** -- Octave complement of the just fifth (4/3 * 3/2 = 2)

## Contrasts With
- **Mean-tone fifth** -- Deliberately flattened to 5^(1/4) ~ 696.58 cents to improve thirds

# Common Errors

- **Error**: Assuming the just fifth and tempered fifth are interchangeable in all contexts
  **Correction**: While the ~2 cent difference is small, it accumulates when fifths are stacked (e.g., twelve just fifths overshoot by ~23.46 cents)

# Common Confusions

- **Confusion**: Confusing the just fifth (3/2 ~ 701.96 cents) with the mean-tone fifth (5^(1/4) ~ 696.58 cents)
  **Clarification**: The mean-tone fifth is deliberately flattened by about 5 cents from the just fifth to improve major thirds

- **Confusion**: Believing twelve just fifths close the circle of fifths
  **Clarification**: Twelve just fifths overshoot seven octaves by the comma of Pythagoras (~23.46 cents)

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 140-145.

# Verification Notes

- Definition source: Direct from p. 140
- Confidence rationale: Explicitly named and defined with cent calculation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complement relationship, iteration examples, mean-tone contrast
