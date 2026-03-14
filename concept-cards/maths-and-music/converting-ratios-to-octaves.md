---
concept: Converting Ratios to Octaves
category: technique
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
An interval ratio r is converted to octaves by the formula x = log_2(r). This is the most fundamental additive measurement, from which semitones (multiply by 12) and cents (multiply by 1200) are derived.

# Formal Definition
Given an interval ratio r in R+, its measurement in octaves is:

x = log_2(r)    (Formula 5.3)

This is the base case from which all other conversions derive. To obtain the octave as the unit interval (distance 1 on a logarithmic axis), we need log_b(2) = 1, which requires b = 2. Therefore base 2 is the natural base for musical logarithms.

# Mathematical Context
The choice of base 2 is determined by the requirement that the octave (ratio 2) should correspond to exactly 1 unit. Since log_b(2) = 1 implies b^1 = 2, i.e., b = 2, base 2 is the unique base for which the octave maps to 1. The formulas for semitones (12 * log_2(r)) and cents (1200 * log_2(r)) are simply rescalings of this fundamental measurement.

# Musical Context
Measuring in octaves is the coarsest standard measurement. It answers the question: "How many octaves does this interval span?" For most practical purposes, the finer measurements (semitones or cents) are more useful, but the octave measurement is conceptually the simplest and serves as the foundation. Plotting pitches by log_2(frequency) produces equally spaced octaves on the axis.

# Examples
- Ratio 2: log_2(2) = 1 octave
- Ratio 4: log_2(4) = 2 octaves
- Ratio 3/2: log_2(3/2) ~ 0.585 octaves (a fifth is slightly more than half an octave)
- Ratio 1: log_2(1) = 0 octaves (unison)
- Ratio 1/2: log_2(1/2) = -1 octave (down one octave)

# Related Concepts
- Converting Ratios to Semitones
- Converting Ratios to Cents
- Logarithmic Pitch Scale
- Logarithmic Functions as Inverses

# Common Confusions
- log_2 is the musically natural base, not log_10 or ln -- though the change of base formula allows using any base for computation
- The octave measurement is not commonly used in practice (semitones and cents are preferred), but it is the conceptual foundation
- A non-integer octave measurement does not mean "part of an octave" in the sense of a fraction; it means the interval is not a whole number of octaves

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 70-71.
