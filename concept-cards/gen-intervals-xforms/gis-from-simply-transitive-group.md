---
concept: GIS from Simply Transitive Group
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
unit: null
authors: David Lewin
---

# Quick Definition
Any simply transitive group of operations on a set can be used to construct a GIS, where the group operations become the formal intervals and the interval function is defined by which operation maps one element to another.

# Formal Definition
Given S and STRANS (a simply transitive group on S):
1. Let IVLS be an "index family" in 1-to-1 correspondence with STRANS
2. Write "OP_i" for the operation corresponding to index i
3. Define binary combination ij = k in IVLS when (OP_i)(OP_j) = OP_k in STRANS
4. Define int(r, s) = i where OP_i(r) = s

Then (S, IVLS, int) is a GIS, and STRANS is its group of transpositions.

# Mathematical Formulation
Construction:
- IVLS is anti-isomorphic to STRANS as a group
- For r, s in S: int(r, s) = that unique i in IVLS such that OP_i(r) = s
- Condition (A): int(r, t) = int(r, s) int(s, t) [follows from group structure]
- Condition (B): Given s and i, unique t exists with int(s, t) = i [follows from simple transitivity]
- T_i = OP_i for every i in IVLS

# Musical Context/Application
This theorem shows that GIS structure and simply transitive group structure are formally equivalent. Any situation with a simply transitive group of transformations can be recast as interval-based, and vice versa. This equivalence enables us to choose whichever perspective is more musically illuminating for a given analytical situation.

# Examples
The twelve pitch-class transpositions form a simply transitive group on pitch classes. This generates the familiar GIS where:
- IVLS = Z_12 (integers mod 12)
- int(C, G) = 7 corresponds to T_7
- The interval 7 and the operation T_7 are two perspectives on the same phenomenon

# Related Concepts
- Simply Transitive Group
- GIS Structure
- Intervals as Transpositions
- Transposition Operations
- Anti-isomorphism

# Common Confusions
- The group IVLS is anti-isomorphic (not isomorphic) to STRANS because of how composition order works
- This construction works for any simply transitive group, not just commutative ones
- The interval function int is defined by the transformation, not measured independently

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.1.1, Theorem proof
