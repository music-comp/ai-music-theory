---
concept: Equivalence Class
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An equivalence class is the collection of all elements equivalent to a given element under an equivalence relation - all elements that "go together."

# Formal Definition
Given an equivalence relation EQUIV on a family S, for each s in S let E(s) be the subfamily of S comprising exactly those members of S which are in the EQUIV relation to s. Then given any s and t in S, either: (A) s and t are equivalent and E(s) = E(t) are the same collection, or (B) s and t are not equivalent and E(s) and E(t) are disjoint. The sets E(s) are called equivalence classes.

# Mathematical Formulation
- E(s) = {t in S : s ~ t} is the equivalence class containing s
- If s ~ t, then E(s) = E(t) (same class)
- If s is not equivalent to t, then E(s) and E(t) are disjoint
- S is partitioned into disjoint equivalence classes
- Every element belongs to exactly one equivalence class

# Musical Context/Application
Pitch classes are equivalence classes of pitches under octave equivalence. Set-types are equivalence classes of pitch-class sets under transposition/inversion equivalence. Beat classes are equivalence classes of time points under metric equivalence. Equivalence classes allow us to work with abstract categories rather than specific instances.

# Examples
From Example 1.9.6.1: The pitch class C is the equivalence class containing C4, C5, C3, C6, etc. - all pitches with the letter name C.

From Example 1.9.6.2: In a waltz, beat-class 1 contains all first beats, beat-class 2 contains all second beats, beat-class 3 contains all third beats.

From Example 1.9.6.3: The set-type 3-11 is the equivalence class containing {C, E, G}, {C#, F, G#}, {D, F, A}, and all other major and minor triads. It contains 24 pitch-class sets total.

Theorem 1.9.3: Two equivalence classes are either identical or disjoint - they never partially overlap.

# Related Concepts
- Equivalence Relation
- Quotient Set
- Natural Map
- Pitch Class
- Set-Type
- Beat Class

# Common Confusions
- An element belongs to exactly one equivalence class
- Different representatives can name the same class: E(C4) = E(C5) = pitch class C
- Equivalence classes partition the set completely (cover it with no overlap)
- The class is the whole collection, not just a single representative element

# Source Reference
Chapter 1: Mathematical Preliminaries, Theorem 1.9.3, Section 1.9.4
