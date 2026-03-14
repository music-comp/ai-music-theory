---
concept: Octave Equivalence of Interval Ratios
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
Two interval ratios r_1 and r_2 are equivalent modulo octave if they differ by some number of octaves, which in ratio terms means r_1 / r_2 = 2^n for some integer n.

# Formal Definition
Two interval ratios r_1 and r_2 are equivalent modulo octave if and only if there exists n in Z such that r_1 * r_2^(-1) = 2^n. Equivalently, r_1 = r_2 * 2^n. This defines an equivalence relation on R+. Each equivalence class contains a unique representative in the interval [1, 2) (i.e., a non-negative interval less than one octave).

# Mathematical Context
Octave equivalence for ratios is the multiplicative analog of congruence modulo 12 for semitones. The equivalence classes partition R+ by the action of the group {2^n : n in Z} under multiplication. The quotient space R+ / {2^n} is isomorphic to the interval [1, 2) under multiplication modulo 2. Taking log_2 transforms this into the familiar Z_12 framework: log_2(r_1) - log_2(r_2) = n, or equivalently, the additive measurements are congruent modulo 1 (in octaves) or modulo 12 (in semitones).

# Musical Context
This formalizes the musical principle that intervals differing by whole octaves are "the same" in an important sense. A twelfth (octave plus a fifth) and a fifth are equivalent modulo octave. This principle is essential for identifying chord types and note classes across different registers.

# Examples
- Ratios 41 and 328 are equivalent modulo octave since 41/328 = 1/8 = 2^(-3)
- Ratio 3 (approximately an octave + a fifth) is equivalent to 3/2 (a fifth), since 3/(3/2) = 2 = 2^1
- Ratios 5 and 20 are equivalent: 5/20 = 1/4 = 2^(-2)

# Related Concepts
- Modular Arithmetic and Intervals
- Interval as Frequency Ratio
- Multiplicative Composition of Intervals
- Chords as Note Class Collections

# Common Confusions
- Octave equivalence for ratios uses multiplication/division by powers of 2, not addition/subtraction of 12
- Two ratios can be equivalent modulo octave even if neither is itself a power of 2
- The "representative" in [1, 2) corresponds to the smallest upward version of the interval class

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 62-63.
