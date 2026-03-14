---
concept: IFUNC Invariance Under Interval-Preserving Operations
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
When both sets X and Y are transformed by the same interval-preserving operation P, their IFUNC remains unchanged.

# Formal Definition
Theorem 5.1.5: Let P be any interval-preserving operation. Then IFUNC(P(X), P(Y)) = IFUNC(X, Y) as a function on IVLS.

An interval-preserving operation P satisfies: int(P(s), P(t)) = int(s, t) for all s, t in S.

# Mathematical Formulation
Proof outline:
Let PAIRS = {(s, t) : s in X, t in Y, int(s, t) = i}
Let PAIRS' = {(s', t') : s' in P(X), t' in P(Y), int(s', t') = i}

IFUNC(X, Y)(i) = |PAIRS|
IFUNC(P(X), P(Y))(i) = |PAIRS'|

Define f: PAIRS -> PAIRS' by f(s, t) = (P(s), P(t))
- f is well-defined: P(s) in P(X), P(t) in P(Y), int(P(s), P(t)) = int(s, t) = i
- f is 1-to-1: If (P(s1), P(t1)) = (P(s2), P(t2)), then s1 = s2 and t1 = t2
- f is onto: For any (s', t') in PAIRS', set s = P^-1(s'), t = P^-1(t')

Therefore |PAIRS| = |PAIRS'|. Q.E.D.

# Musical Context/Application
This theorem justifies why interval-preserving operations are "canonical" - they preserve the intervallic structure of sets. In a commutative GIS, the interval-preserving operations are exactly the transpositions. This means that transposing both sets by the same amount preserves all IFUNC values.

# Examples
In pitch-class space with X = {C, E, G} and Y = {D, F#, A}:
- IFUNC(X, Y)(2) = 3 (three ways to span interval 2)
- Apply T_5 to both: T_5(X) = {F, A, C}, T_5(Y) = {G, B, D}
- IFUNC(T_5(X), T_5(Y))(2) = 3 (same count)

This invariance is why we consider T_5(X) and X to be "the same" in terms of internal intervallic structure.

# Related Concepts
- Interval-Preserving Operations
- IFUNC (Interval Function)
- Transposition Operations
- Canonical Group
- Set Class Equivalence

# Common Confusions
This theorem applies when both X and Y are transformed by the same operation P. Transforming only one set will generally change IFUNC values (as described in Theorem 5.1.6).

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.1.5
