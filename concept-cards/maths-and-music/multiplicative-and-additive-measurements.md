---
concept: Multiplicative and Additive Measurements
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
Interval measurement by frequency ratio is called multiplicative (juxtaposition = multiplication), while measurement in semitones, cents, or octaves is called additive (juxtaposition = addition). The logarithm converts between the two.

# Formal Definition
A multiplicative measurement assigns to each interval a positive real number r in R+, where juxtaposing intervals corresponds to multiplying their measurements. An additive measurement assigns a real number x in R, where juxtaposing intervals corresponds to adding their measurements. The relationship between the two is: if r is the multiplicative measurement and x is the additive measurement in some unit, then r = 2^(x/n) for appropriate n, and x = n * log_2(r).

# Mathematical Context
The multiplicative group (R+, *) and the additive group (R, +) are isomorphic via the logarithm: log: (R+, *) -> (R, +). This isomorphism transforms multiplication to addition, division to subtraction, and exponentiation to scalar multiplication. The choice of logarithm base (or equivalently, the value of n) determines the unit of additive measurement: n = 1 gives octaves, n = 12 gives semitones, n = 1200 gives cents.

# Musical Context
Musicians naturally think additively: "2 semitones plus 3 semitones equals 5 semitones," "a fifth is a major third plus a minor third," "a semitone is a major sixth minus a minor sixth." The ratio framework is more fundamental mathematically, but the additive framework is more intuitive musically. The conversion between the two systems is the central theme connecting Chapters 4 and 5.

# Examples
- Additive: 4 semitones + 3 semitones = 7 semitones (major third + minor third = fifth)
- Multiplicative: 2^(4/12) * 2^(3/12) = 2^(7/12) (same calculation with ratios)
- Additive: a fifth (7 semitones) minus a major third (4 semitones) = a minor third (3 semitones)
- Multiplicative: 2^(7/12) / 2^(4/12) = 2^(3/12) (same as above)

# Related Concepts
- Multiplicative Composition of Intervals
- Interval as Frequency Ratio
- Semitone Ratio
- Cents
- Converting Ratios to Cents
- Logarithm Properties

# Common Confusions
- "Multiplicative" and "additive" describe how intervals combine, not their size
- The ratio framework is not an alternative to semitones/cents -- it is the underlying reality that semitones/cents are derived from
- The logarithm is the bridge between the two frameworks, not a separate concept

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 59-60.
