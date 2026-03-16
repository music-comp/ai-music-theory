---
concept: Transposition
slug: transposition

category: rhythm-and-form
subcategory: melody
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Transposition"

extraction_confidence: high

aliases:
  - "chromatic transposition"
  - "diatonic transposition"
  - "vertical shift"

prerequisites:
  - geometric-transformations-of-graphs
  - melody
  - diatonic-and-chromatic-scales
extends:
  - geometric-transformations-of-graphs
related:
  - translation
  - retrogression
  - musical-intervals
  - symmetry-in-music
contrasts_with:
  - translation

answers_questions:
  - "What is transposition in music?"
  - "What is the difference between chromatic and diatonic transposition?"
  - "How does transposition relate to graph transformations?"
---

# Quick Definition

A vertical shift of a melodic pattern, moving each note up or down by a fixed interval, analogous to the graph transformation $y = f(x) + c$, with two types: chromatic (preserving exact semitone intervals) and diatonic (preserving scale-degree intervals).

# Core Definition

*Transposition* is a vertical shift applied to a repeating melodic pattern, analogous to replacing $y = f(x)$ by $y = f(x) + c$. Two types are distinguished (Wright, pp. 39-40):

**Chromatic transposition**: each note is shifted by a fixed chromatic interval (fixed number of semitones), exactly preserving all interval sizes.

**Diatonic transposition**: each note is shifted by the same number of diatonic scale tones, producing a melody with the same general shape but with chromatic intervals not perfectly preserved due to the non-uniform spacing of diatonic scale steps.

# Prerequisites

- **Geometric Transformations of Graphs** — Transposition is the musical application of vertical shift
- **Melody** — Transposition transforms melodic patterns
- **Diatonic and Chromatic Scales** — The distinction between chromatic and diatonic transposition depends on scale structure

# Key Properties

1. Chromatic transposition is an isometry of the pitch axis (adds constant semitones)
2. Diatonic transposition preserves scale-degree distances but NOT semitone distances
3. Diatonic transposition keeps the melody within the key without new accidentals
4. The difference arises from the non-uniform intervals ($1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$) of the diatonic scale
5. A diatonic transposition may simultaneously be chromatic only in special cases

# Construction / Recognition

## To apply chromatic transposition:

1. Choose a fixed interval (e.g., up a perfect fourth = 5 semitones)
2. Shift every note by exactly that many semitones
3. All intervals are preserved exactly

## To apply diatonic transposition:

1. Choose a fixed scale-degree shift (e.g., up one diatonic step)
2. Shift every note by that many diatonic scale positions
3. Stay within the key — no new accidentals needed
4. Some interval qualities may change (e.g., major third becomes minor third)

# Context & Application

Chromatic transposition exactly preserves the "sound" of a passage. Diatonic transposition preserves the general contour but alters some interval qualities. Both are common compositional techniques. Diatonic transposition is particularly natural for staying within a key.

# Examples

- **Chromatic**: In Gershwin's "Strike Up The Band," the second 8 measures repeat the melody of the first, transposed up by a perfect fourth (p. 39)
- **Diatonic**: In "O Tannenbaum," the first bracketed sequence is shifted down by one diatonic scale tone (p. 39)
- Exercise 7: completing a measure with diatonic transposition vs. chromatic transposition

# Relationships

## Builds Upon
- **Geometric Transformations of Graphs** — Transposition = vertical shift of pitch-time graph
- **Melody** — Transposition transforms melodic patterns

## Enables
- **Symmetry in Music** — Transposition creates symmetrical relationships between sections

## Contrasts With
- **Translation** — Translation is horizontal (time shift); transposition is vertical (pitch shift)

## Related
- **Retrogression** — Another melodic transformation (reflection)
- **Musical Intervals** — Transposition is defined by an interval

# Common Errors

- **Error**: Assuming diatonic transposition preserves exact interval sizes
  **Correction**: Diatonic transposition preserves scale-degree distances but not semitone distances, due to the non-uniform diatonic scale

# Common Confusions

- **Confusion**: Thinking chromatic and diatonic transposition always differ
  **Clarification**: They coincide when all intervals involved happen to be uniform, but this is rare
- **Confusion**: Conflating transposition (pitch shift) with translation (time shift)
  **Clarification**: These are analogous operations in different dimensions — vertical vs. horizontal

# Source Reference

Chapter 2: "Horizontal Structure", "Transposition" section, pp. 39-40 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 39-40
- Confidence rationale: High — explicit definitions of both types with musical examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Gershwin example, O Tannenbaum example, isometry characterization, non-uniform scale explanation
