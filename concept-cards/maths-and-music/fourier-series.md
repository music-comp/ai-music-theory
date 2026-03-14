---
concept: Fourier Series
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The Fourier series decomposes any well-behaved periodic function into an infinite sum of sine and cosine functions at integer multiples of the fundamental frequency. This is the central mathematical tool connecting waveform shape to timbre.

# Formal Definition
**Theorem:** If f(t) is periodic of period 2*pi, bounded, and has a bounded continuous derivative at all but finitely many points in [0, 2*pi), then there exist real numbers C, A1, A2, ... and B1, B2, ... such that at all points of continuity:

f(t) = C + sum_{k=1}^{infinity} [A_k * sin(kt) + B_k * cos(kt)]  (10.5)

For a function of arbitrary period P:

g(t) = C + sum_{k=1}^{infinity} [A_k * sin(2*pi*k*t/P) + B_k * cos(2*pi*k*t/P)]  (10.7)

# Mathematical Context
The theorem requires concepts from calculus: derivatives, infinite summation, and convergence. The conditions on f(t) roughly say that, away from finitely many points, the graph is smooth and does not slope too steeply. The convergence of the infinite series means the partial sums increasingly approximate f(t), as illustrated by the square wave example.

# Musical Context
The Fourier series is the mathematical foundation for understanding timbre. It shows that every musical tone is a superposition of pure tones (sine waves) at frequencies that are integer multiples of the fundamental. The relative amplitudes of these components determine the sound's character. This insight connects abstract mathematics to the physical reality of how we hear.

# Examples
- Square wave: s(t) = (4/pi) * sum_{n=0}^{infinity} sin((2n+1)t)/(2n+1) -- only odd harmonics
- Sawtooth wave: q(t) = -(2/pi) * sum_{k=1}^{infinity} sin(kt)/k -- all harmonics
- sin(t) is its own Fourier series (A1 = 1, all other coefficients zero)
- The series sum_{k=0}^{infinity} 1/2^k = 2 illustrates how infinite sums can converge

# Related Concepts
- Fourier Coefficients
- Harmonics and Overtones
- Timbre as Harmonic Amplitudes
- Square Wave Fourier Analysis
- Fundamental Frequency

# Common Confusions
Students may think the Fourier series is an approximation. It is not -- it converges exactly to f(t) at all points of continuity. At discontinuities, the series converges to the average of the left and right limits. Also, only finitely many overtones fall within the audible range, so a truncated series suffices for the perceived sound.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 125-126, equations (10.5) and (10.7) (PDF page 118).
