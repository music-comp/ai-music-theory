---
concept: Effect of Horizontal Stretching on Pitch
category: technique
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
Compressing a waveform horizontally by factor c multiplies its frequency by c. To produce a tone of desired frequency r from a function of period P, use y = f(rPt).

# Formal Definition
If f(t) is periodic with period P, then f(ct) has period P/c and frequency c/P = c*F. To achieve a specific frequency r Hz from a function of period P, set c = rP, giving y = f(rPt). This function has frequency r*P/P = r Hz.

# Mathematical Context
The transformation f(t) -> f(ct) compresses the time axis by factor c. Since the period becomes P/c, the frequency becomes c*F. This is a linear relationship between the compression factor and the resulting frequency. The formula c = rP ensures the resulting function has exactly frequency r.

# Musical Context
This technique shows how to tune any waveform to any desired pitch. It is also the mathematical explanation for what happens when recordings are sped up or slowed down: the frequencies of all components are multiplied by the same factor. This changes not just the pitch but also shifts the formants, producing unnatural sound (the "chipmunk effect" when sped up, or muddy sound when slowed down).

# Examples
- sin(t) has period 2*pi. For A4 (440 Hz): y = sin(440 * 2*pi * t) = sin(880*pi*t)
- Doubling playback speed (c = 2): frequency doubles, pitch rises one octave
- Halving playback speed (c = 1/2): frequency halves, pitch drops one octave
- "Chipmunk effect": speeding up a recording raises all frequencies proportionally, including formants

# Related Concepts
- Geometric Transformations on Periodic Functions
- Periodic Functions
- Frequency and Period
- Formants
- Timbre

# Common Confusions
Students may confuse horizontal compression with horizontal stretching. Compressing by factor c means replacing t with ct (c > 1), which makes events happen faster and raises pitch. Stretching by factor c means replacing t with t/c, which slows things down and lowers pitch.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 124-125 (PDF page 118).
