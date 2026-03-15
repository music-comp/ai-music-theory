---
# === CORE IDENTIFICATION ===
concept: Scale as Ruler
slug: scale-as-ruler

# === CLASSIFICATION ===
category: scales-modes
subcategory: scale-theory
tier: intermediate-advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 134
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "scale as metric"
  - "scale as distance measure"
  - "musical ruler"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
extends: []
related:
  - scalar-transposition
  - scale-degree-arithmetic
  - scalar-distance-vs-chromatic-distance
  - goldilocks-principle
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a scale, from Tymoczko's geometric perspective?"
  - "How does a scale function as a measure of musical distance?"
  - "Why is it useful to think of scales as rulers?"
---

# Quick Definition
A scale provides an alternative measure of musical distance, functioning like a ruler that defines what counts as "one step." Moving by one scale step is always a "second," regardless of the chromatic distance involved. This is the foundational metaphor of Chapter 4.

# Core Definition
Tymoczko proposes that a scale is fundamentally a "ruler" or metric — a way of measuring distances in pitch and pitch-class space. Any collection of pitches can function as a scale: it need not repeat at the octave, need not have evenly spaced notes, and need not have a designated tonic. All the scale needs to do is define what counts as "one step" — the scale's notes tell us how to move up or down by one unit. From this perspective, the "Do Re Mi" pattern in The Sound of Music repeats the "same" gesture (two ascending scale steps) at different pitch levels, even though the chromatic intervals differ (C-D-E has intervals of 2-2 semitones, while D-E-F has intervals of 2-1). The scale defines the distance metric that makes these gestures equivalent.

# Prerequisites
- Basic chord space geometry
- Distinction between different measures of musical distance

# Key Properties
1. A scale defines an alternative distance metric in pitch space
2. Scale steps are the fundamental unit; chromatic semitones are a different unit
3. Any collection of pitches can function as a scale
4. A scale need not be octave-repeating, need not have equal-sized steps, need not have a tonic
5. Chord space can be drawn using either chromatic distance or scalar distance (Figure 4.1.4)
6. The "crumpled" appearance of chord space with chromatic distance reflects the discrepancy between the two metrics

# Construction / Recognition
## To Use a Scale as Ruler:
1. Identify the scale's notes as the points on your ruler
2. "One step" = the distance between adjacent scale notes
3. Measure all musical distances in these units
4. Scalar transposition, inversion, and chord types are defined using this metric

# Context & Application
The "scale as ruler" metaphor is foundational for all of Chapter 4 and much of the book's second half. It allows Tymoczko to treat scales and chords as instances of the same mathematical structures, define scalar transposition and inversion, and analyze modulation as voice leading between scales. The metaphor also connects to the Goldilocks Principle: a scale whose steps are too uneven makes a poor ruler (scalar transposition distorts patterns too much), while a perfectly even scale makes scalar transposition identical to chromatic transposition (eliminating the interesting variation).

# Examples
**Example 1** (p. 134-135): The opening phrases of "Do, Re, Mi" are "the same" pattern (two ascending steps) repeated at different scalar levels, even though the chromatic intervals differ (Figure 4.1.1).
**Example 2** (p. 136): Figure 4.1.4 shows two-note chord space drawn with chromatic distance (crumpled appearance) and scalar distance (regular grid), illustrating the two available metrics.
**Example 3** (p. 137): The analogy to San Francisco: looking down from a balloon, blocks appear different sizes due to hills, but intrinsically they are all the same size. Scalar distance = intrinsic block size; chromatic distance = apparent size from above.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space that can be drawn with either metric
## Enables
- **scalar-transposition** — Transposition using scale steps
- **scalar-distance-vs-chromatic-distance** — The two metrics defined by the ruler
- **scale-degree-arithmetic** — Arithmetic using the ruler's units
- **goldilocks-principle** — The ruler must be "just right" in evenness
## Related
- All eight important scales are evaluated as rulers

# Common Errors
- **Error**: Thinking a scale must have a tonic note
  **Correction**: A scale-as-ruler has no designated tonic; tonic is an additional concept related to centricity

# Common Confusions
- **Confusion**: Confusing "scale" with "macroharmony"
  **Clarification**: A scale (ruler) and a macroharmony (collection of notes actually sounding) are conceptually distinct, though they often coincide in practice. A scale can measure distances even in highly chromatic contexts (Figure 4.2.4).

# Source Reference
Chapter 4: Scales, Section 4.1, pages 134-137.

# Verification Notes
- Definition source: Directly from Section 4.1, the opening argument
- Confidence rationale: High — the chapter's foundational metaphor, explicitly stated
- Cross-reference status: Verified against the San Francisco analogy and Figure 4.1.4
