---
concept: "SNDW (Sandwich Function)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
SNDW(X, Y, Z) counts forms of Y that can be "sandwiched" between X and Z - that is, forms that both include X and are included in Z.

# Formal Definition
Definition (5.5 Notes): SNDW(X, Y, Z), the sandwich number of Y between X and Z, is the number of forms of Y that both include X and are included in Z.

Special cases:
- SNDW(empty set, Y, Z) = EMB(Y, Z)
- SNDW(X, Y, S) = COV(X, Y) when S is finite

# Mathematical Formulation
SNDW(X, Y, Z) = |{Y' in /Y/ : X subset Y' subset Z}|

Properties:
- If Y' is a form of Y: SNDW(X, Y, Z) = SNDW(X, Y', Z)
- Therefore SNDW(X, /Y/, Z) is well-defined
- But SNDW depends on specific X and Z, not just their classes
- SNDW(X, Y, Z) != SNDW(X', Y, Z) for X' in /X/ in general

Key observation: We can write SNDW(X, /Y/, Z) without ambiguity, but we cannot substitute /X/ or /Z/ as arguments.

# Musical Context/Application
SNDW answers: "Given a specific small chord X and a specific large set Z, how many forms of medium-sized set Y fit between them?" This is useful for analyzing how intermediate harmonies might connect a given chord to a given scale or aggregate.

# Examples
In pitch-class space:
Let Z = C-major scale
Let /Y/ = Forte-class 3-4
Let X1 = {C, E}

SNDW(X1, /Y/, Z) counts 3-4 trichords that contain {C, E} and lie within the C-major scale.
Result: 2 forms - {B, C, E} and {C, E, F}

Now let X2 = {F, A} (same set class as X1)
SNDW(X2, /Y/, Z) = 1 (only {E, F, A} works)

Although X1 and X2 are in the same set class, they have different sandwich numbers with the same Y and Z.

# Related Concepts
- EMB (Embedding Function)
- COV (Covering Function)
- ADJOIN Function
- Set Class

# Common Confusions
SNDW is sensitive to the specific sets X and Z, not just their set classes. This asymmetry (compared to EMB where set class suffices) arises because the sandwich constraint depends on how X and Z are specifically positioned relative to each other and to forms of Y.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section 5.5 Notes
