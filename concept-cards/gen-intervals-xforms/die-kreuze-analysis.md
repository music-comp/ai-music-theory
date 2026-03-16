---
concept: '"Die Kreuze" Wedge Analysis'
slug: die-kreuze-analysis

category: analytical-applications
subcategory: schoenberg-analysis
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: null

extraction_confidence: high

aliases:
  - Pierrot Lunaire no. 14 analysis

prerequisites:
  - inj-function
  - wedge-transformation
extends: []
related:
  - angst-hoffen-analysis
  - system-modulation
contrasts_with: []

answers_questions:
  - "How do dyad-focal wedge transformations work?"
---

# Quick Definition
Lewin's analysis of "Die Kreuze" (Pierrot Lunaire no. 14) demonstrates wedge transformations converging to dyad focal points ({C, C#} and {F#, G}) rather than single pitch classes, with the "missing F" theme connecting to the "Angst und Hoffen" analysis via system modulation.

# Core Definition
The analysis (Lewin, pp. 164-165, Figure 6.6) employs: w^{C/C#} converging to {C, C#} (F#->F->E->Eb->D->C#->C# and G->Ab->A->Bb->B->C->C), w^{F#/G} converging to {F#, G}, and I_G = I_{C#}. Special property: no pitch class p satisfies w^{F#/G}(p) = F# except F# itself, and no p satisfies w^{C/C#}(p) = Db. These "sitting out" notes are compositionally significant.

# Prerequisites
- **INJ Function** — Analytical framework
- **Wedge Transformation** — Dyad-focal wedges generalize single-focal wedges

# Key Properties
1. Dyad-focal wedges converge to two adjacent pitch classes, not one
2. Some pitch classes have no preimage under the wedge (they "sit out")
3. The "missing F" reappears: missing as semitone neighbor to Gb in F#-wedge
4. System modulation T_2 connects E-centered (op. 15) to F#-centered (Pierrot) structures

# Construction / Recognition
## To Construct a Dyad-Focal Wedge:
1. Choose two adjacent pitch classes u, v as focal dyad
2. Pitch classes on the u-side advance toward u; on the v-side toward v
3. u and v are fixed points

## To Recognize:
1. Voice-leading convergence toward a dyad rather than a single pitch

# Context & Application
The analysis extends wedge techniques from "Angst und Hoffen" to a different Schoenberg work, demonstrating the generality of the approach and the connection via system modulation.

# Examples
**Example 1** (p. 165, Figure 6.6): Opening of "Die Kreuze": F# sits out its wedge game (no note wedges to it). From Z3 onward, outer voices almost converge to F#/Gb except missing F. The I_B also structures the second chord as an "internal" transformation.

# Relationships
## Builds Upon
- **Wedge Transformation** — Extends to dyad-focal wedges

## Related
- **Angst und Hoffen Analysis** — Connected via T_2 system modulation

# Common Errors
- **Error**: Assuming all wedges converge to single pitch classes
  **Correction**: Dyad-focal wedges converge to adjacent semitone pairs

# Common Confusions
- **Confusion**: Thinking the "missing F" in this piece is unrelated to the "missing F" in op. 15
  **Clarification**: Lewin explicitly connects them via T_2 modulation: F missing as neighbor to E (op. 15) vs. F missing as neighbor to Gb (Pierrot)

# Source Reference
Chapter 6: Generalized Set Theory (2), Figure 6.6, pp. 164-165.

# Verification Notes
- Definition source: Direct from analytical discussion
- Confidence rationale: Detailed analysis with musical examples
- Re-extraction notes: Re-extracted from v2 card; preserved: dyad-focal wedge mechanics, "sitting out" property, connection to op. 15. Added v3.1 structure.
