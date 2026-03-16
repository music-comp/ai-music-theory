---
# === CORE IDENTIFICATION ===
concept: W-Group Transformations
slug: w-group-transformations

# === CLASSIFICATION ===
category: transformations
subcategory: transformation-groups
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Weitzmann Regions"
chapter_number: 4
pdf_page: 77
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Weitzmann group transformations (glossary)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - w-group
extends: []
related:
  - r-transformation
  - n-transformation
  - s-transformation
  - h-group-transformations
contrasts_with:
  - h-group-transformations

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the W-group transformations?"
  - "What properties do all W-group transformations share?"
---

# Quick Definition
Any transformation that maps a consonant triad within its Weitzmann region and outside of its hexatonic region, comprising R (Relative), N (Nebenverwandt), and S (Slide).

# Core Definition
W-group transformations are defined in the Glossary as the set of three transformations connecting triads that share Weitzmann region membership while belonging to different hexatonic regions. Each involves motion "through or around an augmented triad that serves as the center of the Weitzmann region" (cf. Glossary, p. 229). Properties shared by all three: involution, 2 semitones of voice-leading work, Weitzmann region preservation, hexatonic region shift, and mode change.

# Prerequisites
- **W-group**: The algebraic group these transformations belong to

# Key Properties
1. R preserves the major-third dyad; one voice moves by whole step
2. N preserves the perfect-fifth dyad; two voices move by semitone in contrary motion
3. S preserves the third; root and fifth both move by semitone in same direction
4. All are involutions: applying twice returns to original
5. All involve 2 semitones of voice-leading work
6. All connect triads sharing a Weitzmann region

# Construction / Recognition
- R(C major) = a minor: (C, E, G) -> (A, C, E)
- N(C major) = f minor: (C, E, G) -> (F, Ab, C)
- S(C major) = c# minor: (C, E, G) -> (C#, E, G#)

# Context & Application
W-group transformations produce the characteristic progressions of Weitzmann regions. While H-group transformations keep triads within hexatonic regions, W-group transformations cross hexatonic boundaries while remaining within a Weitzmann region.

# Examples
- R: Relative major/minor (C major to a minor)
- N: Nebenverwandt (C major to f minor)
- S: Slide (C major to c# minor)

# Relationships
## Builds Upon
- w-group: The group to which these transformations belong

## Enables
None directly; individual transformations enable specific structures.

## Related
- r-transformation, n-transformation, s-transformation: The individual members
- h-group-transformations: The complementary set

## Contrasts With
- h-group-transformations: H-group preserves hexatonic region; W-group preserves Weitzmann region

# Common Errors
- **Error**: Treating W-group as including LP and PL
  **Correction**: LP and PL are hexatonic compounds; the W-group contains only mode-reversing single transformations

# Common Confusions
- **Confusion**: W-group and H-group partition all possible transformations
  **Clarification**: They partition the six single-step mode-reversing transformations; same-mode connections (LP, PL) belong to neither group exclusively

# Source Reference
Cohn, Richard. *Audacious Euphony*, Glossary, p. 229.

# Verification Notes
Re-extracted from v2 card; preserved: glossary definition, shared properties list, H-group contrast. High confidence: formally defined in glossary.
