---
# === CORE IDENTIFICATION ===
concept: Sine and Cosine Functions
slug: sine-and-cosine-functions

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: periodic-functions
tier: foundational

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
  - "sinusoidal functions"
  - "trigonometric functions"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - periodic-functions
extends: []
related:
  - trigonometric-summation-formula
  - phase-shift-and-amplitude
  - pure-tone
  - fourier-series
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What role do sine and cosine play in sound analysis?"
  - "How are sine and cosine related to each other?"
---

# Quick Definition

The sine and cosine functions are the fundamental building blocks of all periodic functions in Fourier analysis. They are periodic with period $2\pi$ and related by a phase shift of $\pi/2$: $\cos x = \sin(x + \pi/2)$.

# Core Definition

The functions $y = \sin x$ and $y = \cos x$ are periodic of period $2\pi$. "The graph of $y = \cos x$ is obtained by shifting the graph of $y = \sin x$ to the left by $c = \pi/2$. This is because $\cos x = \sin(x + \pi/2)$" (Wright, Ch. 10, p. 121). This relationship follows from the summation formula with $\beta = \pi/2$, since $\cos(\pi/2) = 0$ and $\sin(\pi/2) = 1$.

# Prerequisites

- **Periodic Functions** -- Sine and cosine are specific periodic functions

# Key Properties

1. Period $2\pi$ for both functions
2. $\cos x = \sin(x + \pi/2)$ -- cosine is sine shifted left by $\pi/2$
3. Both are continuous and smooth (infinitely differentiable)
4. They form an orthogonal basis for Fourier analysis
5. The point $(\cos\beta, \sin\beta)$ lies on the unit circle at arc length $\beta$ from $(1, 0)$

# Construction / Recognition

## Key values:
1. $\sin(0) = 0$, $\cos(0) = 1$
2. $\sin(\pi/2) = 1$, $\cos(\pi/2) = 0$
3. $\sin(\pi) = 0$, $\cos(\pi) = -1$
4. Both oscillate between $-1$ and $+1$

# Context & Application

A pure sine wave produces the simplest possible musical tone -- a "nondescript hum" similar to a tuning fork. All complex timbres are superpositions of sine and cosine waves at different frequencies and amplitudes. Sine is thus the "atom" of sound.

# Examples

**Example 1** (p. 121): $\cos x = \sin(x + \pi/2)$ -- derived from the summation formula.

**Example 2** (p. 125): $\sin(880\pi t)$ produces A4 at 440 Hz.

**Example 3** (p. 123): $3\sin x + 2\cos x = \sqrt{13}\sin(x + \beta)$ where $\beta \approx 0.588$ -- demonstrating the general sinusoidal form.

# Relationships

## Enables
- **Fourier Series** -- Sine and cosine are the basis functions
- **Pure Tone** -- A pure tone is a single sine wave
- **General Sinusoidal Form** -- Combinations of sine and cosine

## Related
- **Trigonometric Summation Formula** -- Connects sine and cosine via angle addition
- **Phase Shift and Amplitude** -- Parameters of sinusoidal functions

# Common Errors

- **Error**: Computing the frequency of $\sin(\alpha t)$ as $\alpha$ Hz
  **Correction**: The frequency is $\alpha/(2\pi)$ Hz, since one complete cycle corresponds to $2\pi$ radians

# Common Confusions

- **Confusion**: Thinking sine and cosine are fundamentally different
  **Clarification**: They are identical except for a phase shift of $\pi/2$; both produce the same pure tone in isolation

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Shifting and Stretching Sine and Cosine" section, pp. 121-123.

# Verification Notes

- Definition source: Direct from pp. 121-123
- Confidence rationale: Explicit treatment with formulas and examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: phase shift relationship, unit circle interpretation
