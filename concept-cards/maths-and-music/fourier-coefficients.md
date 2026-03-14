---
concept: Fourier Coefficients
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The Fourier coefficients C, A_k, and B_k are the specific numerical weights that determine how much each sine and cosine component contributes to a periodic function's Fourier series. They are computed using definite integrals.

# Formal Definition
For a periodic function f(t) of period 2*pi satisfying the Fourier theorem hypotheses, the coefficients are uniquely determined by:

C = (1/(2*pi)) * integral from 0 to 2*pi of f(t) dt

A_k = (1/pi) * integral from 0 to 2*pi of sin(kt)*f(t) dt

B_k = (1/pi) * integral from 0 to 2*pi of cos(kt)*f(t) dt

These are equations (10.6) in the text.

# Mathematical Context
The integrals exploit the orthogonality of sine and cosine functions: the integral of sin(jt)*sin(kt) over [0, 2*pi] is zero when j != k, and pi when j = k. Similarly for cosine. This orthogonality is what allows each coefficient to be isolated via integration. The constant C is the average value of f(t) over one period.

# Musical Context
The Fourier coefficients encode everything about a tone's timbre. The constant C represents a DC offset (no audible effect). The coefficients A_k and B_k combine to give the k-th harmonic's amplitude d_k = sqrt(A_k^2 + B_k^2) and phase shift beta_k. Since only amplitudes affect timbre, the musically relevant information is the sequence {d_k}.

# Examples
For the square wave s(t):
- C = 0 (equal area above and below axis)
- B_k = 0 for all k (by symmetry of cosine around t = pi)
- A_k = 0 for k even
- A_k = 4/(k*pi) for k odd
- This yields s(t) = (4/pi) * sum sin((2n+1)t)/(2n+1)

# Related Concepts
- Fourier Series
- Phase Shift and Amplitude
- Timbre as Harmonic Amplitudes
- Square Wave Fourier Analysis
- Harmonics and Overtones

# Common Confusions
Students sometimes think A_k is the amplitude of the k-th harmonic. It is not -- A_k is the coefficient of sin(kt), and B_k is the coefficient of cos(kt). The amplitude is d_k = sqrt(A_k^2 + B_k^2), which combines both. When B_k = 0 (as in the square wave), then d_k = |A_k|.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 126, equation (10.6) (PDF page 118).
