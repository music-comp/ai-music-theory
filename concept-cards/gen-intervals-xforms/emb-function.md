---
concept: "EMB (Embedding Function)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
EMB(X, Y) counts the number of forms of X (members of the set class /X/) that are included in Y.

# Formal Definition
Definition 5.3.1: Given sets X and Y, the embedding number of X in Y, EMB(X, Y), is the number of forms of X (i.e. members of /X/) that are included in Y.

The embedding number depends on the canonical group CANON, though this is not shown in the notation.

# Mathematical Formulation
EMB(X, Y) = |{X' in /X/ : X' is a subset of Y}|

Key properties:
- EMB(X', Y) = EMB(X, Y) if X' is a form of X
- EMB(X, Y') = EMB(X, Y) if Y' is a form of Y
- Therefore EMB(/X/, /Y/) is well-defined

Extensions:
- EMB(/X/, Y): number of forms of any member of /X/ embedded in Y
- EMB(X, /Y/): number of forms of X embedded in any member of /Y/
- EMB(/X/, /Y/): number of forms embedded across set classes

# Musical Context/Application
EMB generalizes Forte's interval vector. The interval vector counts embeddings of dyad classes (2-note sets) within a set; EMB extends this to arbitrary cardinalities. The "M-class vector" of Y gives EMB(/X/, Y) as /X/ ranges over all M-element set classes.

# Examples
In pitch-class space with CANON = transpositions and inversions:

Let X = any major triad, Y = C major scale {C, D, E, F, G, A, B}:
- If CANON = transpositions only: EMB(X, Y) = 3 (three major triads in scale)
- If CANON = transpositions and inversions: EMB(X, Y) = 6 (six harmonic triads)

The dyad-class vector (interval vector) of Y:
- EMB(SC_1, Y) = number of semitones in scale = 2
- EMB(SC_2, Y) = number of whole tones = 5
- etc.

# Related Concepts
- Set Class
- Canonical Group
- Interval Vector
- IFUNC (Interval Function)
- COV (Covering Function)

# Common Confusions
EMB depends critically on CANON. The notation EMB(X, Y) hides this dependence. Strictly, one should write EMB(CANON, X, Y), but this is too cumbersome. Always be clear about which canonical group is in effect when discussing embedding numbers.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.3.1 and 5.3.2
