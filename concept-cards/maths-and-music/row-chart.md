---
concept: Row Chart
category: technique
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
unit: null
---

# Quick Definition
A $12 \times 12$ (or $n \times n$) array of note classes that serves as the foundation for twelve-tone composition, where each row and column contains every note class exactly once.

# Formal Definition
Given an original row $a_1 = [0], a_2, \ldots, a_n$ in $\mathbb{Z}_n$, the row chart is the $n \times n$ matrix whose entry at position $(i, j)$ is $a_j - a_i$ in $\mathbb{Z}_n$. The top row is the original (prime) row. The left column is the inversion of the top row. Each subsequent row is a transposition of the top row. Each column is a transposition of the inversion.

# Mathematical Context
The formula entry$(i, j) = a_j - a_i$ in $\mathbb{Z}_n$ captures all the structural relationships: the top row is $a_j - a_1 = a_j - [0] = a_j$; the left column is $a_1 - a_i = -a_i$ (the inversion); and row $i$ is a transposition of row 1 by $-a_i$. Columns are transpositions of the inversion. The arithmetic takes place entirely in $\mathbb{Z}_n$.

# Musical Context
A twelve-tone composition draws its melodic and harmonic material exclusively from the sequences found in the rows and columns of the row chart, or from their retrogrades. The row chart provides a complete catalog of all permissible sequences for the composition, ensuring systematic use of all note classes.

# Examples
- For the row E, G, F$\sharp$, A, G$\sharp$, C, F, D, D$\sharp$, C$\sharp$, B, B$\flat$: the modular sequence is [0], [3], [2], [5], [4], [8], [1], [10], [11], [9], [7], [6]
- The entry at position $(8, 5)$ is $a_5 - a_8 = [4] - [10] = [6]$ in $\mathbb{Z}_{12}$
- A 7-tone row chart uses $\mathbb{Z}_7$ arithmetic, e.g., the row $[0], [4], [1], [6], [5], [2], [3]$ generates a $7 \times 7$ chart

# Related Concepts
- Twelve-Tone Technique
- Prime Row
- Inversion and Transposition of Rows
- Retrograde
- Modular Integers

# Common Confusions
- The row chart is not arbitrary: all entries are determined entirely by the original row through the formula $a_j - a_i$
- Converting between modular integers and note names requires choosing a designated note class for $[0]$ and using a modular clock
- The method works for any n-chromatic scale, not just $n = 12$

# Source Reference
Chapter 6, "Twelve-Tone Music" section, p. 74 (PDF); modular arithmetic formulation in Chapter 7, p. 82 (PDF)
