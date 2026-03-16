---
concept: Square Wave Fourier Analysis
slug: square-wave-fourier-analysis

category: harmonics-and-timbre
subcategory: fourier-analysis
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Example: the Square Wave"

extraction_confidence: high

aliases:
  - "square wave"

prerequisites:
  - fourier-series
  - fourier-coefficients
extends:
  - fourier-coefficients
related:
  - odd-harmonics-only
  - timbre-as-harmonic-amplitudes
  - formants
contrasts_with:
  - pure-tone

answers_questions:
  - "What is the Fourier series of a square wave?"
  - "Why does the square wave have only odd harmonics?"
  - "Why does a square wave sound like a clarinet?"
---

# Quick Definition

The square wave alternates between $+1$ and $-1$, and its Fourier series contains only odd harmonics with amplitudes decreasing as $1/k$: $s(t) = \frac{4}{\pi}\sum_{n=0}^{\infty}\frac{1}{2n+1}\sin((2n+1)t)$. Its timbre vaguely resembles a clarinet.

# Core Definition

The square wave is defined on $[0, 2\pi)$ as $s(t) = 1$ for $0 \leq t < \pi$, $s(t) = -1$ for $\pi \leq t < 2\pi$, extended by periodicity. After detailed computation of the Fourier coefficients (equations 10.6), Wright shows: $C = 0$, $B_k = 0$ for all $k$, $A_k = 0$ for even $k$, and $A_k = 4/(k\pi)$ for odd $k$. The resulting series is equation (10.14): $s(t) = \frac{4}{\pi}\sum_{n=0}^{\infty}\frac{1}{2n+1}\sin((2n+1)t)$ (Wright, Ch. 10, pp. 128-132).

# Prerequisites

- **Fourier Series** -- The theorem being applied
- **Fourier Coefficients** -- The integral formulas used

# Key Properties

1. $C = 0$ (equal area above and below axis)
2. $B_k = 0$ for all $k$ (cosine symmetry around $t = \pi$)
3. $A_k = 0$ for even $k$ (sine symmetry when $k$ is even)
4. $A_k = 4/(k\pi)$ for odd $k$
5. All phase shifts $\beta_k = 0$ (no cosine terms)
6. Amplitudes: $d_k = 4/(k\pi)$ for odd $k$, $d_k = 0$ for even $k$
7. The key calculation uses $\int_0^{\pi}\sin t\,dt = 2$ (area under one lobe)

# Construction / Recognition

## Coefficient computation outline:
1. **C = 0**: The integral $\int_0^{2\pi}s(t)\,dt = 0$ by symmetry (equal areas above and below)
2. **B_k = 0**: $\cos(kt)$ is symmetric around $t = \pi$, so $\int_0^{\pi}\cos(kt)\,dt = \int_{\pi}^{2\pi}\cos(kt)\,dt$; multiplying by $s(t)$ (which flips sign) makes the integrals cancel
3. **A_k = 0 (k even)**: When $k$ is even, $\sin(kt)$ looks the same on $[0,\pi]$ and $[\pi,2\pi]$, so multiplication by $s(t)$ produces cancellation
4. **A_k = 4/(k*pi) (k odd)**: When $k$ is odd, the two halves add rather than cancel; the area under one lobe of $\sin(kt)$ is $2/k$

# Context & Application

The square wave's restriction to odd harmonics is shared by the clarinet, due to the physics of a cylindrical closed-at-one-end air column. This explains the faint resemblance between the sounds. The square wave is a standard waveform in electronic synthesis and signal processing.

# Examples

**Example 1** (p. 132): First few harmonics:
- $k = 1$: amplitude $4/\pi \approx 1.27$
- $k = 3$: amplitude $4/(3\pi) \approx 0.42$
- $k = 5$: amplitude $4/(5\pi) \approx 0.25$
- $k = 7$: amplitude $4/(7\pi) \approx 0.18$

**Example 2** (p. 132): Truncated series with increasing $N$ show graphs that progressively better approximate the square wave.

# Relationships

## Builds Upon
- **Fourier Coefficients** -- The integral formulas are applied here

## Enables
- **Odd Harmonics Only** -- The square wave is the primary example

## Related
- **Timbre as Harmonic Amplitudes** -- The amplitudes determine the characteristic sound
- **Formants** -- Clarinet's similar sound explained by physics

## Contrasts With
- **Pure Tone** -- A pure tone has only one harmonic; the square wave has infinitely many (odd)

# Common Errors

- **Error**: Expecting the truncated series to look like a square wave everywhere
  **Correction**: Near discontinuities, truncated series produce smooth "wiggles" (Gibbs phenomenon); more terms improve but never eliminate the overshoot

# Common Confusions

- **Confusion**: Wondering how smooth sine functions can represent a function with sharp corners
  **Clarification**: Infinitely many terms are needed; the series converges to $s(t)$ at points of continuity and to $0$ at the discontinuities (the average of left and right limits)

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Example: the Square Wave" section, pp. 128-132, equation (10.14).

# Verification Notes

- Definition source: Direct from pp. 128-132
- Confidence rationale: Complete worked derivation in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: computation outline, amplitude values, Gibbs phenomenon mention
