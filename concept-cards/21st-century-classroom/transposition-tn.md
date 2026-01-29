---
concept: Transposition (Tn)
category: technique
source: Music Theory for the 21st-Century Classroom
chapter: "Chapter 33: Set Theory"
unit: 12
author: Robert Hutchinson
---

# Quick Definition
Transposition (Tn) is an operation that moves every pitch class in a set up by n semitones, preserving the set's intervallic structure.

# Formal Definition
Transposition in set theory is notated as Tn, where n is the number of semitones by which each pitch class is raised. The operation preserves all intervallic relationships within the set. Calculations use modulo 12 arithmetic, so results greater than 11 are reduced by subtracting 12.

# Construction/Calculation
To transpose a set at Tn:
1. Add n to each pitch class integer
2. If any result exceeds 11, subtract 12 (modulo 12)
3. Result is the transposed set

Example: [1, 2, 4, 6] at T4
- 1 + 4 = 5
- 2 + 4 = 6
- 4 + 4 = 8
- 6 + 4 = 10
- Result: [5, 6, 8, 10]

Example with mod 12: [6, 8, 10, 11] at T9
- 6 + 9 = 15 -> 15 - 12 = 3
- 8 + 9 = 17 -> 17 - 12 = 5
- 10 + 9 = 19 -> 19 - 12 = 7
- 11 + 9 = 20 -> 20 - 12 = 8
- Result: [3, 5, 7, 8]

# Musical Context
Transposition operations are fundamental to set theory analysis and twelve-tone music. Identifying transpositional relationships between sets reveals structural connections in atonal compositions. Two sets related by Tn share the same prime form and interval vector.

# Examples
- [1, 2, 4, 6] at T4 = [5, 6, 8, 10]
- [6, 8, 10, 11] at T9 = [3, 5, 7, 8]
- T0 = no change (identity operation)
- T6 = tritone transposition

# Related Concepts
- Inversion (TnI)
- Modulo 12 arithmetic
- Prime form
- Twelve-tone row transposition

# Common Confusions
- Tn means transpose UP by n semitones
- Always reduce results to 0-11 (modulo 12)
- Transposition preserves prime form and interval vector
- Different from inversion, which flips intervals

# Source Reference
Chapter 33: Set Theory, Unit 12, Section 33.7 Transposition (Tn)
