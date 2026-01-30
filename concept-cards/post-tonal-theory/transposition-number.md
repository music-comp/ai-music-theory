---
concept: Transposition Number
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
unit: null
authors: Joseph N. Straus
---

# Quick Definition
The transposition number (n in Tn) is the interval of transposition, indicating how many semitones each pitch class is shifted.

# Formal Definition
In the operation Tn, n represents the transposition number, which is the pitch-class interval added to each element of the set or line being transposed. The transposition number ranges from 0 to 11, with T0 representing no change (the identity operation). Complementary transposition numbers (pairs that sum to 12) represent inverse operations that undo each other.

# Mathematical Formulation/Recognition
**Finding the transposition number:**
If two sets X and Y are related by Tn:
- For any corresponding elements x and y: n = (y - x) mod 12
- All pairs of corresponding elements will yield the same n

**Inverse relationships:**
- Tn and T(12-n) are inverses
- T3 and T9, T4 and T8, T5 and T7, etc.
- T0 and T6 are their own inverses (T6 + T6 = T0)

**Testing for transposition:**
Put both sets in normal form and check:
1. Same interval succession? (If yes, they're transpositionally related)
2. Calculate n from any pair of corresponding elements

# Musical Context/Application
The choice of transposition level often has compositional significance. Composers frequently choose transposition numbers that duplicate intervals found within the original set, creating unity between small-scale and large-scale intervallic relationships.

# Examples
**Example 2-11** (Stravinsky, Agon): Two sets in normal form with identical interval succession 1-2-1. Comparing corresponding elements: 10-7=3, 11-8=3, 1-10=3 (mod 12), 2-11=3. Therefore n = 3, and Set 2 = T3(Set 1).

# Related Concepts
- Transposition (Tn)
- Normal form
- Index number (for inversion)
- Inverse operations
- Mod 12 arithmetic

# Common Confusions
The transposition number is always expressed as a positive integer 0-11 (using mod 12). While T7 and T(-5) are equivalent, the convention is to use the positive form. Remember that T0 is the identity (no change) and T6 maps onto the tritone transposition.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3, pp. 46-53
