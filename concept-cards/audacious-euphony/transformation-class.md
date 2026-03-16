---
# === CORE IDENTIFICATION ===
concept: Transformation Class
slug: transformation-class

# === CLASSIFICATION ===
category: transformations
subcategory: equivalence classes
tier: advanced

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 124
section: "Voice-Leading Zones"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "equivalence class of transformations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading-zones
  - h-group-transformations
  - w-group-transformations
extends:
  - voice-leading-zones
related:
  - e-class-transformations
  - compound-transformation-classes
  - transformational-substitution
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the equivalence classes for transformations in triadic space?"
  - "How do transformation classes relate to voice-leading zones?"
---

# Quick Definition
Three equivalence classes of transformations defined by their zone-shifting capacity: H-class (L, P, H) shifts zones by 1, W-class (R, N, S) shifts zones by 2, and E-class (LP, PL, identity E) shifts zones by 0.

# Core Definition
Having established voice-leading zones for triads, corresponding equivalence classes apply to transformations. "We define three such transformation classes: an H class, consisting of the three members of the hexatonic group (L, P, and H); a W class, consisting of the three members of the Weitzmann group (R, N, and S); and an E class, consisting of three transformations that map triads within their own zone (LP, PL, and identity operation E)" (p. 124). An H-class operation maps a consonant triad in zone X into the unique zone X +/- 1; a W-class operation maps a triad in zone Y into the unique zone Y +/- 2.

# Prerequisites
- **Voice-leading zones**: The zone system that transformation classes act upon
- **H-group transformations**: L, P, H as the hexatonic-preserving operations
- **W-group transformations**: R, N, S as the Weitzmann-preserving operations

# Key Properties
1. H-class: {L, P, H} -- shifts zone by +/- 1
2. W-class: {R, N, S} -- shifts zone by +/- 2
3. E-class: {LP, PL, E} -- shifts zone by 0 (identity on zones)
4. The notation "1 <--(H)--> 2" summarizes all H-class connections between zones 1 and 2
5. H and W are "exchange operations" (Lewin 1987, appendix B)

# Construction / Recognition
Interpreting class notation:
- "1 <--(H)--> 2" means: any H-group transformation applied to any triad in zone 1 produces a triad in zone 2, and vice versa
- "2 <--(W)--> 4" means: any W-group transformation connects zones 2 and 4

# Context & Application
Transformation classes consolidate large numbers of specific instances into general principles. "The consolidating power is along the lines of a claim about numbers such as odd + odd = even, which consolidates in one statement an infinite variety of propositions covered by it" (p. 126). The abstraction enables analysis of voice-leading trajectories independent of specific transformation choices.

# Examples
- "1 <--(H)--> 2" covers: L applied to any zone-1 triad yields a zone-2 triad; P likewise; H likewise (p. 124)
- "2 <--(W)--> 4" covers all W-class connections between these zones

# Relationships
## Builds Upon
- Voice-leading zones as the objects being acted upon
## Enables
- Compound transformation classes (HW, HWH, HWHW, etc.)
- Abstract trajectory analysis independent of specific transformations
## Related
- E-class transformations (the zone-preserving class)
- Transformational substitution (the practical application)
## Contrasts With
- Specific transformation names (L, P, R, etc.)

# Common Errors
- **Error**: Transformation classes are about general identity of transformations
  **Correction**: They are equivalent only with respect to zone-shifting capacity, not in all respects

# Common Confusions
- **Confusion**: E-class means "do nothing"
  **Clarification**: E-class operations change the specific triad (LP maps C major to E major) but preserve the voice-leading zone

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, pp. 124-126.

# Verification Notes
Re-extracted from v2 card; preserved: exchange operations attribution, consolidating power analogy. High confidence -- clearly defined formal concept.
