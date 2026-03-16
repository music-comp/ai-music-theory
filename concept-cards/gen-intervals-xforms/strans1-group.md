---
concept: STRANS1 Group
slug: strans1-group

category: generalized-interval-systems
subcategory: octatonic-structures
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: null
pdf_page: 282
section: null

extraction_confidence: high

aliases:
  - STRANS1

prerequisites:
  - octatonic-pitch-class-set
  - simply-transitive-group
extends: []
related:
  - strans2-group
  - gis1-octatonic
  - dual-simply-transitive-groups
  - rotation-operations
contrasts_with:
  - strans2-group

answers_questions:
  - "What is STRANS1 in the octatonic GIS?"
  - "How does STRANS1 relate to standard T/I operations?"
---

# Quick Definition
The simply transitive group of eight operations on the octatonic collection comprising RO, R3, R6, R9 (rotations preserving each diminished-seventh chord) and K, L, M, N (operations swapping between diminished-seventh chords), induced from standard atonal operations T_0, T_3, T_6, T_9, I_5, I_E, I_8, I_{A#}.

# Core Definition
"Eight of the standard 'atonal' operations on the twelve pitch-classes transform S into itself; these operations are T_0, T_3, T_6, T_9, I_5, I_E, I_8, and I_{A#}. The eight operations form a group on the twelve pitch-classes and therefore, mapping S into itself, induce a group of corresponding operations on S; we shall call those corresponding operations RO, R3, R6, R9, K, L, M, and N respectively" (Lewin, Appendix B, p. 251). STRANS1 is simply transitive on S: "Given members s and t of S, there is a unique OP among the eight cited operations on S, satisfying OP(s) = t" (p. 251).

# Prerequisites
- **Octatonic pitch-class set** — The space on which STRANS1 acts
- **Simply transitive group** — STRANS1 has this property

# Key Properties
1. |STRANS1| = 8
2. Simply transitive on S
3. RO, R3, R6, R9: "rotations" preserving each diminished-seventh chord
4. K, L, M, N: swap between diminished-seventh chords (induced from inversions)
5. If t is in same dim7 as s: operation is RO, R3, R6, or R9
6. If t is in opposite dim7 from s: operation is K, L, M, or N
7. K, L, M, N are "GIS1-transpositions" even though they derive from twelve-tone inversions

# Construction / Recognition
## To Construct:
1. Identify the eight standard operations preserving S
2. Restrict them to S to get the eight STRANS1 operations
## To Recognize:
1. Operations on the octatonic set that move elements within or between diminished-seventh chords

# Context & Application
"We must be careful to distinguish the operations K, L, M, and N, which are 'GIS1-transpositions' under this formalism, from the operations I_5 etc. that gave rise to them; I_5 etc. are inversion-operations in a different GIS" (p. 251). STRANS2 members are precisely those operations commuting with all STRANS1 members.

# Examples
**Example 1** (p. 251): R3 maps C to D#, D# to F#, F# to A, A to C (rotation of first dim7), and C# to E, E to G, G to A#, A# to C# (rotation of second dim7).

**Example 2** (p. 251): K (from I_5) maps elements between the two diminished-seventh chords.

# Relationships
## Builds Upon
- **Octatonic pitch-class set** — The space of action
## Enables
- **GIS1 octatonic** — GIS with STRANS1 as transpositions
## Related
- **Dual simply transitive groups** — STRANS1 and STRANS2 are duals
- **Rotation operations** — RO, R3, R6, R9 are "rotations"
## Contrasts With
- **STRANS2 group** — Different simply transitive group on the same set

# Common Errors
- **Error**: Treating K, L, M, N as inversions in the octatonic GIS
  **Correction**: In GIS1, they are transpositions, even though they derive from twelve-tone inversions

# Common Confusions
- **Confusion**: Equating octatonic R3 with chromatic T_3
  **Clarification**: "We must be careful to distinguish the octatonic GIS1-transpositions RO, R3, R6, and R9 from the dodecaphonic atonal-GIS-transpositions T_0, T_3, T_6, and T_9" (p. 251)

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 251.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined with verification criteria
- Re-extraction notes: Re-extracted from v2 card; preserved: dim7 membership criterion, K/L/M/N as transpositions, distinction from twelve-tone ops
