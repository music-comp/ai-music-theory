---
concept: Multiplicative Transposition
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (4): Some Further Analyses"
chapter_number: 10
pdf_page: 251
unit: null
authors: David Lewin
---

# Quick Definition
Multiplicative transposition is a transformation that multiplies all duration values in a rhythmic series by a constant factor, producing augmentation (factor > 1) or diminution (factor < 1).

# Formal Definition
Given a durational series S = (d1, d2, d3, ..., dn), multiplicative transposition by factor k produces T_k(S) = (k*d1, k*d2, k*d3, ..., k*dn). This operation preserves the proportional relationships between durations while scaling the absolute values.

# Mathematical Formulation
Let S = (d1, d2, ..., dn) be a durational series.
- T_k(S) = (k*d1, k*d2, ..., k*dn) for multiplication factor k
- T_k is an isomorphism: T_k(T_m(S)) = T_{km}(S)
- T_1 is the identity
- T_{1/k} is the inverse of T_k

Properties:
- Augmentation: k > 1 (e.g., T_2 doubles all durations)
- Diminution: k < 1 (e.g., T_{1/2} halves all durations)

# Musical Context/Application
Multiplicative transposition in the durational domain is analogous to transposition in the pitch domain, but instead of adding a constant to all values, we multiply by a constant. This operation formalizes the traditional concepts of rhythmic augmentation and diminution.

# Examples
In the Mozart K.550 analysis (Figures 10.2-10.3):
- DM = 1 + 2 + 2 (the basic durational motive)
- T_2(DM) = 2 + 4 + 4 (augmentation by factor 2, appearing as series 3)

The augmentation from bracket 2 to bracket 3 transforms the rhythmic setting by T_2. Later, series 7 = 2 + 1 + 1 represents a diminution (T_{1/2}) of series 6 = 4 + 2 + 2, which "undoes the effect of the earlier augmentation."

Lewin notes: "This diminution (multiplicative transposition by 1/2) undoes the effect of the earlier augmentation (transposition by 2), the augmentation we underwent in passing from bracket 2 to bracket 3."

# Related Concepts
- Durational Motive (DM)
- Multiplicative Inversion
- Additive Transposition
- Augmentation
- Diminution
- Rhythmic Transformation

# Common Confusions
Multiplicative transposition in rhythm differs fundamentally from additive transposition in pitch. In pitch space (using pitch-class integers), T_n adds n to each value. In durational space, T_k multiplies each value by k. Students should not conflate these two types of transposition operations despite the similar notation.

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Figures 10.2-10.3
