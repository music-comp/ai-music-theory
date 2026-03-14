---
concept: Inversion and Transposition of Rows
category: technique
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
---

# Quick Definition
Operations on a twelve-tone row that generate the full row chart: inversion reverses intervals from the starting note, while transposition shifts the entire row by a fixed interval.

# Formal Definition
Given a prime row $a_1 = [0], a_2, \ldots, a_n$ in $\mathbb{Z}_n$:
- The **inversion** is the sequence $-a_1, -a_2, \ldots, -a_n$ (negation in $\mathbb{Z}_n$), forming the left column of the row chart.
- The **transpositions** are the subsequent rows, each starting with $-a_i$ (from the inversion column) and maintaining the same interval pattern as the prime row: row $i$ has entries $a_j - a_i$ for $j = 1, \ldots, n$.
- Columns are transpositions of the inversion, or equivalently, inversions of transpositions.

# Mathematical Context
In $\mathbb{Z}_n$, inversion is the map $[k] \mapsto [-k] = [n - k]$. Transposition by $[c]$ is the map $[k] \mapsto [k + c]$. The entry at position $(i, j)$ being $a_j - a_i$ shows that row $i$ is the prime row transposed by $-a_i$, and column $j$ is the inversion transposed by $a_j$. These operations commute: transposing the inversion equals inverting the transposition.

# Musical Context
The inversion of a row reverses the direction of every interval while preserving interval sizes. If the prime row goes up a minor third then down a semitone, the inversion goes down a minor third then up a semitone. Transposition shifts all note classes by the same interval. Together, these operations provide the composer with 48 distinct sequences (12 transpositions $\times$ 2 for prime/inversion $\times$ 2 for forward/retrograde) from which to draw musical material.

# Examples
- Prime row intervals from E: [0], [3], [2], [5], [4], [8], [1], [10], [11], [9], [7], [6]
- Its inversion: [0], [9], [10], [7], [8], [4], [11], [2], [1], [3], [5], [6] (each entry negated in $\mathbb{Z}_{12}$)
- Row 2 of the chart starts at $-a_2 = [9]$ and transposes the prime row by [9]: [9], [0], [11], [2], [1], [5], [10], [7], [8], [6], [4], [3]

# Related Concepts
- Row Chart
- Prime Row
- Twelve-Tone Technique
- Retrograde
- Modular Arithmetic

# Common Confusions
- Inversion in twelve-tone theory negates each interval modulo octave; it is different from "melodic inversion" in tonal music, which may not preserve exact interval sizes
- Each row is a transposition of the prime row, and each column is a transposition of the inversion; these two facts are algebraically equivalent

# Source Reference
Chapter 6, "Twelve-Tone Music" section, p. 74 (PDF); modular arithmetic formulation in Chapter 7, p. 82 (PDF)
