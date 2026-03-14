---
concept: ADJOIN Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
ADJOIN(X, Y, Z) counts forms of Y that are disjoint from X but can be combined with X to fit within some form of Z.

# Formal Definition
Definition (5.5 Notes): ADJOIN(X, Y, Z) is the number of forms Y' of Y satisfying both conditions:
(A): Y' is disjoint from X
(B): There is some form of Z that includes both X and Y'

Unlike SNDW, ADJOIN can be computed from set classes alone: we can write ADJOIN(/X/, /Y/, /Z/).

# Mathematical Formulation
ADJOIN(X, Y, Z) = |{Y' in /Y/ : (X intersect Y' = empty) AND (exists Z' in /Z/ with X union Y' subset Z')}|

Properties:
- Depends only on /X/, /Y/, /Z/ (can substitute set classes)
- Counts "compatible additions" to X within Z-contexts
- Related to combinatorial possibilities for set completion

# Musical Context/Application
ADJOIN answers: "Given chord X, how many forms of chord Y can be added to X (without overlap) such that the combination fits in some scale/aggregate of type Z?" This is useful for exploring harmonic possibilities constrained by a larger context.

# Examples
In pitch-class space:
X = {C, E} (major third)
Y = {D, G} (perfect fourth)
Z = C-major scale

Question: How many fourths can be added to {C, E} while staying within some major scale?

Checking fourths:
- {D, G}: {C, E} + {D, G} = {C, D, E, G} - fits in C major and G major
- {F, Bb}: Does not fit with {C, E} in any major scale
- {F#, B}: {C, E} + {F#, B} fits in... no major scale has both C, E, F#, B
- {A, D}: Fits in G major

ADJOIN({C, E}, {D, G}, major scale) = 4 (the four fourths that work are {D, G}, {A, D}, {E, A}, {B, E})

# Related Concepts
- EMB (Embedding Function)
- COV (Covering Function)
- SNDW (Sandwich Function)
- Set Class Operations

# Common Confusions
ADJOIN differs from SNDW: SNDW requires Y' to contain X (sandwiching), while ADJOIN requires Y' to be disjoint from X (adjoining). ADJOIN is about adding new material; SNDW is about interpolating material. Also, ADJOIN allows any form of Z to work, giving more flexibility.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section 5.5 Notes
