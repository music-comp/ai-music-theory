---
concept: Converting Ratios to Semitones
category: technique
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
---

# Quick Definition
An interval ratio r is converted to semitones by the formula x = 12 * log_2(r). This provides the additive semitone measurement of any multiplicative interval ratio.

# Formal Definition
Given an interval ratio r in R+, its measurement in semitones is:

x = 12 * log_2(r)    (Formula 5.2)

This follows from solving r = 2^(x/12) for x. Alternatively, the conversion can be expressed using an appropriate logarithm base: since r = (2^(1/12))^x, we have x = log_{2^(1/12)}(r), i.e., the base-s logarithm where s = 2^(1/12) is the semitone ratio.

# Mathematical Context
The formula x = 12 * log_2(r) is the composition of log_2 (ratio to octaves) with the scalar 12 (octaves to semitones). It relates to the cents formula by the factor 100: 12 * log_2(r) = (1/100) * 1200 * log_2(r), confirming that 1 semitone = 100 cents. The function is a group homomorphism from (R+, *) to (R, +).

# Musical Context
Converting to semitones tells a musician which chromatic interval best approximates a given ratio. For non-integer results, the nearest integer gives the best chromatic approximation, and the fractional part (converted to cents by multiplying by 100) gives the deviation from equal temperament.

# Examples
- Ratio 3/2: x = 12 * log_2(3/2) ~ 7.02 semitones (close to 7 semitones = a fifth)
- Ratio 2: x = 12 * log_2(2) = 12 semitones (one octave)
- Ratio 5/4: x = 12 * log_2(5/4) ~ 3.86 semitones (close to 4 semitones = a major third)
- The fractional part indicates deviation from the nearest equal-tempered interval

# Related Concepts
- Converting Ratios to Cents
- Converting Ratios to Octaves
- Semitone Ratio
- Change of Base Formula

# Common Confusions
- The result is generally not an integer; only equal-tempered intervals give exact integers
- The formula x = 12 * log_2(r) is equivalent to x = 1200 * log_2(r) / 100 (i.e., cents / 100)
- Negative results indicate downward intervals

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," p. 71.
