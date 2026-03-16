---
# === CORE IDENTIFICATION ===
concept: Transposition Along a Collection
slug: transposition-along-a-collection

# === CLASSIFICATION ===
category: voice-leading
subcategory: geometric-voice-leading
tier: foundational

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: Transposition Along a Collection"
chapter_number: null
pdf_page: 37
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - motion along a collection

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - intrinsic-vs-enclosing-scale
  - chordal-step-distance
  - spiral-diagrams-for-chord-space
  - quadruple-hierarchy
  - doubly-parallel-motion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is transposition along a collection?"
  - "How does transposition along a chord differ from transposition along a scale?"
  - "What foundational knowledge is needed for geometric models of voice leading?"
---

# Quick Definition

The operation that shifts musical material along a chord, a scale, or any other set of notes, using the collection as a "musical ruler" that provides a unit of distance, measuring in steps rather than semitones (p. 37).

# Core Definition

Tymoczko identifies this as "the single most important concept in music theory" (p. 37). The collection "acts like a musical ruler, providing a unit of musical distance that allows us to move objects by one or more steps. Alternatively, we can think of a collection as a slightly uneven ladder: the conceptual trick is to measure distance with ladder rungs (steps) rather than some fixed unit like inches (semitones)" (p. 37). Notation: lowercase t_x for x-step transposition along the chord, uppercase T_y for transposition along the scale, boldface **T** for chromatic transposition. The operations commute (t_x T_y = T_y t_x), subscripts add, and for an n-note chord in an o-note scale, t_n = T_o (pp. 41-42).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Any collection of notes can serve as a ruler for measuring distance
2. Lowercase t_x = transposition along chord; uppercase T_y = along scale; boldface **T** = chromatic
3. The operations commute: t_x T_y = T_y t_x
4. Subscripts add: t_x T_y + t_a T_b = t_{x+a} T_{y+b}
5. t_n = T_o (n chordal steps = o scale steps, completing one octave)
6. These are the *only* operations that preserve both chord type and chordal-step distance

# Construction / Recognition

## To Apply Transposition Along a Collection:
1. Identify the collection serving as the ruler (chord, scale, or other set)
2. Compress the collection's notes into a single octave to form a scale
3. Move each voice by the same number of steps along this scale
4. The result preserves intervals as measured by the collection
5. For melodic contexts, identify nonharmonic tones as neighbors or passing tones relative to collection tones

# Context & Application

The concept spans the border between implicit and explicit knowledge. In motivic contexts, composers intentionally move objects along the chord; in contrapuntal contexts, "these same transformations can arise implicitly and without conscious knowledge" as a byproduct of minimizing contrapuntal motion (p. 41). The concept unifies melodic development (transposition along a scale), motivic development (along a chord), and efficient voice leading (combining both).

# Examples

**Example 1** (p. 37, Figure P2.1): "Doe a deer" shifted up one rung along the diatonic ladder (T_1).

**Example 2** (p. 37, Figure P2.2): Louis Armstrong's "West End Blues," m. 7 -- motive transposed down by chordal step along the Eb major triad (t_{-1}).

**Example 3** (p. 38, Figure P2.3): Scarlatti K.3, mm. 3-6 -- right hand moves up by triadic step (t_1) while left hand ascends by two scalar steps (T_2), producing very similar results.

**Example 4** (p. 39, Figure P2.4): Transposing motives along their own "intrinsic scale."

# Relationships

## Enables
- **Quadruple Hierarchy** -- Transposition along collections operates at each hierarchical level
- **Spiral Diagrams for Chord Space** -- Slides and loops on spirals correspond to transpositions along scale and chord
- **Doubly Parallel Motion** -- Combining t_x and T_y produces doubly parallel motion

## Related
- **Intrinsic vs. Enclosing Scale** -- Any collection has both an intrinsic and enclosing scale
- **Chordal-Step Distance** -- Distance measured along the chord's intrinsic scale

# Common Errors

- **Error**: Confusing transposition along a collection with standard chromatic transposition
  **Correction**: Transposition along a collection preserves distances as measured by that collection, not in semitones

# Common Confusions

- **Confusion**: Thinking t_1 and T_2 are the same thing
  **Clarification**: They produce very similar but not identical results; t_{-1} T_2 "does very little" because they nearly cancel (p. 38)

- **Confusion**: Believing musical identity is fixed by intrinsic constitution
  **Clarification**: Musical identity is "up to us, determined by an object's transformational properties rather than fixed for all time by its intrinsic constitution" (p. 39)

# Source Reference

Prelude to Chapter 2: "Transposition Along a Collection," pp. 37-46, Figures P2.1-P2.11. Notation system first used by Julian Hook (2003, 2008).

# Verification Notes

- Definition source: Direct quotation from p. 37
- Confidence: HIGH -- named as "the single most important concept in music theory"
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: notation system details, commutation/addition properties
