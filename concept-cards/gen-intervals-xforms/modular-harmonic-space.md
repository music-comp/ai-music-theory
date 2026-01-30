---
concept: Modular Harmonic Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Modular harmonic space is a GIS of pitch classes under just intonation, visualized as a two-dimensional game board with intervals measuring "b dominants and c mediants."

# Formal Definition
In Example 2.1.6, the musical space S comprises pitch classes generated from just intonation pitch space by reducing out octaves. Given pitch classes s and t, int(s, t) is the ordered pair (b, c) such that t lies b dominants and c mediants from s. The interval group IVLS is Z x Z (the direct product of integers with itself) under componentwise addition.

# Mathematical Formulation
- S = pitch classes on the infinite game board (Figure 2.2)
- IVLS = Z x Z = {(b, c) : b, c in Z} under addition
- int(s, t) = (b, c) where t is b squares east and c squares north of s
- Composition: (b1, c1) + (b2, c2) = (b1 + b2, c1 + c2)
- Identity: (0, 0)
- Inverse: -(b, c) = (-b, -c)

# Musical Context/Application
This GIS modularizes just intonation by collapsing octaves. The two-dimensional map (Figure 2.2) shows pitch classes arranged by dominant relationships (horizontal) and mediant relationships (vertical). Intervals are "moves" on this game board. The interval (2, 1) is a "knight's move" (2 east, 1 north) taking C to F#.

# Examples
From Example 2.1.6 and Figure 2.2:
- int(C, G) = (1, 0): one dominant, zero mediants
- int(G, D) = (1, 0): one dominant
- int(D, F#) = (0, 1): zero dominants, one mediant
- int(C, F#) = (2, 1): two dominants, one mediant
- int(C, F) = (-1, 0): one subdominant
- int(C, Ab) = (0, -1): one submediant
- int(C, Db) = (-1, -1): one subdominant, one submediant

The "knight's move" (2, 1) takes C to F#, or A1 to D#2, or Db-1 to G0 on the game board.

# Related Concepts
- Just Intonation Pitch Space
- Direct Product
- Generalized Interval System
- Figure 2.2
- Pitch-Class Space

# Common Confusions
- Pitch classes with the same letter but different subscripts (C-1, C0, C1) are DISTINCT in just intonation
- In equal temperament, the space still has meaning conceptually (different places on the map)
- The syntonic comma makes acoustically distinct pitch classes at different subscripts
- This is a reduction of 2.1.5, not 2.1.3 (it uses harmonic, not chromatic intervals)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.6, Figure 2.2, Section 2.4
