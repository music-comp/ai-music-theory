---
concept: Phase Shift and Amplitude
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
Amplitude is the maximum displacement of a wave from its center, determining loudness. Phase shift is a horizontal displacement of the wave, which changes the waveform's shape but does not affect the perceived timbre of a tone.

# Formal Definition
For a function h(x) = A*sin(kx) + B*cos(kx), the amplitude is d = sqrt(A^2 + B^2) and the phase shift is the angle beta such that cos(beta) = A/d and sin(beta) = B/d. The function can be rewritten as h(x) = d*sin(kx + beta). The point (A/d, B/d) lies on the unit circle, determining beta uniquely.

# Mathematical Context
The conversion from h(x) = A*sin(kx) + B*cos(kx) to d*sin(kx + beta) uses:
- d = sqrt(A^2 + B^2) (distance from origin to (A, B))
- a = A/d, b = B/d (normalize to unit circle)
- beta = angle such that cos(beta) = a, sin(beta) = b

This is essentially converting from Cartesian to polar coordinates for the point (A, B).

# Musical Context
Amplitude determines the loudness of a harmonic component. Phase shift affects the shape of the composite waveform but, remarkably, does not affect timbre. This is because the human ear is insensitive to phase relationships between harmonics -- it responds only to the amplitudes. This counterintuitive fact means two waveforms that look very different on a graph can sound identical.

# Examples
- h(x) = 3*sin(x) + 2*cos(x): A = 3, B = 2, d = sqrt(13), beta = arcsin(2/sqrt(13)) ~ 0.588
- h(x) = sqrt(13)*sin(x + 0.588): equivalent polar form, amplitude sqrt(13)
- If A = 0: h(x) = B*cos(kx) = B*sin(kx + pi/2), phase shift is pi/2
- If B = 0: h(x) = A*sin(kx), phase shift is 0

# Related Concepts
- General Sinusoidal Form
- Timbre as Harmonic Amplitudes
- Trigonometric Summation Formula
- Fourier Coefficients

# Common Confusions
The most important misconception: phase shifts DO affect the shape of the composite waveform, so students assume they must affect the sound. They do not. Timbre depends only on the amplitude sequence {dk}, not the phase sequence {beta_k}. Two waveforms with identical amplitudes but different phases sound the same.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 122-123 (PDF page 118).
