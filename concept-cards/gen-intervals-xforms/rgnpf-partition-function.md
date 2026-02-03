---
concept: "RGNPF (Regener Partition Function)"
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
RGNPF(X, Y)(N) counts how many canonical operations A satisfy INJ(X, Y)(A) = N, partitioning CANON by INJ values.

# Formal Definition
Definition (6.9): When CANON is finite, for each integer N between 0 and cardX inclusive, RGNPF(X, Y)(N) is the number of members A of CANON satisfying INJ(X, Y)(A) = N.

The function is named for Eric Regener, who developed related ideas for pitch-class set theory.

# Mathematical Formulation
RGNPF(X, Y): {0, 1, ..., cardX} -> non-negative integers
RGNPF(X, Y)(N) = |{A in CANON : INJ(X, Y)(A) = N}|

Properties:
- Sum over all N of RGNPF(X, Y)(N) = |CANON|
- RGNPF(X, Y)(cardX) = multiplicity of K1-ness
- RGNPF(X, Y)(0) = multiplicity of K2-ness

Formula relating RGNPF to EMB:
EMB(X, Y) = RGNPF(X, Y)(cardX) / RGNPF(X, X)(cardX)

Explanation: RGNPF counts operations; EMB counts forms. If X has symmetry (M operations map X to itself), then M times as many operations embed X in Y as there are forms of X in Y.

# Musical Context/Application
RGNPF provides a complete picture of how INJ(X, Y) varies over CANON. Beyond just asking "can X embed in Y?" (K1), we can ask "how many ways can X embed in Y?" (RGNPF at maximum) and examine the full distribution of INJ values.

# Examples
For X = major triad, Y = major scale, CANON = transpositions:
- RGNPF(X, Y)(3) = 3 (three transpositions embed X fully in Y)
- RGNPF(X, Y)(2) = some number (transpositions with 2 common tones)
- RGNPF(X, Y)(1) = some number
- RGNPF(X, Y)(0) = some number (dispersive transpositions)

Sum = 12 (all transpositions accounted for)

For symmetric sets:
- X = augmented triad = {C, E, G#}
- RGNPF(X, X)(3) = 4 (T_0, T_4, T_8 map X to itself, plus identity)
- EMB(X, X) = 1 (only one form of X embedded in X - itself)
- 4 = 1 * 4 confirms the formula

# Related Concepts
- INJ (Injection Function)
- EMB (Embedding Function)
- K and Kh Relations
- Canonical Group

# Common Confusions
RGNPF counts operations, not forms. A symmetric set has fewer forms than operations mapping it to itself. The formula EMB = RGNPF(max)/RGNPF(self) corrects for this overcounting due to symmetry.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.9
