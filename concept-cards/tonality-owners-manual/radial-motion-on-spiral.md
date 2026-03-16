---
concept: Radial Motion on Spiral
slug: radial-motion-on-spiral

category: voice-leading
subcategory: geometric-voice-leading
tier: intermediate

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Rock Logic"
chapter_number: 2
pdf_page: 47
section: "A melodic principle"

extraction_confidence: high

aliases: []

prerequisites:
  - spiral-diagrams-for-chord-space
  - slide-along-spiral
  - loop-on-spiral
extends: []
related:
  - lp-and-pl-voice-leading
  - voice-leading
  - melodic-principle-of-harmony
contrasts_with: []

answers_questions:
  - "How does radial motion on the spiral diagram produce efficient voice leading?"
---

# Quick Definition

Movement between chords at different radial positions on the spiral, corresponding to efficient voice leadings that combine transposition along both chord and scale, with the two operations largely counteracting each other.

# Core Definition

On the spiral diagram, "transposition-along-the-chord largely counteracts transposition-along-the-scale, leaving efficient voice leading as the result" (p. 49). The algorithm: "slide your finger in the desired direction from one to the other along the spiral; the number of chords you touch, not counting the first, is the transposition along the scale (T_x); the number of times you revisit your initial angular position, y, becomes the transposition along the chord t_{+-y}, with its sign opposite that of the scalar transposition" (p. 50). The combination T_x t_{-y} or T_{-x} t_y represents the most direct voice leading.

# Prerequisites

- **Spiral Diagrams for Chord Space** -- Understanding the spatial framework
- **Slide Along Spiral** -- Sliding component of radial motion
- **Loop on Spiral** -- Looping component of radial motion

# Key Properties

1. Results from combining a slide and loop that partially cancel
2. For purely radial paths, both clockwise and counterclockwise routes yield the same voice leading
3. On the 3-in-12 diagram, outward radial motion = LP; inward = PL
4. A musical preference for descending melody maps to radial or clockwise motion
5. Initial direction matters for non-radial paths

# Construction / Recognition

## To Calculate Radial Voice Leading:
1. Choose a direction (clockwise or counterclockwise)
2. Slide along the spiral from chord A to chord B
3. Count touched chords (not counting A) = scalar transposition T_x
4. Count returns to initial angular position = chordal transposition y
5. Voice leading = T_x t_{-y} (sign of t opposite to T)

# Context & Application

Radial motion on the 3-in-12 diagram produces the LP and PL voice leadings of neo-Riemannian theory. All motions on the diagram can be expressed as combinations of radial motions (efficient voice leadings) and slides (chromatic transpositions).

# Examples

**Example 1** (p. 50, Figure 2.1.4): C to E outward: **T**_4 (4 counterclockwise steps) + t_{-1} (pass 12 o'clock once). Voice leading: (C,E,G) -> (B,E,G#) = LP.

**Example 2** (p. 51, Figure 2.1.5): E to C inward: **T**_{-4} t_1 = PL. G# down to G, E fixed, B up to C.

**Example 3** (p. 51): C to F (90-degree counterclockwise): **T**_5 t_{-1}. Root fixed, third up by semitone, fifth up by two semitones.

# Relationships

## Builds Upon
- **Slide Along Spiral** -- Radial motion's scalar component
- **Loop on Spiral** -- Radial motion's chordal component

## Enables
- **LP and PL Voice Leading** -- These are the specific radial voice leadings on the 3-in-12 diagram

## Related
- **Melodic Principle of Harmony** -- Descending melodies correspond to radial/clockwise motion

# Common Errors

- **Error**: Assuming radial motion is a separate operation from slides and loops
  **Correction**: It is their combination when they nearly cancel each other

# Common Confusions

- **Confusion**: Thinking both directions always give the same voice leading
  **Clarification**: Only for purely radial paths; for non-radial paths, initial direction matters

# Source Reference

Chapter 2, Section 1, pp. 49-52, Figures 2.1.4-2.1.6. Algorithm on p. 50.

# Verification Notes

- Definition source: Direct quotation from pp. 49-50
- Confidence: HIGH -- explicitly formulated with worked examples
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: worked examples, direction clarification
