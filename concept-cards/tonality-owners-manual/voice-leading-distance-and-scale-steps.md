---
concept: Voice-Leading Distance and Scale Steps
slug: voice-leading-distance-and-scale-steps

category: fundamentals
subcategory: voice-leading-foundations
tier: foundational

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 1: Fundamentals"
chapter_number: null
pdf_page: 533
section: null

extraction_confidence: high

aliases:
  - scale-step distance
  - relative distance measurement

prerequisites: []
extends: []
related:
  - voice-leading
  - efficient-voice-leading
  - hierarchical-set-theory
  - chordal-step-distance
contrasts_with: []

answers_questions:
  - "What foundational knowledge is needed for geometric models of voice leading?"
  - "How is distance measured in Tymoczko's geometric framework?"
  - "Why is distance always relative to a scale?"
---

# Quick Definition

The foundational principle that musical distance is always measured in scale steps relative to some contextually relevant scale, making concepts like transposition, inversion, and chord type scale-dependent rather than absolute.

# Core Definition

"Distance is always measured in scale steps. This means that the size of an interval is measured along some contextually relevant scale: E-G has chromatic size 3, diatonic size 2, pentatonic size 1, and triadic size 1 (e.g., along the C major triad)" (p. 534). Notions such as "transposition, inversion, path in pitch-class space, and chord type are also scale-dependent: in the diatonic scale (C, E, G) is transpositionally related to (D, F, A), and hence the same type of chord, but this is not true in the chromatic scale" (p. 534). Pitches are labeled using generalized MIDI numbering where scale steps are size 1 and 60 is as close to middle C as possible.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Distance is always relative to a chosen scale (chromatic, diatonic, triadic, etc.)
2. The same interval has different sizes in different scales
3. Transposition, inversion, and chord type are all scale-dependent
4. Big-T, T, and little-t notation distinguishes chromatic, scalar, and chordal transposition
5. Generalized MIDI numbering: scale steps are size 1, number 60 is near middle C
6. This relativity underwrites a hierarchical set theory operating at multiple levels

# Construction / Recognition

## To Measure Voice-Leading Distance:
1. Choose the relevant scale (chromatic, diatonic, chordal, subchordal)
2. Count the number of scale steps between the two pitches
3. Note the direction (ascending positive, descending negative)
4. Use the appropriate transposition operator (T-bold for chromatic, T for scalar, t for chordal)

# Context & Application

This foundational principle is what makes "geometry" appropriate for music: "Geometry begins in the commitment to take distance seriously" (p. 533). The relativity of distance across hierarchical levels is the basis for the entire spiral diagram framework, and for understanding how transformations at one level can counteract their analogues at another (e.g., chromatic transposition nearly canceling chordal transposition to produce efficient voice leading).

# Examples

**Example 1** (p. 534): E-G has chromatic size 3, diatonic size 2, pentatonic size 1, and triadic size 1 along the C major triad.

**Example 2** (p. 534): Transposing (C4, E4, G4) by ascending triadic step: add 1 to triadic representation (60, 61, 62) to get (61, 62, 63) = (E4, G4, C5).

**Example 3** (p. 534): Inverting (C4, E4, G4) around C4 within the intrinsic scale C-E-G: subtract each from 120 to get (60, 59, 58) = (C4, G3, E2).

# Relationships

## Builds Upon
- None (foundational)

## Enables
- **Voice leading** -- Distance measurement is the basis for defining voice leadings
- **Efficient voice leading** -- Efficiency depends on the scale used for measurement
- **Hierarchical set theory** -- Scale-dependent transformations at multiple levels

## Related
- **Chordal step distance** -- Distance measured within a specific chord

## Contrasts With
- None listed

# Common Errors

- **Error**: Assuming distance is always chromatic (measured in semitones)
  **Correction**: Distance must be measured in the contextually relevant scale; diatonic and chordal distances are equally valid

- **Error**: Treating chord type as scale-independent
  **Correction**: (C, E, G) and (D, F, A) are the same type diatonically but not chromatically

# Common Confusions

- **Confusion**: Thinking paths in pitch-class space are the same as pitch-class intervals
  **Clarification**: Paths distinguish multiple routes: "start at G and move down by four semitones" differs from "start at G and move up by eight semitones" even though both connect G to E-flat (p. 533)

- **Confusion**: Assuming MIDI numbering is always chromatic
  **Clarification**: The generalized numbering system uses scale steps as size 1, so for the white-note scale, B3 = 59, C4 = 60, D4 = 61

# Source Reference

Appendix 1: "Fundamentals," pp. 533-534. See also the discussion of hierarchical transformations on pp. 534-535.

# Verification Notes

- Definition source: Direct from p. 534
- Confidence rationale: HIGH -- explicitly defined as the foundational principle
- Cross-reference status: Verified
- Re-extraction notes: New card; no previous version existed
