---
concept: Logarithmic Pitch Scale
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
unit: null
---

# Quick Definition
Plotting pitches by the logarithm of their frequency (rather than by frequency itself) produces a scale where equal musical intervals appear as equal distances, matching the way musicians perceive and notate pitch.

# Formal Definition
On a logarithmic pitch axis, each pitch with frequency f is plotted at position log_b(f). Property (L2) guarantees that if x/y = x'/y' (same interval ratio), then log_b(x) - log_b(y) = log_b(x') - log_b(y') (same distance on the axis). The base b determines the unit of measurement: b = 2 makes octaves appear as unit distances; b = 2^(1/12) makes semitones appear as unit distances.

# Mathematical Context
On a linear frequency axis, A2 (110 Hz), A3 (220 Hz), A4 (440 Hz), A5 (880 Hz) are not equally spaced (distances 110, 220, 440). On the log_10 axis, they appear at positions log_10(110) ~ 2.041, log_10(220) ~ 2.342, log_10(440) ~ 2.643, log_10(880) ~ 2.944, which are equally spaced (difference ~ 0.301 = log_10(2)). The logarithmic scale transforms the exponential frequency-to-pitch relationship into a linear one.

# Musical Context
The logarithmic pitch scale matches musical intuition and notation. On a musical staff, the vertical distance between any two notes one octave apart appears the same -- this is exactly the logarithmic property. Standard tuning meters, frequency analyzers, and MIDI pitch numbers all use logarithmic pitch scales. The piano keyboard itself is essentially a logarithmic pitch scale made physical.

# Examples
- Linear axis: A2=110, A3=220, A4=440, A5=880 -- distances 110, 220, 440 (unequal)
- Log_10 axis: positions ~2.041, ~2.342, ~2.643, ~2.944 -- equal spacing of ~0.301
- Log_2 axis: positions ~6.78, ~7.78, ~8.78, ~9.78 -- equal spacing of exactly 1 (octave)
- A piano keyboard maps roughly 7+ octaves with equal spacing per semitone

# Related Concepts
- Converting Ratios to Octaves
- Logarithm Properties
- Multiplicative-to-Additive Conversion
- Multiplicative and Additive Measurements

# Common Confusions
- A logarithmic pitch scale does not distort intervals; it represents them more faithfully than a linear frequency scale
- Any logarithm base gives equally spaced intervals; the base only affects the unit spacing
- The musical staff is already (approximately) a logarithmic scale; the logarithm formalizes what notation does visually

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 68-70.
