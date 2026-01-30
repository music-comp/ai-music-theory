---
concept: TLAST Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A serial transformation that transposes a series by its last interval, making the last note of the original series become the next-to-last note of the transformed series.

# Formal Definition
TLAST operates on a series s:
- TLAST transposes s by its last interval (the interval from second-to-last to last element)
- Effect: The last note of s becomes the next-to-last note of TLAST(s)
- TLAST is the "dual" of TFIRST^(-1) in a certain sense

# Mathematical Formulation
For series s with last interval i = int(s_{N-1}, s_N):
- TLAST(s) = T_i(s)
- The specific transposition depends on the series' final interval

TLAST and TFIRST^(-1) duality:
- TLAST makes last note become next-to-last
- TFIRST^(-1) makes first note become second note
- These are "dual" effects on opposite ends of the series

# Musical Context/Application
TLAST and TFIRST appear in Webern's op. 5, no. 4 for string quartet, where they help explain the relationships between unaccompanied appearances of the FLYAWAY motive.

# Examples
From Webern op. 5, no. 4 (Figure 8.10):
- Three forms of FLYAWAY: C-E-F#-B-C#-G-Bb, Ab-C-D-G-A-Eb-F#, F-A-B-E-F#-C-Eb
- TLAST connects two of these forms
- TFIRST^(-1) connects two of these forms
- The Ab form is "central" - balanced between the other two by these transformations

The visual "centrality" of the Ab form (cadential, piece-ending) is captured by its transformational balance in the network.

# Related Concepts
- TFIRST Transformation
- Serial Transformations
- FLYAWAY Motive
- Webern Op. 5 No. 4 Analysis
- Cadential Function

# Common Confusions
- TLAST is not a fixed transposition; the interval depends on the series
- The "duality" with TFIRST^(-1) is structural, not about identical operations
- Labeling arrows as T_5 and T_2 would obscure the balancing centrality
- TLAST relates to how a series ends, TFIRST to how it begins

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.1, Figure 8.10
