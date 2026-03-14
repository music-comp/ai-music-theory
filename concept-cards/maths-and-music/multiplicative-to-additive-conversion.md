---
concept: Multiplicative-to-Additive Conversion
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
The logarithm is the mathematical tool that converts multiplicative interval measurements (frequency ratios) to additive measurements (octaves, semitones, cents), and vice versa via the exponential. This conversion is the central theme of Chapters 4-5.

# Formal Definition
The complete conversion framework between multiplicative and additive interval measurements:

Ratio to additive:
- Octaves: x = log_2(r)
- Semitones: x = 12 * log_2(r)
- Cents: x = 1200 * log_2(r)
- General n-chromatic units: x = n * log_2(r)

Additive to ratio:
- From octaves: r = 2^x
- From semitones: r = 2^(x/12)
- From cents: r = 2^(x/1200)
- From n-chromatic units: r = 2^(x/n)

The exponential and logarithm are inverse functions, so these conversions are exact inverses of each other.

# Mathematical Context
The pair of functions (exp: R -> R+, log: R+ -> R) form an isomorphism between the additive group (R, +) and the multiplicative group (R+, *). Choosing the base determines the scaling:
- Base 2: octaves as unit
- Base 2^(1/12): semitones as unit
- Base 2^(1/1200): cents as unit

The conversion x = n * log_2(r) can also be written as x = log_b(r) where b = 2^(1/n), the ratio of one n-chromatic unit.

# Musical Context
This conversion framework unifies the two ways musicians think about intervals. When a musician says "a fifth plus a fourth equals an octave," they are working additively (7 + 5 = 12 semitones). When a physicist says "the frequency ratio 3/2 times 4/3 equals 2," they are working multiplicatively. The logarithm shows these are the same statement: log_2(3/2) + log_2(4/3) = log_2(2) = 1 octave.

# Examples
- Ratio 3/2 -> 1200 * log_2(3/2) ~ 701.955 cents -> best chromatic approximation is 7 semitones (a fifth), error ~ 2 cents
- 7 semitones -> 2^(7/12) ~ 1.4983 (equal-tempered fifth ratio)
- The "circle" is complete: start with a ratio, convert to cents, convert back to ratio, recover the original

# Related Concepts
- Logarithm Properties
- Exponents and Exponential Functions
- Converting Ratios to Cents
- Converting Ratios to Semitones
- Converting Ratios to Octaves
- Multiplicative and Additive Measurements

# Common Confusions
- The conversion is exact (lossless); rounding only occurs when reducing to integer semitones or integer cents
- The logarithm converts multiplicative to additive; the exponential converts additive to multiplicative -- do not confuse the directions
- Base 2 is for octaves, not "because we use binary"

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 70-72.
