---
# === CORE IDENTIFICATION ===
concept: "GIS2 (Octatonic GIS with STRANS2)"
slug: gis2-octatonic

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: octatonic-structures
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: null
pdf_page: 282
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - GIS2

# === TYPED RELATIONSHIPS ===
prerequisites:
  - strans2-group
  - gis-from-simply-transitive-group
extends:
  - generalized-interval-system
related:
  - gis1-octatonic
  - dual-simply-transitive-groups
contrasts_with:
  - gis1-octatonic

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is GIS2 for the octatonic collection?"
  - "How do queer and exchanging operations become transpositions?"
---

# Quick Definition
GIS2 = (S, IVLS2, int2) is a Generalized Interval System on the octatonic collection where STRANS2 operations (including queer and exchanging operations) serve as formal transposition operations and STRANS1 operations are the interval-preserving operations.

# Core Definition
"Using the method of 7.1.1, we can develop another GIS involving the family S, a GIS for which the members of STRANS2 are exactly the formal transposition operations. We can call this structure GIS2 = (S, IVLS2, int2). In this GIS, applying any of the operations RO, Q3, R6, Q9, X1, X2, X4, or X5 to a member s of S amounts precisely to transposing s, formally, by a suitable corresponding interval of GIS2" (Lewin, Appendix B, p. 252). The interval-preserving operations for GIS2 are exactly STRANS1 members.

# Prerequisites
- **STRANS2 group** — Provides the transposition operations
- **GIS from simply transitive group** — The construction method

# Key Properties
1. GIS2 = (S, IVLS2, int2)
2. Formal transpositions: STRANS2 = {RO, Q3, R6, Q9, X1, X2, X4, X5}
3. Interval-preserving operations: STRANS1
4. Q3, Q9, X1-X5 are transpositions in GIS2
5. Duality: STRANS1 transpositions preserve STRANS2 intervals, and vice versa
6. Demonstrates that the same space can support multiple GIS structures

# Construction / Recognition
## To Construct:
1. Apply Theorem 7.1.1 to S and STRANS2
2. Create IVLS2 in bijection with STRANS2
3. Define int2(s, t) = unique member of STRANS2 mapping s to t
## To Recognize:
1. Intervals measured by STRANS2 operations
2. Queer and exchanging operations functioning as transpositions

# Context & Application
GIS2 offers a genuinely novel perspective on octatonic music. The STRANS2-forms of a set (e.g., applying all eight STRANS2 operations to (C, E, G)) yield "a more novel sort of family" than STRANS1-forms, which are "exactly the dodecaphonically transposed and inverted forms of the set that lie within S" (p. 252).

# Examples
**Example 1** (p. 252): STRANS2-forms of {C, E, G}: RO(C,E,G) = (C,E,G), Q3(C,E,G) = (D#,C#,E), R6(C,E,G) = (F#,A#,C#), Q9(C,E,G) = (A,G,A#), X1(C,E,G) = (C#,D#,F#), X2(C,E,G) = (A#,F#,A), X4(C,E,G) = (F#,C,D#), X5(C,E,G) = (G,A,C).

# Relationships
## Builds Upon
- **STRANS2 group** — Transpositions of GIS2
- **GIS from simply transitive group** — Construction method
## Related
- **Dual simply transitive groups** — GIS1 and GIS2 exemplify the duality
## Contrasts With
- **GIS1 octatonic** — Different GIS on the same set; roles of transposition and interval-preservation swap

# Common Errors
- **Error**: Assuming STRANS2-forms are the same as standard T/I forms
  **Correction**: STRANS2-forms are "a more novel sort of family" distinct from standard forms

# Common Confusions
- **Confusion**: Thinking GIS2 is less "natural" than GIS1
  **Clarification**: The duality is symmetric; GIS2 may reveal structures invisible from GIS1's perspective

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 252.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined with worked example
- Re-extraction notes: Re-extracted from v2 card; preserved: STRANS2-forms example, duality, "novel" forms
