---
concept: COV (Covering Function)
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
COV(X, Y) counts the number of forms of Y that include X - the "covering number" measuring how many canonical transforms of Y can serve as supersets of X.

# Formal Definition
Definition (5.5 Notes): COV(X, Y), the covering number of X in Y, is the number of forms of Y that include X.

This is not necessarily the same as EMB(X, Y), which counts forms of X embedded in Y.

# Mathematical Formulation
COV(X, Y) = |{Y' in /Y/ : X is a subset of Y'}|

Relationship to EMB:
- EMB(X, Y) counts forms of the smaller set embedded in the larger
- COV(X, Y) counts forms of the larger set containing the smaller
- These need not be equal

If S is finite:
COV(X, Y) = EMB(complement of Y, complement of X)

The complement relationship connects covering and embedding via set-theoretic duality.

# Musical Context/Application
COV answers: "In how many transposed/inverted forms of scale Y does chord X appear?" This is useful when the analyst has a fixed chord and wants to know which key areas or scale contexts could contain it.

# Examples
In pitch-class space with CANON = transpositions and inversions:

X = {C, E} (major third dyad)
Y = {C, E, G} (major triad)

EMB(X, Y) = 3: Three major thirds are embedded in a major triad
(C-E, E-G#... wait, that's not in {C,E,G}. Actually: the three are the pairs within the triad)

COV(X, Y) = 1: Only one major triad contains the specific dyad {C, E}

Different example:
X = {C, E}, Y = C-major scale
- EMB(X, Y) = 1 (only {C, E} itself among major-third forms is in the scale)
- COV(X, Y) = number of major scales containing both C and E = ?

# Related Concepts
- EMB (Embedding Function)
- SNDW (Sandwich Function)
- Set Class
- Complement Relation

# Common Confusions
EMB and COV are not symmetric: EMB(X, Y) != COV(X, Y) in general. EMB counts small-set forms in the large set; COV counts large-set forms around the small set. The complement relation provides a duality but requires finite S.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section 5.5 Notes
