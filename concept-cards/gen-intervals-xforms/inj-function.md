---
concept: "INJ (Injection Function)"
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
INJ(X, Y)(f) counts the number of elements in set X whose images under transformation f are members of set Y.

# Formal Definition
Definition 6.2.1: Given sets X and Y, given a transformation f on S, then the injection number of X into Y for f, denoted INJ(X, Y)(f), is the number of elements s in X such that f(s) is a member of Y.

INJ answers: "If I apply transformation f to set X, how many members of X will map into members of Y?"

# Mathematical Formulation
INJ(X, Y)(f) = |{s in X : f(s) is in Y}|

Key properties:
- 0 <= INJ(X, Y)(f) <= |X|
- If f is 1-to-1: INJ(X, Y)(f) = |f(X) intersect Y|
- If f is not 1-to-1: INJ may exceed |f(X) intersect Y|

When f is an operation OP (Theorem 6.5.1):
INJ(X, Y)(OP) = card(OP(X) intersect Y)

This generalizes Regener's Common-Note Function.

# Musical Context/Application
INJ is more general than IFUNC: it handles any transformation, not just transpositions. This allows analysis of wedge transformations, non-invertible mappings, and other musically significant transformations that are not operations.

INJ can engage inversional relationships, non-standard transformations, and compositional processes that IFUNC cannot capture.

# Examples
From Schoenberg's "Angst und Hoffen" op.15 no.7:
- X = {Gb, Bb, D} (Angst chord)
- Y = {Fb, Bb, Eb} (Hoffen chord)
- w^E = wedge-to-E transformation

INJ(X, Y)(w^E) = 2:
- D wedges to Eb (in Y)
- Bb wedges to Bb (in Y)
- Gb does NOT wedge to anything in Y (would wedge to F, not Fb)

"Two-thirds of X" maps into Y by the wedge. The Fb is a "wrong note" - if it were F natural, INJ would be 3.

# Related Concepts
- IFUNC (Interval Function)
- Transformation vs Operation
- Progressive Transformation
- Internal Transformation
- Common-Note Function

# Common Confusions
INJ counts elements of X that map into Y, not the number of shared elements after transformation. When f is not 1-to-1, multiple elements of X might map to the same element of Y, so INJ can exceed the intersection size. Only when f is 1-to-1 does INJ equal the intersection cardinality.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Definition 6.2.1 and Example 6.2.3
