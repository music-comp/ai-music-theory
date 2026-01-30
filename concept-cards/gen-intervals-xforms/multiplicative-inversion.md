---
concept: Multiplicative Inversion
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (4): Some Further Analyses"
chapter_number: 10
pdf_page: 251
unit: null
authors: David Lewin
---

# Quick Definition
Multiplicative inversion is a transformation on durational series that divides a fixed product by each duration value, producing an inversion that preserves multiplicative relationships rather than additive ones.

# Formal Definition
Given a durational series S = (d1, d2, d3) and an inversional product p, multiplicative inversion I_p produces I_p(S) = (p/d1, p/d2, p/d3). The product p serves as the inversional axis in multiplicative space, analogous to the inversional index in additive pitch-class inversion.

# Mathematical Formulation
For durational series S = (d1, d2, ..., dn) and product p:
- I_p(S) = (p/d1, p/d2, ..., p/dn)
- I_p(I_p(S)) = S (involution property)
- Product p = d_i * I_p(d_i) for any element

Equivalence to additive inversion: For series S = (d1, d2, d3), additive inversion about sum s gives:
- I_s(S) = (s - d1, s - d2, s - d3)

When p and s are chosen appropriately, multiplicative and additive inversions can yield identical results.

# Musical Context/Application
Multiplicative inversion formalizes a type of durational transformation that maintains proportional relationships. When a durational series inverts multiplicatively, longer durations become shorter and vice versa, but the multiplicative ratios between corresponding pairs remain constant.

# Examples
In the Mozart K.550 analysis (Figure 10.3):

Series 3 = 2 + 4 + 4 inverts to Series 4b = 4 + 2 + 2

Multiplicative inversion about product 8:
- 8/2 = 4
- 8/4 = 2
- 8/4 = 2
- Result: (4, 2, 2)

Equivalently, additive inversion about sum 6:
- 6 - 2 = 4
- 6 - 4 = 2
- 6 - 4 = 2
- Result: (4, 2, 2)

Lewin notes: "We can regard the inversion as multiplicative, about the numerical product 8: 8 divided by 2, 4, and 4 (series 3) yields 4, 2, and 2 (series 4b). Or we can regard the inversion as additive, about the numerical sum 6."

# Related Concepts
- Multiplicative Transposition
- Additive Inversion
- Durational Motive (DM)
- RICH-Relations in Rhythm
- Inversional Index

# Common Confusions
Students may not realize that multiplicative and additive inversions can produce identical results in certain cases. The choice between multiplicative and additive frameworks depends on which preserves more meaningful relationships for the analytical context. Neither is inherently "correct."

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Figures 10.2-10.3
