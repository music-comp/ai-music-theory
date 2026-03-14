---
concept: One-to-One Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A one-to-one (1-to-1) function is a function where no two distinct arguments share the same value; each output comes from exactly one input.

# Formal Definition
The function f from S into S' is 1-to-1 if no two distinct arguments share the same value. Equivalently, no member of S' appears more than once in the right-hand column of the function table.

# Mathematical Formulation
- f: S -> S' is 1-to-1 (injective) if:
  - For all s1, s2 in S: f(s1) = f(s2) implies s1 = s2
  - Equivalently: s1 != s2 implies f(s1) != f(s2)
- A 1-to-1 function preserves distinctness of elements

# Musical Context/Application
In music theory, 1-to-1 functions are crucial for transformations that preserve the distinctness of musical elements. Transposition and inversion on pitch classes are both 1-to-1: different pitch classes always map to different pitch classes. This property ensures that no musical information is "collapsed" or lost in the transformation.

# Examples
Musical example: Transposition T2 on pitch classes is 1-to-1. T2(C) = D, T2(C#) = D#, T2(D) = E, etc. No two pitch classes map to the same result.

Non-example: A function mapping all pitches to their pitch class is NOT 1-to-1, because C4 and C5 both map to pitch class C.

Mathematical example: f(s) = 2s on positive integers is 1-to-1 (distinct inputs give distinct outputs). But g(s) = s^2 on all integers is not 1-to-1 (both 2 and -2 map to 4).

# Related Concepts
- Function
- Onto Function
- Operation
- Inverse Function
- Isomorphism

# Common Confusions
- 1-to-1 is about inputs mapping to distinct outputs, not about covering all possible outputs (that's "onto")
- A function can be 1-to-1 without being onto, or onto without being 1-to-1
- The inverse function only exists when f is both 1-to-1 AND onto

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.2.6.2
