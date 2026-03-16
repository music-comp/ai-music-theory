---
concept: Non-Intervallic Transformations
slug: non-intervallic-transformations

category: transformation-theory
subcategory: non-intervallic-transformations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.2"

extraction_confidence: high

aliases: []

prerequisites:
  - simply-transitive-group
  - klang-representation
  - med-transformation
  - par-transformation
extends:
  - generalizing-power-of-transformational-theory
related:
  - tarnhelm-network
  - valhalla-network
contrasts_with:
  - gis-from-simply-transitive-group

answers_questions:
  - "What distinguishes non-intervallic transformations from intervallic ones?"
  - "Why can't some Klang networks be analyzed as GIS structures?"
---

# Quick Definition
Transformations that cannot be expressed within the framework of a simply transitive group (GIS structure), demonstrated by Klang networks using both SUBM/MED and PAR, where two distinct operations map the same input to the same output, violating simple transitivity.

# Core Definition
A transformation network is non-intervallic when the transformation group involved is not simply transitive on the relevant family of objects. For Klangs: any group containing both SUBM (or MED) and PAR cannot be simply transitive, because "given elements (C, +) and (C, -) in our family of elements, there would not be one unique member of the group transforming the former Klang into the latter; both MED^7 and PAR would do the job" (Lewin, 8.1.2, p. 179). The Tarnhelm and Valhalla networks (Figure 8.2) reference both SUBM and PAR, making them genuinely non-intervallic.

# Prerequisites
- **Simply transitive group** — Understanding what simple transitivity requires
- **Klang representation** — The space on which these transformations act
- **MED transformation** — MED generates a simply transitive group whose powers are formal intervals
- **PAR transformation** — PAR combined with MED breaks simple transitivity

# Key Properties
1. A group containing both MED and PAR is not simply transitive
2. Both MED^7 and PAR map (C, +) to (C, -), violating uniqueness
3. Networks using only MED powers CAN be formally intervallic
4. Adding PAR, REL, LT, or SLIDE to MED-based networks makes them non-intervallic
5. Non-intervallic networks are legitimate and analytically revealing
6. The only rescue for a simply transitive formalism would be "cleaving so firmly to just intonation" with an infinite number of distinct Klangs (p. 179)

# Construction / Recognition
## To Construct:
1. Build a Klang network using transformations from multiple families (e.g., SUBM and PAR)
2. Verify that the combined group is not simply transitive
## To Recognize:
1. Two different transformations in the group map the same input to the same output
2. The network uses transformations that cannot all belong to one simply transitive group

# Context & Application
Non-intervallic transformations demonstrate that "transformational theory genuinely extends beyond GIS structure" (synthesis). They are essential for analyzing mode-changing relationships in tonal music, serial transformations in post-tonal music, and inversional relationships in any GIS.

# Examples
**Example 1** (Figure 8.2, pp. 178-179): The Tarnhelm and Valhalla networks from Wagner's Ring use both SUBM and PAR. "We shall not be able to find a simply transitive group on a suitable family of Klangs that enables us to consider figure 8.2(a) and (b) as formally 'intervallic' graphs" (p. 179).

# Relationships
## Builds Upon
- **Generalizing power of transformational theory** — Non-intervallic transformations exemplify this power
## Enables
- **Tarnhelm network** — A demonstration of non-intervallic analysis
- **Valhalla network** — A demonstration of non-intervallic analysis
## Contrasts With
- **GIS from simply transitive group** — GIS requires simple transitivity; non-intervallic networks do not have it

# Common Errors
- **Error**: Labeling all Klang transformations as non-intervallic
  **Correction**: Networks using only MED powers (or only DOM/SUBD powers) can be formally intervallic

# Common Confusions
- **Confusion**: Thinking "non-intervallic" means arbitrary or unstructured
  **Clarification**: Non-intervallic transformations form well-defined groups; they are simply not simply transitive
- **Confusion**: Believing non-intervallic analyses are less rigorous
  **Clarification**: The analyses are equally formal; they use transformation groups rather than GIS intervals

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.2, pages 178-179.

# Verification Notes
- Definition source: Direct from 8.1.2 discussion
- Confidence rationale: Explicit proof with MED^7 vs. PAR example
- Re-extraction notes: Re-extracted from v2 card; preserved: MED^7/PAR proof, just-intonation escape clause
