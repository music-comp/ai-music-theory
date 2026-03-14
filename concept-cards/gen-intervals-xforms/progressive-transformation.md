---
concept: Progressive Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
A progressive transformation is one for which INJ(X, Y)(f) is maximal or relatively high - it transforms X into something much like Y.

# Formal Definition
Definition (6.4): Given sets X and Y, and a family INSPECT of transformations, a transformation f in INSPECT is called progressive (for the X-to-Y progression) if INJ(X, Y)(f) is maximal or at least relatively high compared to other transformations in INSPECT.

Progressive transformations "push X toward Y" - they map a lot of X into Y.

# Mathematical Formulation
f is progressive for X -> Y if:
INJ(X, Y)(f) is near or at max{INJ(X, Y)(g) : g in INSPECT}

Algebraic tendency:
- (X-internal) followed by (X-Y-progressive) tends to be X-Y-progressive
- (X-Y-progressive) followed by (Y-internal) tends to be X-Y-progressive

This follows from the nature of composition: if f maps much of X into X, and g maps much of X into Y, then gf maps much of X into Y.

# Musical Context/Application
Progressive transformations model the "forward motion" from one chord to another. They capture the sense that one harmony "becomes" or "leads to" another. In a progression X -> Y, progressive transformations are those that most directly connect the two.

Contrast with internal transformations, which keep X "being itself" or Y "being itself."

# Examples
From "Angst und Hoffen":
- w^E (wedge-to-E) is progressive for Angst -> Hoffen
  - INJ(X, Y)(w^E) = 2 (high relative to other transformations)
- I = I_E^Bb (inversion about E/Bb) is internal for both chords
  - INJ(X, X)(I) = 3 (maps X to itself)
  - INJ(Y, Y)(I) = 2 (maps much of Y to itself)

The wedge pushes Angst toward Hoffen; the inversion keeps each chord self-related.

From Figure 6.5(a) in the melodic analysis:
- T6 is progressive for X_1^4 -> X_5^8 (first tetrad to second)
- I and w are internal for each tetrad

# Related Concepts
- Internal Transformation
- Dispersive Transformation
- External Transformation
- INJ (Injection Function)

# Common Confusions
"Progressive" does not mean "good" or "correct" - it is a technical term describing high INJ(X, Y) values. A progression might involve multiple transformations, some progressive and some internal, working together. The terminology describes function, not aesthetic value.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.4
