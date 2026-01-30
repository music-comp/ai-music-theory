---
concept: Equivalence Relation
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An equivalence relation on a set S is a relation that is reflexive (every element relates to itself), symmetric (if s relates to t then t relates to s), and transitive (if r relates to s and s relates to t, then r relates to t).

# Formal Definition
Given a family S, an equivalence relation on S is a subfamily EQUIV of S x S that satisfies three conditions:
(A) Reflexive: For every s in S, (s, s) is in EQUIV
(B) Symmetric: If (s, t) is in EQUIV, then so is (t, s)
(C) Transitive: If (r, s) and (s, t) are in EQUIV, then so is (r, t)

# Mathematical Formulation
- Notation: s ~ t means (s, t) is in the equivalence relation
- Reflexive: s ~ s for all s
- Symmetric: s ~ t implies t ~ s
- Transitive: r ~ s and s ~ t implies r ~ t
- Every function f: S -> S' induces an equivalence relation: s ~ t iff f(s) = f(t)

# Musical Context/Application
Equivalence relations partition musical spaces into meaningful categories. Octave equivalence groups all C pitches together into pitch class C. Set-type equivalence groups all transpositions and inversions of a pitch-class set together. Understanding equivalence relations is fundamental to pitch-class theory, set theory, and quotient spaces in transformation theory.

# Examples
Example 1.9.6.1: Let S be all pitches under equal temperament. Define (s, t) in EQUIV if s and t have the same letter name (modulo enharmonic equivalence). The quotient S/EQUIV comprises the twelve pitch classes.

Example 1.9.6.2: Let S be all beats in a waltz. Define s ~ t if they share the same beat number (1, 2, or 3). The equivalence classes are the three "beat classes."

Example 1.9.6.3: Let S be all pitch-class sets. Define s ~ t if t is a transposed or inverted form of s. The equivalence classes are Forte's set-types (e.g., 3-11 contains all major and minor triads).

# Related Concepts
- Equivalence Class
- Quotient Set
- Natural Map
- Congruence
- Partition

# Common Confusions
- All three properties (reflexive, symmetric, transitive) are required
- An equivalence relation partitions S into disjoint equivalence classes
- Two elements are equivalent iff they belong to the same equivalence class
- Not all relations are equivalence relations (e.g., "less than" is not symmetric)

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.9.1, Examples 1.9.6.1-1.9.6.3
