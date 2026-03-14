---
concept: Octave Equivalence Formalized
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
The mathematical formalization of the musical practice of treating notes separated by octaves as equivalent, using modular arithmetic and equivalence classes.

# Formal Definition
Octave identification equates notes whose frequencies differ by a factor of $2^m$ for integer $m$. Under additive measurement in semitones, two intervals $k$ and $\ell$ are octave-equivalent if $k - \ell$ is a multiple of 12, i.e., $k \equiv \ell \pmod{12}$. The 12 note classes correspond to the 12 elements of $\mathbb{Z}_{12}$: C = [0], C$\sharp$ = [1], D = [2], ..., B = [11].

# Mathematical Context
Octave equivalence is an equivalence relation on the set of all pitches (or intervals), partitioning them into 12 equivalence classes. Going up an octave becomes the identity operation: $[12] = [0]$ in $\mathbb{Z}_{12}$. Interval composition becomes addition in $\mathbb{Z}_{12}$: a fourth + a fifth = $[5] + [7] = [12] = [0]$ (unison). Two fifths = $[7] + [7] = [14] = [2]$ (a step).

# Musical Context
Musical notation implicitly equates notes differing by octaves. A C in any register is "the same note" as any other C. This practice is nearly universal across musical traditions and stems from the 2:1 frequency ratio being perceived as a strong consonance. Under octave equivalence, the chromatic scale contains exactly 12 distinct note classes, and all interval arithmetic reduces to operations in $\mathbb{Z}_{12}$.

# Examples
- Numbering from C: C = 0, C$\sharp$/D$\flat$ = 1, D = 2, D$\sharp$/E$\flat$ = 3, E = 4, F = 5, F$\sharp$/G$\flat$ = 6, G = 7, G$\sharp$/A$\flat$ = 8, A = 9, A$\sharp$/B$\flat$ = 10, B = 11
- Minor third + octave + fourth = $[3] + [12] + [5] = [3] + [0] + [5] = [8]$ (augmented fifth), since $20 \equiv 8 \pmod{12}$
- Fourth + fifth = unison (modulo octave): $[5] + [7] = [0]$

# Related Concepts
- Modular Equivalence on the Integers
- Modular Chromatic Intervals
- Modular Integers
- Group of Modular Intervals
- Wrapping Real Line Around Circle

# Common Confusions
- Octave equivalence is a convention, not a physical necessity; it reflects the perceptual similarity of octave-related pitches
- Under octave equivalence, "going up a fourth" and "going down a fifth" are the same operation: $[5] = [-7] = [5]$ in $\mathbb{Z}_{12}$
- Enharmonic equivalence (C$\sharp$ = D$\flat$) is a separate convention from octave equivalence

# Source Reference
Chapter 7, "Octave identification" section, p. 82 (PDF)
