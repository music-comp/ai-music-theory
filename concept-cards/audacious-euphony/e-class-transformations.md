---
# === CORE IDENTIFICATION ===
concept: E-Class Transformations
slug: e-class-transformations

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
  - "zone-preserving transformations"
  - "identity class"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-class
  - voice-leading-zones
extends:
  - transformation-class
related:
  - h-group-transformations
  - w-group-transformations
contrasts_with:
  - h-group-transformations
  - w-group-transformations

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Which transformations preserve voice-leading zone?"
  - "What is the E-class and how does it complement H-class and W-class?"
---

# Quick Definition
The class of transformations that map triads within their own voice-leading zone (zone shift = 0), consisting of LP, PL, and the identity operation E.

# Core Definition
E-class transformations are defined alongside H-class and W-class: "an E class, consisting of three transformations that map triads within their own zone (LP, PL, and identity operation E)" (p. 124). E-class completes the system of transformation classes by handling the zero-distance case. LP and PL connect T4-related triads (e.g., C major to E major via LP; C major to Ab major via PL), while E maps a triad to itself.

# Prerequisites
- **Transformation class**: The general concept of transformation equivalence classes
- **Voice-leading zones**: The zones that E-class preserves

# Key Properties
1. Members: LP, PL, identity E
2. Zone shift: 0
3. LP and PL produce balanced voice leading (no net upshift or downshift)
4. LP and PL connect T4-related triads within the same Cube Dance radius
5. Complements H-class (shift 1) and W-class (shift 2)

# Construction / Recognition
- LP: C major -> e minor (L) -> E major (P) -- zone 11 -> zone 11
- PL: C major -> c minor (P) -> Ab major (L) -- zone 11 -> zone 11
- E: C major -> C major -- zone 11 -> zone 11

# Context & Application
E-class operations explain why T4-related triads can substitute for each other: they are connected by zone-preserving transformations. The class is less prominent analytically than H- and W-classes but is essential for completing the theoretical system.

# Examples
- Major-third progressions within hexatonic cycles
- Passages cycling through T4-related triads without net voice-leading shift

# Relationships
## Builds Upon
- Transformation class system
## Enables
- Complete classification of all zone-shifting possibilities (0, 1, 2)
## Related
- H-group and W-group transformations
## Contrasts With
- H-class (zone shift 1) and W-class (zone shift 2)

# Common Errors
- **Error**: E-class means "do nothing"
  **Correction**: LP and PL change the specific triad (e.g., C major to E major) but preserve the zone

# Common Confusions
- **Confusion**: E-class is trivial
  **Clarification**: It is essential for explaining T4 equivalence and completing the theoretical system

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, p. 124.

# Verification Notes
Re-extracted from v2 card; preserved: LP/PL examples, complement to H/W classes. High confidence -- briefly but clearly defined.
