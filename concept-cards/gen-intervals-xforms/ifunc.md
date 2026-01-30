---
concept: IFUNC (Interval Function)
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
IFUNC(X, Y) is a function that counts, for each interval i in IVLS, the number of ways that interval can be spanned between members of set X and members of set Y.

# Formal Definition
Definition 5.1.3: Given a GIS, given sets X and Y, the X/Y interval function IFUNC(X, Y) maps the group IVLS into the non-negative integers as follows: For each interval i in IVLS, IFUNC(X, Y)(i) counts the number of distinct pairs (s, t) in S x S such that s is in X, t is in Y, and int(s, t) = i.

IFUNC(X, Y)(i) tells us in how many different ways the interval i can be spanned between (members of) X and (members of) Y.

# Mathematical Formulation
IFUNC(X, Y): IVLS -> {0, 1, 2, 3, ...}

For each i in IVLS:
IFUNC(X, Y)(i) = |{(s, t) : s in X, t in Y, int(s, t) = i}|

Key properties:
- IFUNC(Y, X)(i) = IFUNC(X, Y)(i^-1) (Theorem 5.1.4)
- IFUNC(P(X), P(Y)) = IFUNC(X, Y) for any interval-preserving operation P (Theorem 5.1.5)
- IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(ni) (Theorem 5.1.6A)
- IFUNC(X, T_n(Y))(i) = IFUNC(X, Y)(in^-1) (Theorem 5.1.6B)

# Musical Context/Application
IFUNC generalizes and extends the interval vector from traditional set theory. While Forte's interval vector counts interval classes within a single set, IFUNC can measure intervallic relationships between two different sets, capturing melodic progressions, harmonic successions, and contrapuntal relationships.

# Examples
From Webern's op.7, no.3 analysis (Figure 5.2-5.3):
- X = {Ab, Bb, Eb} (melodic phrase 1)
- Y = 7-note set (melodic phrase 2)
- IFUNC(X, Y) tabulated shows:
  - IFUNC(X, Y)(3) = 3 (maximum), indicating T_3(X) embeds in Y
  - IFUNC(X, Y)(8) = 3 (also maximum), indicating T_8(X) embeds in Y

From Figure 5.1: X1 = {E, Bb}, Y1 = {F, A, C#}
- IFUNC(X1, Y1)(i) = 0 if i is even, = 1 if i is odd
- Multiple different pairs (X, Y) can produce identical IFUNC values

# Related Concepts
- Interval Vector (Forte)
- INJ (Injection Function)
- Common-Note Function (Regener)
- Set Class
- Transposition Operations

# Common Confusions
IFUNC is not the same as Forte's interval vector. The interval vector counts intervals within a single set (essentially IFUNC(X, X)), while IFUNC measures intervals between two potentially different sets. Also, IFUNC returns a function on all of IVLS, not just interval classes; in non-commutative GIS structures, this distinction is crucial.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.1.3 and Figures 5.1-5.8
