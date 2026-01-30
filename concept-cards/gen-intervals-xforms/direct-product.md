---
concept: Direct Product
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
The direct product of two semigroups combines their elements as ordered pairs, with the operation applied component-wise.

# Formal Definition
Let SGP1 = (X1, BIN1) and SGP2 = (X2, BIN2) be semigroups. The direct product SGP3 = (X3, BIN3) is constructed as follows: X3 is the Cartesian product X1 x X2. Given (x1, x2) and (y1, y2) in X3, BIN3((x1, x2), (y1, y2)) is defined as (BIN1(x1, y1), BIN2(x2, y2)). In multiplicative notation: (x1, x2)(y1, y2) = (x1y1, x2y2).

# Mathematical Formulation
- SGP3 = SGP1 x SGP2 denotes the direct product
- Elements of SGP3 are pairs (x1, x2) where x1 is in X1 and x2 is in X2
- Operation: (x1, x2)(y1, y2) = (x1y1, x2y2)
- If e1, e2 are identities, then (e1, e2) is the identity of SGP3
- If SGP1 and SGP2 are groups, then SGP3 is a group with (x1, x2)^(-1) = (x1^(-1), x2^(-1))

# Musical Context/Application
Direct products model musical spaces with multiple independent dimensions. The GIS for just-intonation pitch classes (Example 2.1.6) uses the direct product Z x Z as its interval group - one dimension for dominants, one for mediants. Time-pitch spaces can be modeled as direct products of temporal and pitch interval groups. Direct products allow independent measurement along each dimension.

# Examples
Example 2.1.6: The interval group for modular harmonic space is Z x Z (integers cross integers), the direct product of the integers with themselves.
- int(C, G) = (1, 0): one dominant, zero mediants
- int(C, E) = (0, 1): zero dominants, one mediant
- int(C, F#) = (2, 1): two dominants, one mediant
- Composition: (1, 0) + (1, 1) = (2, 1)

General example: If G1 has identity e1 and G2 has identity e2:
- Identity of G1 x G2 is (e1, e2)
- (x1, x2)^(-1) = (x1^(-1), x2^(-1)) in a direct product of groups

# Related Concepts
- Semigroup
- Group
- Cartesian Product
- GIS for Harmonic Space
- Multi-Dimensional Interval Systems

# Common Confusions
- Elements of the direct product are pairs, not singletons
- The operation is applied independently in each component
- The direct product of two groups is always a group
- Direct products can be extended to any finite number of factors

# Source Reference
Chapter 1: Mathematical Preliminaries, Section 1.13
