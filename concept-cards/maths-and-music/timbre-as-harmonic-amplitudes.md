---
concept: Timbre as Harmonic Amplitudes
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
The timbre (tonal quality) of a sustained musical tone is determined solely by the relative sizes of the harmonic amplitudes d1, d2, d3, ..., and is independent of the phase shifts. This is the central insight connecting Fourier analysis to music.

# Formal Definition
Given g(t) = C + sum_{k=1}^{infinity} d_k*sin(2*pi*F*k*t + beta_k), the timbre depends only on the sequence of non-negative amplitudes {d_k} where d_k = sqrt(A_k^2 + B_k^2). The phase shifts {beta_k} affect the shape of the graph of g(t) but not the perceived sound quality. We can think of d_k as the "weight" or "degree of presence" of the k-th harmonic.

# Mathematical Context
Two functions can have identical amplitude sequences but different phase shifts. Their graphs will look different, but they will sound the same. This is because the human auditory system performs something analogous to computing |Fourier transform|^2, which discards phase information. Mathematically, this means timbre is a function of {d_k} alone.

# Musical Context
This principle explains why different instruments sound different: a violin, trumpet, and clarinet playing the same pitch have different amplitude profiles across their harmonics. The clarinet's distinctive sound comes from having predominantly odd harmonics. A flute has a strong fundamental with weak upper harmonics. A trumpet has many strong harmonics.

# Examples
- Pure tone (tuning fork): d1 > 0, d_k = 0 for k >= 2 -- bland, featureless sound
- Square wave: d_k = 4/(k*pi) for odd k, d_k = 0 for even k -- hollow, clarinet-like
- If we change all phase shifts beta_k while keeping all d_k the same: the graph changes shape completely, but the sound is identical
- Two instruments playing the same note: same F, different {d_k} profiles

# Related Concepts
- Timbre
- Fourier Series
- Phase Shift and Amplitude
- Harmonics and Overtones
- Square Wave Fourier Analysis
- Formants

# Common Confusions
This is deeply counterintuitive: the waveform shape (which is what oscilloscopes display) does NOT directly determine timbre. Two waveforms that look completely different can produce the same timbre if they have the same harmonic amplitudes. Phase shifts change the wave shape but not the sound.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," p. 127 (PDF page 118).
