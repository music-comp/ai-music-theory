---
concept: Commutativity
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
Elements x and y commute if xy = yx; a group or binary composition is commutative (abelian) if all pairs of elements commute.

# Formal Definition
Given a binary composition BIN on a family X, elements x and y commute if BIN(y, x) = BIN(x, y), that is, if yx = xy in multiplicative notation. The composition BIN is commutative if every pair of elements commutes. A semigroup or group is commutative if its binary composition is commutative.

# Mathematical Formulation
- x and y commute if xy = yx
- A group (G, *) is commutative (abelian) if a * b = b * a for all a, b in G
- The center of a group is the set of elements that commute with everything
- Non-commutative groups require attention to order of operations

# Musical Context/Application
The T/I group on pitch classes is non-commutative - the order of transposition and inversion matters. However, the group of transpositions alone (T0 through T11) is commutative: TmTn = TnTm = Tm+n. Understanding which operations commute helps in analyzing transformation networks and simplifying chains of operations.

# Examples
From Chapter 1: The group of transposition and inversion operations on the twelve pitch classes is non-commutative. Let T2 be transposing-by-2, I be inverting-about-C, J be inverting-about-B, and K be inverting-about-C#.
- IT2 = J: Given any pitch class s, inverting about C after transposing by 2 yields inversion about B.
- T2I = K: Transposing by 2 after inverting about C yields inversion about C#.
- IT2 != T2I, so T2 and I do not commute.

Commutative example: T3T5 = T5T3 = T8 (transpositions commute with each other).

# Related Concepts
- Group
- Binary Composition
- Center of a Group
- Central Element
- Non-Commutative Group

# Common Confusions
- Commutativity is about order (xy vs yx), not grouping (that's associativity)
- A group can be associative without being commutative
- All finite groups have some commuting pairs, but non-commutative groups have non-commuting pairs too
- "Abelian" and "commutative" mean the same thing for groups

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.8.1-1.8.2
