---
concept: Transformation vs Operation
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
A transformation is any mapping from S to itself; an operation is a transformation that is both 1-to-1 (injective) and onto (surjective), hence invertible.

# Formal Definition
Convention 6.1:
- Transformation f: S -> S maps each element of S to some element of S
- f need not be 1-to-1: distinct elements may map to the same image
- f need not be onto: some elements may not be images of anything

Operation OP: S -> S is a transformation that is:
- 1-to-1 (injective): OP(s) = OP(t) implies s = t
- Onto (surjective): every element of S is OP(s) for some s
- Therefore invertible: OP^-1 exists and is also an operation

Operations form groups; transformations that are not operations cannot belong to groups.

# Mathematical Formulation
For transformation f:
- May have f(s) = f(t) with s != t (not 1-to-1)
- May have elements y with no x satisfying f(x) = y (not onto)
- No inverse f^-1 exists in general

Consequences for INJ:
- If f is an operation: INJ(X, Y)(f) = |f(X) intersect Y|
- If f is not an operation: INJ(X, Y)(f) may exceed |f(X) intersect Y|

Example: f maps all white keys to C, all black keys to F#
- f is not 1-to-1 (many keys map to C)
- f is not onto (most pitch classes are not images)
- INJ(X, Y)(f) can be large even if f(X) intersect Y is small

# Musical Context/Application
The distinction is crucial for INJ theory:
- IFUNC only involves transpositions (operations)
- INJ handles wedges, projections, and other non-operations
- Many musically significant transformations (wedges, contractions, registral projections) are not operations

This generality is why Lewin develops INJ as the "master function" for set theory.

# Examples
Wedge w^E is not an operation:
- Not 1-to-1: w^E(E) = w^E(F) = E
- Not onto: No pitch class maps to F under w^E

Transposition T_5 is an operation:
- 1-to-1: T_5(s) = T_5(t) implies s = t
- Onto: Every pitch class is T_5(something)
- Inverse: T_5^-1 = T_7

Inversion I_0 is an operation:
- 1-to-1 and onto
- Inverse: I_0^-1 = I_0 (self-inverse)

Projection "map every pitch to its pitch class" is not an operation on pitches:
- Not 1-to-1: C4 and C5 both map to pitch class C

# Related Concepts
- INJ (Injection Function)
- IFUNC (Interval Function)
- Group of Operations
- Wedge Transformation

# Common Confusions
Operations can form groups and have inverses; transformations in general cannot. When f is not an operation, theorems that depend on bijectivity (like 6.5.1) require modification or don't apply. The greater generality of INJ (handling all transformations) comes with corresponding care in applying theorems.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Convention 6.1 and throughout
