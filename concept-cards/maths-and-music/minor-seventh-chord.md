---
# === CORE IDENTIFICATION ===
concept: Minor Seventh Chord
slug: minor-seventh-chord

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
section: "Minor Seventh Chord"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - minor-triad
extends:
  - minor-triad
related:
  - chord-labeling
contrasts_with:
  - seventh-chord
  - major-seventh-chord

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a minor seventh chord?"
  - "How does the minor seventh chord relate to the minor triad?"
---

# Quick Definition

The minor seventh chord is a four-note chord defined by the interval sequence (3, 4, 3, 2) in semitones, built by adding a minor seventh above the root of a minor triad.

# Core Definition

The minor seventh chord is defined by the sequence of modular intervals (3, 4, 3, 2) in Z_12:

root --(3)--> third --(4)--> fifth --(3)--> seventh --(2)--> (root)

This sequence admits no non-trivial cyclic symmetries -- although it contains two 3s, the four cyclic permutations (3,4,3,2), (4,3,2,3), (3,2,3,4), (2,3,4,3) are all distinct. The chord contains the minor triad with the same root, third, and fifth (Wright, p. 48).

# Prerequisites

- **Minor Triad** -- The minor seventh chord extends the minor triad by adding a fourth note

# Key Properties

1. Contains exactly four distinct note classes
2. Interval sequence: (3, 4, 3, 2) in semitones, summing to 12
3. Contains the minor triad (3, 4, 5) with the interval 5 split as 3 + 2
4. Root to seventh: minor seventh (10 semitones = 3 + 4 + 3)
5. No non-trivial cyclic symmetries; root is uniquely identifiable
6. Labeled with suffix m7 (e.g., Dm7, F#m7)

# Construction / Recognition

## To Construct a Minor Seventh Chord

1. Build a minor triad (root, up 3, up 4)
2. From the fifth, go up 3 more semitones (minor third) to the seventh
3. Verify: the seventh is 2 semitones below the root (completing the cycle)

# Context & Application

The minor seventh chord is prominent in jazz and popular music. It appears naturally on several scale degrees in the major mode. In the key of D major, F#m7 would be labeled IIIm7; in the key of G minor, it would be #VIIm7 (Wright, p. 48).

# Examples

**Example 1** (p. 48): Musical notation examples of minor seventh chords are shown.

**Example 2** (p. 50): F#m7 is used to illustrate chord labeling: in D major it is IIIm7, in G minor it is #VIIm7.

**Example 3**: Dm7: D-F-A-C; Am7: A-C-E-G.

# Relationships

## Builds Upon

- **Minor Triad** -- The minor seventh chord adds a seventh to the minor triad

## Enables

- **Chord Labeling** -- Demonstrates the m7 suffix convention

## Related

- **Half-Diminished Seventh Chord** -- Both contain a minor third from root; differ in fifth and seventh intervals

## Contrasts With

- **Seventh Chord** -- (4, 3, 3, 2): has a major triad base; the distinction is in the triad quality, not the seventh interval
- **Major Seventh Chord** -- (4, 3, 4, 1): has both a major triad base and a major seventh interval

# Common Errors

- **Error**: Confusing m7 (minor seventh chord) with M7 (major seventh chord) in notation
  **Correction**: Lowercase "m" indicates minor triad base; uppercase "M" indicates major seventh interval

# Common Confusions

- **Confusion**: Thinking the "seventh" in a minor seventh chord is different from the "seventh" in a dominant seventh
  **Clarification**: Both chords have a minor seventh interval (10 semitones) above the root; the difference is the triad base (minor vs. major)

# Source Reference

Chapter 3: "Harmony and Related Numerology," p. 48.

# Verification Notes

- Definition source: Direct from p. 48
- Confidence rationale: High -- explicitly defined with interval sequence and examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: labeling examples (F#m7 in different keys), observation that both m7 and dom7 share the minor seventh interval
