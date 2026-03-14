---
concept: General Sinusoidal Form
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The equivalence between two representations of a sinusoidal function: the sum form A*sin(kx) + B*cos(kx) and the transformation form d*sin(kx + beta), where d is the amplitude and beta is the phase shift.

# Formal Definition
For any real numbers A, B (not both zero) and positive k, the function h(x) = A*sin(kx) + B*cos(kx) can be written as h(x) = d*sin(kx + beta), where:
- d = sqrt(A^2 + B^2) (amplitude)
- beta is the angle with cos(beta) = A/d and sin(beta) = B/d (phase shift)

This is equation (10.3)/(10.4) in the text: g(x) = d*sin(kx + beta) = d*(cos(beta)*sin(kx) + sin(beta)*cos(kx)).

# Mathematical Context
The equivalence is proved using the trigonometric summation formula. The function g(x) is obtained from sin(x) by three transformations: (1) shift left by beta, (2) compress horizontally by factor k, (3) stretch vertically by factor d. The period of g(x) is 2*pi/k, and its frequency is k/(2*pi).

# Musical Context
This equivalence is essential for Fourier analysis of sound. The sum form (A*sin + B*cos) is what the Fourier theorem produces; the transformation form (d*sin with phase shift) reveals the physically meaningful amplitude and phase. Since only amplitude affects timbre, the transformation form isolates the musically relevant parameter d.

# Examples
- 3*sin(x) + 2*cos(x) = sqrt(13)*sin(x + 0.588)
- sin(x) + cos(x) = sqrt(2)*sin(x + pi/4)
- 5*sin(2x) = 5*sin(2x + 0): amplitude 5, phase shift 0, period pi

# Related Concepts
- Phase Shift and Amplitude
- Trigonometric Summation Formula
- Fourier Coefficients
- Harmonics and Overtones

# Common Confusions
Students sometimes think A is the amplitude of the sine component and B is the amplitude of the cosine component, and that these independently contribute to loudness. In fact, the combined amplitude is sqrt(A^2 + B^2), not A + B or max(A, B).

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 122-123, equations (10.3) and (10.4) (PDF page 118).
