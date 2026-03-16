---
concept: Just Major Third
slug: just-major-third

category: rational-intervals
subcategory: just-intervals
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Just Major Third"

extraction_confidence: high

aliases:
  - pure major third
  - "5:4 third"
  - "ratio 5:4"

prerequisites:
  - just-interval
  - rational-interval
extends:
  - just-interval
related:
  - just-minor-third
  - greater-whole-tone
  - lesser-whole-tone
  - justly-tuned-chords
contrasts_with:
  - pythagorean-major-third

answers_questions:
  - "What is the just major third?"
  - "How does the just major third compare to the tempered major third?"
---

# Quick Definition

The just major third is the interval with frequency ratio 5/4, approximately 386.31 cents, about 14 cents flat of the equal-tempered major third.

# Core Definition

The just major third is the rational interval corresponding to the ratio 5/4 in Q+. "The interval 5/4 is the integral interval 5 minus two octaves. Recall that 5 is about 14 cents less than the keyboard's two octaves plus a major third. Hence 5/4 is the same amount flat of the keyboard major third" (Wright, p. 141). Its prime factorization is 2^(-2) * 5^1, making it the simplest interval involving the prime 5.

# Prerequisites

- **Just interval** -- The just major third is a specific just interval
- **Rational interval** -- Understanding frequency ratios as elements of Q+

# Key Properties

1. Ratio: 5/4
2. Cents: ~386.31
3. Deviation from equal temperament: ~13.69 cents flat
4. Prime factorization: 2^(-2) * 5^1 (5-limit interval)
5. Just major third + just minor third = just fifth: 5/4 * 6/5 = 3/2
6. Greater whole tone + lesser whole tone = just major third: 9/8 * 10/9 = 5/4
7. Three just major thirds do not equal an octave: (5/4)^3 = 125/64 < 2

# Construction / Recognition

1. Start with the 5th harmonic of a fundamental tone (integral interval 5)
2. Lower by two octaves (divide by 4)
3. Result: frequency ratio 5/4
4. Alternatively, compose a greater whole tone (9/8) with a lesser whole tone (10/9): 9/8 * 10/9 = 5/4

# Context & Application

The just major third is the cornerstone of 5-limit tuning and just intonation. It forms the middle voice of the justly tuned major triad (4:5:6). The equal-tempered major third at 400 cents is noticeably sharper -- this ~14-cent discrepancy is one of the most audible compromises of equal temperament. The acceptance of thirds into Western music in the 14th-15th centuries drove the development of mean-tone temperament, which prioritizes just major thirds.

# Examples

**Example 1** (p. 141): 1200 * log2(5/4) ~ 386.31 cents; the keyboard major third is 400 cents, about 14 cents sharp.

**Example 2** (p. 142): Just major third + just minor third = just fifth: 5/4 * 6/5 = 3/2.

**Example 3** (p. 142): Greater whole tone + lesser whole tone = just major third: 9/8 * 10/9 = 5/4.

**Example 4** (Ch. 12, p. 157): (5/4)^3 = 125/64 < 2, so three just major thirds fall short of an octave by the lesser diesis (128/125 ~ 41.06 cents).

# Relationships

## Builds Upon
- **Just interval** -- The just major third is a fundamental 5-limit just interval

## Enables
- **Justly tuned chords** -- The just major triad is 4:5:6, built on the just major third
- **Just intonation scale** -- Tuned so that I, IV, and V triads have just major thirds
- **Mean-tone scale** -- Designed to achieve exact just major thirds
- **Lesser diesis** -- Defined as the shortfall of three just major thirds from an octave

## Related
- **Just minor third** -- Together with the just major third, spans a just fifth (5/4 * 6/5 = 3/2)
- **Greater whole tone** and **Lesser whole tone** -- Compose to form the just major third

## Contrasts With
- **Pythagorean major third** -- At 81/64 ~ 407.82 cents, it is sharper by the comma of Didymus (81/80)

# Common Errors

- **Error**: Assuming the tempered major third (400 cents) is a good approximation of the just major third
  **Correction**: The ~14 cent discrepancy is clearly audible to trained ears; this is one of equal temperament's biggest compromises

# Common Confusions

- **Confusion**: Confusing the just major third (5/4 ~ 386 cents) with the Pythagorean major third (81/64 ~ 408 cents)
  **Clarification**: These differ by the comma of Didymus (~21.5 cents); the Pythagorean version is even sharper than the tempered one

- **Confusion**: Assuming three just major thirds equal an octave (as they do in equal temperament)
  **Clarification**: (5/4)^3 = 125/64, which falls short of 2 by the ratio 128/125 (~41 cents)

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," p. 141.

# Verification Notes

- Definition source: Direct from p. 141
- Confidence rationale: Explicitly named and defined with cent calculation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: composition relationships (with minor third, whole tones), Pythagorean contrast, three-thirds calculation
