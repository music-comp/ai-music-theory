---
concept: Row-Retrograde as Complement
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
In the PROT model, a twelve-tone row and its retrograde are set-theoretic complements: L-bar (complement of L) = R(L), connecting serial retrograde to Fortean complementation.

# Formal Definition
For a row L as a subset of PROT:
- L contains 66 protocol pairs (all pairs (p, q) where p precedes q)
- PROT has 132 pairs total
- L-bar (complement) contains the remaining 66 pairs
- L-bar = {(p, q) : (q, p) in L} = R(L)

The retrograde operation R on PROT is: R(p, q) = (q, p).

# Mathematical Formulation
For row L:
(p, q) in L iff p precedes q in L
(p, q) in L-bar iff p does NOT precede q in L
iff q precedes p in L
iff (p, q) in R(L)

Therefore: L-bar = R(L)

This places row/retrograde in the same formal position as hexachord/complement:
- cardL = 66 = 1/2 * cardPROT
- L and L-bar partition PROT
- Generalized Hexachord Theorem (6.6.1E) applies

# Musical Context/Application
The complement relationship between row and retrograde extends Babbitt's hexachord theorem to serial structure. The formal analogy suggests deep connections between set-class theory and serial theory, unified through the PROT model and INJ function.

# Examples
Row L = A-Bb-E-D-Eb-C#-G-F-F#-G#-B-C (Moses und Aron)
Contains pairs like: (A, Bb), (A, E), (A, D), ..., (B, C)
Total: 66 pairs

Retrograde R(L) = C-B-G#-F#-F-G-C#-Eb-D-E-Bb-A
Contains pairs like: (C, B), (C, G#), ..., (Bb, A)

Complement L-bar contains exactly the pairs in R(L):
- (Bb, A) in L-bar because A precedes Bb in L, so Bb does NOT precede A in L
- (Bb, A) in R(L) because A precedes Bb in R(L)

Application of Theorem 6.6.1E:
For any operation OP on PROT:
INJ(L, L)(OP) = INJ(L-bar, L-bar)(OP) = INJ(R(L), R(L))(OP)

This says the "internal INJ structure" of a row equals that of its retrograde.

# Related Concepts
- Protocol Pairs (PROT)
- INJ Complement Theorem
- Hexachord Theorem (Generalized)
- Retrograde Operation
- Set Complementation

# Common Confusions
The complement relationship holds in PROT, not in pitch-class space. A row and its retrograde have the same pitch-class content; their complementarity is in terms of ordering relations (protocol pairs), not pitch classes.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.6.2
