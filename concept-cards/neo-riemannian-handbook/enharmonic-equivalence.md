---
concept: Enharmonic Equivalence
slug: enharmonic-equivalence

category: pitch-space
subcategory: pitch identity and equivalence
tier: foundational

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Edward Gollin"
chapter: "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann"
chapter_number: 9
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "enharmonic identification"
  - "enharmonic reinterpretation"

prerequisites:
  - tonnetz
extends: []
related:
  - toroidal-tonnetz
  - nonconforming-tonnetz
  - syntonic-comma-striche
  - hexatonic-systems
contrasts_with: []

answers_questions:
  - "What is enharmonic equivalence and how does it affect the Tonnetz?"
  - "How does enharmonic equivalence relate to equal temperament?"
  - "How did Riemann's view of enharmonic equivalence evolve?"
---

# Quick Definition

The condition under equal temperament where differently-spelled pitches (e.g., C# and Db) sound identical, enabling the Tonnetz to wrap from an infinite plane into a finite torus and permitting enharmonic reinterpretation in harmonic progressions.

# Core Definition

**Enharmonic equivalence** is the identification of differently-spelled pitches that have the same acoustic frequency under equal temperament. In neo-Riemannian theory, this concept determines whether the Tonnetz is an infinite plane (no equivalence, just intonation) or a finite torus (full equivalence, equal temperament). Gollin traces the evolving treatment of this concept through three positions (Ch. 9): Oettingen regarded enharmonic distinctions as acoustically real, treating Beethoven's enharmonic modulation in Op. 26 as a compositional error because Bbb and A differ by an enharmonic diesis (128:125, approximately 41 cents). In *Skizze* (1880), Riemann distinguished between **orthographic enharmonicism** (mere respelling for convenience) and **real/genuine enharmonicism** (arising through harmonic progression). By "Ideen" (1914-15), Riemann embraced enharmonic equivalence as "absolutely indispensable for our hearing of music," arguing that the imagination equates enharmonic equivalents while preserving their distinct derivational meanings.

# Prerequisites

- **Tonnetz**: Enharmonic equivalence determines the Tonnetz's fundamental geometry

# Key Properties

1. **Pitch identity**: C# = Db, F# = Gb, etc., under equal temperament (12-TET)
2. **Geometric consequence**: Enforcing equivalence causes the infinite Tonnetz plane to wrap into a torus
3. **Enharmonic diesis**: In just intonation, enharmonic equivalents differ by 128:125 (approximately 41 cents)
4. **Transformational closure**: Equivalence enables the PLR group to be a finite group of order 24
5. **Distinct from syntonic equivalence**: Syntonic equivalence (collapsing comma differences) and enharmonic equivalence (collapsing spelling differences) are separate but both required for the conforming Tonnetz

# Construction / Recognition

To identify where enharmonic equivalence matters:
1. Trace a chain of thirds or fifths on the Tonnetz
2. After 12 fifths or 3 major thirds, the path returns to the "same" pitch class (under ET) but a different Tonnetz position (under JI)
3. Enforcing enharmonic equivalence identifies these positions, creating the wrapping that produces the torus

Riemann's distinction: orthographic enharmonicism is simple respelling (B major for Cb major); real enharmonicism arises when a progression traverses tonal space such that the "true" harmonic derivation differs from the spelled result.

# Context & Application

Enharmonic equivalence is a foundational assumption of modern neo-Riemannian theory. Without it, the PLR group cannot be finite, hexatonic cycles cannot close, and the entire apparatus of transformation groups on the 24 consonant triads breaks down. Historically, the acceptance of enharmonic equivalence was neither obvious nor uncontroversial: Oettingen and early Riemann resisted it, and the gradual acceptance tracks the broader shift from just-intonation thinking to equal-temperament thinking in the late 19th century.

# Examples

Gollin's central example (Ch. 9): Oettingen criticized Beethoven's Op. 26 funeral march, where a descent by three major thirds (Ab to E to C to Ab, enharmonically) crosses an enharmonic seam. On the nonconforming Tonnetz, this path leads to Bbb, not A — the two are an enharmonic diesis apart. Oettingen regarded this as a compositional error.

Riemann's mature position from "Ideen" (1914-15): "Our imagination knows nothing of the intonational difference between d and d-bar, but rather equates both, imagining d as the lower fifth of a and yet at the same time also as the upper fifth of g." The Tonnetz mediates between tempered sounds and unbounded mental meanings.

# Relationships

## Builds Upon
- tonnetz (enharmonic equivalence determines its geometry)

## Enables
- toroidal-tonnetz (enharmonic equivalence creates the wrapping)
- hexatonic-systems (LP cycles require enharmonic closure to form finite cycles)

## Related
- nonconforming-tonnetz (defined by the absence of enharmonic equivalence)
- syntonic-comma-striche (a related but distinct type of pitch equivalence)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Assuming enharmonic equivalence means C# and Db are "the same thing" in all analytical contexts
  **Correction**: Even under equal temperament, different spellings may carry different analytical meanings reflecting distinct derivational pathways

# Common Confusions

- **Confusion**: Enharmonic equivalence was always assumed in Riemannian theory
  **Clarification**: Oettingen rejected it entirely; Riemann resisted it for decades before embracing it in his mature work (1914-15)

- **Confusion**: Enharmonic and syntonic equivalence are the same thing
  **Clarification**: Enharmonic equivalence collapses differently-spelled pitches (C# = Db, differing by the diesis 128:125); syntonic equivalence collapses like-named pitches at different Tonnetz positions (differing by the comma 81:80)

# Source Reference

Gollin, Edward. "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 9.

Cohn, Richard. Chapter 11 (enharmonic equivalence as default analytical assumption).

Harrison, Daniel. "Nonconformist Notions of Nineteenth-Century Enharmonicism" (2002).

# Verification Notes

Re-extracted from v2 card; preserved: Oettingen's Beethoven Op. 26 critique, Riemann's orthographic vs. real distinction, "Ideen" quotation, enharmonic diesis value. High confidence: central to Gollin's argument and explicitly discussed throughout Ch. 9.
