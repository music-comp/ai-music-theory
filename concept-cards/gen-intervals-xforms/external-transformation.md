---
concept: External Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
An external transformation is one for which INJ(X, X)(f) is minimal or relatively small - it maps X largely outside itself.

# Formal Definition
Definition (6.4): A transformation f is X-external if INJ(X, X)(f) is minimal or relatively small given the constraints of X and INSPECT.

External transformations map X largely outside itself, in contrast to internal transformations which keep X "like itself."

# Mathematical Formulation
f is X-external if:
INJ(X, X)(f) is near or at min{INJ(X, X)(g) : g in INSPECT}

Algebraic tendencies:
- X-internal followed by X-external tends to be X-external
- Progressive followed by Y-external tends to be dispersive

The definitions avoid mentioning set complements, which may not be "sets" if S is infinite.

# Musical Context/Application
External transformations model "departure" from a harmony. They take notes of a chord to notes outside that chord. In traditional theory, operations like certain transpositions of a set by intervals that produce maximum divergence are external.

The semi-combinatorial hexachord property is a classic example: certain inversions map a hexachord entirely outside itself (to its complement).

# Examples
Semi-combinatorial hexachord:
- Let X be a hexachord that inverts to its complement under I = I_0^E
- Then INJ(X, X)(I) = 0
- I is maximally X-external

From op.19 no.6 analysis:
- If I is the inversion that maps hexachord X to complement(X)
- Then I is X-external: INJ(X, X)(I) = 0
- This is Babbitt's hexachord combinatoriality condition

External transformations set up the potential for progressive motion: if X is externalized, it has "left itself" and can more readily "become" something else.

# Related Concepts
- Internal Transformation
- Progressive Transformation
- Dispersive Transformation
- Combinatoriality
- INJ (Injection Function)

# Common Confusions
External is not the same as dispersive. External concerns X's relationship to itself; dispersive concerns X's relationship to Y. A transformation can be X-external (mapping X outside X) while being progressive (mapping X into Y).

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.4
