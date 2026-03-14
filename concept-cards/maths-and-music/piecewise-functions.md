---
concept: Piecewise Functions
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
A piecewise function is defined by different formulas on different portions of its domain. Such functions are essential for describing waveforms like the square wave.

# Formal Definition
A piecewise-defined function assigns different expressions to different subsets of its domain. For example, g(x) = x for x <= 1 and g(x) = 1 for x > 1 uses two pieces. Piecewise functions may or may not be continuous, depending on whether the pieces "match up" at the boundaries.

# Mathematical Context
Piecewise definitions are needed to describe waveforms that cannot be expressed by a single formula. The conditions for the Fourier theorem require that f(t) be bounded and have only finitely many discontinuities on each period [0, P). This allows piecewise functions with finitely many "jumps."

# Musical Context
Many important waveforms in acoustics and signal processing are piecewise-defined: the square wave (alternating between 1 and -1), the sawtooth wave (linear ramp with periodic jumps), and the triangle wave (piecewise linear with no jumps). Each produces a distinctive timbre.

# Examples
- Square wave: s(t) = 1 for 0 <= t < pi, s(t) = -1 for pi <= t < 2*pi
- Sawtooth wave: q(t) = t/pi - 1 on [0, 2*pi)
- Triangle wave: r(t) = 2t/pi - 1 for 0 <= t < pi, r(t) = -2t/pi + 3 for pi <= t < 2*pi
- Continuous piecewise example: g(x) = x for x <= 1, g(x) = 1 for x > 1

# Related Concepts
- Continuity and Discontinuity
- Periodic Functions
- Square Wave Fourier Analysis
- Fourier Series

# Common Confusions
Not all piecewise functions have discontinuities. The triangle wave is piecewise linear but continuous everywhere. The presence of multiple "pieces" does not imply jumps -- it depends on whether the formulas agree at boundary points.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 118-120 (PDF page 118).
