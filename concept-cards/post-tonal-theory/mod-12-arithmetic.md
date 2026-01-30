---
concept: Mod 12 Arithmetic
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 17
unit: null
authors: Joseph N. Straus
---

# Quick Definition
Arithmetic modulo 12 is a number system where any integer can be reduced to a value from 0 to 11 by adding or subtracting multiples of 12, reflecting the twelve-pitch-class cycle.

# Formal Definition
In modular arithmetic with modulus 12 (abbreviated mod 12), any number larger than 11 or smaller than 0 is equivalent to some integer from 0 to 11. To find the equivalent, add or subtract 12 (or any multiple of 12). In a mod 12 system, -12 = 0 = 12 = 24, and similarly -13, -1, 23, and 35 are all equivalent to 11 because they are related to 11 by adding or subtracting 12.

# Mathematical Formulation/Recognition
Basic mod 12 operations:
- Any integer n (mod 12) = n - (12 * floor(n/12)) for positive n
- For negative integers, add 12 until result is in range 0-11
- Examples:
  - 15 mod 12 = 3
  - 27 mod 12 = 3
  - -1 mod 12 = 11
  - -3 mod 12 = 9

The formula for pitch-class equivalence:
- Pitch class = pitch mod 12
- Going up 12 semitones returns to the same pitch class

# Musical Context/Application
Mod 12 arithmetic reflects the cyclical nature of pitch-class space. Going up an octave (adding 12 semitones) or down an octave (subtracting 12 semitones) produces another member of the same pitch class. For example, starting on Eb above middle C (pitch class 3) and going up 12 semitones returns to pitch class 3: 3 + 12 = 15 = 3 (mod 12).

This is analogous to clock time (a mod 12 system where 11 + 1 = 12 = 0) or days of the week (a mod 7 system).

# Examples
The text provides this analogy: "Just as our lives unfold simultaneously in linear and modular time, music unfolds simultaneously in pitch and pitch-class space." If it's eleven o'clock now, it will be eleven o'clock in twelve hours (mod 12). If it's Friday today, it will be Friday again in seven days (mod 7).

# Related Concepts
- Pitch class
- Pitch-class space
- Pitch-class clockface
- Integer notation
- Complementary intervals

# Common Confusions
Students sometimes forget to reduce results to the 0-11 range. When performing calculations with pitch-class integers, always reduce the final answer using mod 12. Also note that while mod 12 arithmetic is useful for pitch-class calculations, pitch space itself is linear (not modular).

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.5, pp. 6-7
