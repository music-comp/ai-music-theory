---
# === CORE IDENTIFICATION ===
concept: Symmetric Chords and Irrational Temperament
slug: symmetric-chords-and-irrational-temperament

# === CLASSIFICATION ===
category: tuning-systems
subcategory: temperament
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "Justly Tuned Chords"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - irrationality-of-equally-tempered-intervals
  - justly-tuned-chords
extends:
  - irrationality-of-equally-tempered-intervals
related:
  - just-major-third
  - just-minor-third
  - equal-temperament-versus-just-intonation
contrasts_with:
  - justly-tuned-chords

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Can augmented and diminished chords be justly tuned?"
  - "Why do symmetric chords require irrational temperament?"
---

# Quick Definition

Augmented triads and diminished seventh chords, which divide the octave into equal parts, can only be rendered using irrational intervals and are therefore unachievable in just intonation. Their symmetry requires equal temperament.

# Core Definition

Wright observes: "The theorem stated and proved earlier in this chapter shows that such equal partitions of the octave are unachievable in just intonation; completely symmetric chords can only be rendered using irrational temperament. Thus one might argue that augmented and full diminished chords are best rendered in equal temperament; perhaps they are even a result of equal temperament. This accounts for their unstable character" (p. 149).

# Prerequisites

- **Irrationality of equally tempered intervals** -- The theorem that proves equal divisions of the octave require irrational ratios
- **Justly tuned chords** -- Understanding what it means for a chord to be justly tuned

# Key Properties

1. Augmented triads divide the octave into three equal parts: each interval is 2^(1/3) (irrational)
2. Diminished seventh chords divide the octave into four equal parts: each interval is 2^(1/4) (irrational)
3. Three just major thirds do not close the octave: (5/4)^3 = 125/64 < 2
4. Four just minor thirds overshoot the octave: (6/5)^4 = 1296/625 > 2
5. Symmetry = equal intervals = no discernible root = maximum harmonic ambiguity
6. Just approximations lose the root-ambiguity that makes these chords musically useful

# Construction / Recognition

1. Equal division of the octave requires solving x^n = 2, giving x = 2^(1/n)
2. By the irrationality theorem, 2^(1/n) is irrational for n > 1
3. Therefore any chord with perfectly equal intervals must use irrational ratios
4. Just approximations introduce unequal intervals, destroying the symmetry

# Context & Application

The functionality of augmented and diminished seventh chords relies on having equal intervals, hence no discernible root. This is what makes them useful for modulation (any note can serve as root) and gives them their unstable, tension-filled character. These chords arguably did not exist in pre-tempered tuning systems and may be considered a product of equal temperament itself.

# Examples

**Example 1** (p. 149): Augmented triad requires three equal major thirds of 2^(1/3) each (irrational). Three just major thirds: (5/4)^3 = 125/64 < 2 (falls short by 128/125 ~ 41 cents).

**Example 2** (p. 149): Diminished seventh requires four equal minor thirds of 2^(1/4) each (irrational). Four just minor thirds: (6/5)^4 = 1296/625 > 2 (overshoots by ~62.6 cents).

# Relationships

## Builds Upon
- **Irrationality of equally tempered intervals** -- The theorem underlying the impossibility

## Related
- **Just major third** -- Three just major thirds fail to close the octave
- **Just minor third** -- Four just minor thirds fail to close the octave
- **Equal temperament versus just intonation** -- Symmetric chords exemplify the unique capabilities of equal temperament

## Contrasts With
- **Justly tuned chords** -- Symmetric chords are the opposite: they cannot be justly tuned

# Common Errors

- **Error**: Trying to justify-tune an augmented triad and expecting equal intervals
  **Correction**: Any just approximation necessarily introduces unequal intervals, destroying the chord's symmetry and root ambiguity

# Common Confusions

- **Confusion**: Thinking symmetric chords are "poorly tuned" in equal temperament
  **Clarification**: They are perfectly tuned for their mathematical purpose -- equal division of the octave is exactly what equal temperament provides

- **Confusion**: Thinking the impossibility is an approximation issue
  **Clarification**: It is a theorem (exact mathematical impossibility), not a matter of precision

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 148-149.

# Verification Notes

- Definition source: Direct quote from p. 149
- Confidence rationale: Explicitly discussed with mathematical justification
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: specific calculations for augmented and diminished, root-ambiguity insight, "product of equal temperament" argument
