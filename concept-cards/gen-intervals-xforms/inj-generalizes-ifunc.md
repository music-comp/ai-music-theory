---
concept: INJ Generalizes IFUNC Theorem
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
In any GIS, IFUNC can be expressed as a special case of INJ: IFUNC(X, Y)(i) = INJ(X, Y)(T_i).

# Formal Definition
Theorem 6.7.1: Let (S, IVLS, int) be a GIS. Then for each interval i and for all sets X and Y,
IFUNC(X, Y)(i) = INJ(X, Y)(T_i)

This shows that INJ is the more fundamental concept, with IFUNC being the special case when the transformation is a transposition.

# Mathematical Formulation
Proof: Let IFUNC(X, Y)(i) = M and INJ(X, Y)(T_i) = N.

Claim: N >= M.
Since IFUNC(X, Y)(i) = M, there are M distinct pairs (x1, y1), ..., (xM, yM) with:
- xm in X, ym in Y
- int(xm, ym) = i, hence ym = T_i(xm)

The xm are distinct (else the pairs wouldn't be). So X has at least M elements whose T_i-images are in Y. Thus N >= M.

Claim: M >= N.
Let z1, ..., zN be the N distinct elements of X mapping into Y under T_i.
Each (zn, T_i(zn)) was counted among the pairs (xm, ym).
So M >= N.

Therefore M = N. Q.E.D.

# Musical Context/Application
This theorem justifies the claim that INJ is the "master function" for generalized set theory. IFUNC, developed in Chapter 5, is powerful but limited to GIS contexts. INJ works in any context with transformations, including non-GIS settings and non-operation transformations.

# Examples
Geometric visualization:
- Imagine X and Y as point configurations in a plane
- Let i be "move right 5 inches at 30 degrees"
- IFUNC(X, Y)(i): How many distinct arrows of that vector go from X-points to Y-points?
- INJ(X, Y)(T_i): Move all of X by that vector; how many points coincide with Y?

These are the same question phrased differently.

In pitch-class space:
- IFUNC({C, E}, {G, B})(7) = 2 (two perfect fifths from first dyad to second)
- INJ({C, E}, {G, B})(T_7) = 2 (same: two elements of first dyad map to second)

# Related Concepts
- INJ (Injection Function)
- IFUNC (Interval Function)
- Transposition Operations
- GIS (Generalized Interval System)

# Common Confusions
This theorem only equates IFUNC with INJ for transposition operations. INJ can handle any transformation, including non-transpositions and non-operations. IFUNC cannot engage wedge transformations, inversions directly, or non-GIS contexts.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Theorem 6.7.1
