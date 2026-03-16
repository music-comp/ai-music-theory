---
concept: Interscalar Transposition
slug: interscalar-transposition

category: fundamentals
subcategory: voice-leading-geometry
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 1: Fundamentals"
chapter_number: null
pdf_page: 536
section: null

extraction_confidence: high

aliases:
  - spacing-preserving voice leading between unrelated chords
  - crossing-free voice leading

prerequisites:
  - voice-leading-geometry
  - basic-voice-leading
extends: []
related:
  - spiral-diagram-derivation
  - cardinality-equivalence
  - quadruple-hierarchy
contrasts_with: []

answers_questions:
  - "What is a spacing-preserving voice leading?"
  - "How do interscalar transpositions relate to the spiral diagrams?"
  - "Why do voice crossings never make a voice leading smaller?"
---

# Quick Definition

A spacing-preserving voice leading between unrelated chords (neither transpositionally nor inversionally related), which preserves chordal-step distance and cannot produce voice crossings -- the most general form of the crossing-free voice leadings that underlie the spiral diagrams.

# Core Definition

A voice leading is "spacing-preserving (strongly crossing free) if it preserves chordal-step distance, meaning its voices can never be arranged in register so that they cross" (p. 536). For transpositionally related chords, spacing-preserving voice leadings combine T (scale transposition) and t (chord transposition). For inversionally related chords, they combine I and i. "For unrelated chords, a spacing-preserving voice leading is called an interscalar transposition; all the interscalar transpositions between any two chords can be derived by combining any one interscalar transposition with transpositions along the chord" (p. 536). Crucially, "voice crossings never make a voice leading smaller: hence between any two chords, there is always a minimal voice leading that is crossing-free" (p. 537). This means "hierarchical self-similarity can arise as the byproduct of the search for efficient voice leading" (p. 537).

# Prerequisites

- **Voice-leading geometry** -- The mathematical framework within which interscalar transposition is defined
- **Basic voice leading** -- Understanding voice crossings, transposition, and inversion

# Key Properties

1. Preserves chordal-step distance between voices
2. Cannot be arranged in register to produce voice crossings
3. Between transpositionally related chords: combines T and t
4. Between inversionally related chords: combines I and i
5. Between unrelated chords: called interscalar transposition
6. All interscalar transpositions derivable from any one plus chord transpositions
7. Minimal voice leadings are always crossing-free

# Construction / Recognition

## To Identify an Interscalar Transposition:
1. Verify the two chords are neither transpositionally nor inversionally related
2. Check that the voice leading preserves chordal-step distance
3. Plot the paths on the pitch-class circle: each voice should glide smoothly without crossing
4. Verify that no rearrangement in register produces voice crossings

# Context & Application

The spiral diagrams represent all spacing-preserving voice leadings between the transpositions of one or more chords -- "in other words, the possibilities that remain when we ignore voice exchanges" (p. 537). The concept connects to Schenkerian analysis, which "deemphasizes voice exchanges by assigning them to the surface, preserving crossing-free backgrounds that are recognizably scalar" (p. 530).

# Examples

**Example 1** (p. 537): The basic voice leading on the spiral diagrams -- the minimal spacing-preserving voice leading connecting angular neighbors.

**Example 2** (pp. 537-538): Figures A1.3-A1.4 -- symbolic diagrams showing how near-symmetries (T-symmetry, I-symmetry, P-symmetry) affect the shape of the spiral graph.

**Example 3** (p. 534): Hierarchical cancellation -- chromatic transposition by four ascending semitones nearly counteracts transposition downward by one major-triad step, producing efficient voice leading as residue (Figure 2.1.4).

# Relationships

## Builds Upon
- **Voice-leading geometry** -- Interscalar transposition is defined within this framework
- **Basic voice leading** -- Voice crossings and transposition are prerequisite concepts

## Enables
- **Spiral diagram derivation** -- The diagrams represent spacing-preserving voice leadings
- **Cardinality equivalence** -- Spacing-preserving voice leadings are preserved at the background level

## Related
- **Quadruple hierarchy** -- Hierarchical self-similarity arises from searching for efficient (crossing-free) voice leading

## Contrasts With
- None listed

# Common Errors

- **Error**: Confusing interscalar transposition with ordinary transposition
  **Correction**: Interscalar transposition applies specifically between chords not related by transposition or inversion

- **Error**: Assuming "spacing-preserving" means "small"
  **Correction**: It means crossing-free, which is a structural property -- the voice leading may still involve large motions

# Common Confusions

- **Confusion**: Thinking the spiral diagrams represent all possible voice leadings
  **Clarification**: They represent only spacing-preserving voice leadings; voice exchanges are excluded

- **Confusion**: Conflating voice-leading distance with harmonic similarity
  **Clarification**: Contrapuntal proximity (efficient voice leading) does not necessarily imply harmonic similarity (shared notes), except for maximally even chords with specific properties

# Source Reference

Appendix 1: "Fundamentals," pp. 536-537; Appendix 2, pp. 553-554.

# Verification Notes

- Definition source: Direct from source, pp. 536-537
- Confidence rationale: HIGH -- formally defined with clear mathematical properties
- Cross-reference status: Verified against spiral diagram derivation in Appendix 2
- Re-extraction notes: Re-extracted from v2 card; preserved: formal definitions of spacing-preserving/crossing-free, T+t/I+i decompositions, "voice crossings never make smaller" principle, hierarchical self-similarity connection
