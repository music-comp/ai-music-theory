---
concept: Logarithm Properties
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
Logarithms transform multiplication into addition and division into subtraction. These properties (L1-L3) are the mathematical reason logarithms are essential for converting between multiplicative (ratio) and additive (semitone/cent) interval measurements.

# Formal Definition
For any positive reals x, y and any real number p, with base b > 0, b != 1:

(L1) log_b(xy) = log_b(x) + log_b(y)
(L2) log_b(x/y) = log_b(x) - log_b(y)
(L3) log_b(x^p) = p * log_b(x)

Property (L1) derives from the law of exponents b^(s+t) = b^s * b^t: letting s = log_b(x) and t = log_b(y), we have b^(s+t) = b^s * b^t = x * y, so s + t = log_b(xy).

# Mathematical Context
These three properties characterize the logarithm as a group homomorphism from (R+, *) to (R, +). Property (L1) is the homomorphism condition; (L2) follows from (L1) and the inverse property; (L3) extends (L1) to repeated multiplication and generalizes to all real exponents. Together they show that logarithms convert:
- Multiplication -> Addition
- Division -> Subtraction
- Exponentiation -> Scalar multiplication

# Musical Context
Property (L2) is particularly significant: it ensures that if pitches x and y have the same interval ratio as x' and y', then log_b(x) - log_b(y) = log_b(x') - log_b(y'). This means that on a logarithmic pitch axis, equal intervals appear as equal distances -- exactly the behavior musicians expect. This is why logarithmic scales are natural for representing pitch.

# Examples
- (L1): log_2(3 * 5) = log_2(3) + log_2(5) -- composing a "3-ratio" interval with a "5-ratio" interval
- (L2): log_2(3/2) = log_2(3) - log_2(2) = log_2(3) - 1 -- used in the worked example converting 3/2 to cents
- (L3): log_2(r^2) = 2 * log_2(r) -- doubling an interval (in the multiplicative sense) doubles its additive measurement

# Related Concepts
- Logarithmic Functions as Inverses
- Exponents and Exponential Functions
- Converting Ratios to Cents
- Logarithmic Pitch Scale
- Multiplicative-to-Additive Conversion

# Common Confusions
- log_b(x + y) != log_b(x) + log_b(y) -- the logarithm converts multiplication (not addition) to addition
- log_b(x * y) is not log_b(x) * log_b(y)
- Property (L3) requires the exponent p to be outside the logarithm, not inside: log_b(x^p) = p * log_b(x), not (log_b(x))^p

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 68-69.
