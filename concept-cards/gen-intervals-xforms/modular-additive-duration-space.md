---
concept: Modular Additive Duration Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Modular additive duration space is a GIS that rescues additive duration intervals by wrapping them around an M-hour clock, giving meaning to "negative" duration-classes.

# Formal Definition
In Example 2.2.6, we restrict to durations that are positive integral multiples of a basic unit, then wrap them around an M-hour clock. Two durations belong to the same duration-class if their lengths differ by some integral multiple of M. The interval int(s, t) = (t - s) mod M. This forms a valid GIS because "negative" duration-classes have meaning: "-5 mod M" equals "M - 5".

# Mathematical Formulation
- S = {0, 1, 2, ..., M-1} (M duration-classes)
- IVLS = ZM = integers under addition mod M
- int(s, t) = (t - s) mod M
- Duration-class s = durations of length s, s + M, s + 2M, ... units
- Negative is reinterpreted: -5 mod M = M - 5

# Musical Context/Application
This GIS gives mathematical structure to rhythmic analysis with a fixed "measure" size M. A duration-class represents all durations that are equivalent modulo M time units. The interval between duration-classes measures how much longer (mod M) one is than another. This can model rhythmic patterns that repeat every M units.

# Examples
From Example 2.2.6, with M = 16 (time unit = sixteenth note):
- Duration-class s = 8 (half note, give or take whole notes)
- Duration-class t = 4 (quarter note, give or take whole notes)
- int(s, t) = 4 - 8 = -4 = 12 mod 16
- Interpretation: "A quarter note, tied to an extra whole note, is a dotted half longer than a half note"

Arithmetic: -4 mod 16 = 12. So int(8, 4) = 12.

This is the rhythmic analog of pitch-class intervals: just as going "down 4 semitones" from C gives G# (= up 8 semitones mod 12), going "-4 units" in duration gives "+12 units mod 16".

# Related Concepts
- Additive Duration Space
- Duration-Class Space
- Beat-Class Space
- Pitch-Class Space
- Integers Mod M

# Common Confusions
- This rescues Example 2.2.5 by using modular arithmetic
- "Negative" duration-classes make sense: -5 mod M = M - 5
- The modulus M is the "measure" or cycle length
- This differs from 2.2.4 (multiplicative) - here intervals are additive

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.6, Section 2.4
