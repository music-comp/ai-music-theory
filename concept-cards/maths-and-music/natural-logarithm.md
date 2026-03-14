---
concept: Natural Logarithm
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
The natural logarithm (ln x = log_e x) uses the transcendental number e ~ 2.71828 as its base. It is the most commonly available logarithm on calculators and is used with the change of base formula to compute any other logarithm.

# Formal Definition
The natural logarithm is log_e(x), commonly denoted ln(x), where e ~ 2.71828 is the base of the natural exponential function. Using the change of base formula (L4) with a = e:

(L5) log_b(x) = ln(x) / ln(b)

This allows any logarithm to be computed using ln. Similarly, log_10 (also commonly available on calculators) can be used via the same formula with a = 10.

# Mathematical Context
The number e is highly significant in mathematics for reasons including its role as the base of the natural exponential function, whose derivative equals itself (d/dx e^x = e^x). While e has no special musical significance, the natural logarithm is the standard computational tool. For musical applications, the key formula is:

log_2(r) = ln(r) / ln(2)

which converts any ratio to octaves, and can then be scaled to semitones or cents.

# Musical Context
In practice, converting a ratio to cents involves computing:

x = 1200 * (ln(r) / ln(2))

This is the standard calculation a musician or acoustician performs to determine the size of an interval given as a frequency ratio. The natural logarithm serves purely as a computational bridge; the musically meaningful base is 2 (for octaves) or 2^(1/12) (for semitones).

# Examples
- ln(2) ~ 0.6931
- ln(3/2) ~ 0.4055
- Ratio 3/2 in cents: 1200 * (ln(3/2) / ln(2)) = 1200 * (0.4055 / 0.6931) ~ 701.955
- Any calculator with ln can compute any musical interval conversion

# Related Concepts
- Change of Base Formula
- Logarithmic Functions as Inverses
- Converting Ratios to Cents
- Logarithm Properties

# Common Confusions
- The natural logarithm base e ~ 2.71828 has no intrinsic musical meaning; it is simply a computational convenience
- ln(x) and log_10(x) both work for musical conversions; just use the change of base formula with the appropriate denominator
- "Natural" does not mean "more natural for music"; log_2 is the musically natural base

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," p. 70.
