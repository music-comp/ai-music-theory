---
# === CORE IDENTIFICATION ===
concept: STRANS2 Group
slug: strans2-group

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
  - STRANS2

# === TYPED RELATIONSHIPS ===
prerequisites:
  - octatonic-pitch-class-set
  - simply-transitive-group
  - strans1-group
extends: []
related:
  - gis2-octatonic
  - dual-simply-transitive-groups
  - queer-operations
  - exchanging-operations
contrasts_with:
  - strans1-group

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is STRANS2 in the octatonic GIS?"
  - "What are the queer and exchanging operations?"
---

# Quick Definition
An alternative simply transitive group of eight operations on the octatonic collection comprising RO, Q3, R6, Q9 (including "queer" operations that rotate the two diminished-seventh chords in opposite directions) and X1, X2, X4, X5 (exchanging operations), forming the dual to STRANS1.

# Core Definition
STRANS2 contains RO and R6 from STRANS1, plus "queer" operations Q3, Q9 and "exchanging" operations X1, X2, X4, X5. "Q3 rotates each of the diminished-seventh chords within S, but in opposite directions; it maps C to D#, D# to F#, F# to A, A to C, and also C# to A# (not to E), A# to G, G to E, and E to C#" (Lewin, Appendix B, p. 251). STRANS2 is simply transitive on S, and "every member of STRANS2 commutes with every member of STRANS1" (p. 252). STRANS2 consists precisely of those operations commuting with all STRANS1 members.

# Prerequisites
- **Octatonic pitch-class set** — The space of action
- **Simply transitive group** — STRANS2 has this property
- **STRANS1 group** — STRANS2 is defined as its dual/commutant

# Key Properties
1. |STRANS2| = 8, simply transitive on S
2. RO, R6: shared with STRANS1
3. Q3: rotates dim7 chords in opposite directions; Q9 = Q3^{-1}
4. X1, X2, X4, X5: exchange pitch classes at specified semitone distances
5. Every STRANS2 member commutes with every STRANS1 member
6. STRANS2 = {f : S -> S | fg = gf for all g in STRANS1}

# Construction / Recognition
## To Construct:
1. Find all operations on S that commute with every member of STRANS1
2. These form STRANS2
## To Recognize:
1. Operations that rotate dim7 chords in opposite directions (Q3, Q9) or exchange elements at fixed distances (X1-X5)

# Context & Application
STRANS2 provides an alternative perspective for analyzing octatonic music. Its "queer" operations have no standard twelve-tone counterpart, offering genuinely novel analytical tools. The dual relationship means both STRANS1 and STRANS2 perspectives are equally valid.

# Examples
**Example 1** (p. 251): Q3: C->D#, D#->F#, F#->A, A->C (forward rotation of dim7 #1), C#->A#, A#->G, G->E, E->C# (backward rotation of dim7 #2).

**Example 2** (p. 251): X1 exchanges pairs one semitone apart: C<->C#, D#<->E, F#<->G, A<->A#.

# Relationships
## Builds Upon
- **STRANS1 group** — STRANS2 is its commutant/dual
## Enables
- **GIS2 octatonic** — GIS with STRANS2 as transpositions
## Related
- **Dual simply transitive groups** — STRANS1 and STRANS2 exemplify duality
- **Queer operations** — Q3, Q9 are unique to STRANS2
- **Exchanging operations** — X1, X2, X4, X5 are unique to STRANS2
## Contrasts With
- **STRANS1 group** — Different group on the same set

# Common Errors
- **Error**: Trying to express Q3 or Q9 as standard T or I operations
  **Correction**: Queer operations have no standard twelve-tone counterpart

# Common Confusions
- **Confusion**: Thinking STRANS2 is less natural than STRANS1
  **Clarification**: The duality is symmetric; neither group is more fundamental

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, pages 251-252.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined with all operations specified
- Re-extraction notes: Re-extracted from v2 card; preserved: Q3 mapping, X1 mapping, commutation property
