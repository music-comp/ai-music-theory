---
concept: Odd Harmonics Only
category: analysis
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
Some waveforms contain only odd-numbered harmonics (1st, 3rd, 5th, 7th, ...), with all even harmonics absent. This produces a distinctive hollow or nasal timbre. The square wave and the clarinet are the primary examples.

# Formal Definition
A periodic function has "only odd harmonics" when its Fourier coefficients satisfy A_k = B_k = 0 (hence d_k = 0) for all even k. Equivalently, the Fourier series contains terms only at frequencies F, 3F, 5F, 7F, ... and not at 2F, 4F, 6F, ....

# Mathematical Context
For the square wave, the vanishing of even harmonics follows from a symmetry argument: when k is even, the function sin(kt) has the same integral over [0, pi] as over [pi, 2*pi], so multiplying by the square wave (which flips sign at pi) produces a net integral of zero. The triangle wave also has only odd harmonics, but with amplitudes decreasing as 1/k^2 rather than 1/k.

# Musical Context
The clarinet has predominantly odd harmonics because of the physics of its cylindrical bore, which is closed at one end (the reed). This is why the square wave sound vaguely resembles a clarinet. Open-ended instruments (like the flute) and conical bore instruments (like the oboe and saxophone) produce both odd and even harmonics.

# Examples
- Square wave: amplitudes d_k = 4/(k*pi) for k odd, d_k = 0 for k even
- Triangle wave: amplitudes proportional to 1/k^2 for k odd, zero for k even (softer sound than square wave because higher harmonics drop off faster)
- Clarinet: predominantly odd harmonics due to closed cylindrical bore
- Contrast with sawtooth wave: q(t) = -(2/pi)*sum sin(kt)/k has ALL harmonics

# Related Concepts
- Square Wave Fourier Analysis
- Fourier Coefficients
- Timbre as Harmonic Amplitudes
- Harmonics and Overtones

# Common Confusions
Students may assume "odd harmonics only" means the sound is at half the frequency or sounds an octave lower. It does not -- the fundamental (1st harmonic) is still present and determines the pitch. The absence of even harmonics affects only the timbre, not the perceived pitch.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 130-132 (PDF page 118). Triangle wave with odd harmonics in exercises, p. 137.
