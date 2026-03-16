---
concept: Regional Space
slug: regional-space

category: pitch-space
subcategory: key representation and modulation
tier: intermediate

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Richard Cohn"
chapter: "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz"
chapter_number: 11
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "diatonic regions"
  - "regional encapsulation"
  - "key space on the Tonnetz"

prerequisites:
  - tonnetz
  - syntonic-comma-striche
extends:
  - tonnetz
related:
  - tonal-pitch-space
  - geometric-duals
  - hexatonic-systems
contrasts_with: []

answers_questions:
  - "How does the Tonnetz connect pitch space to key space?"
  - "How are diatonic keys represented on the Tonnetz?"
  - "How does modulation appear on the Tonnetz?"
---

# Quick Definition

The representation of diatonic keys as bounded parallelogram-shaped regions on the Tonnetz, whose positions and movements model tonal relationships and modulation, demonstrating how the Tonnetz simultaneously encodes pitch, chord, and key information.

# Core Definition

**Regional space** describes how diatonic collections (keys) map onto the Tonnetz as geometric regions. On the nonconforming Tonnetz, a diatonic collection appears as a **parallelogram** bounded by syntonic images of the same pitch (e.g., two versions of D mark the boundaries of the C major region). This parallelogram contains all 7 scale degrees and the triads constructible from them. Cohn (Ch. 11) uses regional encapsulation as a key argument against Lerdahl's claim that pitch, chord, and key spaces are structurally distinct: regions emerge naturally from the triadic structure of the Tonnetz through geometric duality, making a separate "regional space" unnecessary. Modulation appears as shifting the parallelogram to a new position, with the overlap between old and new regions indicating the number of common pitch classes (and hence the closeness of the modulation).

# Prerequisites

- **Tonnetz**: The spatial framework from which regions are derived
- **Syntonic comma and Striche**: The syntonic seam marks regional boundaries on the nonconforming Tonnetz

# Key Properties

1. **Parallelogram shape**: Each diatonic collection occupies a connected parallelogram on the Tonnetz
2. **Syntonic bounding**: The boundaries are syntonic images of the same pitch (e.g., D and D-bar for C major)
3. **Contains 7 pitch classes**: All degrees of the diatonic scale fall within the parallelogram
4. **Modulation as sliding**: Key change is visualized as repositioning the parallelogram
5. **Overlap indicates closeness**: More shared pitch classes between regions = closer keys
6. **Third level of duality**: Regions form the third level of geometric duality (after pitch-class and triadic levels)

# Construction / Recognition

To construct regional encapsulation on the Tonnetz:
1. Plot all 7 pitch classes of a diatonic key on the Tonnetz
2. Draw the minimal parallelogram enclosing them
3. Identify the syntonic seam (where two versions of the same pitch mark the boundary)
4. To model modulation, slide the parallelogram to enclose a different set of 7 pitch classes
5. Count overlapping pitch classes to measure key distance

On the conforming (toroidal) Tonnetz, the syntonic seam becomes a theoretical rather than acoustic boundary, but the regional encapsulation principle still holds.

# Context & Application

Regional encapsulation is Cohn's answer to Lerdahl's regional space. Where Lerdahl proposes a separate hierarchical level for key relationships, Cohn shows that regions emerge naturally from the Tonnetz's pitch-class and triadic structure. The concept is especially useful for:
- Visualizing modulation as geometric movement
- Analyzing regional indeterminacy (passages that resist key assignment)
- Comparing key distances through parallelogram overlap
- Understanding how hexatonic and octatonic cycles interact with diatonic regions

# Examples

Cohn (Ch. 11) demonstrates how:
- The C major region is a parallelogram on the Tonnetz containing C, D, E, F, G, A, B, bounded by two versions of D (syntonic seam)
- Moving one step right on the fifth axis (adding a sharp) shifts the parallelogram to the dominant key (G major)
- Hexatonic LP cycles span multiple regional parallelograms, which is why such passages resist regional determination
- In regionally indeterminate passages, the Tonnetz allows tracking triadic motion without forcing premature key assignment

# Relationships

## Builds Upon
- tonnetz (the spatial framework from which regions are derived)
- syntonic-comma-striche (the syntonic seam bounds regions)

## Enables
- Analysis of modulation as geometric movement
- Regional indeterminacy analysis (tracking motion without key assignment)

## Related
- tonal-pitch-space (Lerdahl's alternative approach to modeling key relationships)
- geometric-duals (regions form the third level of the duality chain)
- hexatonic-systems (hexatonic cycles interact with regional boundaries)

## Contrasts With
(none specific — the concept complements rather than contrasts with others)

# Common Errors

- **Error**: Forcing regional determination when a passage is genuinely indeterminate
  **Correction**: The Tonnetz allows tracking triadic motion without assigning a key; regional indeterminacy is an analytically valid state

# Common Confusions

- **Confusion**: A region is the same as a key
  **Clarification**: A region is a geometric area on the Tonnetz; a key is a functional organization with a tonic, hierarchy, and directionality. Regions model the pitch content of keys, not their full functional structure.

- **Confusion**: Regions are only meaningful on the nonconforming Tonnetz
  **Clarification**: Regional encapsulation works on both conforming and nonconforming Tonnetze; on the conforming version, the syntonic seam is a theoretical boundary rather than an acoustic one

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 11.

# Verification Notes

Re-extracted from v2 card; preserved: parallelogram construction, syntonic seam discussion, modulation-as-sliding model, Lerdahl comparison, regional indeterminacy concept. High confidence: explicitly discussed by Cohn in Ch. 11 as a key component of his argument.
