---
concept: Chipmunk Effect
category: application
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The "chipmunk effect" is the unnatural sound produced when a recording is sped up, raising all frequencies proportionally and shifting the formants away from their natural positions, distorting the timbre beyond simply changing pitch.

# Formal Definition
When a recorded tone is played at a rate c times its original recording speed, the sound wave is compressed by factor c: f(t) becomes f(ct). This multiplies all frequencies by c, including the fundamental and all harmonics. The formants, which depend on fixed physical resonating chambers, are thereby shifted to c times their natural frequencies, destroying the original timbre.

# Mathematical Context
In the Fourier representation g(t) = C + sum d_k*sin(2*pi*F*k*t + beta_k), simple time-scaling to g(ct) changes F to cF while keeping all d_k and beta_k unchanged. The formant structure -- which determines which d_k values are large -- is preserved in the mathematical description but no longer matches the physical formant frequencies of the original source.

# Musical Context
Speeding up produces the familiar high-pitched, cartoonish "chipmunk" sound. Slowing down produces a dark, muddy sound. In both cases, the character of the music is "changed in a rather comical way." Modern studio technology can now transpose pitch while preserving formants (pitch-shifting without time-stretching), representing "a great triumph in signal analysis technology."

# Examples
- Speeding up a male voice by factor 2: pitch rises an octave, formants shift up by an octave, voice sounds like a cartoon chipmunk
- Slowing down music by factor 1/2: pitch drops an octave, formants shift down, sound becomes muddy and unnatural
- Modern pitch-shifting algorithms: can change pitch while keeping formants intact, preserving natural character

# Related Concepts
- Formants
- Effect of Horizontal Stretching on Pitch
- Geometric Transformations on Periodic Functions
- Timbre as Harmonic Amplitudes

# Common Confusions
Students often think speeding up a recording merely raises the pitch. It also raises all formant frequencies, which is why the sound becomes unnatural rather than simply sounding like a higher-pitched version of the same instrument or voice. Preserving formants during pitch change requires sophisticated signal processing, not simple time-scaling.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 133-134 (PDF page 118).
