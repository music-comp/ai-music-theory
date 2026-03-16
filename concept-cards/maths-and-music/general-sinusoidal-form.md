---
concept: General Sinusoidal Form
slug: general-sinusoidal-form

category: harmonics-and-timbre
subcategory: periodic-functions
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Shifting and Stretching Sine and Cosine"

extraction_confidence: high

aliases: []

prerequisites:
  - trigonometric-summation-formula
  - sine-and-cosine-functions
extends:
  - trigonometric-summation-formula
related:
  - phase-shift-and-amplitude
  - fourier-coefficients
  - harmonics-and-overtones
contrasts_with: []

answers_questions:
  - "How do the rectangular and polar forms of a sinusoid relate?"
  - "What is the general transformation of a sine function?"
---

# Quick Definition

Any function of the form $A\sin kx + B\cos kx$ can be written as $d\sin(kx + \beta)$ where $d = \sqrt{A^2 + B^2}$ is the amplitude and $\beta$ is the phase shift. This equivalence is central to Fourier analysis.

# Core Definition

"Therefore $h(x)$ is a transformation of $\sin x$ having the form [10.3], where $d = \sqrt{A^2 + B^2}$. The angle $\beta$ is called the *phase shift*, and the number $d \geq 0$ is the *amplitude*" (Wright, Ch. 10, p. 123). The general transformation of $\sin x$ is $g(x) = d\sin(kx + \beta) = d(\cos\beta\sin kx + \sin\beta\cos kx)$ (equation 10.3).

# Prerequisites

- **Trigonometric Summation Formula** -- The conversion relies on angle addition
- **Sine and Cosine Functions** -- The building blocks

# Key Properties

1. $A\sin kx + B\cos kx = d\sin(kx + \beta)$ for appropriate $d, \beta$
2. $d = \sqrt{A^2 + B^2}$ (amplitude)
3. $\cos\beta = A/d$, $\sin\beta = B/d$ (determines phase shift)
4. The function $g(x)$ has period $2\pi/k$ and frequency $k/(2\pi)$
5. The conversion is essentially Cartesian to polar coordinates for $(A, B)$

# Construction / Recognition

## To convert from rectangular to polar form:
1. Given $h(x) = A\sin kx + B\cos kx$
2. Compute $d = \sqrt{A^2 + B^2}$
3. Compute $a = A/d$, $b = B/d$ (normalize to unit circle)
4. Find $\beta$ with $\cos\beta = a$, $\sin\beta = b$
5. Result: $h(x) = d\sin(kx + \beta)$

# Context & Application

This equivalence is essential for Fourier analysis. The Fourier theorem produces coefficients in the sum form ($A_k\sin kx + B_k\cos kx$), but the transformation form ($d_k\sin(kx + \beta_k)$) reveals the physically meaningful amplitude and phase. Since only amplitude affects timbre, the transformation form isolates the musically relevant parameter.

# Examples

**Example 1** (p. 123): $3\sin x + 2\cos x$: $d = \sqrt{13}$, $\beta \approx 0.588$. Result: $\sqrt{13}\sin(x + 0.588)$. Amplitude $\sqrt{13}$, phase shift $\approx 0.588$.

**Example 2**: $\sin x + \cos x = \sqrt{2}\sin(x + \pi/4)$.

**Example 3**: $5\sin 2x = 5\sin(2x + 0)$: amplitude 5, phase shift 0, period $\pi$.

# Relationships

## Builds Upon
- **Trigonometric Summation Formula** -- The algebraic identity enabling conversion

## Enables
- **Phase Shift and Amplitude** -- These are the polar parameters
- **Fourier Coefficients** -- Each harmonic has this form
- **Harmonics and Overtones** -- Each overtone is described by this form

# Common Errors

- **Error**: Thinking $A$ is the amplitude of the $k$-th harmonic
  **Correction**: The amplitude is $d = \sqrt{A^2 + B^2}$, not $A$ or $B$ individually

# Common Confusions

- **Confusion**: Thinking $A$ and $B$ independently contribute to loudness
  **Clarification**: The combined amplitude is $\sqrt{A^2 + B^2}$, not $A + B$ or $\max(A, B)$

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 122-123, equations (10.3) and (10.4).

# Verification Notes

- Definition source: Direct quote from p. 123
- Confidence rationale: Explicit derivation with worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Cartesian-to-polar analogy, worked example
