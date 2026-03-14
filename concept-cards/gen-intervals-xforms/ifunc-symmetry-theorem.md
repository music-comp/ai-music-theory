---
concept: IFUNC Symmetry Theorem
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
When the roles of sets X and Y are exchanged in IFUNC, the resulting function is "inverted" in the sense that IFUNC(Y, X)(i) = IFUNC(X, Y)(i^-1).

# Formal Definition
Theorem 5.1.4: IFUNC(Y, X)(i) = IFUNC(X, Y)(i^-1)

This theorem establishes that swapping the order of sets in IFUNC corresponds to taking the inverse of the interval argument.

# Mathematical Formulation
Proof: IFUNC(X, Y)(i^-1) is the number of pairs (s, t) such that:
- s is in X
- t is in Y
- int(s, t) = i^-1

This equals the number of pairs (t, s) such that:
- t is in Y
- s is in X
- int(t, s) = i

And that number is precisely IFUNC(Y, X)(i). Q.E.D.

Consequence: In a commutative GIS where i^-1 = -i (using additive notation), we have:
IFUNC(Y, X)(i) = IFUNC(X, Y)(-i)

# Musical Context/Application
This theorem captures the intuition that if we can span a certain interval "from X to Y," then we can span the inverse interval "from Y to X." In pitch-class terms, if there are 3 ways to go up a perfect fifth from notes in X to notes in Y, there are 3 ways to go down a perfect fifth from notes in Y to notes in X.

# Examples
In the standard pitch-class GIS with X = {C, E} and Y = {G, B}:
- IFUNC(X, Y)(7) counts perfect fifths from X to Y: C->G (7), E->B (7), so = 2
- IFUNC(Y, X)(5) counts perfect fourths from Y to X: G->C (5), B->E (5), so = 2
- Note that 5 = -7 (mod 12), confirming the theorem

# Related Concepts
- IFUNC (Interval Function)
- Inverse Interval
- GIS Axioms
- Commutative GIS

# Common Confusions
Students sometimes expect IFUNC(X, Y) = IFUNC(Y, X), but this is only true when all intervals equal their own inverses. The theorem states the correct relationship involving inverse intervals, not equality of functions.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.1.4
