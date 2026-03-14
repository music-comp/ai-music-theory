---
concept: Center of a Group
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
The center of a group consists of all elements that commute with every other element in the group.

# Formal Definition
Given a binary composition BIN on a family X, an element c of X is central if c commutes with every x in X. The family of all central elements c is the center of the system (X, BIN).

# Mathematical Formulation
- c is central if cx = xc for all x in X
- Center(G) = {c in G : cx = xc for all x in G}
- The center is always a subgroup of G
- In a commutative group, the center equals the entire group
- The center is always non-empty (contains at least the identity)

# Musical Context/Application
In the T/I group on pitch classes, the center consists of T0 and T6 (the tritone transposition). These are the only operations that commute with all transpositions and all inversions. The center helps identify which transformations can be freely reordered in transformation networks without changing the result.

# Examples
In the T/I group:
- T0 (identity) is central: T0X = XT0 = X for any operation X
- T6 is central: T6 commutes with all transpositions (Tm) and all inversions (In)
- T3 is NOT central: T3I0 = I9 but I0T3 = I3, so T3I0 != I0T3

In a commutative group (like Z12 under addition):
- Every element is central
- The center equals the whole group

# Related Concepts
- Commutativity
- Group
- Central Element
- Subgroup
- Abelian Group

# Common Confusions
- The center is a subgroup, not just a set
- Every group has at least the identity in its center
- Non-commutative groups have proper centers (smaller than the whole group)
- The center of a commutative group is the entire group

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.8.2
