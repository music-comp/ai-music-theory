---
concept: Non-Commutative GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
A non-commutative GIS is a Generalized Interval System whose interval group is non-abelian (non-commutative), meaning the order of operations matters: int(s,t) composed with int(t,u) may not equal int(t,u) composed with int(s,t).

# Formal Definition
A GIS (S, IVLS, int) is non-commutative when its interval group IVLS is non-abelian, meaning there exist intervals i and j such that i * j is not equal to j * i. Non-commutative GIS structures arise naturally when the simply transitive group of transposition operations is itself non-commutative, as with STRANS1 or STRANS2 on the octatonic collection.

# Mathematical Formulation
For a GIS to be non-commutative:
- IVLS must be non-abelian
- There exist i, j in IVLS with i * j != j * i

Octatonic example:
- STRANS1 and STRANS2 are both non-abelian groups of order 8
- Both are isomorphic to D_4 (dihedral group of order 8)
- D_4 is non-abelian: rotations commute with each other, but not with reflections

Consequence for GIS:
- In non-commutative GIS, "interval from s to t" followed by "interval from t to u" depends on order
- The path through interval space matters, not just start and end points

# Musical Context/Application
Non-commutative GIS structures reveal that in certain musical spaces (like the octatonic), the concept of "interval" is path-dependent. This differs from familiar chromatic space where the twelve transpositions form a commutative (cyclic) group. Non-commutative GIS may model situations where the order of transformations produces musically distinct results.

# Examples
**Octatonic non-commutativity:**
The octatonic GIS structures (GIS1 and GIS2) are both non-commutative:
- STRANS1 = {RO, R3, R6, R9, K, L, M, N} is non-abelian
- STRANS2 = {RO, Q3, R6, Q9, X1, X2, X4, X5} is non-abelian

**D_4 structure:**
Both STRANS groups are isomorphic to D_4, the symmetries of a square:
- 4 rotations: RO, R3, R6, R9 (commute with each other)
- 4 reflections: K, L, M, N (or Q3, Q9, X1, X2, X4, X5 for STRANS2)
- Rotation * Reflection != Reflection * Rotation in general

**Contrast with chromatic:**
"If STRANS is commutative, then STRANS' will be precisely STRANS itself."

The familiar chromatic GIS with transpositions Z_12 is commutative, so its dual group equals itself. The octatonic case is interesting precisely because STRANS1 != STRANS2.

**Interval-preserving duality:**
In non-commutative GIS, the interval-preserving operations form a distinct group (the dual). In commutative GIS, interval-preserving operations are the same as transpositions.

# Related Concepts
- Commutative GIS
- STRANS1 Group
- STRANS2 Group
- Dual Simply Transitive Groups
- GIS1 and GIS2
- Dihedral Group

# Common Confusions
Non-commutativity means the order of operations matters for the interval group, not that intervals cannot be measured. The GIS structure still provides well-defined intervals; it is just that composing intervals is non-commutative. Students familiar only with chromatic Z_12 may find this counterintuitive.

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
