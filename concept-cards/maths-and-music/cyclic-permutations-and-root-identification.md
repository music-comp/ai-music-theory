---
concept: Cyclic Permutations and Root Identification
category: theory
source: "Mathematics and Music"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
authors: "David Wright"
---

# Quick Definition
A chord has a discernible root if and only if no non-trivial cyclic permutation of its interval sequence reproduces the original sequence. Chords with fully symmetric interval sequences (like augmented triads and diminished seventh chords) have no inherent root.

# Formal Definition
Given a chord defined by a sequence of modular intervals (a_1, a_2, ..., a_k) in Z_12, a cyclic permutation is a sequence of the form (a_i, a_{i+1}, ..., a_k, a_1, ..., a_{i-1}). The chord has a discernible root if and only if the sequence has no non-trivial cyclic symmetries -- that is, no cyclic permutation other than the identity reproduces the original sequence. When the root is discernible, each note in the chord has a unique role (root, third, fifth, seventh, etc.).

# Mathematical Context
This is a direct application of the theory of cyclic groups. A sequence of length k has at most k cyclic permutations. The number of distinct permutations divides k. If all k permutations are distinct (the symmetry group is trivial), the root is unique. If the sequence has full cyclic symmetry (all permutations are identical), any note can serve as root. For example:
- (4, 3, 5) has 3 distinct permutations: root is discernible
- (4, 4, 4) has only 1 distinct permutation: no discernible root
- (3, 3, 3, 3) has only 1 distinct permutation: no discernible root

# Musical Context
Root identification is fundamental to chord labeling and harmonic analysis. Most common chords (major, minor, seventh, minor seventh, major seventh, half-diminished seventh) have asymmetric interval sequences and therefore unique roots. The augmented triad (4, 4, 4) and diminished seventh chord (3, 3, 3, 3) are the notable exceptions. For these symmetric chords, the root is conventionally assigned based on voicing (lowest note) or spelling.

# Examples
- Major triad (4, 3, 5): permutations are (4,3,5), (3,5,4), (5,4,3) -- all distinct, root is unique
- Augmented triad (4, 4, 4): all permutations are (4,4,4) -- no discernible root
- Diminished seventh (3, 3, 3, 3): all permutations are (3,3,3,3) -- no discernible root
- Seventh chord (4, 3, 3, 2): all four permutations are distinct -- root is unique

# Related Concepts
- Chord Types and Interval Sequences
- Augmented Triad
- Diminished Seventh Chord
- Chord Labeling
- Chord Spelling

# Common Confusions
- Having repeated intervals does not automatically mean no root: (3, 4, 3, 2) has a repeated 3 but all cyclic permutations are distinct
- The root is a structural property of the interval sequence, not just "the bottom note"
- For rootless chords, spelling conventions or voicing context assign a practical root

# Source Reference
Chapter 3: "Harmony and Related Numerology," pp. 46-49.
