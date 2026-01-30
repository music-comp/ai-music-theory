---
concept: FLIPSTART Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A transformation on three-element series that inverts the first element about the second element, preserving the last two elements: FLIPSTART transforms s_1-s_2-s_3 into a-s_2-s_3, where a is the inversion-about-s_2 of s_1.

# Formal Definition
FLIPSTART operates on series of three pitches or pitch classes:
- FLIPSTART(s_1-s_2-s_3) = a-s_2-s_3
- Where a = I^{s_2}(s_1), the inversion of s_1 about s_2
- int(s_1, a) = int(s_1, s_2) + int(s_2, a) where int(s_2, a) = int(s_1, s_2)

FLIPSTART^(-1) transforms t_1-t_2-t_3 into t_1-b-t_3, where b is the inversion-about-t_1 of t_3.

# Mathematical Formulation
For s = s_1-s_2-s_3:
- a = 2*s_2 - s_1 (in pitch or pitch-class arithmetic)
- FLIPSTART(s) = a-s_2-s_3

Properties:
- FLIPSTART preserves the last two elements
- FLIPSTART "flips" the first element about the second
- FLIPSTART and FLIPEND are "dual" operations (acting on opposite ends)

# Musical Context/Application
FLIPSTART and FLIPEND were identified by Jonathan W. Bernard for analyzing Varese's treatment of registral space. When alternated with FLIPEND, these operations generate systematic expansions and contractions of pitch intervals.

# Examples
From Figure 8.11:
- (a) and (b) show different starting series
- FLIPEND arrows above, FLIPSTART^(-1) arrows below
- Alternating creates chains exploring registral space

Specific transformation:
- If s = C-E-G, then FLIPSTART(s) = G#-E-G (where G# = inversion of C about E)
- int(C, E) = 4; int(E, G#) = 4 (same interval, continued upward)

# Related Concepts
- FLIPEND Transformation
- Varese Analysis
- Registral Space
- Inversion Operations
- Three-Element Series

# Common Confusions
- FLIPSTART only operates on three-element series
- The "flip" is about the second element, not the first
- FLIPSTART is different from FLIPSTART^(-1)
- FLIPSTART and FLIPEND are "dual" but not inverse operations

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.2, Figure 8.11
