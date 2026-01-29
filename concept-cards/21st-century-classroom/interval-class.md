---
concept: Interval Class
category: theory
source: Music Theory for the 21st-Century Classroom
chapter: "Chapter 33: Set Theory"
unit: 12
author: Robert Hutchinson
---

# Quick Definition
An interval class is the shortest distance between two pitch classes, measured in semitones, ranging from 1 to 6.

# Formal Definition
An interval class (abbreviated "ic") represents the shortest distance between two pitch classes, measured in semitones. Since an interval and its inversion (complement to 12) span the same pitch-class distance, interval class uses the smaller of the two. The largest interval class is 6 (the tritone), as any larger interval has a smaller inversion.

# Construction/Calculation
Interval class mappings (interval -> ic):
- m2/M7 -> ic1 (1 or 11 semitones -> use 1)
- M2/m7 -> ic2 (2 or 10 semitones -> use 2)
- m3/M6 -> ic3 (3 or 9 semitones -> use 3)
- M3/m6 -> ic4 (4 or 8 semitones -> use 4)
- P4/P5 -> ic5 (5 or 7 semitones -> use 5)
- Tritone -> ic6 (6 semitones, its own inversion)

To find interval class:
1. Count semitones between two pitch classes
2. If result > 6, subtract from 12
3. Result is the interval class

# Musical Context
Interval class simplifies intervallic analysis by treating complementary intervals as equivalent. In atonal music, a major 6th and minor 3rd have similar sonic functions despite their different sizes, so they share interval class 3. This equivalence is fundamental to interval vector calculation and set theory analysis.

# Examples
- C to E: 4 semitones = ic4
- C to A: 9 semitones -> 12-9 = 3 = ic3 (not ic9)
- C to G: 7 semitones -> 12-7 = 5 = ic5 (use P4, not P5)
- C to F#: 6 semitones = ic6 (tritone)

# Related Concepts
- Interval vector
- Semitones
- Pitch class
- Complementary intervals

# Common Confusions
- Interval class is NOT the same as interval - it uses the smaller inversion
- Maximum interval class is 6 (the tritone)
- P5 (7 semitones) is ic5 because P4 (5 semitones) is smaller
- M6 (9 semitones) is ic3 because m3 (3 semitones) is smaller

# Source Reference
Chapter 33: Set Theory, Unit 12, Section 33.4 Interval Vector
