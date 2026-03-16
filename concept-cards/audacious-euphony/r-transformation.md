---
# === CORE IDENTIFICATION ===
concept: "R Transformation (Relative)"
slug: r-transformation

# === CLASSIFICATION ===
category: transformations
subcategory: weitzmann-transformations
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Weitzmann Regions"
chapter_number: 4
pdf_page: 79
section: "Weitzmann Transformations and N/R Cycles"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Relative transformation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - consonant-triad
extends: []
related:
  - n-transformation
  - s-transformation
  - l-transformation
  - p-transformation
  - nr-cycle
  - weitzmann-transformations
  - w-group
contrasts_with:
  - l-transformation
  - p-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes L, P, R, N, and S transformations?"
  - "What is the R transformation?"
  - "Why does R appear closer than it really is on the Tonnetz?"
---

# Quick Definition
The Relative transformation (R) takes a consonant triad to its relative major or minor, preserving two common tones while moving one voice by whole step (2 semitones).

# Core Definition
R maps any major triad to its relative minor and vice versa (e.g., C major to a minor). It "preserves the two tones common to both triads and moves the third voice by whole step" (p. 79). R is the only common-tone-maximizing transformation that requires 2 units of voice-leading work, compared to L and P which require 1 unit each. On the Tonnetz, "R passes across an edge that connects the two tones common to C major and a minor" (p. 83), creating the illusion that it covers less distance than N or S.

# Prerequisites
- **Consonant triad**: R operates on consonant triads, connecting relative major/minor pairs

# Key Properties
1. R maps major to relative minor and vice versa (e.g., C major <-> a minor)
2. One voice moves by whole step (2 semitones); two voices stationary
3. R preserves 2 common tones (the major-third dyad)
4. R requires 2 units of voice-leading work
5. R is an involution: R(R(X)) = X
6. R preserves Weitzmann region and shifts hexatonic region
7. R appears closer on the uninflected Tonnetz than N and S, but this is an "illusion" from the standpoint of voice-leading work (p. 83)

# Construction / Recognition
Voice-leading action:
- C major {C, E, G} -> a minor {A, C, E}: G moves down to A (whole step), C and E retained
- a minor {A, C, E} -> C major {C, E, G}: A moves up to G (whole step), C and E retained

# Context & Application
R is the most familiar W-group transformation, corresponding to the traditional relative major/minor relationship. In Weitzmann's conception, it is one of two fundamental relations (with N) that structure each region. R combined with N generates complete Weitzmann regions.

# Examples
**Chopin, Nocturne Op. 9 No. 1** (p. 88): bb minor -> Db major (R) at m. 5.

**N/R cycles** (pp. 80-82): R appears throughout Schubert's N/R chains, alternating with N.

**Tonnetz illusion** (p. 83): "Considering the augmented-triad stalk as an object dispels that illusion" that R involves less distance than other Weitzmann transformations.

# Relationships
## Builds Upon
- consonant-triad: R connects relative major/minor pairs

## Enables
- nr-cycle: R alternating with N generates the Weitzmann region cycle

## Related
- n-transformation: Together with R, generates the N/R cycle
- s-transformation: The third W-group transformation
- w-group: R is a member of the W-group

## Contrasts With
- l-transformation: L preserves the minor-third dyad and requires 1 unit; R preserves the major-third dyad and requires 2 units
- p-transformation: P preserves the perfect-fifth dyad and requires 1 unit; R preserves the major-third dyad and requires 2 units

# Common Errors
- **Error**: Treating R as equivalent in distance to L and P because all three maximize common tones
  **Correction**: R requires 2 voice-leading units vs. 1 for L and P; common-tone count does not equal voice-leading distance

# Common Confusions
- **Confusion**: R is closer than N and S within a Weitzmann region
  **Clarification**: "R is a more distant relation than L and P, just as surely as a whole step is larger than a semitone, and is neither closer nor farther than the other four relations internal to a Weitzmann region" (p. 83)

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 4: Weitzmann Regions, pp. 79-80, 83.

# Verification Notes
Re-extracted from v2 card; preserved: Tonnetz illusion discussion, voice-leading work comparison with L and P. High confidence: clearly defined transformation.
