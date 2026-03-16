---
concept: Fourier Series
slug: fourier-series

category: harmonics-and-timbre
subcategory: fourier-analysis
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Fourier Theory"

extraction_confidence: high

aliases:
  - "Fourier decomposition"
  - "harmonic series expansion"

prerequisites:
  - periodic-functions
  - sine-and-cosine-functions
  - general-sinusoidal-form
extends: []
related:
  - fourier-coefficients
  - fourier-series-for-arbitrary-period
  - harmonics-and-overtones
  - timbre-as-harmonic-amplitudes
  - square-wave-fourier-analysis
contrasts_with: []

answers_questions:
  - "What is a Fourier series?"
  - "How can any periodic function be expressed in terms of sines and cosines?"
---

# Quick Definition

The Fourier series decomposes any well-behaved periodic function into an infinite sum of sine and cosine functions at integer multiples of the fundamental frequency. This is the central mathematical tool connecting waveform shape to timbre.

# Core Definition

**Theorem:** "Suppose $f(t)$ is periodic of period $2\pi$ which is bounded and has a bounded continuous derivative at all but finitely many points in $[0, 2\pi)$. Then there is a real number $C$ and sequences of real numbers $A_1, A_2, A_3, \ldots$ and $B_1, B_2, B_3, \ldots$ such that, for all $t$ at which $f(t)$ is continuous we have $f(t)$ represented by the convergent sum $f(t) = C + \sum_{k=1}^{\infty}[A_k \sin(kt) + B_k \cos(kt)]$" (equation 10.5) (Wright, Ch. 10, pp. 125-126).

# Prerequisites

- **Periodic Functions** -- The functions being decomposed
- **Sine and Cosine Functions** -- The basis functions
- **General Sinusoidal Form** -- Each term can be written in amplitude-phase form

# Key Properties

1. Applies to functions that are bounded, periodic, and have bounded continuous derivatives at all but finitely many points
2. The series converges at all points of continuity
3. The coefficients $C$, $A_k$, $B_k$ are uniquely determined (the Fourier coefficients)
4. Based on the work of Joseph Fourier (1768-1830)
5. The convergence involves calculus concepts (limits, infinite summation)

# Construction / Recognition

## Structure of the Fourier series:
1. Constant term $C$ (the average value)
2. For each $k = 1, 2, 3, \ldots$: terms $A_k \sin(kt) + B_k \cos(kt)$
3. Each term can be rewritten as $d_k \sin(kt + \beta_k)$ with $d_k = \sqrt{A_k^2 + B_k^2}$
4. The $k$-th term oscillates $k$ times faster than the fundamental

# Context & Application

The Fourier series is the mathematical foundation for understanding timbre. It shows that every musical tone is a superposition of pure tones (sine waves) at frequencies that are integer multiples of the fundamental. The relative amplitudes of these components determine the sound's character. Only finitely many overtones fall within the audible range, so a truncated series suffices for the perceived sound.

# Examples

**Example 1** (p. 132): Square wave: $s(t) = \frac{4}{\pi}\sum_{n=0}^{\infty}\frac{1}{2n+1}\sin((2n+1)t)$ -- only odd harmonics.

**Example 2** (Exercise 9, p. 136): Sawtooth wave: $q(t) = -\frac{2}{\pi}\sum_{k=1}^{\infty}\frac{1}{k}\sin(kt)$ -- all harmonics.

**Example 3** (p. 155): $\sin t$ is its own Fourier series ($A_1 = 1$, all other coefficients zero).

**Example 4** (p. 155): The sum $\sum_{k=0}^{\infty}\frac{1}{2^k} = 2$ illustrates convergence of infinite series.

# Relationships

## Builds Upon
- **Periodic Functions** -- Functions being decomposed
- **Sine and Cosine Functions** -- Basis functions
- **General Sinusoidal Form** -- Each harmonic term

## Enables
- **Fourier Coefficients** -- Uniquely determined by the function
- **Harmonics and Overtones** -- Each term is a harmonic
- **Timbre as Harmonic Amplitudes** -- Amplitudes determine timbre

## Related
- **Square Wave Fourier Analysis** -- The detailed worked example
- **Fourier Series for Arbitrary Period** -- Generalization beyond $2\pi$

# Common Errors

- **Error**: Thinking the Fourier series is merely an approximation
  **Correction**: It converges exactly to $f(t)$ at all points of continuity; at discontinuities it converges to the average of left and right limits

# Common Confusions

- **Confusion**: Wondering why infinitely many smooth sine functions can represent a discontinuous function
  **Clarification**: Infinitely many terms are needed precisely because of the discontinuities; any finite truncation produces smooth "wiggles" near the jumps (Gibbs phenomenon)

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Fourier Theory" section, pp. 125-126, equation (10.5).

# Verification Notes

- Definition source: Direct quote of theorem from pp. 125-126
- Confidence rationale: Explicit theorem statement
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: convergence clarification, sawtooth and square wave examples
