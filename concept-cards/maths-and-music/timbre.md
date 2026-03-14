---
concept: Timbre
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
Timbre is the quality of a musical tone that distinguishes different instruments or voices playing the same pitch -- what makes a violin sound different from a flute or a human voice.

# Formal Definition
Timbre refers to the distinguishing properties of a musical tone other than its pitch. Mathematically, timbre is determined by the relative amplitudes d1, d2, d3, ... of the harmonics in a tone's Fourier series decomposition. Crucially, the phase shifts do not affect timbre -- only the amplitudes matter.

# Mathematical Context
Given a periodic function g(t) = C + sum(dk * sin(2*pi*F*k*t + beta_k)), the timbre is determined solely by the sequence of amplitudes {dk} where dk = sqrt(Ak^2 + Bk^2). The phase shifts {beta_k} affect the shape of the waveform graph but not the perceived sound quality. This is a counterintuitive but experimentally confirmed result.

# Musical Context
Timbre enables listeners to distinguish a violin from a trumpet from a clarinet from a human vowel, even when all produce the same pitch. Each instrument or voice has a characteristic pattern of harmonic amplitudes, often shaped by formants (resonant frequency ranges determined by the instrument's physical structure).

# Examples
- A pure sine wave has only one harmonic (d1 > 0, all others zero) -- sounds like a tuning fork
- A square wave has only odd harmonics with amplitudes decreasing as 1/k -- resembles a clarinet
- A violin has a rich spectrum with many harmonics of varying amplitudes
- Human vowels differ in timbre due to different formant structures

# Related Concepts
- Timbre as Harmonic Amplitudes
- Fourier Series
- Formants
- Pure Tone
- Square Wave Fourier Analysis
- Harmonics and Overtones

# Common Confusions
Students often think the shape of a waveform determines timbre. While there is a relationship, two very different-looking waveforms can sound identical if they have the same harmonic amplitudes but different phase shifts. The ear is insensitive to phase.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," p. 118 and pp. 126-127 (PDF page 118).
