---
concept: N-Tone Row Chart
category: technique
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
A generalization of the twelve-tone row chart to any n-chromatic scale, using $\mathbb{Z}_n$ arithmetic to construct an $n \times n$ array for composition in non-standard equal temperaments.

# Formal Definition
Given an original row $a_1 = [0], a_2, \ldots, a_n$ from $\mathbb{Z}_n$, the $n \times n$ row chart is constructed by setting entry$(i, j) = a_j - a_i$ in $\mathbb{Z}_n$. This formula simultaneously generates all transpositions and inversions. The method requires detuning a synthesizer to play in n-tone equal temperament, where the chromatic unit is $1200/n$ cents.

# Mathematical Context
The formula entry$(i,j) = a_j - a_i$ works identically in $\mathbb{Z}_n$ for any positive integer $n$. The chart has the same structural properties as the 12-tone case: the top row is the original, the left column is the inversion, each row is a transposition of the original, and each column is a transposition of the inversion. The arithmetic differs only in the modulus.

# Musical Context
Seven-tone row charts have been used for composition with detuned white keys. The detuning required for a 7-chromatic scale starting on C assigns each white key an interval of $1200/7 \approx 171.43$ cents. The resulting music has a distinctive quality, with intervals unfamiliar to ears trained on 12-tone temperament. The technique extends serialist principles to any equal division of the octave.

# Examples
- 7-tone example: original row [0], [4], [1], [6], [5], [2], [3] in $\mathbb{Z}_7$, using detuned white keys C, D, E, F, G, A, B with equal intervals of $\approx 171.43$ cents
- Detuning for 7-tone from C: D = -28.57, E = -57.14, F = +114.29, G = +85.71, A = +57.14, B = +28.57 cents
- 3-tone, 5-tone, and 6-tone row charts can be similarly constructed

# Related Concepts
- Row Chart
- Twelve-Tone Technique
- Non-Standard Chromatic Scales
- Modular Arithmetic
- Inversion and Transposition of Rows

# Common Confusions
- The formula entry$(i,j) = a_j - a_i$ is the same regardless of $n$; only the modular arithmetic changes
- Composing with an n-tone row chart requires physically detuning a synthesizer (unless $n$ divides 12)
- The musical results may sound very different from 12-tone music due to the unfamiliar interval sizes

# Source Reference
Chapter 7, "Creating an n-Tone Row Chart Using Modular Arithmetic" section, p. 82 (PDF)
