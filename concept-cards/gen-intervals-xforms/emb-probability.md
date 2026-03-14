---
concept: EMB Probability Theorem
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
EMB can be interpreted probabilistically: the probability of randomly extracting a set of class /X/ from Y equals EMB(/X/, Y) divided by the number of ways to choose that many elements from Y.

# Formal Definition
Theorem 5.3.4: Let the cardinality of Y be N. Let M be a positive integer less than N. Pull M members of Y at random. Then the probability that you have pulled a set of class /X/ is given by EMB(/X/, Y)/COMB(M, N).

Here COMB(M, N) = N!/(M!(N-M)!) is the binomial coefficient.

# Mathematical Formulation
P(random M-subset of Y is in class /X/) = EMB(/X/, Y) / COMB(M, N)

Where:
- M = cardinality of sets in class /X/
- N = cardinality of Y
- COMB(M, N) = number of M-element subsets of an N-element set

Properties:
- 0 <= probability <= 1
- Sum over all M-element classes /X/ of P = 1
- If EMB(/X/, Y) = 0, probability = 0

# Musical Context/Application
This probabilistic interpretation, like the similar result for IFUNC, allows EMB to model statistical textures. If one randomly selects notes from a scale, EMB predicts the probability of forming various chord types. This provides a baseline expectation against which compositional choices can be measured.

# Examples
Let Y = C major scale (7 notes), M = 3 (trichords):
- COMB(3, 7) = 35 (total trichords extractable from scale)

If CANON = transpositions and inversions:
- EMB(3-11, Y) = 6 (harmonic triads)
- P(harmonic triad) = 6/35 = 17.1%

- EMB(3-7, Y) = 6 (diminished triads and major-minor sevenths without fifth)
- P(class 3-7) = 6/35 = 17.1%

These equal probabilities arise from the diatonic scale's structure, not mere coincidence.

# Related Concepts
- EMB (Embedding Function)
- IFUNC Probability
- Binomial Coefficients
- Statistical Set Theory

# Common Confusions
The probability interpretation assumes truly random selection. In actual composition, selections are not random, so probabilities provide a null hypothesis rather than a prediction. Comparing actual frequencies to EMB-based probabilities reveals compositional preferences.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.3.4
