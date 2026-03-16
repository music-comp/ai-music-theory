---
# === CORE IDENTIFICATION ===
concept: Voice-Leading Geometry
slug: voice-leading-geometry

# === CLASSIFICATION ===
category: fundamentals
subcategory: voice-leading-geometry
tier: advanced

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 1: Fundamentals"
chapter_number: null
pdf_page: 533
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - geometrical voice-leading theory
  - pitch-class voice-leading formalism

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-voice-leading
extends: []
related:
  - spiral-diagram-derivation
  - quadruple-hierarchy
  - collectional-hierarchy
  - interscalar-transposition
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are voice leadings formally defined in Tymoczko's framework?"
  - "What is the relationship between distance, scale steps, and transposition?"
  - "How do neo-Riemannian transformations arise from hierarchical cancellation?"
---

# Quick Definition

The mathematical framework in which voice leadings are defined as collections of paths in pitch-class space, measured along contextually relevant scales, with distance always measured in scale steps -- enabling a unified treatment of chromatic, diatonic, and chordal transposition and inversion.

# Core Definition

"Geometry begins in the commitment to take distance seriously" (p. 533). The analytical atoms are "paths in pitch class space," combining an initial pitch class with a directed magnitude. "Distance is always measured in scale steps" (p. 534), making interval size relative to context: "E-G has chromatic size 3, diatonic size 2, pentatonic size 1, and triadic size 1" (p. 534). Transposition is "represented by addition on input scale degrees" -- T for scalar, t for chordal, bold-T for chromatic (p. 534). Inversion is "represented by subtraction from a constant value, sending x to c - x" (p. 534). A voice leading is formally "a collection of paths in pitch-class space, measured along some scale" (p. 535). Any voice leading can be "factored into a voice exchange and a remainder that is spacing-preserving or strongly crossing free" (p. 536).

# Prerequisites

- **Basic voice leading** -- Understanding intervals, transposition, and inversion as musical operations

# Key Properties

1. Distance is always relative to a contextually relevant scale
2. Transposition at one level often counteracts its analogue at another, producing efficient voice leading as residue
3. Neo-Riemannian transformations arise from combining little-i inversion within the chord and big-I inversion within the chromatic scale
4. Voice leadings are abstract and register-free -- the written ordering of paths is immaterial
5. Any voice leading factors into voice exchange plus spacing-preserving remainder
6. The spiral diagrams represent all spacing-preserving voice leadings

# Construction / Recognition

## To Work Within Voice-Leading Geometry:
1. Choose the relevant scale (chromatic, diatonic, chordal, etc.)
2. Measure distances in that scale's steps
3. Represent transposition as addition, inversion as subtraction from a constant
4. Factor voice leadings into voice-exchange and spacing-preserving components
5. Use the spiral diagrams for the spacing-preserving component

# Context & Application

This formalism underlies the entire book's analytical apparatus. It supports "hierarchical set theory in which transposition and inversion can apply not just chromatically or diatonically, but at the chordal level as well" (p. 533). Transformations at one level often counteract analogues at another: Figure A1.1 shows how combining triadic and chromatic inversions "nearly cancel out, here keeping two notes fixed and moving the third 'parsimoniously,' by one or two semitones" (p. 535) -- producing neo-Riemannian transformations.

# Examples

**Example 1** (p. 535): (C, E, G) to (C, F, A) -- bijective voice leading from C major to F major, with chromatic distances (0, 1, 2) or diatonic distances (0, 1, 1).

**Example 2** (p. 535): Neo-Riemannian transformations -- combining little-i inversion within the chord and big-I inversion within the chromatic scale, which nearly cancel to produce efficient voice leading with two notes fixed (Figure A1.1).

**Example 3** (pp. 542-543): Beethoven's Op. 109 variations theme -- voice leadings analyzed into crossing (c_xy), scalar transposition (T_x), and chordal transposition (t_x) components (Figure A1.8).

# Relationships

## Builds Upon
- **Basic voice leading** -- The musical intuitions formalized by voice-leading geometry

## Enables
- **Spiral diagram derivation** -- The diagrams are a geometrical representation of spacing-preserving voice leadings
- **Interscalar transposition** -- Defined within the voice-leading geometry framework
- **Cardinality equivalence** -- The hierarchical solution to doublings in geometrical space

## Related
- **Quadruple hierarchy** -- The hierarchical structure that voice-leading geometry illuminates
- **Collectional hierarchy** -- The synthesis of multiple levels of voice-leading geometry

## Contrasts With
- None listed

# Common Errors

- **Error**: Treating voice leadings as concrete pitch-space transformations
  **Correction**: Voice leadings are abstract -- (C4, E4, G4) to (C4, F4, A4) and (E3, C4, G4) to (F3, C4, A4) instantiate the same voice leading

- **Error**: Equating contrapuntal proximity with harmonic similarity
  **Correction**: Efficient voice leading between chords does not imply they share notes or sound similar (except for maximally even chords)

# Common Confusions

- **Confusion**: Thinking voice exchange is the same as registral inversion
  **Clarification**: Voice exchange is a bijective voice leading whose paths sum to zero -- (C4, E4) to (E4, C5) is NOT a voice exchange; it is transposition along the chord

- **Confusion**: Assuming voice crossings are always undesirable
  **Clarification**: Voice crossings never make a voice leading smaller, so minimal voice leadings are crossing-free -- but crossings may appear on the musical surface

# Source Reference

Appendix 1: "Fundamentals," pp. 533-543.

# Verification Notes

- Definition source: Direct from source, pp. 533-543
- Confidence rationale: HIGH -- formal mathematical definitions with musical applications
- Cross-reference status: Verified against spiral diagram derivation and Op. 109 analysis
- Re-extraction notes: Re-extracted from v2 card; preserved: "geometry begins" opening, E-G distance relativity example, T/t/bold-T notation, neo-Riemannian derivation, factoring theorem, Op. 109 analysis, voice-exchange definition
