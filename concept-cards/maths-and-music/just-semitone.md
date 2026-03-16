---
concept: Just Semitone
slug: just-semitone

category: rational-intervals
subcategory: just-intervals
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Just Semitone"

extraction_confidence: high

aliases:
  - "ratio 16:15"
  - diatonic semitone

prerequisites:
  - just-interval
extends:
  - just-interval
related:
  - just-fourth
  - just-major-third
  - just-intonation-scale
contrasts_with:
  - pythagorean-hemitone

answers_questions:
  - "What is the just semitone?"
---

# Quick Definition

The just semitone is the interval with frequency ratio 16/15, approximately 111.73 cents, about 12 cents sharp of the equal-tempered semitone. It is the half-step interval in the just intonation scale.

# Core Definition

The just semitone is the rational interval corresponding to the ratio 16/15 = 2^4 / (3 * 5) in Q+. Wright notes it is "the first ratio we have listed whose denominator involves more than one prime" and that "by virtue of the fact that it has larger numerator and denominator than any of those previously discussed, it gives an interval that might be considered 'less just,' and which one might expect to be less consonant" (p. 143). Measured in cents: 1200 * log2(16/15) ~ 111.73.

# Prerequisites

- **Just interval** -- The just semitone is a specific just interval

# Key Properties

1. Ratio: 16/15
2. Cents: ~111.73
3. Deviation from equal temperament: ~11.73 cents sharp
4. Prime factorization: 2^4 * 3^(-1) * 5^(-1) (5-limit interval)
5. First common just interval with two odd primes in the denominator
6. Derivable as just fourth minus just major third: (4/3) / (5/4) = 16/15
7. Also derivable as just major third minus just fourth = just semitone downward (Exercise 6e)

# Construction / Recognition

1. Take the just fourth (4/3) and subtract the just major third (5/4)
2. Compute: (4/3) / (5/4) = (4/3) * (4/5) = 16/15
3. Result: ~111.73 cents

# Context & Application

The just semitone is the half-step interval in the just intonation scale, appearing between scale degrees 3-4 and 7-8. It is noticeably wider than the equal-tempered semitone (100 cents) by about 12 cents. In the Pythagorean scale, the half-step is instead the hemitone (256/243 ~ 90.22 cents), a substantially narrower interval. Interestingly, the "exotic" interval 17/16 (~104.96 cents) is a better approximation of the tempered semitone than the just semitone.

# Examples

**Example 1** (p. 143): 1200 * log2(16/15) ~ 111.73 cents; tempered semitone = 100 cents, about 12 cents sharp.

**Example 2** (Ch. 12, p. 156): In the just intonation scale, both half-step intervals (3-to-4 and 7-to-8) are 16/15.

**Example 3** (p. 144): Compare with 17/16 ~ 104.96 cents, which is only ~5 cents from the tempered semitone.

# Relationships

## Builds Upon
- **Just interval** -- A specific 5-limit just interval

## Enables
- **Just intonation scale** -- Provides the half-step intervals at 3-4 and 7-8

## Related
- **Just fourth** and **Just major third** -- The just semitone is their difference

## Contrasts With
- **Pythagorean hemitone** -- At 256/243 ~ 90.22 cents, a different and narrower half-step used in Pythagorean tuning

# Common Errors

- **Error**: Assuming the just semitone is close to the tempered semitone
  **Correction**: The 12-cent discrepancy is substantial; the "exotic" 17/16 is actually closer to the tempered semitone

# Common Confusions

- **Confusion**: Thinking the just semitone and Pythagorean hemitone are the same
  **Clarification**: The just semitone (16/15 ~ 112 cents) and the Pythagorean hemitone (256/243 ~ 90 cents) differ by about 22 cents -- they are "half steps" in different tuning systems

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 142-143.

# Verification Notes

- Definition source: Direct from pp. 142-143
- Confidence rationale: Explicitly named and defined with cent calculation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: "less just" observation, 17/16 comparison, Pythagorean hemitone contrast
