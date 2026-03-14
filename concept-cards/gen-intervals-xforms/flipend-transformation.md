---
concept: FLIPEND Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A transformation on three-element series that inverts the last element about the second element, leaving the first two elements in place: FLIPEND transforms s_1-s_2-s_3 into s_1-s_2-a, where a is the inversion-about-s_2 of s_3.

# Formal Definition
FLIPEND operates on series of three pitches or pitch classes:
- FLIPEND(s_1-s_2-s_3) = s_1-s_2-a
- Where a = I^{s_2}(s_3), the inversion of s_3 about s_2
- int(s_3, a) = int(s_2, s_3), so a is equidistant from s_2 on the opposite side

FLIPEND^(-1) transforms t_1-t_2-t_3 into t_1-b-t_3, where b is the inversion-about-t_3 of t_1.

# Mathematical Formulation
For s = s_1-s_2-s_3:
- a = 2*s_2 - s_3 (in pitch or pitch-class arithmetic)
- FLIPEND(s) = s_1-s_2-a

Properties:
- FLIPEND preserves the first two elements
- FLIPEND "flips" the third element about the second
- FLIPEND and FLIPSTART are "dual" operations

# Musical Context/Application
FLIPEND and FLIPSTART were identified by Jonathan W. Bernard in studying how Varese's music expands, contracts, and displaces registral space. The operations model pitch-space manipulations in post-tonal music.

# Examples
From Figure 8.11:
- Arrows above staff: FLIPEND applications
- Arrows below staff: FLIPSTART^(-1) applications
- Alternating these creates chains of three-pitch series
- The chains show systematic registral expansion/contraction

Specific transformation:
- If s = C-E-G, then FLIPEND(s) = C-E-D (where D = inversion of G about E)
- int(E, G) = 3; int(E, D) = -3 (same interval, opposite direction)

# Related Concepts
- FLIPSTART Transformation
- Varese Analysis
- Registral Space
- Inversion Operations
- Three-Element Series

# Common Confusions
- FLIPEND only operates on three-element series (not longer series)
- The "flip" is about the second element, not a general inversion
- FLIPEND is different from FLIPEND^(-1) (which flips the second element about the third)
- Bernard's original usage was for registral analysis; Lewin generalizes to pitch classes

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.2, Figure 8.11
