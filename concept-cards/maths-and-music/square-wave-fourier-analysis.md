---
concept: Square Wave Fourier Analysis
category: analysis
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The square wave is a periodic function alternating between +1 and -1, whose Fourier series contains only odd harmonics with amplitudes decreasing as 1/k. Its sound vaguely resembles a clarinet, which also has predominantly odd harmonics.

# Formal Definition
The square wave is defined on [0, 2*pi) as s(t) = 1 for 0 <= t < pi, s(t) = -1 for pi <= t < 2*pi, extended by periodicity. Its Fourier series is:

s(t) = (4/pi) * sum_{n=0}^{infinity} sin((2n+1)t) / (2n+1)

= (4/pi) * [sin(t) + sin(3t)/3 + sin(5t)/5 + sin(7t)/7 + ...]

# Mathematical Context
The Fourier coefficients are computed as follows:
- C = 0 (equal area above and below axis)
- B_k = 0 for all k (cosine symmetry around t = pi makes the integral vanish)
- A_k = 0 for even k (sine function looks the same on [0, pi] and [pi, 2*pi] when k is even)
- A_k = 4/(k*pi) for odd k (when k is odd, the contributions from the two halves add rather than cancel)

The key calculation uses the fact that the area under one lobe of sin(t) equals 2 (from the Fundamental Theorem of Calculus: integral from 0 to pi of sin(t) dt = 2).

# Musical Context
The square wave's restriction to odd harmonics is shared by the clarinet, due to the physics of a cylindrical closed-at-one-end air column. This explains the faint resemblance between a square wave and a clarinet sound. The square wave is a standard waveform in electronic synthesis and signal processing.

# Examples
- 1st harmonic (k=1): amplitude 4/pi ~ 1.27
- 3rd harmonic (k=3): amplitude 4/(3*pi) ~ 0.42
- 5th harmonic (k=5): amplitude 4/(5*pi) ~ 0.25
- 7th harmonic (k=7): amplitude 4/(7*pi) ~ 0.18
- All even harmonics: amplitude 0
- All phase shifts: beta_k = 0 (no cosine terms)

Truncated series with increasing N show graphs that progressively better approximate the square wave shape.

# Related Concepts
- Fourier Series
- Fourier Coefficients
- Timbre as Harmonic Amplitudes
- Odd Harmonics Only
- Pure Tone
- Formants

# Common Confusions
Students sometimes wonder why a function with sharp corners (discontinuities) can be represented by smooth sine functions. The answer is that infinitely many sine terms are needed -- any finite truncation produces smooth "wiggles" near the discontinuities (Gibbs phenomenon). Also, the series converges at points of continuity only; at discontinuities it converges to the midpoint (0 for the square wave).

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 128-132, equation (10.14) (PDF page 118).
