---
concept: Error Calculation in Cents
category: technique
source: "Mathematics and Music"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
authors: "David Wright"
unit: null
---

# Quick Definition
A method for quantifying how closely an equally tempered keyboard interval approximates a pure integer ratio, expressed in cents (hundredths of a semitone).

# Formal Definition
For a positive integer n, the exact interval in cents is 1200 * log2(n). The keyboard approximation is the nearest multiple of 100 cents. The error E = 1200 * log2(n) - 100 * k, where k is the nearest integer to 1200 * log2(n) / 100. Positive E means the keyboard is sharp; negative E means flat.

# Mathematical Context
The logarithmic nature of cents means errors are additive under composition of intervals. If integer n has error E(n) and integer m has error E(m), then the composite interval n * m has error E(n) + E(m) (modulo the fact that powers of 2 have zero error). This follows from log2(nm) = log2(n) + log2(m).

# Musical Context
The cent is a logarithmic unit dividing each semitone into 100 equal parts. Most listeners can perceive differences of about 5-10 cents. Errors under ~5 cents are generally imperceptible; errors of ~14 cents (as with the major third) are noticeable upon careful listening; errors of ~31 cents or more are clearly audible.

# Examples
- 1200 * log2(3) = 1901.96 cents; nearest keyboard interval = 1900 cents (19 semitones); error = ~2 cents flat
- 1200 * log2(5) = 2786.31 cents; nearest = 2800 cents; error = ~14 cents sharp
- 1200 * log2(7) = 3368.83 cents; nearest = 3400 cents; error = ~31 cents sharp
- 1200 * log2(11) = 4151.32 cents; nearest = 4200 cents; error = ~49 cents sharp

# Related Concepts
- Keyboard Approximation of Integer Ratios
- Powers of Two as Exact Keyboard Intervals
- In-the-Cracks Intervals
- Consonance and Dissonance from Integer Ratios

# Common Confusions
Students sometimes forget that cents are logarithmic. Doubling a frequency ratio does not double the cent value. Also, "sharp" and "flat" here refer to the keyboard being higher or lower than the pure ratio, not to musical sharps and flats.

# Source Reference
Chapter 9: "The Integers as Intervals," pp. 110-116 (PDF page 110).
