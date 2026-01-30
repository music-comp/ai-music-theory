---
concept: IFUNC Inversion Theorem
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
In a commutative GIS, when both sets are transformed by the same inversion operation, IFUNC exchanges the roles of the two sets.

# Formal Definition
Theorem 5.1.7: If I is any inversion operation in a commutative GIS, then IFUNC(I(X), I(Y)) = IFUNC(Y, X).

Note: This theorem specifically requires a commutative GIS. In non-commutative GIS structures, the relationship between IFUNC and inversion is more complex and is better handled through the Injection Function (Chapter 6).

# Mathematical Formulation
Proof sketch: In a commutative GIS, inversions are interval-reversing operations (Theorem 3.6.3). That is:
int(I(s), I(t)) = int(s, t)^-1 = int(t, s)

Using techniques similar to Theorem 5.1.5:
- The pairs (s, t) with s in X, t in Y, int(s, t) = i
- Correspond bijectively to pairs (I(s), I(t)) with I(s) in I(X), I(t) in I(Y), int(I(s), I(t)) = i^-1

This gives: IFUNC(I(X), I(Y))(i) = IFUNC(X, Y)(i^-1) = IFUNC(Y, X)(i)

The last equality uses Theorem 5.1.4.

# Musical Context/Application
This theorem explains why inversion "flips" intervallic relationships. If X tends to precede Y by ascending intervals, then I(X) will tend to precede I(Y) by descending intervals - which is equivalent to I(Y) preceding I(X) by ascending intervals.

# Examples
In pitch-class space with X = {C, E} and Y = {G, B}:
- IFUNC(X, Y)(7) = 2 (two perfect fifths from X to Y)
- Apply I_0 (inversion around C): I_0(X) = {C, Ab}, I_0(Y) = {F, Eb}
- IFUNC(Y, X)(7) = IFUNC(X, Y)(5) = 2 (two perfect fourths from Y to X)
- IFUNC(I_0(X), I_0(Y))(7) = IFUNC(Y, X)(7) = 2

The inversion swaps which set is "source" and which is "target" in terms of intervallic direction.

# Related Concepts
- IFUNC (Interval Function)
- Inversion Operations
- Commutative GIS
- INJ (Injection Function)
- Interval-Reversing Operations

# Common Confusions
This theorem only holds in commutative GIS structures. For general questions about IFUNC and inversion in non-commutative settings, or for more refined analysis, the Injection Function (INJ) from Chapter 6 is more appropriate.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.1.7
