---
concept: Z12 as Chromatic Interval Group
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
The identification of the group $\mathbb{Z}_{12}$ with the set of modular chromatic intervals, making it the central algebraic object for chromatic music theory under octave equivalence.

# Formal Definition
The group $(\mathbb{Z}_{12}, +)$ is identified with the group of modular chromatic intervals by mapping $[k]$ to the interval of $k$ semitones modulo octave. The elements $[0]$ through $[11]$ correspond to the 12 distinct chromatic intervals: unison, semitone, whole step, minor third, major third, fourth, tritone, fifth, minor sixth, major sixth, minor seventh, major seventh. Addition in $\mathbb{Z}_{12}$ corresponds to composition of intervals.

# Mathematical Context
This identification works because: (1) keyboard intervals are measured in whole semitones (elements of $\mathbb{Z}$); (2) octave equivalence means $k \equiv \ell \pmod{12}$; (3) the resulting quotient group is $\mathbb{Z}_{12}$. The group is cyclic with $\phi(12) = 4$ generators: $[1]$ (semitone), $[5]$ (fourth), $[7]$ (fifth), $[11]$ (major seventh). As a ring, $\mathbb{Z}_{12}$ is NOT an integral domain since $12$ is composite (e.g., $[3] \cdot [4] = [0]$).

# Musical Context
$\mathbb{Z}_{12}$ is arguably the single most important algebraic structure in chromatic music theory. It governs interval arithmetic, twelve-tone row charts, pitch-class set theory, and the theory of transposition and inversion. The four generators correspond to the four interval types that cycle through all 12 note classes, with the circle of fifths ($[7]$) being the most musically prominent.

# Examples
- $[5] + [7] = [0]$: fourth + fifth = octave (unison mod octave)
- $[3] + [3] + [3] + [3] = [0]$: four minor thirds = octave (diminished seventh chord)
- $[4] + [4] + [4] = [0]$: three major thirds = octave (augmented triad)
- $[3] \cdot [4] = [12] = [0]$: a zero divisor, showing $\mathbb{Z}_{12}$ is not an integral domain
- The generators $[1], [5], [7], [11]$ come in inverse pairs: $[1]$ and $[11]$, $[5]$ and $[7]$

# Related Concepts
- Modular Chromatic Intervals
- Modular Integers
- Cyclic Group and Generator
- Generating Interval
- Twelve-Chromatic Scale
- Row Chart

# Common Confusions
- $\mathbb{Z}_{12}$ models chromatic intervals modulo octave, not pitches per se; the same structure applies regardless of which note is assigned to $[0]$
- $\mathbb{Z}_{12}$ as a ring has zero divisors (e.g., $[3] \cdot [4] = [0]$), so it is NOT an integral domain
- The musical relevance of $\mathbb{Z}_{12}$ depends on equal temperament; in other tuning systems, this specific group structure does not apply

# Source Reference
Chapter 7, "The Group of Modular Chromatic Intervals" section, p. 82 (PDF)
