---
concept: Chordal-Step Distance
slug: chordal-step-distance

category: voice-leading
subcategory: distances
tier: foundational

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: Transposition Along a Collection"
chapter_number: null
pdf_page: 37
section: null

extraction_confidence: high

aliases:
  - triadic step distance

prerequisites:
  - transposition-along-a-collection
  - intrinsic-vs-enclosing-scale
extends: []
related:
  - voice-leading
  - spiral-diagrams-for-chord-space
  - efficient-voice-leading
contrasts_with:
  - diatonic-distance

answers_questions:
  - "What is chordal-step distance?"
  - "Why is chordal-step distance relevant to efficient voice leading?"
---

# Quick Definition

Distance measured in steps along a chord's intrinsic scale rather than in semitones or diatonic steps, relevant whenever composers are concerned with efficient voice leading.

# Core Definition

Chordal-step distance measures intervals using the notes of a chord as the unit of measurement. Transposition along the chord preserves intervals as measured by the chord: "the middle note is always one chordal step above the lowest, while the top is four chordal steps above the middle" (p. 39). "The most efficient voice leading between two chords -- the mapping that collectively moves the voices by the smallest overall amount -- will always connect chords that are voiced in the same way" (p. 39). Transposition along the chord and along the scale are "the only operations that preserve both chord type and distance in chordal steps" (p. 42).

# Prerequisites

- **Transposition Along a Collection** -- Chordal-step distance is a specific instance of collectional distance
- **Intrinsic vs. Enclosing Scale** -- The chord's intrinsic scale provides the ruler

# Key Properties

1. Uses the chord itself as the ruler for measuring intervals
2. Voicing = pattern of spacing in chordal steps
3. Efficient voice leading connects chords with the same chordal-step voicing
4. For an n-note chord, there are n fundamentally distinct voicings
5. Composers may use chordal-step distance unknowingly, as a byproduct of minimizing motion
6. Applies to any chord size, from dyads to seven-note scales

# Construction / Recognition

## To Measure Chordal-Step Distance:
1. Compress the chord's notes into a single octave
2. Number the notes consecutively (0, 1, 2, ...)
3. Measure intervals between voices in these chordal steps
4. Compare the voicing pattern with another chord's voicing pattern
5. Same voicing pattern = same chordal-step distances = candidates for efficient voice leading

# Context & Application

Chordal-step distance bridges motivic and contrapuntal contexts. In motivic contexts, composers intentionally move objects along the chord; in contrapuntal contexts, "these same transformations can arise implicitly and without conscious knowledge" (p. 41). The concept is familiar in guitar pedagogy, where it constructs inversions keeping the left hand in roughly the same position (p. 39).

# Examples

**Example 1** (p. 39, Figure P2.6): Forming registral inversions by moving each voice along the intrinsic scale -- middle note always one chordal step above lowest.

**Example 2** (p. 39, Figure P2.7): Three spacing-preserving voice leadings from C3-D3-A4 to a minor triad, each voicing the target chord with the same chordal-step pattern.

**Example 3** (p. 37, Figure P2.2): Armstrong's "West End Blues" -- last two notes of motive move along the Eb major triad, measured in triadic steps.

# Relationships

## Builds Upon
- **Transposition Along a Collection** -- Chordal-step distance is transposition along the chord
- **Intrinsic vs. Enclosing Scale** -- The chord's intrinsic scale defines chordal steps

## Enables
- **Efficient Voice Leading** -- Efficient voice leading preserves chordal-step voicing

## Contrasts With
- **Diatonic Distance** -- Diatonic distance measures along the scale; chordal-step distance along the chord

# Common Errors

- **Error**: Confusing chordal-step distance with semitone or diatonic-step distance
  **Correction**: Chordal-step distance uses the chord itself as the ruler

# Common Confusions

- **Confusion**: Thinking chords with the same chordal-step voicing must be in the same register
  **Clarification**: They share an abstract voicing pattern but can appear at any pitch level

# Source Reference

Prelude to Chapter 2, pp. 39-42, Figures P2.6-P2.7.

# Verification Notes

- Definition source: Direct quotations from pp. 39, 42
- Confidence: HIGH -- explicitly discussed with examples
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: guitar pedagogy reference
