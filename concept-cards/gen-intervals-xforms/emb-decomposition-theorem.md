---
concept: EMB Decomposition Theorem
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
The number of ways to embed a small set X in a large set Z can be computed by summing over intermediate-sized set classes Y, with an adjustment factor accounting for multiplicity.

# Formal Definition
Theorem 5.3.5.2: Let L, M, and N be positive integers with L < M < N. Let ADJUST = 1/COMB(N-M, N-L). Let Z be a set of cardinality N and let X be a set of cardinality L. Then:

EMB(X, Z) = ADJUST * SUM over all M-member set-classes /Y/ of [EMB(X, /Y/) * EMB(/Y/, Z)]

# Mathematical Formulation
Lemma 5.3.5.1: COMB(L, N)/(COMB(L, M) * COMB(M, N)) = 1/COMB(N-M, N-L)

Theorem proof uses probabilistic reasoning:
1. Pull M-member subset from Z: P(/Y/) = EMB(/Y/, Z)/COMB(M, N)
2. Pull L-member subset from that: P(/X/ from /Y/) = EMB(/X/, /Y/)/COMB(L, M)
3. Total probability: P(/X/ from Z) = SUM[P(/Y/) * P(/X/ from /Y/)]
4. P(/X/ from Z) = EMB(X, Z)/COMB(L, N)
5. Algebra yields the theorem

The ADJUST factor corrects for overcounting: each L-member subset of Z appears in multiple M-member subsets.

# Musical Context/Application
This theorem provides a way to compute EMB values for small sets by using intermediate-sized reference sets. For instance, to count how many tritones are in a hexachord, one could sum over all trichord types, weighting by how each trichord type relates to both tritones and hexachords.

# Examples
From Figures 5.9-5.10: Z = {A, B, C, D} (tetrachord), X = {A, C} (dyad)
- L = 2, M = 3, N = 4
- ADJUST = 1/COMB(1, 2) = 1/2

The tetrachord Z has 6 edges (2-element subsets), grouped by Forte class:
- Class 2-1: 1 edge
- Class 2-2: 2 edges
- Class 2-3: 2 edges
- Class 2-5: 1 edge

Each edge appears in exactly 2 triangular faces, so summing over faces overcounts by factor of 2, which ADJUST corrects.

# Related Concepts
- EMB (Embedding Function)
- Combinatorics (COMB function)
- Algebraic Topology
- Set Class Vectors

# Common Confusions
The ADJUST factor is crucial. Without it, the sum overcounts because the same small set appears in multiple intermediate-sized sets. The value 1/COMB(N-M, N-L) is often easier to compute than the raw fraction.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Lemma 5.3.5.1 and Theorem 5.3.5.2
