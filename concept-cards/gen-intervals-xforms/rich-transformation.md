---
concept: RICH Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A serial transformation that produces the retrograde-inverted form of a series whose first two elements are the last two elements of the original series (in order), enabling RI-chaining.

# Formal Definition
Given a series s = s_1, s_2, ..., s_N:
- RICH(s) is that retrograde-inverted form of s whose first two elements are s_{N-1} and s_N, in that order
- RICH enables the chaining technique where successive forms overlap by two elements

# Mathematical Formulation
For series s with elements s_1, s_2, ..., s_N:
- RICH(s) begins with s_{N-1}, s_N
- RICH(s) is an RI form of s

Key property (TCH relationship):
- RICH(RICH(s)) = T_i(s), where i = int(s_1, s_N) + int(s_2, s_{N-1})
- This i is the "TCH-interval for s"

For RI forms of s, the TCH interval is the same; for R or I forms, it is the negative (inverse).

# Musical Context/Application
RICH is the fundamental operation behind RI-chaining, used by composers including Wagner (Parsifal) and Webern (Piano Variations). When RICH is applied repeatedly, it generates structural sequences at transposition levels determined by the series' internal structure.

# Examples
For Zauber series s = A-C-Eb-E:
- RICH(s) = Eb-E-G-Bb (begins with Eb and E, the last two notes of s)
- RICH(RICH(s)) = G-Bb-Db-D = T_10(s)
- TCH interval = int(A, E) + int(C, Eb) = 7 + 3 = 10

For Webern's Piano Variations row:
- s = Eb-B-Bb-D-C#-C-F#-E-G-F-A-G#
- TCH interval = int(Eb, G#) + int(B, A) = 5 + 10 = 3

# Related Concepts
- TCH Transformation
- RI-Chaining
- Structural Sequencing
- MUCH Transformation
- Serial Transformations

# Common Confusions
- RICH is not the same as generic RI; it specifies which RI form (the one overlapping by two elements)
- The TCH interval depends on the series structure, not on a fixed transposition level
- RICH applied twice gives a transposition, not a return to the original form
- RICH is a well-defined operation on series, not just a compositional technique

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.2.1
