---
concept: Sine and Cosine Functions
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The sine and cosine functions are the fundamental building blocks of all periodic functions in Fourier analysis. They are periodic with period 2*pi and are related by a phase shift of pi/2.

# Formal Definition
The functions y = sin(x) and y = cos(x) are periodic of period 2*pi. They are related by cos(x) = sin(x + pi/2), meaning cosine is sine shifted left by pi/2. This relationship follows from the summation formula sin(alpha + beta) = sin(alpha)*cos(beta) + cos(alpha)*sin(beta), with beta = pi/2.

# Mathematical Context
The sine and cosine functions are central to Fourier analysis because they form an orthogonal basis for the space of periodic functions. Every well-behaved periodic function can be decomposed into a (possibly infinite) sum of sine and cosine terms at integer multiples of the fundamental frequency. The point (cos(beta), sin(beta)) lies on the unit circle at arc length beta from (1, 0).

# Musical Context
A pure sine wave produces the simplest possible musical tone -- a "nondescript hum" similar to a tuning fork. All complex timbres can be understood as superpositions of sine and cosine waves at different frequencies and amplitudes. The sine function is thus the "atom" of sound.

# Examples
- sin(t) has period 2*pi, frequency 1/(2*pi) Hz (far below audibility)
- sin(880*pi*t) produces A4 at 440 Hz
- cos(x) = sin(x + pi/2): cosine is sine shifted left by a quarter period
- 3*sin(x) + 2*cos(x) = sqrt(13)*sin(x + beta) where beta = arcsin(2/sqrt(13)) ~ 0.588

# Related Concepts
- Trigonometric Summation Formula
- Phase Shift and Amplitude
- Pure Tone
- Fourier Series

# Common Confusions
Students may think sine and cosine are fundamentally different functions. They are identical except for a phase shift of pi/2. In Fourier analysis, both are needed to represent arbitrary phase shifts, but they produce the same sound in isolation (a pure tone at the same frequency).

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 121-123 (PDF page 118).
