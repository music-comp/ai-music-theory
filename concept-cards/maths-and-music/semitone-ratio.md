---
concept: Semitone Ratio
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
unit: null
---

# Quick Definition
In equal temperament, the semitone has the ratio s = 2^(1/12) = the twelfth root of 2, approximately 1.05946. An interval of x semitones has ratio 2^(x/12).

# Formal Definition
Let s denote the ratio of one semitone. Since twelve semitones compose to one octave (ratio 2), we require s^12 = 2, giving s = 2^(1/12). More generally, the interval of x semitones (for any x in R, not necessarily an integer) has ratio:

r = 2^(x/12)    (Formula 4.2)

This extends naturally from the integer case (2^(1/12))^n = 2^(n/12) to all real x by continuity of the exponential function.

# Mathematical Context
The formula r = 2^(x/12) defines the exponential function that converts additive semitone measurement to multiplicative ratio measurement. Its domain is all of R (allowing non-integer and negative semitone values) and its range is R+ (all positive reals). The function is strictly increasing, one-to-one, and onto R+. The inverse function (converting ratios back to semitones) requires the logarithm: x = 12 * log_2(r).

# Musical Context
Equal temperament divides the octave into 12 equal semitones, each with the same ratio 2^(1/12). This system allows free modulation between keys because all semitones are identical. The equal-tempered semitone ratio enables calculation of the frequency of any keyboard note given a reference pitch (e.g., A4 = 440 Hz).

# Examples
- Semitone ratio: s = 2^(1/12) ~ 1.05946
- Major third (4 semitones): 2^(4/12) = 2^(1/3) = cube root of 2 ~ 1.25992
- Down a minor third (-3 semitones): 2^(-3/12) = 2^(-1/4) = 1/fourth root of 2 ~ 0.840896
- C#4 frequency: 220 * 2^(1/3) ~ 277.18 Hz (a major third above A3)

# Related Concepts
- Interval as Frequency Ratio
- Multiplicative Composition of Intervals
- N-Chromatic Units
- Cents
- Converting Ratios to Semitones

# Common Confusions
- The semitone ratio is 2^(1/12) ~ 1.0595, not 1/12 or 2/12
- The formula works for non-integer x, allowing measurement of intervals that are not whole semitones
- "Equal temperament" means equal ratios between semitones, not equal frequency differences

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 60-61.
