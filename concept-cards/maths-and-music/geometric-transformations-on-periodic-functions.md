---
concept: Geometric Transformations on Periodic Functions
category: technique
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
Shifting, stretching, and compressing periodic functions changes their musical properties in predictable ways: vertical changes affect loudness, horizontal compression changes pitch, and horizontal shifts (delays) preserve both pitch and timbre.

# Formal Definition
If y = f(x) is periodic with period P, then:
- Vertical shift y = f(x) + c: period P (unchanged)
- Horizontal shift y = f(x - c): period P (unchanged)
- Vertical stretch y = c*f(x): period P (unchanged)
- Horizontal stretch y = f(x/c): period c*P (frequency divided by c)

Equivalently, horizontal compression by factor c gives y = f(cx) with period P/c and frequency c/P = c*F.

# Mathematical Context
These results follow from the definition of periodicity. For the horizontal stretch: f((x + cP)/c) = f(x/c + P) = f(x/c), confirming period cP. The key insight is that only horizontal scaling changes the period (and hence pitch). All other transformations preserve the period.

# Musical Context
- Horizontal shift = delay: does not change pitch or timbre (just starts later)
- Vertical stretch = amplitude change: adjusts loudness without changing pitch or timbre
- Vertical shift = DC offset: no audible effect (shifts the equilibrium position)
- Horizontal compression by factor c = pitch multiplication by c: this is how we derive tones of any desired frequency from a prototype waveform

# Examples
- To produce A4 (440 Hz) from sin(t) which has period 2*pi: use sin(880*pi*t), compressing horizontally by factor 880*pi
- Doubling playback speed (c = 2): doubles frequency, raises pitch by one octave
- Halving playback speed (c = 1/2): halves frequency, lowers pitch by one octave

# Related Concepts
- Periodic Functions
- Frequency and Period
- Effect of Horizontal Stretching on Pitch
- Timbre

# Common Confusions
Students sometimes confuse horizontal stretch with horizontal compression. Replacing x by cx (for c > 1) compresses the graph horizontally, making the period shorter and the frequency higher. Replacing x by x/c stretches it, making the period longer and the frequency lower.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 120-121 (PDF page 118).
