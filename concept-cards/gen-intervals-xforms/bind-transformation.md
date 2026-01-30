---
concept: BIND Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A serial transformation that takes a pitch-class series and produces the retrograde-inverted form with the same first and last notes as the original, "binding" the series to its endpoints.

# Formal Definition
BIND operates on a pitch-class series s:
- BIND(s) is that retrograde-inverted form of s with the same first and last notes as s
- If s begins on x and ends on y, then BIND(s) also begins on x and ends on y
- BIND preserves the "frame" while inverting and reversing the interior

# Mathematical Formulation
For series s = s_1, s_2, ..., s_N:
- BIND(s) is an RI form of s
- BIND(s)_1 = s_1 and BIND(s)_N = s_N
- The specific RI form is determined by these endpoint constraints

Properties:
- BIND commutes with TCH
- This commutativity is crucial for the Todesverkuendigung network (Figure 9.11)

# Musical Context/Application
BIND appears in the analysis of Wagner's Todesverkuendigung, connecting the bass-line FATE chain to the melodic FATE chain. The BIND arrows on Figure 9.11 are diagonal, showing how bass and melody forms relate while preserving their shared endpoints.

# Examples
From Figure 9.11 (Todesverkuendigung):
- Bass form: A-C-B (begins A, ends B)
- Melody form: A-G#-B (begins A, ends B)
- BIND(A-C-B) could yield A-G#-B (same endpoints, RI-related)
- The diagonal BIND arrows connect bass and melody chains

The input function of A-C-B:
- Lower left node (A-C-B) is the unique input
- This form has "special generative function"
- The entire network grows from the LOVE-derived bass motive

# Related Concepts
- RICH Transformation
- TCH Transformation
- Wagner Todesverkuendigung Analysis
- FATE Motive
- Serial Transformations

# Common Confusions
- BIND is not the same as RICH (different endpoint constraints)
- BIND preserves first AND last notes; RICH only constrains first two notes of output
- The commutativity with TCH is a key property for network construction
- BIND requires specific endpoint matching, which may not always exist for arbitrary series

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.6.4, Figure 9.11
