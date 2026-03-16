---
# === CORE IDENTIFICATION ===
concept: Seventh Chord
slug: seventh-chord

# === CLASSIFICATION ===
category: chord-theory
subcategory: seventh-chords
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Seventh Chord"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - dominant seventh chord
  - dominant seventh

# === TYPED RELATIONSHIPS ===
prerequisites:
  - major-triad
extends:
  - major-triad
related:
  - chord-labeling
  - chord-progressions-and-circle-of-fifths
contrasts_with:
  - minor-seventh-chord
  - major-seventh-chord

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a seventh chord (dominant seventh)?"
  - "How does a seventh chord relate to the major triad?"
---

# Quick Definition

The seventh chord (also called the dominant seventh) is a four-note chord defined by the interval sequence (4, 3, 3, 2) in semitones, built by adding a minor seventh above the root of a major triad.

# Core Definition

The seventh chord is defined by the sequence of modular intervals (4, 3, 3, 2) in Z_12:

root --(4)--> third --(3)--> fifth --(3)--> seventh --(2)--> (root)

This sequence has no non-trivial cyclic symmetries -- its four permutations (4,3,3,2), (3,3,2,4), (3,2,4,3), (2,4,3,3) are all distinct -- so the root, third, fifth, and seventh are uniquely identifiable. The chord contains the major triad with the same root, third, and fifth (Wright, pp. 47-48).

# Prerequisites

- **Major Triad** -- The seventh chord extends the major triad by adding a fourth note

# Key Properties

1. Contains exactly four distinct note classes
2. Interval sequence: (4, 3, 3, 2) in semitones, summing to 12
3. Contains the major triad (4, 3, 5) with the interval 5 split as 3 + 2
4. Root to seventh: minor seventh (10 semitones = 4 + 3 + 3)
5. No non-trivial cyclic symmetries; root is uniquely identifiable
6. Labeled with superscript 7 (e.g., G7, V7)
7. Enharmonically equivalent to the augmented sixth chord (different spelling)

# Construction / Recognition

## To Construct a Seventh Chord

1. Build a major triad (root, up 4, up 3)
2. From the fifth, go up 3 more semitones (minor third) to the seventh
3. Verify: the seventh is 2 semitones (major second) below the root (completing the cycle)

# Context & Application

The seventh chord is one of the most important chords in Western harmony, particularly in its role as the dominant seventh (V7) which creates strong resolution to the tonic. The text notes that this chord plays a significant role "in the development of Western harmony" and that "tuning obstacles" are associated with it. It is enharmonically equivalent to the augmented sixth chord, but when it appears in a "dominant" role (leading around the circle of fifths), it should always be spelled as a seventh chord (Wright, pp. 47-48, 53).

# Examples

**Example 1** (p. 47): Musical notation examples of seventh chords are shown with the interval sequence.

**Example 2** (p. 50): F#m7 in the key of D major would be IIIm7; this illustrates the Roman numeral labeling system applied to seventh-type chords.

**Example 3** (p. 54): The progression V7 -> I appears in the classical circle-of-fifths pattern; the passage I -> V7 -> I (F -> C7 -> F) harmonizes a melody in F major.

# Relationships

## Builds Upon

- **Major Triad** -- The seventh chord adds a seventh to the major triad

## Enables

- **Chord Progressions and Circle of Fifths** -- V7 -> I is the central progression in tonal harmony
- **Functional Harmony** -- The dominant seventh is the primary tension chord in functional harmony

## Related

- **Chord Labeling** -- Labeled with suffix ^7
- **Chord Spelling** -- The augmented sixth chord is enharmonically equivalent but spelled differently

## Contrasts With

- **Minor Seventh Chord** -- (3, 4, 3, 2): has a minor triad base instead of major
- **Major Seventh Chord** -- (4, 3, 4, 1): has a major seventh interval instead of minor seventh

# Common Errors

- **Error**: Spelling the seventh as an augmented sixth when the chord functions as a dominant
  **Correction**: In a dominant role (resolving around the circle of fifths), always spell as a seventh chord

# Common Confusions

- **Confusion**: Thinking "seventh chord" without qualifier could mean any seventh-type chord
  **Clarification**: "Seventh chord" without qualifier specifically means the dominant seventh (4, 3, 3, 2), not the minor seventh or major seventh
- **Confusion**: Believing the seventh chord and augmented sixth chord are different harmonies
  **Clarification**: They are enharmonically equivalent -- same pitches, different spellings reflecting different harmonic functions

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 47-48. Augmented sixth equivalence discussed on p. 53.

# Verification Notes

- Definition source: Direct from pp. 47-48
- Confidence rationale: High -- explicitly defined with interval sequence diagram and examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: augmented sixth equivalence, note about tuning obstacles, V7->I progression
