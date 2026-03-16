---
concept: Hierarchical Set Theory
slug: hierarchical-set-theory

category: fundamentals
subcategory: set-theory
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 1: Fundamentals"
chapter_number: null
pdf_page: 533
section: null

extraction_confidence: high

aliases:
  - multilevel set theory
  - scale-relative set theory

prerequisites:
  - voice-leading-distance-and-scale-steps
  - double-transposition
extends:
  - generalized-set-theory
related:
  - cardinality-equivalence
  - interscalar-transposition
  - tinctoris-transform
contrasts_with: []

answers_questions:
  - "What foundational knowledge is needed for geometric models of voice leading?"
  - "How does Tymoczko's set theory differ from traditional pitch-class set theory?"
  - "How do transformations at one level counteract those at another?"
---

# Quick Definition

A set theory in which transposition and inversion apply not just chromatically or diatonically but at the chordal level as well, with transformations at one hierarchical level often counteracting those at another to produce efficient voice leading as a residue.

# Core Definition

The "relativity of musical distances underwrites a hierarchical set theory in which transposition and inversion can apply not just chromatically or diatonically, but at the chordal level as well" (p. 534). Transformations at one level "often counteract their analogues at another": chromatic transposition by four ascending semitones can "nearly counteract transposition downward by one major-triad step, producing efficient voice leading as its residue" (p. 534). Similarly, inversional cancellation (combining big-I and little-i) "keeps two notes fixed and moves the third parsimoniously, by one or two semitones" (p. 535), producing neo-Riemannian transformations. Gesualdo's trick generalizes this, with "two voices moving in parallel, combining a generalized neo-Riemannian transformation with a transposition" (p. 535).

# Prerequisites

- **Voice-leading distance and scale steps** -- The foundation of scale-relative measurement
- **Double transposition** -- The combination of transpositions at different levels

# Key Properties

1. Transposition and inversion operate at chromatic, diatonic, and chordal levels
2. Transformations at different levels can nearly cancel, producing efficient voice leading
3. Neo-Riemannian transformations arise from combining big-I and little-i
4. Gesualdo's trick combines neo-Riemannian transformation with transposition
5. Voice leadings that preserve distance between at least two voices result
6. Chords can function as abstract scale-like objects whose notes appear in multiple octaves

# Construction / Recognition

## To Apply Hierarchical Cancellation:
1. Label notes in two different ways (e.g., triadic and chromatic scale degrees)
2. Choose two notes to remain fixed
3. Apply inversion at both levels: send each note n to (x + y) - n
4. The two inversions nearly cancel, producing an efficient voice leading
5. The result is a neo-Riemannian transformation (L, P, or R for triads)

# Context & Application

This perspective significantly increases the power of geometrical models by allowing chords to function as abstract scales. It provides "cardinality equivalence" -- treating CGG and CCG as different configurations of surface voices within a single underlying "scale" CG (p. 540). It also enables background voice leadings containing notes not found on the surface, bridging "the gap between mathematics and musical intuition" (p. 540). The cross-section of three-note voice-leading space (Figure A1.7) shows how set-class voice leadings generalize from specific chords to chord types.

# Examples

**Example 1** (pp. 534-535): Inverting (C4, E4, G4) around C4 within the triadic scale produces (C4, G3, E2); within the diatonic scale, (C4, A3, F3); within the chromatic scale, (C4, Ab3, F3).

**Example 2** (p. 535): Combining triadic and chromatic inversions on the C major triad: fixing C and E, the inversions nearly cancel, moving G to Ab (the L transformation).

**Example 3** (pp. 542-543): Beethoven's Op. 109 variations theme analyzed in set-class space, with voice leadings decomposed into voice crossings, scale transposition, and chordal transposition.

# Relationships

## Builds Upon
- **Voice-leading distance and scale steps** -- Scale-relative measurement
- **Double transposition** -- Hierarchical combination of transformations

## Enables
- **Cardinality equivalence** -- Treating different-cardinality chords as configurations within scales
- **Generalized set theory** -- The broader algebraic framework

## Related
- **Interscalar transposition** -- Spacing-preserving voice leadings between unrelated chords
- **Tinctoris transform** -- Equivalence classes of voice leadings under transposition

## Contrasts With
- None listed

# Common Errors

- **Error**: Assuming set theory operates only at the chromatic level
  **Correction**: The hierarchical framework applies transposition and inversion at chromatic, diatonic, and chordal levels simultaneously

- **Error**: Treating neo-Riemannian transformations as unrelated to traditional set theory
  **Correction**: They arise naturally from combining inversions at two hierarchical levels

# Common Confusions

- **Confusion**: Thinking contrapuntal proximity always implies harmonic similarity
  **Clarification**: Chords sharing the same angular position on a spiral diagram need not be close in an absolute sense (p. 538)

- **Confusion**: Assuming cardinality equivalence requires a specific geometric space
  **Clarification**: It can be modeled hierarchically by treating chords as abstract scales with surface voices moving within them (p. 540)

# Source Reference

Appendix 1: "Fundamentals," pp. 534-543. Key figures: A1.1, A1.7-A1.8.

# Verification Notes

- Definition source: Direct from pp. 534-535, 540
- Confidence rationale: HIGH -- explicitly presented as a foundational framework
- Cross-reference status: Verified
- Re-extraction notes: New card; no previous version existed
