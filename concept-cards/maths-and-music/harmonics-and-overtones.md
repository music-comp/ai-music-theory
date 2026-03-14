---
concept: Harmonics and Overtones
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
The k-th harmonic of a tone with fundamental frequency F is the component at frequency kF. The k-th overtone is the (k+1)-th harmonic. These two numbering systems differ by one, which is a persistent source of confusion.

# Formal Definition
Given a periodic function g(t) with fundamental frequency F, its Fourier decomposition gives:

g(t) = C + sum_{k=1}^{infinity} d_k * sin(2*pi*F*k*t + beta_k)

The k-th summand d_k*sin(2*pi*F*k*t + beta_k) is called the k-th harmonic. For k >= 1, it is also called the (k-1)-th overtone. The k-th harmonic has frequency kF, amplitude d_k, and phase shift beta_k.

# Mathematical Context
Each harmonic is obtained from sin(t) via: shifting by beta_k, compressing horizontally by factor k (giving frequency kF), and stretching vertically by factor d_k. The function d_k*sin(2*pi*F*k*t + beta_k) has period 1/(kF), but it also has period 1/F (since k is a positive integer). This is why all harmonics share the fundamental period.

# Musical Context
Harmonics are generally not perceived individually as pitches. Instead, the totality of audible harmonics is heard as an integrated single tone whose timbre is determined by the amplitudes. However, individual harmonics can become audible through overtone singing (manipulating vocal resonance to isolate specific overtones) or through reinforced overtones in well-tuned chords.

# Examples
For fundamental F:
- 1st harmonic = fundamental = F (0th overtone)
- 2nd harmonic = 1st overtone = 2F (one octave above)
- 3rd harmonic = 2nd overtone = 3F (octave + fifth above)
- 4th harmonic = 3rd overtone = 4F (two octaves above)
- 5th harmonic = 4th overtone = 5F (two octaves + major third)

# Related Concepts
- Overtone Series
- Fundamental Frequency
- Fourier Series
- Timbre as Harmonic Amplitudes
- Overtone Singing
- Reinforced Overtone

# Common Confusions
The off-by-one difference between harmonics and overtones is the single most common confusion. The 1st harmonic is the fundamental -- it is NOT an overtone (or is the "0th overtone"). The 1st overtone is the 2nd harmonic. When someone says "3rd harmonic," they mean frequency 3F; when they say "3rd overtone," they mean frequency 4F.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 126-127 (PDF page 118).
