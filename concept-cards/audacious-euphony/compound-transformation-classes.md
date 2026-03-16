---
# === CORE IDENTIFICATION ===
concept: Compound Transformation Classes
slug: compound-transformation-classes

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
pdf_page: 125
section: "Voice-Leading Zones"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "compound HW classes"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-class
  - voice-leading-zones
extends:
  - transformation-class
related:
  - chromatic-sequences
  - zone-diametric-relations
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are triads in non-adjacent zones connected by compound transformations?"
  - "How many specific transformations does each compound class cover?"
---

# Quick Definition
Classes of compound transformations connecting triads in non-adjacent voice-leading zones, built by alternating H-class and W-class operations: HW connects zones 3 apart, HWH connects 4 apart, WHW connects 5 apart, and HWHW/WHWH connects the maximum of 6 apart.

# Core Definition
With H- and W-group transformation classes as basic elements, compound classes connect triads from non-adjacent zones. "Triads that are transpositionally related by an odd value, three voice-leading units apart, are connected by a compound HW or WH transformation; in effect, this is the generalization that underlies all of the work with sequences and transformational substitutions in this chapter" (pp. 125-126). Four units: HWH; five units: WHW; six units (maximum): HWHW or WHWH.

# Prerequisites
- **Transformation class**: The basic H, W, and E classes
- **Voice-leading zones**: The zone system these compounds operate on

# Key Properties
1. HW or WH: zones 3 apart (9 specific combinations = 3 x 3)
2. HWH: zones 4 apart (27 specific sets of transformations = 3^3)
3. WHW: zones 5 apart (27 specific sets)
4. HWHW or WHWH: zones 6 apart (81 specific sets = 3^4)
5. The generalizing power parallels "odd + odd = even" -- a single statement consolidating many specific instances

# Construction / Recognition
For any two triads, determine their zone distance:
- Distance 1: single H-class operation
- Distance 2: single W-class operation
- Distance 3: HW or WH compound
- Distance 4: HWH compound
- Distance 5: WHW compound
- Distance 6: HWHW or WHWH compound

# Context & Application
Compound classes provide the abstract framework underlying all chromatic sequence analysis. The three-term compounds cover 27 specific sets of transformations; four-term compounds cover 81. This level of abstraction consolidates vast numbers of specific cases into general principles. Cohn notes that "for readers mystified by this mode of discourse, be assured that the remainder of the book proceeds independently of these abstractions" (p. 126).

# Examples
- Distance 3 (HW): the generalization underlying all odd-transposition chromatic sequences (the central work of ch5)
- Distance 4 (HWH): d minor (zone 4) to G major (zone 8) requires HWH
- Distance 6 (HWHW): connects zone-diametric triads (T2 or T6 related), maximum distance

# Relationships
## Builds Upon
- Transformation classes as the building blocks
## Enables
- Zone-diametric analysis (distance 6)
- General trajectory analysis independent of specific transformations
## Related
- Chromatic sequences (HW compound generates all odd-transposition sequences)
- Zone-diametric relations (HWHW connects maximum-distance triads)
## Contrasts With
- Individual transformation names (the specific level these generalize away from)

# Common Errors
- **Error**: Compound classes require specific transformations in specific order
  **Correction**: Any combination of the specified class types in the specified order suffices

# Common Confusions
- **Confusion**: These abstractions are required for practical analysis
  **Clarification**: They are optional; the book proceeds without them after this section
- **Confusion**: HWHW and WHWH are different distances
  **Clarification**: Both connect zones 6 apart; they represent different orderings of the same distance

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, pp. 125-126.

# Verification Notes
Re-extracted from v2 card; preserved: specific combination counts (27, 81), "odd + odd = even" analogy. High confidence -- clearly articulated formal system.
