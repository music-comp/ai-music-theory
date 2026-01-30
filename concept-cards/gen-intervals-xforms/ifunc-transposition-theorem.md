---
concept: IFUNC Transposition Theorem
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
When one or both sets are transposed, IFUNC values shift according to specific formulas involving composition of intervals in the group IVLS.

# Formal Definition
Theorem 5.1.6: For any transposition operation T_n, the following formulas obtain:
- (A): IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(ni)
- (B): IFUNC(X, T_n(Y))(i) = IFUNC(X, Y)(in^-1)
- (C): IFUNC(T_n(X), T_n(Y))(i) = IFUNC(X, Y)(nin^-1)

# Mathematical Formulation
Proof of (A):
Let PAIRS = {(s, t) : s in X, t in Y, int(s, t) = ni}
Then |PAIRS| = IFUNC(X, Y)(ni)

Let PAIRS' = {(s', t) : s' in T_n(X), t in Y, int(s', t) = i}
Then |PAIRS'| = IFUNC(T_n(X), Y)(i)

Define f: PAIRS -> PAIRS' by f(s, t) = (T_n(s), t)
- f is well-defined because int(T_n(s), t) = n^-1 * int(s, t) = n^-1 * ni = i
- f is bijective

Therefore IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(ni). Q.E.D.

Proof of (B): Uses Theorem 5.1.4 and Formula (A).
Proof of (C): Applies (A) then (B) sequentially.

Note: In a commutative GIS using additive notation:
- IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(n + i)
- IFUNC(X, T_n(Y))(i) = IFUNC(X, Y)(i - n)
- IFUNC(T_n(X), T_n(Y))(i) = IFUNC(X, Y)(i)

# Musical Context/Application
These formulas show how transposition "shifts" the IFUNC function. In the commutative case (pitch classes), transposing X by n shifts the IFUNC argument by n. This is crucial for understanding how intervallic relationships change under transposition and why T_n(X) may or may not embed well in Y.

# Examples
With X = {C, E} and Y = {F, A, C#}:
- IFUNC(X, Y)(5) counts how many ways interval 5 spans from X to Y
- IFUNC(T_2(X), Y)(5) = IFUNC(X, Y)(5 + 2) = IFUNC(X, Y)(7)

This shows that to find interval-5 connections from T_2(X) to Y, we look at interval-7 connections from X to Y.

# Related Concepts
- IFUNC (Interval Function)
- Transposition Operations
- Group Composition
- Non-commutative GIS

# Common Confusions
The formulas involve group multiplication, not addition. In non-commutative GIS structures, nin^-1 may not equal i, so IFUNC(T_n(X), T_n(Y)) may differ from IFUNC(X, Y). Only in commutative groups does formula (C) reduce to invariance.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.1.6
