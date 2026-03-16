---
concept: Fourier Coefficients
slug: fourier-coefficients

category: harmonics-and-timbre
subcategory: fourier-analysis
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Fourier Theory"

extraction_confidence: high

aliases: []

prerequisites:
  - fourier-series
extends:
  - fourier-series
related:
  - phase-shift-and-amplitude
  - timbre-as-harmonic-amplitudes
  - square-wave-fourier-analysis
  - harmonics-and-overtones
contrasts_with: []

answers_questions:
  - "How do Fourier coefficients determine timbre?"
  - "How are Fourier coefficients computed?"
  - "What do the Fourier coefficients represent?"
---

# Quick Definition

The Fourier coefficients $C$, $A_k$, and $B_k$ are the specific numerical weights determining how much each sine and cosine component contributes to a periodic function's Fourier series. They are uniquely determined by definite integrals.

# Core Definition

For a periodic function $f(t)$ of period $2\pi$, the coefficients are uniquely determined by (equation 10.6):

$C = \frac{1}{2\pi}\int_0^{2\pi} f(t)\,dt$, $A_k = \frac{1}{\pi}\int_0^{2\pi}\sin(kt)f(t)\,dt$, $B_k = \frac{1}{\pi}\int_0^{2\pi}\cos(kt)f(t)\,dt$

(Wright, Ch. 10, p. 126).

# Prerequisites

- **Fourier Series** -- Coefficients are defined within the Fourier series framework

# Key Properties

1. The coefficients are uniquely determined by the function
2. $C$ is the average value of $f(t)$ over one period (DC offset)
3. The integrals exploit the orthogonality of sine and cosine functions
4. The $k$-th harmonic amplitude is $d_k = \sqrt{A_k^2 + B_k^2}$
5. The $k$-th phase shift $\beta_k$ satisfies $\cos\beta_k = A_k/d_k$, $\sin\beta_k = B_k/d_k$
6. Only the amplitudes $\{d_k\}$ affect timbre; the phase shifts $\{\beta_k\}$ do not

# Construction / Recognition

## To compute Fourier coefficients:
1. Compute $C$: integrate $f(t)$ over $[0, 2\pi]$ and divide by $2\pi$
2. For each $k$: compute $A_k$ by integrating $\sin(kt) \cdot f(t)$ over $[0, 2\pi]$ and dividing by $\pi$
3. For each $k$: compute $B_k$ by integrating $\cos(kt) \cdot f(t)$ over $[0, 2\pi]$ and dividing by $\pi$
4. The harmonic amplitude is $d_k = \sqrt{A_k^2 + B_k^2}$

# Context & Application

The Fourier coefficients encode everything about a tone's timbre. The constant $C$ has no audible effect. The coefficients $A_k$ and $B_k$ combine to give each harmonic's amplitude $d_k$ and phase shift $\beta_k$. Since only amplitudes affect timbre, the musically relevant information is the sequence $\{d_k\}$.

# Examples

**Example: Square wave** (pp. 128-132):
- $C = 0$ (equal area above and below axis)
- $B_k = 0$ for all $k$ (by symmetry of cosine around $t = \pi$)
- $A_k = 0$ for $k$ even
- $A_k = 4/(k\pi)$ for $k$ odd
- Yielding $s(t) = \frac{4}{\pi}\sum\frac{1}{2n+1}\sin((2n+1)t)$

# Relationships

## Builds Upon
- **Fourier Series** -- Coefficients appear in the Fourier series

## Enables
- **Timbre as Harmonic Amplitudes** -- Amplitudes $d_k$ determine timbre
- **Square Wave Fourier Analysis** -- Specific coefficient calculation

## Related
- **Phase Shift and Amplitude** -- $d_k$ and $\beta_k$ are computed from $A_k$ and $B_k$
- **Harmonics and Overtones** -- Each harmonic's weight is determined by its coefficient

# Common Errors

- **Error**: Thinking $A_k$ is the amplitude of the $k$-th harmonic
  **Correction**: $A_k$ is the sine coefficient; the amplitude is $d_k = \sqrt{A_k^2 + B_k^2}$. When $B_k = 0$, then $d_k = |A_k|$.

# Common Confusions

- **Confusion**: Thinking the integral formulas are approximations
  **Clarification**: The coefficients computed by the integrals are exact; the Fourier series converges to $f(t)$ at points of continuity

# Source Reference

Chapter 10: "Timbre and Periodic Functions," p. 126, equation (10.6).

# Verification Notes

- Definition source: Direct from equation (10.6) on p. 126
- Confidence rationale: Explicit formulas with complete worked example (square wave)
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: square wave coefficient computation, amplitude vs. coefficient distinction
