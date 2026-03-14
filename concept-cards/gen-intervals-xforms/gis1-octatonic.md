---
concept: "GIS1 (Octatonic GIS with STRANS1)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
GIS1 is a Generalized Interval System on the octatonic collection where STRANS1 operations (RO, R3, R6, R9, K, L, M, N) serve as the formal transposition operations, with STRANS2 operations as the interval-preserving operations.

# Formal Definition
GIS1 = (S, IVLS1, int1) is a GIS structure on the octatonic set S where:
- S is the set of eight octatonic pitch classes
- IVLS1 is the group of intervals isomorphic to STRANS1
- int1 is the interval function defined by simple transitivity of STRANS1
- Formal transpositions are the STRANS1 operations
- Interval-preserving operations are exactly the STRANS2 operations

# Mathematical Formulation
GIS1 structure:
- S = {C, C#, D#, E, F#, G, A, A#}
- IVLS1 isomorphic to STRANS1 (group of order 8)
- For s, t in S: int1(s, t) = unique OP in STRANS1 such that OP(s) = t

Transposition in GIS1:
- T_i(s) = OP_i(s) where OP_i corresponds to interval i in IVLS1
- All STRANS1 operations are "transpositions" in GIS1

Interval-preserving:
- STRANS2 operations preserve GIS1 intervals
- For all f in STRANS2, s, t in S: int1(f(s), f(t)) = int1(s, t)

# Musical Context/Application
GIS1 captures the familiar T/I relationships restricted to octatonic space. When analyzing octatonic music, GIS1 intervals correspond to the operations that would typically be discussed using transposition and inversion language, though in GIS1 all eight operations function as transpositions.

# Examples
**GIS1 construction:**
"Using the method discussed in 7.1.1, we can develop a GIS structure for S in which the members of STRANS1 are exactly the formal transposition operations. We can call this structure GIS1 = (S, IVLS1, int1)."

**Transposition in GIS1:**
"In GIS1, then, applying any one of the operations RO, R3, R6, R9, K, L, M, or N to a member s of S amounts formally precisely to 'transposing' the given s by a suitable corresponding interval of IVLS1."

**Distinction from twelve-tone:**
"We must be careful to distinguish the operations K, L, M, and N, which are 'GIS1-transpositions' under this formalism, from the operations I_5 etc. that gave rise to them; I_5 etc. are inversion-operations in a different GIS, a GIS involving a different family of (twelve not eight) objects, a different group of (twelve not eight) formal intervals, and a different function int."

**Interval-preserving operations:**
"As it turns out, the members of STRANS2 are exactly the interval-preserving operations for GIS1. Every member of STRANS2 commutes with every member of STRANS1."

# Related Concepts
- GIS2 (Octatonic GIS with STRANS2)
- STRANS1 Group
- STRANS2 Group
- Octatonic Pitch-Class Set
- Simply Transitive Groups
- Interval-Preserving Operations

# Common Confusions
In GIS1, operations like K, L, M, N are transpositions, not inversions. This differs from their origin as twelve-tone inversions. Students must think of GIS1 as a distinct system where "transposition" means something specific to the GIS structure, not necessarily matching twelve-tone terminology.

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
