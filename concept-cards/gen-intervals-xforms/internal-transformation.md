---
concept: Internal Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
An internal transformation is one for which INJ(X, X)(f) or INJ(Y, Y)(f) is high - it transforms a set into something much like itself.

# Formal Definition
Definition (6.4): Given sets X and Y, and a family INSPECT of transformations:
- f is X-internal if INJ(X, X)(f) is high
- f is Y-internal if INJ(Y, Y)(f) is high
- f is internal (for the progression X-Y) if it is both X-internal and Y-internal

Internal transformations tend to "extend/elaborate/develop/prolong" a set rather than transform it into something else.

# Mathematical Formulation
f is X-internal if:
INJ(X, X)(f) is near or at max{INJ(X, X)(g) : g in INSPECT}

Algebraic tendencies:
- Composition of two X-internal transformations tends to be X-internal
- Inverse of an X-internal operation tends to be X-internal
- These follow because operations that keep X "like itself" compose to keep X "like itself"

# Musical Context/Application
Internal transformations model symmetry and self-reference within a harmony. When a chord has high internal transformation values, it exhibits structural regularities that the transformation reveals. Operations like inversion often serve as internal transformations for symmetric chords.

# Examples
From "Angst und Hoffen":
- I = I_E^Bb (inversion about E/Bb axis)
- Angst chord X = {Gb, Bb, D}
- Hoffen chord Y = {Fb, Bb, Eb}

INJ(X, X)(I) = 3: All of X maps into X under I
- Gb <-> D (swap under I)
- Bb -> Bb (fixed point)

INJ(Y, Y)(I) = 2: Most of Y maps into Y
- Fb is "bereft of its I-partner" (would be F)
- Bb fixed, Eb <-> ... (incomplete symmetry)

The missing F breaks the inversional symmetry of Y.

From melodic analysis (Figure 6.4):
- (1, I) and (2, w) are internal for tetrad X_1^4
- (2, I) and (3, w) are internal for tetrad X_5^8

# Related Concepts
- Progressive Transformation
- External Transformation
- Dispersive Transformation
- INJ (Injection Function)
- Inversional Symmetry

# Common Confusions
Internal does not mean "trivial" or "identity." A transformation can significantly rearrange elements of X while still mapping X into itself. The identity is maximally internal, but non-identity transformations can also be highly internal for symmetric sets.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.4
