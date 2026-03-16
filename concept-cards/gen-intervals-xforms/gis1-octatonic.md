---
# === CORE IDENTIFICATION ===
concept: "GIS1 (Octatonic GIS with STRANS1)"
slug: gis1-octatonic

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
  - GIS1

# === TYPED RELATIONSHIPS ===
prerequisites:
  - strans1-group
  - gis-from-simply-transitive-group
extends:
  - generalized-interval-system
related:
  - gis2-octatonic
  - interval-preserving-operations-octatonic
contrasts_with:
  - gis2-octatonic

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is GIS1 for the octatonic collection?"
  - "How do twelve-tone inversions become octatonic transpositions?"
---

# Quick Definition
GIS1 = (S, IVLS1, int1) is a Generalized Interval System on the octatonic collection where STRANS1 operations serve as the formal transposition operations and STRANS2 operations are the interval-preserving operations.

# Core Definition
"Using the method discussed in 7.1.1, we can develop a GIS structure for S in which the members of STRANS1 are exactly the formal transposition operations. We can call this structure GIS1 = (S, IVLS1, int1). In GIS1, then, applying any one of the operations RO, R3, R6, R9, K, L, M, or N to a member s of S amounts formally precisely to 'transposing' the given s by a suitable corresponding interval of IVLS1" (Lewin, Appendix B, p. 251). The interval-preserving operations are exactly the STRANS2 members.

# Prerequisites
- **STRANS1 group** — Provides the transposition operations
- **GIS from simply transitive group** — The construction method (Theorem 7.1.1)

# Key Properties
1. GIS1 = (S, IVLS1, int1)
2. Formal transpositions: STRANS1 = {RO, R3, R6, R9, K, L, M, N}
3. Interval-preserving operations: STRANS2
4. K, L, M, N are transpositions in GIS1 (not inversions!)
5. IVLS1 is anti-isomorphic to STRANS1 (group of order 8)
6. Every STRANS2 member commutes with every STRANS1 member

# Construction / Recognition
## To Construct:
1. Apply Theorem 7.1.1 to S and STRANS1
2. Create IVLS1 in bijection with STRANS1
3. Define int1(s, t) = unique member of STRANS1 mapping s to t
## To Recognize:
1. Intervals measured by STRANS1 operations
2. K, L, M, N treated as transpositions

# Context & Application
GIS1 captures familiar T/I relationships restricted to octatonic space, but reframes inversions as transpositions. "We must be careful to distinguish the operations K, L, M, and N, which are 'GIS1-transpositions' under this formalism, from the operations I_5 etc. that gave rise to them; I_5 etc. are inversion-operations in a different GIS, a GIS involving a different family of (twelve not eight) objects, a different group of (twelve not eight) formal intervals, and a different function int" (p. 251).

# Examples
**Example 1** (p. 251): In GIS1, applying K to C maps it to a specific element; this is a "transposition" by the interval corresponding to K, not an "inversion."

# Relationships
## Builds Upon
- **STRANS1 group** — Transpositions of GIS1
- **GIS from simply transitive group** — Construction method
## Related
- **Interval-preserving operations (octatonic)** — STRANS2 in GIS1
## Contrasts With
- **GIS2 octatonic** — Different GIS on the same set; transpositions/interval-preserving roles swap

# Common Errors
- **Error**: Calling K, L, M, N "inversions" in GIS1
  **Correction**: They are transpositions in GIS1; "inversion" belongs to the twelve-tone GIS

# Common Confusions
- **Confusion**: Thinking GIS1 is the only possible GIS on the octatonic set
  **Clarification**: GIS2 provides an equally valid alternative structure

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 251.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: K/L/M/N as transpositions, distinction from twelve-tone GIS
