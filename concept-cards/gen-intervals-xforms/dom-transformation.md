---
concept: DOM Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A Klang transformation where the input Klang becomes the dominant of its output; DOM transposes by the inverse of the dominant interval, so (C, +)DOM = (F, +) means "C major becomes the dominant of F major."

# Formal Definition
DOM is defined as transposition by the inverse of the dominant interval:
- (p, sign)DOM = (q, sign), where q is that pitch class of which p is the dominant
- Reading: (p, sign) becomes the dominant of (q, sign)
- Preserves mode: major stays major, minor stays minor

# Mathematical Formulation
DOM transformation:
- (p, sign)DOM = (p - 7, sign) or equivalently (p + 5, sign) mod 12
- DOM is a transposition T_5 on the pitch-class component
- Mode is preserved

Inverse relation:
- SUBD = DOM^(-1)
- (p, sign)SUBD = (p + 7, sign)

# Musical Context/Application
The unusual definition (where Klangs point to their tonics via DOM) makes graphs move naturally with harmonic intuition. A DOM arrow points from dominant to tonic, showing functional dependency. This contrasts with DOM' (the "usual" idea) where a Klang transforms into its own dominant.

# Examples
Applications:
- (C, +)DOM = (F, +): C major becomes dominant of F major
- (G, -)DOM = (C, -): G minor becomes dominant of C minor

On a network (Figure 8.1):
- A DOM arrow from (C, +) to (F, +) means "following DOM from C major, arrive at F major"
- Visual layout: DOM arrows point "forward" in functional terms

# Related Concepts
- Klang Representation
- SUBD Transformation
- MED Transformation
- Right Orthography
- Riemann Function Theory
- Klang Transformation Networks

# Common Confusions
- DOM is NOT "take the dominant" but "become the dominant of"
- DOM arrows point from dominant to tonic, not tonic to dominant
- This definition differs from the "usual" DOM' where (F, +)DOM' = (C, +)
- Lewin's definition makes graphs flow naturally with harmonic function

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1
