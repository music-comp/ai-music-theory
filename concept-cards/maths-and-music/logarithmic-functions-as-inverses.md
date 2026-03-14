---
concept: Logarithmic Functions as Inverses
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
---

# Quick Definition
The logarithm log_b(x) is defined as the inverse of the exponential function b^x. The statement log_b(x) = y means exactly the same as b^y = x.

# Formal Definition
The function g(x) = log_b(x) is the inverse of f(x) = b^x. This means:
- f(g(x)) = x, i.e., b^(log_b(x)) = x
- g(f(x)) = x, i.e., log_b(b^x) = x

The domain of log_b is R+ (positive reals) and its range is R (all reals):

g: R+ -> R

For b > 1, log_b is strictly increasing and one-to-one. Its graph is obtained by reflecting the graph of b^x across the line y = x. The base b is always positive and != 1, usually taken > 1.

# Mathematical Context
The logarithm establishes the inverse isomorphism from (R+, *) to (R, +), reversing the exponential's mapping. If we recognize x as a power of b, we can evaluate log_b(x) directly: log_3(9) = 2 (since 3^2 = 9), log_b(sqrt(b)) = 1/2 (since b^(1/2) = sqrt(b)). The logarithm is the essential tool for converting multiplicative interval ratios to additive measurements.

# Musical Context
The logarithm answers the question: "Given a frequency ratio r, how many semitones (or cents, or octaves) does it represent?" This is the inverse of the question answered by the exponential function. Without logarithms, we can convert semitones to ratios (r = 2^(x/12)) but cannot convert ratios back to semitones. The logarithm completes the conversion toolkit.

# Examples
- log_2(8) = 3 because 2^3 = 8 (the ratio 8 spans 3 octaves)
- log_2(2) = 1 (one octave)
- log_3(9) = 2 (since 3^2 = 9)
- log_b(sqrt(b)) = 1/2 (since b^(1/2) = sqrt(b))
- log_b(1) = 0 for any base (since b^0 = 1)

# Related Concepts
- Exponents and Exponential Functions
- Logarithm Properties
- Change of Base Formula
- Converting Ratios to Cents

# Common Confusions
- log_b(x) is only defined for x > 0 -- you cannot take the logarithm of zero or a negative number
- The base b must be positive and != 1
- log_b(x) = y means b^y = x; students often confuse which quantity is the argument and which is the result
- The logarithm is a function, not a number -- log_b by itself is meaningless without an argument

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 67-68.
