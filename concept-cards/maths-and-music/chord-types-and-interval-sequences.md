---
concept: Chord Types and Interval Sequences
category: theory
source: "Mathematics and Music"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
authors: "David Wright"
unit: null
---

# Quick Definition
A chord's type is determined by its sequence of modular intervals (measured in semitones as elements of Z_12) between successive note classes. Each standard chord type has a unique defining interval sequence.

# Formal Definition
A chord type is an equivalence class of chords determined by the ordered cyclic sequence of intervals (in semitones, modulo 12) between successive note classes. The standard chord types and their defining sequences are:

Triads:
- Major triad: (4, 3, 5)
- Minor triad: (3, 4, 5)
- Diminished triad: (3, 3, 6)
- Augmented triad: (4, 4, 4)

Four-note chords:
- Seventh chord: (4, 3, 3, 2)
- Minor seventh: (3, 4, 3, 2)
- Major seventh: (4, 3, 4, 1)
- Diminished seventh: (3, 3, 3, 3)
- Half-diminished seventh: (3, 3, 4, 2)

Each sequence sums to 12 (the octave).

# Mathematical Context
A chord type is formally a cyclic sequence of positive integers summing to 12, considered up to cyclic equivalence of sequences that differ only in starting position. Whether a chord type admits a unique root depends on whether the sequence has non-trivial cyclic symmetries. The nine standard chord types above represent the most musically significant partitions of 12 into 3 or 4 parts, ordered cyclically.

# Musical Context
Chord types form the vocabulary of harmony. The interval sequence tells a musician exactly what intervals to stack from the root. For example, a seventh chord (4, 3, 3, 2) means: start at the root, go up 4 semitones (major third) to the third, up 3 more (minor third) to the fifth, up 3 more (minor third) to the seventh, and the remaining 2 semitones (major second) return to the root. Later chapters examine the mathematical basis for why certain chord types sound consonant and others dissonant.

# Examples
- Major triad (4, 3, 5): root to third is a major third (4), third to fifth is a minor third (3), fifth back to root is a perfect fourth (5)
- Seventh chord (4, 3, 3, 2): contains the major triad (4, 3, 5) with the fifth split as 3 + 2
- The augmented triad (4, 4, 4) and diminished seventh (3, 3, 3, 3) have uniform interval sequences

# Related Concepts
- Major Triad
- Minor Triad
- Diminished Triad
- Augmented Triad
- Seventh Chord
- Cyclic Permutations and Root Identification
- Modular Arithmetic and Intervals

# Common Confusions
- The intervals in the sequence are between *successive* note classes in the chord, not all measured from the root
- The sum of all intervals in the sequence must equal 12 (completing the octave)
- Two different chord types may share some interval values but differ in their ordering

# Source Reference
Chapter 3: "Harmony and Related Numerology," pp. 45-49.
