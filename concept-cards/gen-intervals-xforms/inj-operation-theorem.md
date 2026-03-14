---
concept: INJ Operation Theorem
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
When f is an operation (1-to-1 and onto), INJ(X, Y)(f) equals the cardinality of the intersection of f(X) with Y.

# Formal Definition
Theorem 6.5.1: If f is an operation OP, then INJ(X, Y)(OP) is the cardinality of OP(X) intersect Y, that is the number of common members shared by the sets OP(X) and Y.

This connects INJ to the Common-Note Function developed by Regener.

# Mathematical Formulation
For operation OP:
INJ(X, Y)(OP) = |OP(X) intersect Y|

Proof: Let M = INJ(X, Y)(OP), the number of elements x in X such that OP(x) is in Y.
Let N = |OP(X) intersect Y|.

Since OP is 1-to-1:
- The M elements x1, ..., xM map to M distinct elements OP(x1), ..., OP(xM)
- These are exactly the elements of OP(X) intersect Y
- Therefore M = N. Q.E.D.

Corollary 6.5.2: If OP is an operation, then
INJ(Y, X)(OP) = INJ(X, Y)(OP^-1)

# Musical Context/Application
This theorem shows that for operations, INJ counts common tones between a transformed set and a target set. This is the fundamental connection to Regener's Common-Note Function, which counts common tones between T_i(X) and Y for pitch-class sets.

When f is not an operation, INJ can exceed the intersection cardinality (multiple elements mapping to the same image).

# Examples
For transposition operation T_5 on pitch classes:
- X = {C, E, G}, Y = {C, F, A}
- T_5(X) = {F, A, C}
- T_5(X) intersect Y = {F, A, C} intersect {C, F, A} = {C, F, A}
- INJ(X, Y)(T_5) = 3

For inversion operation I_0:
- X = {C, E, G} = {0, 4, 7}
- I_0(X) = {0, 8, 5} = {C, Ab, F}
- INJ(X, Y)(I_0) = |{C, Ab, F} intersect {C, F, A}| = 2

# Related Concepts
- INJ (Injection Function)
- Common-Note Function (Regener)
- Operations vs Transformations
- Set Intersection

# Common Confusions
This theorem applies only when f is an operation (bijection on S). For non-operations like wedge transformations, INJ can exceed the intersection size because multiple source elements may map to the same target element.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Theorem 6.5.1 and Corollary 6.5.2
