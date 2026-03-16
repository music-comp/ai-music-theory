---
# === CORE IDENTIFICATION ===
concept: "Rotation Operations (RO, R3, R6, R9)"
slug: rotation-operations

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
  - RO
  - R3
  - R6
  - R9

# === TYPED RELATIONSHIPS ===
prerequisites:
  - octatonic-pitch-class-set
extends: []
related:
  - strans1-group
  - strans2-group
contrasts_with:
  - queer-operations

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the rotation operations on the octatonic collection?"
---

# Quick Definition
Operations that rotate elements within the octatonic collection while preserving each diminished-seventh chord: RO (identity), R3 (rotation by minor third), R6 (rotation by tritone), R9 (rotation by major sixth). Induced from T_0, T_3, T_6, T_9 respectively.

# Core Definition
"The operations RO, R3, R6, and R9 may be thought of as 'rotations,' to justify the use of the letter R in their names" (Lewin, Appendix B, p. 251). These are induced from T_0, T_3, T_6, T_9 on the twelve pitch classes. They preserve each diminished-seventh chord within S: if s is in dim7 #1, then RO(s), R3(s), R6(s), R9(s) are all in dim7 #1 as well. RO and R6 appear in both STRANS1 and STRANS2.

# Prerequisites
- **Octatonic pitch-class set** — The rotations act on S

# Key Properties
1. RO: identity (from T_0)
2. R3: rotation by minor third (from T_3)
3. R6: rotation by tritone (from T_6)
4. R9: rotation by major sixth / minor third down (from T_9)
5. All preserve each diminished-seventh chord
6. R3 and R9 are in STRANS1 but not STRANS2
7. RO and R6 are in both STRANS1 and STRANS2

# Construction / Recognition
## To Construct:
1. Apply T_0, T_3, T_6, or T_9 to elements of S
## To Recognize:
1. Elements of each dim7 chord map to elements of the same dim7 chord
2. Both dim7 chords rotate in the same direction

# Context & Application
Rotations are the "familiar" part of both STRANS groups, corresponding to the transpositions that preserve the octatonic collection. They contrast with the queer operations (which rotate in opposite directions) and the exchanging/K-L-M-N operations (which swap between dim7 chords).

# Examples
**Example 1** (p. 251): R3 maps C->D#, D#->F#, F#->A, A->C within dim7 #1, and C#->E, E->G, G->A#, A#->C# within dim7 #2.

# Relationships
## Related
- **STRANS1 group** — RO, R3, R6, R9 are the rotation subgroup of STRANS1
- **STRANS2 group** — RO, R6 appear in STRANS2 (but R3, R9 are replaced by Q3, Q9)
## Contrasts With
- **Queer operations** — Q3, Q9 rotate dim7 chords in opposite directions; R3, R9 rotate in the same direction

# Common Errors
- **Error**: Equating R3 on S with T_3 on the twelve pitch classes
  **Correction**: They are different operations on different spaces, though R3 is induced from T_3

# Common Confusions
- **Confusion**: Thinking all rotations are in both STRANS groups
  **Clarification**: Only RO and R6 are in both; R3, R9 are in STRANS1 only

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 251.

# Verification Notes
- Definition source: Direct from Appendix B
- Confidence rationale: Explicitly named and defined
- Re-extraction notes: Re-extracted from v2 card; preserved: dim7 preservation, shared RO/R6 membership
