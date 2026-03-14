---
concept: Cyclic Permutations
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
unit: null
---

# Quick Definition
A rearrangement of a finite sequence obtained by moving elements from the beginning to the end, equivalent to rotating the sequence on a circular arrangement.

# Formal Definition
Given a finite sequence $x_1, x_2, \ldots, x_n$, a cyclic permutation is obtained by choosing an integer $i$ with $1 \leq i \leq n$, taking entries $x_1, \ldots, x_i$ from the beginning and placing them at the end, yielding $x_{i+1}, x_{i+2}, \ldots, x_n, x_1, x_2, \ldots, x_i$. The case $i = n$ returns the original sequence. The cyclic permutations for $i = 1, \ldots, n-1$ are called non-trivial cyclic permutations.

# Mathematical Context
Cyclic permutations can be visualized by arranging the sequence on a clock with $n$ positions in clockwise fashion with $x_1$ at the top, then rotating by $i$ positions. A sequence can be a non-trivial cyclic permutation of itself if it has internal periodicity (e.g., the sequence $3, 5, 3, 3, 5, 3$ is invariant under the permutation with $i = 3$). This concept relates to cyclic groups in abstract algebra.

# Musical Context
Cyclic permutations are the mathematical foundation for understanding modal scales. The seven ecclesiastical modes are obtained as cyclic permutations of the standard (Ionian) diatonic scale. The interval sequence $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ has no non-trivial cyclic permutation equal to itself, which is why the seven modal permutations produce seven distinct modes.

# Examples
- The sequence $7, 4, 1, 7$ has cyclic permutations: $4, 1, 7, 7$ (i=1), $1, 7, 7, 4$ (i=2), $7, 7, 4, 1$ (i=3), and $7, 4, 1, 7$ (i=4, trivial)
- The sequence $3, 5, 3, 3, 5, 3$ is a non-trivial cyclic permutation of itself (using $i = 3$)
- The Dorian mode is the cyclic permutation of the Ionian scale starting on the second note: D E F G A B C (intervals: $1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}, 1$)
- Exercise 8: an eight-note scale with only whole and half steps can be a non-trivial cyclic permutation of itself (e.g., the octatonic scale)

# Related Concepts
- Ecclesiastical Modes
- Diatonic and Chromatic Scales
- Modality and Key
- Major and Minor Modes

# Common Confusions
- The identity permutation ($i = n$) is trivial -- every sequence is a cyclic permutation of itself
- A sequence being a non-trivial cyclic permutation of itself requires internal periodicity; the major scale interval sequence does NOT have this property
- Cyclic permutations preserve the circular ordering but change the starting point, which is exactly what happens when changing the mode of a scale

# Source Reference
Chapter 1, "Cyclic Permutations" section, pp. 25-26 (PDF)
