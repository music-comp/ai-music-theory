---
concept: Trigonometric Summation Formula
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
The identity sin(alpha + beta) = sin(alpha)*cos(beta) + cos(alpha)*sin(beta), which enables the conversion between the sum-of-sines-and-cosines form and the amplitude-phase form of a harmonic.

# Formal Definition
The sine summation formula is:
sin(alpha + beta) = sin(alpha)*cos(beta) + cos(alpha)*sin(beta)  (10.1)

The cosine summation formula is:
cos(alpha + beta) = cos(alpha)*cos(beta) - sin(alpha)*sin(beta)  (10.9)

Treating alpha as the variable x and beta as a fixed parameter:
sin(x + beta) = cos(beta)*sin(x) + sin(beta)*cos(x)  (10.2)

# Mathematical Context
These formulas are the algebraic bridge between two representations of a harmonic: the "rectangular" form A*sin(kx) + B*cos(kx) and the "polar" form d*sin(kx + beta). The conversion uses A = d*cos(beta), B = d*sin(beta), d = sqrt(A^2 + B^2). The cosine formula (10.9) is used in the square wave Fourier analysis to prove all B_k coefficients are zero.

# Musical Context
The summation formula is essential for understanding how phase shifts work in the Fourier decomposition of sound. It shows that any shifted sine wave can be decomposed into a weighted sum of sine and cosine at the same frequency, and vice versa. This is the mathematical mechanism behind the equivalence of the two forms of Fourier series.

# Examples
- cos(x) = sin(x + pi/2) follows from the formula with beta = pi/2
- 3*sin(x) + 2*cos(x) = sqrt(13)*sin(x + arctan(2/3)): converting from rectangular to polar form
- The formula with alpha = k(pi - t) is used to show cos(k(pi - t)) = cos(k(pi + t)), needed for the square wave analysis

# Related Concepts
- Phase Shift and Amplitude
- Sine and Cosine Functions
- Fourier Series
- General Sinusoidal Form

# Common Confusions
Students sometimes mix up the signs in the cosine summation formula (which has a minus sign) versus the sine summation formula (which has all plus signs). The mnemonic: sine of a sum uses "sin-cos + cos-sin"; cosine of a sum uses "cos-cos - sin-sin."

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 121-123, equations (10.1), (10.2), and (10.9) (PDF page 118).
