---
concept: GIS2 (Octatonic GIS with STRANS2)
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
GIS2 is a Generalized Interval System on the octatonic collection where STRANS2 operations (RO, Q3, R6, Q9, X1, X2, X4, X5) serve as the formal transposition operations, with STRANS1 operations as the interval-preserving operations.

# Formal Definition
GIS2 = (S, IVLS2, int2) is a GIS structure on the octatonic set S where:
- S is the set of eight octatonic pitch classes
- IVLS2 is the group of intervals isomorphic to STRANS2
- int2 is the interval function defined by simple transitivity of STRANS2
- Formal transpositions are the STRANS2 operations
- Interval-preserving operations are exactly the STRANS1 operations

# Mathematical Formulation
GIS2 structure:
- S = {C, C#, D#, E, F#, G, A, A#}
- IVLS2 isomorphic to STRANS2 (group of order 8)
- For s, t in S: int2(s, t) = unique OP in STRANS2 such that OP(s) = t

Transposition in GIS2:
- T_i(s) = OP_i(s) where OP_i corresponds to interval i in IVLS2
- All STRANS2 operations are "transpositions" in GIS2

Interval-preserving:
- STRANS1 operations preserve GIS2 intervals
- For all f in STRANS1, s, t in S: int2(f(s), f(t)) = int2(s, t)

# Musical Context/Application
GIS2 offers an alternative perspective on octatonic music, where the "queer" and "exchanging" operations function as transpositions. This may reveal structural relationships invisible from the GIS1 perspective. The duality with GIS1 demonstrates that the same musical space can support multiple equally valid GIS structures.

# Examples
**GIS2 construction:**
"Using the method of 7.1.1, we can develop another GIS involving the family S, a GIS for which the members of STRANS2 are exactly the formal transposition operations. We can call this structure GIS2 = (S, IVLS2, int2)."

**Transposition in GIS2:**
"In this GIS, applying any of the operations RO, Q3, R6, Q9, X1, X2, X4, or X5 to a member s of S amounts precisely to transposing s, formally, by a suitable corresponding interval of GIS2."

**Interval-preserving operations:**
"The interval-preserving operations for GIS2 are exactly the members of STRANS1; those are in fact precisely the transformations on S that commute with every member of STRANS2."

**Dual relationship:**
The duality is complete: STRANS1 transpositions preserve STRANS2 intervals, and STRANS2 transpositions preserve STRANS1 intervals.

# Related Concepts
- GIS1 (Octatonic GIS with STRANS1)
- STRANS2 Group
- STRANS1 Group
- Queer Operations
- Exchanging Operations
- Dual Simply Transitive Groups

# Common Confusions
In GIS2, the queer operations Q3 and Q9 and the exchanging operations X1, X2, X4, X5 are all transpositions. This may seem counterintuitive since they involve exchanges and opposite-direction rotations. Students should recognize that "transposition" in a GIS is a formal concept depending on the chosen group structure, not necessarily matching intuitive notions of pitch transposition.

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
