---
concept: Ordinal-Pitch Pairs
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
The space of ordinal-pitch pairs models melodies as sets of pairs (n, p) where n is the ordinal position and p is the pitch class, enabling INJ analysis of serial melodic structure.

# Formal Definition
The space S consists of pairs (n, p) where:
- n is a positive integer (ordinal position: 1st, 2nd, 3rd, ... note)
- p is a pitch class

A melody (non-repeating pitch-class series) is modeled as a set of such pairs:
- If the nth note is pitch class p, then (n, p) is in the set

Transformations on S:
- (k, OP) maps (n, p) to (n + k, OP(p))
- k shifts the ordinal position
- OP transforms the pitch class

# Mathematical Formulation
S = {(n, p) : n in positive integers, p in pitch classes}

For melody M with notes p1, p2, ..., pN:
M = {(1, p1), (2, p2), ..., (N, pN)}

Transformation (k, OP): S -> S
(k, OP)(n, p) = (n + k, OP(p))

Key property: (k, OP) is NOT an operation on S (not onto).
- No (n, p) maps to (1, q) under (k, OP) when k > 0
- This is fine for INJ analysis

# Musical Context/Application
This model captures both serial position and pitch-class identity. Analyzing INJ((k, OP)) reveals how melodic segments relate transformationally. Unlike the PROT model (which captures only ordering), this model captures both "which note" and "which position."

# Examples
From "Angst und Hoffen" melodic analysis (Figure 6.4):

Melody = (2, Gb), (1, D), (10, Fb), (3, Eb), ...
Reading: "The 2nd note is Gb, the 1st note is D, the 10th note is Fb, ..."

Transformations:
- (1, I): shift one position, invert pitch class
- (2, w): shift two positions, wedge pitch class

First tetrad X_1^4 = {(1, D), (2, Gb), (3, Eb), (4, Fb)}
- INJ(X_1^4, X_1^4)(1, I) = 2 (internal)
- INJ(X_1^4, X_1^4)(2, w) = 2 (internal, allowing Fb as F)

Second tetrad X_5^8:
- INJ(X_5^8, X_5^8)(2, I) = 2 (I-relations at distance 2, augmented from 1)
- INJ(X_5^8, X_5^8)(3, w) = 1 (w-relations at distance 3, augmented from 2)

Progressive relation:
- INJ(X_1^4, X_5^8)(n, T_6) is positive for several n
- T_6 is progressive between the tetrads (no I or w arrows cross)

# Related Concepts
- INJ (Injection Function)
- Protocol Pairs (PROT)
- Serial Melody
- Progressive/Internal Transformations

# Common Confusions
This space is NOT a GIS (transformations like (k, OP) are not operations). INJ handles this gracefully. The ordinal-pitch model complements PROT: PROT captures pure ordering relations, while ordinal-pitch captures positional identity.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, melodic analysis following Figure 6.4
