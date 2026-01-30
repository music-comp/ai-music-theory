---
concept: RI-Chaining
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
unit: null
authors: David Lewin
---

# Quick Definition
A compositional and analytical technique where serial forms are linked by retrograde-inversion, with each new form beginning on the final two notes of the previous form, creating chains of related motivic statements.

# Formal Definition
Given a serial motive, RI-chaining produces a sequence where:
1. Each form is the retrograde-inversion of the preceding form
2. The RI form specifically uses the final two notes of the preceding form as its opening two notes
3. When applied repeatedly, alternate forms are transposed forms of each other

# Mathematical Formulation
For a series s = s_1, s_2, ..., s_N:
- RICH(s) is the retrograde-inverted form whose first two elements are s_{N-1} and s_N
- RICH(RICH(s)) = T_i(s), where i = int(s_1, s_N) + int(s_2, s_{N-1})
- This interval i is the "TCH-interval" for series s

The operation TCH = (RICH)(RICH) is always some transposition, but which transposition depends on the series' internal structure.

# Musical Context/Application
RI-chaining creates structural sequencing where the foreground events of sequenced passages may differ entirely while sharing the same underlying transformational scheme. The technique is used by Wagner in Parsifal and extensively by Webern in his serial works.

# Examples
From Parsifal (Figure 7.4):
- Z_1 (Zauber) chains to Z_2 via RICH
- Z_2 chains to Z_3 via RICH
- Z_3 = T_10(Z_1); Z_4 = T_10(Z_2)
- The TCH interval 10 governs the "structural sequencing"

For Zauber (A-C-Eb-E):
- RICH(s) = Eb-E-G-Bb
- RICH(RICH(s)) = G-Bb-Db-D = T_10(A-C-Eb-E)
- TCH interval = int(A, E) + int(C, Eb) = 7 + 3 = 10

# Related Concepts
- RICH Transformation
- TCH Transformation
- Structural Sequencing
- Webern Piano Variations Analysis
- Wagner Parsifal Zauber Motive Analysis

# Common Confusions
- RI-chaining is not the same as generic RI operations; the specific overlap requirement (last two to first two) is essential
- The TCH interval depends on the series structure, so different series produce different transposition levels
- Structural sequencing via RI-chaining may not correspond to foreground sequential patterns

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.2, and Chapter 8 for formal RICH/TCH definitions
