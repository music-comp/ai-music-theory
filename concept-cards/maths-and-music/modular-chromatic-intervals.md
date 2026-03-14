---
concept: Modular Chromatic Intervals
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
Chromatic intervals considered modulo octave, forming equivalence classes identified with elements of $\mathbb{Z}_{12}$ (or $\mathbb{Z}_n$ for non-standard scales), with composition given by addition in the group.

# Formal Definition
A modular chromatic interval is an equivalence class of keyboard intervals under octave equivalence. Since keyboard intervals are measured in whole semitones, the set of modular chromatic intervals is identified with $\mathbb{Z}_{12}$. Each modular chromatic interval has a unique representative $n$ semitones with $0 \leq n \leq 11$. The law of composition is addition in $\mathbb{Z}_{12}$, and iteration of intervals corresponds to a sequence of rotations on the modular clock.

# Mathematical Context
The group $(\mathbb{Z}_{12}, +)$ is the group of modular chromatic intervals. It is cyclic with generators $[1], [5], [7], [11]$. For non-standard chromatic scales dividing the octave into $n$ equal parts, modular chromatic intervals form $\mathbb{Z}_n$. The modular clock provides a visualization where addition corresponds to clockwise rotation.

# Musical Context
Under octave equivalence, there are exactly 12 distinct chromatic intervals. Every chromatic interval composition reduces to addition in $\mathbb{Z}_{12}$. For example, ascending by a minor third (3), an octave (12 = 0), and a fourth (5) gives $[3] + [0] + [5] = [8]$, an augmented fifth. The modular clock labeled with note names allows quick conversion between numerical and musical representations.

# Examples
- Semitone = [1], whole step = [2], minor third = [3], major third = [4], fourth = [5], tritone = [6], fifth = [7], minor sixth = [8], major sixth = [9], minor seventh = [10], major seventh = [11], unison/octave = [0]
- [3] + [12] + [5] = [8] in $\mathbb{Z}_{12}$: minor third + octave + fourth = augmented fifth
- Six fifths: $6 \cdot [7] = [42] = [6]$ in $\mathbb{Z}_{12}$ (a tritone)
- Up three minor thirds, down six steps: $3 \cdot [3] + (-6) \cdot [2] = [9] + [-12] = [9]$ in $\mathbb{Z}_{12}$

# Related Concepts
- Octave Equivalence Formalized
- Modular Integers
- Modular Arithmetic
- Group of Modular Intervals
- Modular Clock
- Generating Interval

# Common Confusions
- A modular chromatic interval is an equivalence class, not a specific number of semitones; [5] includes 5, 17, 29, -7, etc.
- The unique representative in $\{0, 1, \ldots, 11\}$ is conventional; negative representatives are equally valid
- "Going up 14 semitones" and "going up 2 semitones" are the same modular chromatic interval

# Source Reference
Chapter 7, "The Group of Modular Chromatic Intervals" section, p. 82 (PDF)
