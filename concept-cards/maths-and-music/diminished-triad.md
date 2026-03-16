---
# === CORE IDENTIFICATION ===
concept: Diminished Triad
slug: diminished-triad

# === CLASSIFICATION ===
category: chord-theory
subcategory: triads
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Diminished and Augmented Chords"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - diminished chord
  - dim chord

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-types-and-interval-sequences
extends: []
related:
  - triads
  - diminished-seventh-chord
  - half-diminished-seventh-chord
contrasts_with:
  - augmented-triad
  - major-triad
  - minor-triad

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a diminished triad?"
  - "Does the diminished triad have a discernible root?"
---

# Quick Definition

The diminished triad is a three-note chord defined by the interval sequence (3, 3, 6) in semitones, consisting of a root, a minor third, and a diminished fifth (tritone) above the root.

# Core Definition

The diminished triad is defined by the sequence of modular intervals (3, 3, 6) in Z_12:

root --(3)--> third --(3)--> fifth --(6)--> (root)

The cyclic permutations (3, 3, 6), (3, 6, 3), and (6, 3, 3) are all distinct, so the chord has a discernible root despite having two equal intervals (Wright, p. 47).

# Prerequisites

- **Chord Types and Interval Sequences** -- The diminished triad is defined by a specific interval sequence in Z_12

# Key Properties

1. Contains exactly three distinct note classes
2. Interval sequence: (3, 3, 6) in semitones, summing to 12
3. Root to third: minor third (3 semitones)
4. Root to fifth: tritone (6 semitones = 3 + 3), a diminished fifth
5. Has a discernible root (all cyclic permutations are distinct)
6. Labeled with suffix "dim" or superscript degree symbol (e.g., Bdim or B^0)

# Construction / Recognition

## To Construct a Diminished Triad

1. Choose a root note
2. Go up 3 semitones (minor third) to find the third
3. Go up 3 more semitones (another minor third) to find the fifth
4. Verify: root to fifth spans 6 semitones (tritone/diminished fifth)

# Context & Application

The diminished triad has a tense, unstable quality due to the tritone interval between root and fifth. It naturally occurs on scale degree VII in the major mode (e.g., B-D-F in C major). It often functions as a leading-tone chord resolving to the tonic. Rules of chord spelling tend to be followed less rigorously for diminished triads (Wright, pp. 47, 52).

# Examples

**Example 1** (p. 47): Musical notation examples of diminished chords are shown with the interval sequence (3, 3, 6).

**Example 2**: B diminished: B-D-F, the naturally occurring diminished triad on scale degree VII in C major.

# Relationships

## Builds Upon

- **Chord Types and Interval Sequences** -- The diminished triad is defined by (3, 3, 6)

## Enables

- **Diminished Seventh Chord** -- Extends the diminished triad by adding another minor third
- **Half-Diminished Seventh Chord** -- Extends the diminished triad; shares the first two intervals (3, 3)

## Related

- **Triads** -- One of four standard triad types

## Contrasts With

- **Augmented Triad** -- (4, 4, 4): also symmetric but with major thirds and no discernible root
- **Major Triad** -- (4, 3, 5): has a perfect fifth instead of a tritone
- **Minor Triad** -- (3, 4, 5): shares the minor third from root but has a perfect fifth

# Common Errors

- **Error**: Confusing the diminished triad (3, 3, 6) with the diminished seventh chord (3, 3, 3, 3)
  **Correction**: The diminished triad has three notes; the diminished seventh has four

# Common Confusions

- **Confusion**: Assuming the diminished triad has no discernible root because it has two equal intervals
  **Clarification**: Despite having two 3s, the presence of the distinct interval 6 breaks full cyclic symmetry, making the root identifiable
- **Confusion**: Calling the fifth of a diminished triad a "perfect fifth"
  **Clarification**: The "fifth" spans 6 semitones (a diminished fifth/tritone), not 7 semitones (a perfect fifth)

# Source Reference

Chapter 3: "Harmony and Related Numerology," p. 47. Chord spelling for diminished triads is also discussed on p. 52.

# Verification Notes

- Definition source: Direct from p. 47
- Confidence rationale: High -- explicitly defined with interval sequence
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: observation about less rigorous spelling rules, note about root discernibility despite repeated intervals
