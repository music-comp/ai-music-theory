---
concept: Pure Tone
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
A pure tone is the sound produced by a single sine wave -- the simplest possible musical sound, containing only one frequency with no overtones. It resembles the sound of a tuning fork.

# Formal Definition
A pure tone is a vibration described by y = d*sin(2*pi*F*t + beta) for some amplitude d, frequency F, and phase shift beta. Equivalently, it is a tone whose Fourier series contains only the first harmonic (k = 1), with all higher harmonics having zero amplitude (dk = 0 for k >= 2).

# Mathematical Context
In the Fourier series g(t) = C + sum(dk*sin(2*pi*F*k*t + beta_k)), a pure tone has d1 > 0 and dk = 0 for all k >= 2. It is the fundamental building block from which all complex tones are constructed via superposition.

# Musical Context
A pure tone is described as a "nondescript hum" -- it has pitch but minimal timbral character. The tuning fork produces an approximately pure tone. Electronic synthesizers can generate exact pure tones. Real musical instruments always produce complex tones with multiple harmonics, which is what gives them their distinctive timbres.

# Examples
- y = sin(880*pi*t): pure tone at A4 (440 Hz)
- A tuning fork struck gently: approximately pure
- Electronic sine wave generator: exactly pure
- Contrast with clarinet, violin, voice: all produce complex tones with many harmonics

# Related Concepts
- Timbre
- Fourier Series
- Harmonics and Overtones
- Sine and Cosine Functions
- Square Wave Fourier Analysis

# Common Confusions
A pure tone is not "better" or "more musical" than a complex tone -- it is simply the simplest. Most musical richness comes from the interaction of multiple harmonics. A pure tone sounds bland precisely because it lacks overtones to give it character.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," p. 125 (PDF page 118).
