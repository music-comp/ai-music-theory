---
concept: Keyboard Approximation of Integer Ratios
category: analysis
source: "Mathematics and Music"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
authors: "David Wright"
---

# Quick Definition
The process of finding the closest equally tempered chromatic scale interval to each positive integer ratio, and measuring the approximation error in cents.

# Formal Definition
For a positive integer n, its interval measured in cents is 1200 * log2(n). The best keyboard approximation is the nearest integer number of semitones, and the error is the difference between 1200 * log2(n) and that nearest multiple of 100 cents.

# Mathematical Context
The cent value of integer n is 1200 * log2(n). The best approximation in semitones is round(1200 * log2(n) / 100), and the error in cents is 1200 * log2(n) - 100 * round(1200 * log2(n) / 100). Positive error means the keyboard is sharp; negative means flat.

# Musical Context
This analysis reveals which integer ratios the equal-tempered keyboard renders well and which it distorts significantly. The tempered scale was historically controversial precisely because of these discrepancies, particularly the 14-cent error on the ratio 5 (major third).

# Examples
Summary of approximations for integers 1-13:
- 1: unison, exact (0 cents error)
- 2: octave, exact (0 cents error)
- 3: octave + fifth, ~2 cents flat
- 4: two octaves, exact
- 5: two octaves + major third, ~14 cents sharp
- 6: two octaves + fifth, ~2 cents flat
- 7: two octaves + minor seventh, ~31 cents sharp
- 8: three octaves, exact
- 9: three octaves + major second, ~4 cents flat
- 10: three octaves + major third, ~14 cents sharp
- 11: three octaves + tritone, ~49 cents sharp (worst)
- 12: three octaves + fifth, ~2 cents flat
- 13: three octaves + minor sixth, ~41 cents flat

# Related Concepts
- Error Calculation in Cents
- Powers of Two as Exact Keyboard Intervals
- In-the-Cracks Intervals
- Consonance and Dissonance from Integer Ratios

# Common Confusions
The error for a composite number equals the sum of errors of its prime factors (modulo octave exactness). For example, 6 = 2 * 3 has the same ~2 cent error as 3, since 2 is rendered exactly. Students sometimes expect each integer to have an independent error.

# Source Reference
Chapter 9: "The Integers as Intervals," pp. 110-116 (PDF page 110).
