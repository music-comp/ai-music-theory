---
concept: Modular Arithmetic and Intervals
category: theory
source: "Mathematics and Music"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
authors: "David Wright"
unit: null
---

# Quick Definition
Modular arithmetic in Z_12 provides the mathematical framework for octave equivalence, where two intervals differing by any number of octaves are treated as the same modular interval.

# Formal Definition
For a fixed positive integer n, the relation k ≡ l (mod n), defined by n | (k - l), is an equivalence relation on Z. When n = 12, this captures octave identification for intervals measured in semitones: two intervals of k and l semitones are equivalent modulo octave if and only if k ≡ l (mod 12). The set of equivalence classes is denoted Z_12, containing exactly 12 classes represented by {0, 1, 2, ..., 11}.

# Mathematical Context
Each equivalence class in Z_12 contains a unique representative r with 0 <= r < 12, obtained as the remainder in the Division Algorithm with n = 12. The set Z_n of equivalence classes inherits well-defined addition from Z. This structure is fundamental to analyzing chords and their interval sequences, where intervals between note classes are elements of Z_12.

# Musical Context
Octave equivalence is one of the most fundamental principles in music: notes separated by octaves are perceived as "the same" in an important sense. Modular arithmetic formalizes this. A ninth (14 semitones) is equivalent to a second (2 semitones) since 14 ≡ 2 (mod 12). Going down a fourth (-5 semitones) is equivalent to going up a fifth (7 semitones) since -5 ≡ 7 (mod 12).

# Examples
- A ninth (14 semitones) ≡ a second (2 semitones) since 14 ≡ 2 (mod 12)
- Down a fourth (-5 semitones) ≡ up a fifth (7 semitones) since -5 ≡ 7 (mod 12)
- The interval from Eb to B is represented by 8 semitones (a minor sixth) in Z_12
- There is a well-defined interval class between any ordered pair of note classes

# Related Concepts
- Chords as Note Class Collections
- Chord Types and Interval Sequences
- Cyclic Permutations and Root Identification
- Octave Equivalence of Interval Ratios

# Common Confusions
- Z_12 elements represent interval *classes*, not specific intervals -- a major ninth and a major second are different intervals but the same element of Z_12
- The "modular interval" is always represented by a non-negative value less than 12
- Modular intervals apply to note *classes*, not to specific pitches in a voicing

# Source Reference
Chapter 3: "Harmony and Related Numerology," pp. 44-45.
