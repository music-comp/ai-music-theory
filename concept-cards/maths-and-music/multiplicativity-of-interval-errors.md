---
concept: Multiplicativity of Interval Errors
category: technique
source: "Mathematics and Music"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
authors: "David Wright"
unit: null
---

# Quick Definition
When approximating composite integer ratios on the keyboard, the cent errors add: the error for n * m equals the error for n plus the error for m, because powers of 2 contribute zero error.

# Formal Definition
For positive integers n and m, the cent value of their product is 1200 * log2(nm) = 1200 * log2(n) + 1200 * log2(m). If E(n) denotes the approximation error for integer n, then E(nm) = E(n) + E(m) (modulo the fact that the rounding to the nearest semitone may introduce small discrepancies). In particular, E(2^k * n) = E(n) since E(2^k) = 0 for all k.

# Mathematical Context
This follows directly from the logarithmic property: log2(nm) = log2(n) + log2(m). Since cent values are additive under composition, and keyboard approximations are the nearest semitones, the errors are approximately additive. Exactly, 1200 * log2(6) = 1200 * (log2(2) + log2(3)) = 1200 + 1901.96 = 3101.96 cents, confirming the ~2 cent error of 6 matches that of 3.

# Musical Context
This principle explains why multiplying by 2 (adding an octave) never changes the approximation error, and why 9 = 3^2 has double the error of 3 (~4 cents flat vs. ~2 cents flat). It provides a systematic way to predict keyboard approximation quality for any composite number from its prime factors.

# Examples
- E(6) = E(2) + E(3) = 0 + (-2) = -2 cents (verified: ~2 cents flat)
- E(9) = E(3) + E(3) = (-2) + (-2) = -4 cents (verified: ~4 cents flat)
- E(10) = E(2) + E(5) = 0 + 14 = +14 cents (verified: ~14 cents sharp)
- E(12) = E(4) + E(3) = 0 + (-2) = -2 cents (verified: ~2 cents flat)

# Related Concepts
- Error Calculation in Cents
- Keyboard Approximation of Integer Ratios
- Prime Interval Personality

# Common Confusions
The additivity of errors is approximate when the rounding targets (nearest semitone) differ. In practice, for the integers 1-13 discussed in the text, the additivity holds exactly because the constituent errors are small enough not to shift the rounding.

# Source Reference
Chapter 9: "The Integers as Intervals," pp. 112-115 (PDF page 110).
