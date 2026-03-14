---
concept: Converting Ratios to Cents
category: technique
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
An interval ratio r is converted to cents by the formula x = 1200 * log_2(r). This is the fundamental formula for expressing any frequency ratio as an additive measurement.

# Formal Definition
Given an interval ratio r in R+, its measurement in cents is:

x = 1200 * log_2(r)    (Formula 5.1)

This is derived by solving r = 2^(x/1200) for x: applying log_2 to both sides gives log_2(r) = x/1200, hence x = 1200 * log_2(r). Using the natural logarithm for computation:

x = 1200 * (ln(r) / ln(2))

If r < 1, then ln(r) < 0, so x is negative, correctly indicating a downward interval.

# Mathematical Context
This formula is the composition of log_2 (converting ratio to octaves) with the scalar 1200 (converting octaves to cents). It is a group homomorphism from (R+, *) to (R, +), mapping the multiplicative structure of ratios to the additive structure of cents. The formula inherits the logarithm properties: log of a product becomes sum (composing intervals), log of a quotient becomes difference.

# Musical Context
This conversion is the workhorse formula for comparing intervals across different tuning systems, measuring deviations from equal temperament, and expressing the sizes of just intervals. It answers questions like "how many cents is a just fifth?" or "how far off is the Pythagorean third from the equal-tempered third?"

# Examples
- Ratio 3/2: x = 1200 * (ln(3/2) / ln(2)) = 1200 * ((ln 3 - ln 2) / ln 2) = 1200 * (ln 3/ln 2 - 1) ~ 701.955 cents (close to a fifth at 700 cents, 2 cents sharp)
- Ratio 2: x = 1200 * log_2(2) = 1200 cents (one octave, by definition)
- Ratio 1: x = 1200 * log_2(1) = 0 cents (unison)
- If r < 1, x is negative (downward interval)

# Related Concepts
- Converting Ratios to Semitones
- Converting Ratios to Octaves
- Cents
- Change of Base Formula
- Natural Logarithm
- Logarithm Properties

# Common Confusions
- The formula uses log_2, not log_10 or ln directly -- though the change of base formula allows using any logarithm
- Negative cent values indicate downward intervals, not errors
- 1200 is the number of cents per octave; the factor comes from the definition of cents, not from the logarithm

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 70-72.
