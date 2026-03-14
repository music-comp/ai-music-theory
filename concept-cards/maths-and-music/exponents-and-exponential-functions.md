---
concept: Exponents and Exponential Functions
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
The exponential function f(x) = b^x maps real numbers to positive reals, providing the mathematical foundation for converting additive interval measurements to multiplicative ratios.

# Formal Definition
For a positive real number b != 1 (the base), the exponential function f: R -> R+ is defined by f(x) = b^x. For positive integers n, b^n is the n-fold product; b^(-n) = 1/b^n; b^(1/n) = the nth root of b. The rule of exponents b^(st) = (b^s)^t, together with the calculus concept of limit, extends the definition to all real x, making f continuous. For b > 1, f is strictly increasing, hence one-to-one, with domain R and range R+.

# Mathematical Context
The exponential function is central to interval theory because the formula r = 2^(x/12) (semitones to ratio) and r = 2^(x/1200) (cents to ratio) are exponential functions with base 2^(1/12) and 2^(1/1200) respectively. Key properties include:
- b^(s+t) = b^s * b^t (multiplicativity from additivity)
- b^(st) = (b^s)^t (exponent rule)
- b^0 = 1 (identity)
- b^(-x) = 1/b^x (inverse)

The function establishes a group isomorphism from (R, +) to (R+, *).

# Musical Context
The exponential function converts additive interval measurements (semitones, cents, octaves) into frequency ratios. When a musician says "go up 7 semitones," the actual frequency change is multiplication by 2^(7/12). The exponential function explains why equal intervals in the additive sense produce non-equal frequency differences: the graph of f(x) = 2^(x/12) curves upward, so equal horizontal steps produce increasingly large vertical steps.

# Examples
- b^(-2/3) = (1/b^2)^(1/3) = cube root of (1/b^2)
- 2^(7/12) ~ 1.498 (ratio of an equal-tempered fifth)
- The graph of f(x) = b^x passes through (0, 1) and increases for b > 1
- Plotting keyboard note frequencies vs. pitch number gives an exponential curve

# Related Concepts
- Logarithmic Functions as Inverses
- Semitone Ratio
- Multiplicative and Additive Measurements
- Converting Ratios to Cents

# Common Confusions
- The base must be positive and not equal to 1
- b^x is defined for all real x (not just integers or rationals), via limits
- The exponential function is one-to-one and onto R+, which is why it has an inverse (the logarithm)

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 66-67.
