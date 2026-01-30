---
concept: Ordered Pitch-Class Interval
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 17
unit: null
authors: Joseph N. Straus
---

# Quick Definition
An ordered pitch-class interval (opci) is the directed distance between two pitch classes, calculated by counting semitones clockwise on the pitch-class clockface from the first note to the second.

# Formal Definition
An ordered pitch-class interval (abbreviated opci) is the interval between two pitch classes, calculated by counting the number of semitones on the pitch-class clockface from the first note to the second. Clockwise movement is considered equivalent to ascending motion, and counterclockwise to descending. By convention, ordered pitch-class intervals are usually denoted by positive integers from 0 to 11, though negative equivalents are sometimes used for intervals larger than 6.

# Mathematical Formulation/Recognition
Formula for ordered pitch-class interval from pitch class x to pitch class y:
- opci = (y - x) mod 12

Example calculations:
| From | To | Calculation | opci |
|------|-----|-------------|------|
| C# (1) | Eb (3) | 3 - 1 = 2 | 2 |
| Eb (3) | C# (1) | 1 - 3 = -2 = 10 (mod 12) | 10 |
| B (11) | F (5) | 5 - 11 = -6 = 6 (mod 12) | 6 |
| D (2) | Bb (10) | 10 - 2 = 8 | 8 (or -4) |

Negative notation for large intervals:
- 7 = -5, 8 = -4, 9 = -3, 10 = -2, 11 = -1

# Musical Context/Application
Ordered pitch-class intervals preserve directional information within the mod 12 context. They are useful for analyzing melodic motion and transformational relationships. When the order of pitch classes is reversed, the resulting intervals are complements mod 12 (they add up to 12).

# Examples
**Example 1-12**: Calculation examples showing that the ordered pitch-class interval from C# to Eb is 2, while from Eb to C# it is 10 (the complement mod 12).

**Example 1-15** (Schoenberg, String Quartet No. 3): Analysis showing that the first melodic interval (B to Bb) is opci 11, while subsequent intervals C#-D and F-F# are opci 1. As ordered intervals, they differ; as unordered intervals, all three represent interval class 1.

# Related Concepts
- Unordered pitch-class interval
- Complementary intervals (mod 12)
- Pitch-class clockface
- Interval class
- Ordered pitch interval

# Common Confusions
The ordered pitch-class interval from x to y is different from the interval from y to x (they are complements mod 12). Remember that "ascending" means clockwise on the clockface, which represents positive motion. An opci of 11 represents an ascending major seventh or a descending semitone.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.8, pp. 9-11
