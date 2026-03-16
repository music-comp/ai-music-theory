---
concept: Nonconforming Tonnetz
slug: nonconforming-tonnetz

category: pitch-space
subcategory: spatial representations of pitch
tier: intermediate

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Edward Gollin"
chapter: "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann"
chapter_number: 9
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "infinite Tonnetz"
  - "just-intoned Tonnetz"
  - "planar Tonnetz"

prerequisites:
  - tonnetz
extends: []
related:
  - syntonic-comma-striche
  - oettingens-acoustical-matrix
  - enharmonic-equivalence
contrasts_with:
  - toroidal-tonnetz

answers_questions:
  - "What distinguishes the conforming from nonconforming Tonnetz?"
  - "What is the geometry of the Tonnetz under just intonation?"
---

# Quick Definition

The version of the Tonnetz that assumes just intonation, producing an infinite plane where enharmonically distinct pitches (e.g., C# and Db) and syntonically distinct pitches (e.g., D and D with Strich) occupy different locations.

# Core Definition

The **nonconforming Tonnetz** (Daniel Harrison's term) is the Tonnetz as it exists under just intonation, where generating intervals never produce exact cycles. Because four perfect fifths do not equal a major third (they differ by a syntonic comma, 81:80) and twelve fifths do not equal seven octaves (they differ by a Pythagorean comma), the lattice extends infinitely in all directions. Each enharmonically or syntonically distinct pitch occupies a unique position: C# and Db are different nodes, as are D-natural reached by two fifths from C versus D-natural reached by a fifth up then a third down. Gollin explains this was the default understanding of Oettingen and early Riemann, where the table literally recorded acoustically distinct tones in just intonation (Ch. 9).

# Prerequisites

- **Tonnetz**: The general concept of the tone network that the nonconforming version specifies
- **Just intonation**: The tuning system that prevents cyclic closure and produces the infinite geometry

# Key Properties

1. **Infinite plane geometry**: The lattice extends infinitely in all directions with no wrapping
2. **No enharmonic equivalence**: C# and Db occupy different locations (separated by an enharmonic diesis, 128:125)
3. **No syntonic equivalence**: Like-named pitches at different positions differ by syntonic commas (81:80), marked by Striche
4. **Multiple instances of each letter name**: The same pitch name appears at many different positions
5. **Bounded diatonic regions**: Diatonic collections appear as parallelograms bounded by syntonic images of the same pitch

# Construction / Recognition

The nonconforming Tonnetz is constructed identically to the standard Tonnetz but without identifying enharmonic or syntonic equivalents:
1. Extend the fifth axis infinitely (no closure after 12 steps)
2. Extend the third axis infinitely (no closure after 3 or 4 steps)
3. Mark syntonic differences with Striche (underlines/overlines)
4. Treat each unique position as a distinct pitch, even if it bears the same letter name as another position

# Context & Application

The nonconforming Tonnetz is the historically original form, used by Oettingen (1866) and early Riemann (1870s-1880s). It is appropriate for analyzing music under just intonation assumptions or for representing the full richness of tonal derivation paths. Modern neo-Riemannian theory generally assumes the conforming (toroidal) Tonnetz instead, but the nonconforming version remains relevant for historical study, microtonal theory, and any analysis where enharmonic or syntonic distinctions matter. Cohn notes that diatonic regions appear as parallelograms on this infinite plane, bounded by syntonic images of the same pitch (e.g., two versions of D bounding the C major region) (Ch. 11).

# Examples

Gollin describes Oettingen's use of the nonconforming Tonnetz to critique Beethoven's enharmonic modulation in Op. 26: on the infinite plane, a descent by three major thirds leads to Bbb, not A (they differ by an enharmonic diesis of about 41 cents). Oettingen regarded this as a compositional error precisely because the nonconforming Tonnetz does not permit such identification (Ch. 9).

# Relationships

## Builds Upon
- tonnetz (the nonconforming version is a specific realization of the general Tonnetz concept)

## Enables
- syntonic-comma-striche (Striche are needed to disambiguate positions on the nonconforming Tonnetz)
- regional-space (diatonic regions appear as bounded parallelograms)

## Related
- oettingens-acoustical-matrix (the original conception was inherently nonconforming)
- enharmonic-equivalence (its absence defines the nonconforming version)

## Contrasts With
- toroidal-tonnetz (the conforming version that wraps into a torus under equal temperament)

# Common Errors

- **Error**: Assuming the nonconforming Tonnetz is an obsolete historical curiosity
  **Correction**: It remains analytically useful for just-intonation contexts, microtonal theory, and for representing the full derivational richness of tonal paths

# Common Confusions

- **Confusion**: The nonconforming Tonnetz is "wrong" because equal temperament is standard
  **Clarification**: Both forms are valid models for different purposes; Harrison's terminology ("conforming"/"nonconforming") refers to whether the model conforms to equal temperament, not whether it is correct or incorrect

# Source Reference

Gollin, Edward. "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 9.

Harrison, Daniel. "Nonconformist Notions of Nineteenth-Century Enharmonicism" (2002) — source of the conforming/nonconforming terminology.

# Verification Notes

New card (no previous version; the concept was previously folded into the tonnetz and toroidal-tonnetz cards). High confidence: explicitly defined by Gollin in Ch. 9 and discussed by Cohn in Ch. 11, with Harrison's terminology widely used in the literature.
