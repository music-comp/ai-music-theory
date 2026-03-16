---
concept: Fourier Series for Arbitrary Period
slug: fourier-series-for-arbitrary-period

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

aliases: []

prerequisites:
  - fourier-series
extends:
  - fourier-series
related:
  - fourier-coefficients
  - fundamental-frequency
  - harmonics-and-overtones
  - frequency-and-period
contrasts_with: []

answers_questions:
  - "How does the Fourier series generalize to functions of arbitrary period?"
  - "What is equation 10.7?"
---

# Quick Definition

The Fourier series for a function of arbitrary period $P$ replaces $\sin(kt)$ and $\cos(kt)$ with $\sin(2\pi kt/P)$ and $\cos(2\pi kt/P)$. This generalization is essential since real musical tones have arbitrary periods, not just $2\pi$.

# Core Definition

For a function $g(t)$ of period $P$: $g(t) = C + \sum_{k=1}^{\infty}[A_k\sin\frac{2\pi kt}{P} + B_k\cos\frac{2\pi kt}{P}]$ (equation 10.7) (Wright, Ch. 10, p. 126). This is derived by substituting $g(Pt/(2\pi))$ into the standard Fourier theorem (period $2\pi$), then recovering $g(t)$ by replacing $t$ with $2\pi t/P$.

# Prerequisites

- **Fourier Series** -- The general theory for period $2\pi$

# Key Properties

1. The $k$-th term has frequency $k/P = kF$ (integer multiple of fundamental)
2. Reduces to the standard form when $P = 2\pi$
3. The transformation from period $P$ to period $2\pi$ is $f(t) = g(Pt/(2\pi))$
4. Confirms that harmonics are at frequencies $F, 2F, 3F, \ldots$

# Construction / Recognition

## To derive the Fourier series for period P:
1. Given $g(t)$ with period $P$
2. Define $f(t) = g(Pt/(2\pi))$ -- this has period $2\pi$
3. Apply the standard theorem to get $f(t) = C + \sum[A_k\sin kt + B_k\cos kt]$
4. Replace $t$ by $2\pi t/P$ to recover $g(t) = C + \sum[A_k\sin(2\pi kt/P) + B_k\cos(2\pi kt/P)]$

# Context & Application

The period $2\pi$ has no special musical significance (its frequency $\approx 0.159$ Hz is inaudible). Real tones have arbitrary periods determined by pitch. A tone at 440 Hz has period $P = 1/440$ seconds, and its $k$-th harmonic has frequency $440k$ Hz.

# Examples

**Example 1** (p. 126): For a 440 Hz tone ($P = 1/440$): $g(t) = C + \sum[A_k\sin(2\pi \cdot 440k \cdot t) + B_k\cos(2\pi \cdot 440k \cdot t)]$.

**Example 2**: The $k$-th harmonic frequency is $k/P = 440k$ Hz. For $k = 1$: 440 Hz; $k = 2$: 880 Hz; $k = 3$: 1320 Hz.

# Relationships

## Builds Upon
- **Fourier Series** -- Generalizes the standard theorem

## Enables
- **Harmonics and Overtones** -- Confirms harmonics at integer multiples of $F$

## Related
- **Fourier Coefficients** -- Same coefficients, different basis functions
- **Fundamental Frequency** -- $F = 1/P$ is the fundamental
- **Frequency and Period** -- The reciprocal relationship $F = 1/P$

# Common Errors

- **Error**: Using $\sin(kt)$ instead of $\sin(2\pi kt/P)$ for period $P \neq 2\pi$
  **Correction**: The factor $2\pi/P$ is essential to match the correct period

# Common Confusions

- **Confusion**: Wondering why the "standard" Fourier series uses period $2\pi$
  **Clarification**: This is a mathematical convenience matching the natural period of sine and cosine, not a physical requirement

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 126-127, equation (10.7).

# Verification Notes

- Definition source: Direct from equation (10.7) on p. 126
- Confidence rationale: Explicit derivation with formula
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: 440 Hz example, derivation via substitution
