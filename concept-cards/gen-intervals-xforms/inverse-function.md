---
concept: Inverse Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
The inverse function of a 1-to-1 onto function f reverses the mapping: if f(s) = s', then f^(-1)(s') = s.

# Formal Definition
Let f be a 1-to-1 function from S onto S'. Then f^(-1), the inverse function of f, is defined as the family of pairs (s', s) within S' x S such that (s, s') is a member of f. The inverse function f^(-1) is itself a 1-to-1 function from S' onto S, and the inverse of f^(-1) is f.

# Mathematical Formulation
- If f: S -> S' is 1-to-1 and onto, then f^(-1): S' -> S exists
- f^(-1)(s') = s if and only if f(s) = s'
- f^(-1)(f(s)) = s for all s in S
- f(f^(-1)(s')) = s' for all s' in S'
- (f^(-1))^(-1) = f

# Musical Context/Application
Inverse functions are essential in music theory for "undoing" transformations. The inverse of transposition by n semitones is transposition by -n semitones. The inverse of inversion about a pitch p is inversion about the same pitch p (inversion is its own inverse). Understanding inverse operations is crucial for analyzing retrograde motion and for constructing symmetrical musical structures.

# Examples
Musical example: T5 (transpose up 5 semitones) has inverse T7 (which equals T(-5) mod 12). If T5(C) = F, then T7(F) = C.

Musical example: Inversion about C (I_C) is its own inverse: I_C(I_C(E)) = I_C(Ab) = E.

Mathematical example: If f(s) = s + 3 on integers, then f^(-1)(s') = s' - 3. f(5) = 8 and f^(-1)(8) = 5.

# Related Concepts
- Function
- One-to-One Function
- Onto Function
- Operation
- Group Inverse

# Common Confusions
- Only functions that are BOTH 1-to-1 AND onto have inverses
- The inverse function is not the same as the reciprocal (1/f)
- For composition: (fg)^(-1) = g^(-1)f^(-1) (order reverses)
- The notation f^(-1) for inverse function differs from f^(-1)(x) which means applying the inverse

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.2.6.3-1.2.6.5
