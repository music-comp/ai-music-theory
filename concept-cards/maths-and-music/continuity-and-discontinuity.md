---
concept: Continuity and Discontinuity
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
A function is continuous at a point if its value does not "jump" there -- small changes in input produce small changes in output. A discontinuity is a point where the function fails to be continuous.

# Formal Definition
A function y = f(x) is continuous at x = a if given any epsilon > 0 there exists delta > 0 such that |f(x) - f(a)| < epsilon whenever |x - a| < delta. This means f(x) will be arbitrarily close to f(a) when x is sufficiently close to a. A discontinuity is a point where this condition fails.

# Mathematical Context
The epsilon-delta definition captures the intuitive notion of "no jumps" rigorously. A discontinuity at x = a cannot be removed by reassigning f(a) if the left and right limits differ (as in h(x) = x for x < 1, h(x) = 2 for x >= 1, where the jump from 1 to 2 at x = 1 is unavoidable).

# Musical Context
Physical vibrations are continuous (an object's position cannot jump instantaneously). However, mathematical models of sound waves may include discontinuities as idealizations of very rapid transitions. The Fourier theory in Chapter 10 accommodates functions with finitely many discontinuities per period, interpreting them as moments of near-instantaneous change.

# Examples
- g(x) = x for x <= 1, g(x) = 1 for x > 1: continuous everywhere (no jump at x = 1)
- h(x) = x for x <= 1, h(x) = 2 for x > 1: discontinuous at x = 1 (jump from 1 to 2)
- The square wave s(t): discontinuous at t = 0 and t = pi (jumps between 1 and -1)

# Related Concepts
- Piecewise Functions
- Periodic Functions
- Square Wave Fourier Analysis
- Fourier Series

# Common Confusions
Students sometimes think a discontinuity can always be "fixed" by choosing the right value at the point. This is only possible for removable discontinuities. Jump discontinuities (where left and right limits differ) cannot be repaired by reassigning a single point.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 118-120 (PDF page 118).
