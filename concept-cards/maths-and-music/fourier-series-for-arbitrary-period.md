---
concept: Fourier Series for Arbitrary Period
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
The Fourier series can represent periodic functions of any period P, not just period 2*pi. The general form replaces sin(kt) and cos(kt) with sin(2*pi*k*t/P) and cos(2*pi*k*t/P).

# Formal Definition
For a function g(t) of period P satisfying the Fourier theorem hypotheses:

g(t) = C + sum_{k=1}^{infinity} [A_k * sin(2*pi*k*t/P) + B_k * cos(2*pi*k*t/P)]  (10.7)

This is derived from the period-2*pi form by substituting g(P*t/(2*pi)) into the standard Fourier theorem, then recovering g(t) by replacing t with 2*pi*t/P.

# Mathematical Context
The transformation from arbitrary period P to period 2*pi is achieved by the substitution: if g(t) has period P, then f(t) = g(P*t/(2*pi)) has period 2*pi. Applying the standard Fourier theorem to f and then undoing the substitution yields equation (10.7). The k-th term has frequency k/P = kF, confirming that harmonics are integer multiples of the fundamental.

# Musical Context
This generalization is essential for real musical applications, since the period 2*pi has no special musical significance (its frequency ~0.159 Hz is inaudible). Real tones have arbitrary periods determined by their pitch. A tone at 440 Hz has period P = 1/440 seconds, and its k-th harmonic has frequency 440k Hz.

# Examples
- For a 440 Hz tone (P = 1/440): g(t) = C + sum [A_k * sin(2*pi*440*k*t) + B_k * cos(2*pi*440*k*t)]
- The k-th harmonic frequency is k/P = 440k Hz
- For k=1: 440 Hz (fundamental); k=2: 880 Hz; k=3: 1320 Hz

# Related Concepts
- Fourier Series
- Fourier Coefficients
- Fundamental Frequency
- Harmonics and Overtones
- Frequency and Period

# Common Confusions
Students may wonder why the "standard" Fourier series uses period 2*pi. This is a mathematical convenience (matching the natural period of sin and cos), not a physical requirement. The generalization to arbitrary period P is straightforward but essential for musical applications.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 126-127, equation (10.7) (PDF page 118).
