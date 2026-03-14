---
concept: Quotient Set
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
The quotient set (or quotient family) of S modulo an equivalence relation is the set of all equivalence classes - it "collapses" equivalent elements into single representatives.

# Formal Definition
Given an equivalence relation EQUIV on a family S, the family of equivalence classes is called the quotient family of S modulo EQUIV, denoted symbolically by S/EQUIV. The function E that maps each argument s to the value E(s) (the equivalence class containing s) is called the natural map of S onto S/EQUIV.

# Mathematical Formulation
- S/EQUIV = {E(s) : s in S} is the quotient set
- Elements of S/EQUIV are equivalence classes, not elements of S
- Natural map E: S -> S/EQUIV defined by E(s) = the class containing s
- E is onto S/EQUIV by definition
- |S/EQUIV| <= |S| (equality when each class has one element)

# Musical Context/Application
The twelve pitch classes form the quotient of all pitches modulo octave equivalence. Set-types form the quotient of pitch-class sets modulo transposition/inversion. Quotient structures allow us to work at higher levels of abstraction, treating all octave-equivalent pitches as "the same" pitch class, or all transpositionally equivalent sets as "the same" set-type.

# Examples
Example 1.9.6.1: S = all pitches, EQUIV = octave equivalence. S/EQUIV = the 12 pitch classes. The natural map E takes any pitch (like C4 or C5) to its pitch class (C).

Example 1.9.6.2: S = all beats in a waltz, EQUIV = same position in measure. S/EQUIV = the 3 beat classes {1, 2, 3}. The natural map E takes any beat to its beat class.

Example 1.10.4.1: S = all integers, EQUIV = congruence mod 12. S/EQUIV = the 12 congruence classes C(0), C(1), ..., C(11), which model the 12 pitch classes.

# Related Concepts
- Equivalence Relation
- Equivalence Class
- Natural Map
- Congruence
- Quotient Group
- Homomorphism

# Common Confusions
- Elements of S/EQUIV are classes, not elements of S
- The quotient "reduces" S by collapsing equivalent elements
- The natural map E is always onto S/EQUIV
- S/EQUIV has fewer (or equal) elements than S

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.9.5, Section 1.9.7
