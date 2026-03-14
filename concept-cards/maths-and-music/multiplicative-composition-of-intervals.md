---
concept: Multiplicative Composition of Intervals
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
When two intervals are juxtaposed (one followed by another), the resulting interval's ratio is the product of the two individual ratios. This multiplicative property is the fundamental arithmetic of interval ratios.

# Formal Definition
If x_1 = f_2/f_1 represents the interval from f_1 to f_2, and x_2 = f_3/f_2 represents the interval from f_2 to f_3, then the composed interval from f_1 to f_3 has ratio x_1 * x_2 = (f_2/f_1)(f_3/f_2) = f_3/f_1. This multiplicativity follows directly from the cancellation of the common frequency f_2.

# Mathematical Context
The multiplicative property makes (R+, *) the natural group for interval arithmetic. Juxtaposition of intervals corresponds to group multiplication; the identity element is 1 (unison); the inverse of r is r^(-1) (opposite interval). This is in contrast to additive measurement systems (semitones, cents, octaves) where juxtaposition corresponds to addition. The transition between multiplicative and additive viewpoints is mediated by the logarithm.

# Musical Context
This property explains why intervals "add" in the usual musical sense when their ratios are multiplied. A major third (ratio 2^(4/12)) followed by a minor third (ratio 2^(3/12)) gives a perfect fifth (ratio 2^(7/12)) because 2^(4/12) * 2^(3/12) = 2^(7/12). The multiplicative framework is more fundamental than the additive one; the additive framework is derived from it via logarithms.

# Examples
- Two octaves: 2 * 2 = 4 (ratio of a double octave)
- Major third + minor third = fifth: 2^(4/12) * 2^(3/12) = 2^(7/12)
- Up a fifth then down a fourth: (3/2)(2/3)... but in equal temperament: 2^(7/12) * 2^(-5/12) = 2^(2/12) = one whole step
- Twelve semitones: (2^(1/12))^12 = 2 (one octave)

# Related Concepts
- Interval as Frequency Ratio
- Semitone Ratio
- Multiplicative and Additive Measurements
- Converting Ratios to Cents
- Logarithm Properties

# Common Confusions
- Intervals compose by multiplication of ratios, not addition -- "a third plus a third equals a fifth" is additive language for a multiplicative operation
- This is why equal temperament uses 2^(1/12) rather than 2/12 for a semitone
- The shift from multiplicative to additive thinking is the key conceptual transition of Chapters 4-5

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 59-60.
