---
concept: M-Class Vector
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
The M-class vector of a set Y lists the values EMB(/X/, Y) as the variable /X/ runs through all set classes whose members have cardinality M.

# Formal Definition
Definition 5.3.3: Given a set Y and a positive integer M, the M-class vector of Y is the function giving EMB(/X/, Y) as the variable /X/ runs through the various set-classes whose members have cardinality M.

When M = 2, this is Forte's interval vector (with CANON = transpositions and inversions).

# Mathematical Formulation
M-class-vector(Y): {M-element set classes} -> non-negative integers
M-class-vector(Y)(/X/) = EMB(/X/, Y)

Properties:
- Only finitely many /X/ have non-zero embedding numbers (Y is finite)
- Sum of all entries = COMB(M, |Y|) = number of M-subsets of Y
- If S is infinite, there may be infinitely many M-element classes, but only finitely many with EMB > 0

Special cases:
- 2-class vector = interval vector (dyad-type vector)
- 3-class vector = trichord-type vector
- etc.

# Musical Context/Application
The interval vector (2-class vector) is familiar from Forte's atonal theory. The M-class vector generalizes this to any cardinality. A trichord-type vector tells how many of each trichord type are embedded in a set; a tetrachord-type vector does likewise for tetrachords.

# Examples
Let Y = C major scale, CANON = transpositions and inversions:

2-class vector (interval vector): [2, 5, 4, 3, 6, 1]
- 2 semitones, 5 whole tones, 4 minor thirds, etc.

3-class vector values:
- EMB(3-11, Y) = 6 (harmonic triads)
- EMB(3-7, Y) = 6
- EMB(3-9, Y) = 3 (quartal trichords)
- etc.

If CANON = transpositions only, the number of 3-note set classes changes from 12 to 19, and the 3-class vector has 19 entries.

# Related Concepts
- EMB (Embedding Function)
- Interval Vector
- Set Class
- Canonical Group

# Common Confusions
The M-class vector depends on CANON. With transpositions only, there are more set classes and hence more vector entries. With transpositions and inversions, some classes merge and there are fewer entries. Always specify which canonical group is being used.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.3.3
