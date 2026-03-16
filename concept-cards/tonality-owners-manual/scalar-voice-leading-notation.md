---
concept: Scalar Voice Leading Notation
slug: scalar-voice-leading-notation

category: analysis
subcategory: notation-systems
tier: intermediate

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: Chromatic or Diatonic?"
chapter_number: null
pdf_page: 357
section: null

extraction_confidence: high

aliases:
  - "T and t notation"
  - "chromatic-scalar transposition notation"

prerequisites:
  - basic-voice-leading-of-scales
extends:
  - basic-voice-leading-of-scales
related:
  - chromatic-vs-diatonic-perspective
  - enharmonic-equivalence
  - key-distance-pitch-class-changing
contrasts_with: []

answers_questions:
  - "How do you notate voice leadings between scales?"
  - "What do the T and t symbols mean in scalar voice-leading analysis?"
---

# Quick Definition

A notation system for describing how scales transform into other scales, using bold **T** for chromatic transposition and plain t for scalar transposition, with subscripts indicating the size of each motion.

# Core Definition

Scalar voice leading notation describes the motion of a collection within a larger collection using two components: **T**_n (bold T with subscript) represents chromatic transposition by n semitones, while t_n represents scalar transposition by n scale steps. The combination **T**_n t_m describes the net effect. For diatonic scales in chromatic space, the descending basic voice leading is **T**_{-7} t_4 (or equivalently **T**_5 t_{-3}), which "lowers the leading tone semitonally while keeping every other note fixed" (p. 358). Each step on the scalar spiral diagram corresponds to one application of the basic voice leading.

# Prerequisites

- **Basic voice leading of scales** -- The operations that the notation represents

# Key Properties

1. Bold **T**: chromatic transposition (motion measured in semitones)
2. Plain t: scalar transposition (motion measured in scale steps)
3. The combination **T**_n t_m fully describes a voice leading between two instances of the same chord type
4. For 7-in-12 diatonic scales: basic voice leading = **T**_{-7} t_4
5. The notation applies identically to chords within scales and scales within the aggregate
6. Notation tracks abstract voices (melodic slots), not specific pitches

# Construction / Recognition

## To Use the Notation:
1. Identify the starting collection and the target collection
2. Determine the chromatic transposition **T**_n (how many semitones is the whole collection shifted?)
3. Determine the scalar transposition t_m (how many positions around the spiral?)
4. Combine as **T**_n t_m
5. Verify: applying **T**_n t_m to the starting collection should produce the target collection

# Context & Application

The notation is essential for analyzing modulatory passages at the scalar level and for describing the nested voice leadings in passages like Beethoven's Ninth Symphony scherzo. There, the chordal voice leading takes a clockwise step in the triadic spiral diagram while the scalar voice leading moves counterclockwise -- "hierarchically self-similar transformations occurring on two different structural levels" (p. 358).

# Examples

**Example 1** (p. 358, Figure P8.2): The spiral diagram for diatonic scales -- **T**_{-7} t_4 shifts B to B-flat while keeping all other melodic slots fixed.

**Example 2** (p. 358, Figure P8.3): Beethoven's Ninth takes eighteen counterclockwise steps on the triadic spiral and eight clockwise steps on the scalar spiral.

**Example 3** (p. 366, Figure 8.2.1): Modulatory sequences described using accumulated **T** and t, distinguishing trivial from nontrivial voice leadings.

# Relationships

## Builds Upon
- **Basic voice leading of scales** -- The operations notated by this system

## Enables
- **Chromatic vs. diatonic perspective** -- The notation makes hierarchical analysis precise
- **Enharmonic equivalence** -- The notation clarifies when modulatory paths form loops

## Related
- **Key distance (pitch-class-changing)** -- The notation quantifies the distance measure

# Common Errors

- **Error**: Confusing **T** (bold, chromatic) with t (plain, scalar)
  **Correction**: **T** moves through the chromatic aggregate; t rotates within the collection

# Common Confusions

- **Confusion**: Thinking the subscripts are root intervals
  **Clarification**: **T**_{-7} means "down 7 semitones in chromatic space," not "down a fifth"; the correspondence to a fifth is a consequence, not the definition

# Source Reference

Prelude: Chromatic or Diatonic?, pp. 357--359. Chapter 8, Section 2, pp. 366--368. Figures P8.1--P8.2, 8.2.1.

# Verification Notes

- Definition source: Direct from pp. 357--358
- Confidence rationale: High -- explicitly defined with clear notation and examples
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: T/t distinction, diatonic basic voice leading formula, Beethoven's Ninth example -- all confirmed in source
