---
# === CORE IDENTIFICATION ===
concept: Trigonometric Summation Formula
slug: trigonometric-summation-formula

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: periodic-functions
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Shifting and Stretching Sine and Cosine"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "angle addition formula"
  - "sine addition formula"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - sine-and-cosine-functions
extends: []
related:
  - general-sinusoidal-form
  - phase-shift-and-amplitude
  - fourier-series
  - square-wave-fourier-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the trigonometric summation formula?"
  - "How does the summation formula connect sine/cosine sums to phase-shifted sinusoids?"
---

# Quick Definition

The identities $\sin(\alpha + \beta) = \sin\alpha\cos\beta + \cos\alpha\sin\beta$ and $\cos(\alpha + \beta) = \cos\alpha\cos\beta - \sin\alpha\sin\beta$, which enable conversion between the sum-of-sines-and-cosines form and the amplitude-phase form of a harmonic.

# Core Definition

The sine summation formula is $\sin(\alpha + \beta) = \sin\alpha\cos\beta + \cos\alpha\sin\beta$ (equation 10.1). The cosine summation formula is $\cos(\alpha + \beta) = \cos\alpha\cos\beta - \sin\alpha\sin\beta$ (equation 10.9). Treating $\alpha$ as variable $x$ and $\beta$ as a fixed parameter: $\sin(x + \beta) = \cos\beta\sin x + \sin\beta\cos x$ (equation 10.2) (Wright, Ch. 10, pp. 121-122).

# Prerequisites

- **Sine and Cosine Functions** -- The formulas involve these functions

# Key Properties

1. The sine formula has all plus signs: "sin-cos + cos-sin"
2. The cosine formula has a minus sign: "cos-cos - sin-sin"
3. These formulas bridge the "rectangular" form $A\sin kx + B\cos kx$ and the "polar" form $d\sin(kx + \beta)$
4. The cosine formula (10.9) is used in the square wave analysis to show all $B_k = 0$

# Construction / Recognition

## Converting between forms:
1. Given $A\sin kx + B\cos kx$, identify $A = d\cos\beta$ and $B = d\sin\beta$
2. Compute $d = \sqrt{A^2 + B^2}$
3. Find $\beta$ such that $\cos\beta = A/d$ and $\sin\beta = B/d$
4. Result: $A\sin kx + B\cos kx = d\sin(kx + \beta)$

# Context & Application

The summation formula is essential for understanding how phase shifts work in Fourier decomposition. It shows that any shifted sine wave decomposes into a weighted sum of sine and cosine at the same frequency, and vice versa.

# Examples

**Example 1** (p. 121): $\cos x = \sin(x + \pi/2)$ -- follows from the formula with $\beta = \pi/2$.

**Example 2** (p. 123): $3\sin x + 2\cos x = \sqrt{13}\sin(x + \beta)$ where $\beta = \arcsin(2/\sqrt{13}) \approx 0.588$.

**Example 3** (p. 129): The formula $\cos(k(\pi - t)) = \cos(k(\pi + t))$ (used for square wave $B_k = 0$) follows from the cosine summation formula.

# Relationships

## Enables
- **General Sinusoidal Form** -- The equivalence between rectangular and polar forms
- **Phase Shift and Amplitude** -- Computed using the summation formula
- **Square Wave Fourier Analysis** -- Cosine formula used to show $B_k = 0$

## Related
- **Fourier Series** -- The summation formula underlies Fourier series manipulations

# Common Errors

- **Error**: Mixing up the signs in the cosine formula
  **Correction**: Cosine of a sum has a minus sign: $\cos(\alpha + \beta) = \cos\alpha\cos\beta - \sin\alpha\sin\beta$

# Common Confusions

- **Confusion**: Thinking the conversion from $A\sin + B\cos$ to $d\sin(\cdot + \beta)$ loses information
  **Clarification**: The conversion is exact and reversible; $d$ and $\beta$ encode the same information as $A$ and $B$

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 121-123, equations (10.1), (10.2), and (10.9).

# Verification Notes

- Definition source: Direct from pp. 121-123
- Confidence rationale: Explicit formulas with derivations
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: sign mnemonic, square wave connection
