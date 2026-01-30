---
concept: Z-Relation (Generalized)
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
Two sets are Z-related if they have the same IFUNC with themselves but are not canonically equivalent - they share identical internal interval structure without being related by transposition or inversion.

# Formal Definition
In Forte's theory, pitch-class sets X1 and X2 which are not transposed or inverted forms of each other are Z-related if and only if IFUNC(X1, X1) = IFUNC(X2, X2) as functions on IVLS.

Generalized: In any GIS, X1 and X2 are Z-related if:
1. IFUNC(X1, X1) = IFUNC(X2, X2)
2. X2 is not in /X1/ (not canonically equivalent)

# Mathematical Formulation
The Z-relation asks: Under what conditions do non-equivalent sets have the same internal interval content?

Known: If X2 = P(X1) for some interval-preserving operation P, then IFUNC(X2, X2) = IFUNC(X1, X1) by Theorem 5.1.5.

In non-commutative GIS: If X2 = T_n(X1), then IFUNC(X2, X2)(i) = IFUNC(X1, X1)(nin^-1), which may not equal IFUNC(X1, X1)(i).

Open questions:
- Under what conditions on X1, X2 does IFUNC(X1, X1) = IFUNC(X2, X2)?
- When does IFUNC(X1, Y1) = IFUNC(X2, Y2)?
- Can these be characterized in terms of group structure?

# Musical Context/Application
Z-related sets in Forte's theory are "different" sets that share identical interval vectors. They have the same intervallic "color" or "texture" without being transpositions or inversions of each other. This phenomenon reveals that interval content does not uniquely determine set class.

# Examples
Classic Z-related pair in pitch-class space:
- {0, 1, 4, 6} (Forte 4-Z15)
- {0, 1, 3, 7} (Forte 4-Z29)

Both have interval vector [1, 1, 1, 1, 1, 1] but neither is a transposition or inversion of the other.

Generalized question (from text): Given four sets X1, Y1, X2, Y2, under what conditions is IFUNC(X1, Y1) = IFUNC(X2, Y2)? Figure 5.1 showed examples where different pairs produce identical IFUNC values.

# Related Concepts
- IFUNC (Interval Function)
- Interval Vector
- Set Class
- Canonical Equivalence
- INJ (Injection Function)

# Common Confusions
Z-related sets are not "the same" - they are specifically distinct set classes that happen to share interval vectors. The Z-relation is about having identical IFUNC, not about being equivalent under any operation. The phenomenon shows that IFUNC does not completely characterize set structure.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section following Theorem 5.1.8
