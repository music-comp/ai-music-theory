---
# === CORE IDENTIFICATION ===
concept: Piecewise Functions
slug: piecewise-functions

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
section: "Piecewise Definitions and Continuity"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "piecewise-defined function"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - continuity-and-discontinuity
  - periodic-functions
  - square-wave-fourier-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a piecewise function?"
  - "Can piecewise functions be continuous?"
---

# Quick Definition

A piecewise function is defined by different formulas on different portions of its domain. Such functions are essential for describing waveforms like the square wave and triangle wave, and may or may not be continuous.

# Core Definition

A piecewise-defined function assigns different expressions to different subsets of its domain (Wright, Ch. 10, pp. 118-119). The conditions for the Fourier theorem require that $f(t)$ be bounded and have only finitely many discontinuities on each period $[0, P)$.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Different formulas apply on different subsets of the domain
2. May or may not be continuous, depending on whether pieces "match up" at boundaries
3. The Fourier theorem accepts piecewise functions with finitely many discontinuities
4. Many important waveforms are piecewise-defined

# Construction / Recognition

## To define a piecewise function:
1. Partition the domain into intervals
2. Assign a formula to each interval
3. Specify boundary behavior (which formula applies at endpoints)
4. Check continuity at boundaries: do the formulas agree at junction points?

# Context & Application

Many important waveforms in acoustics and signal processing are piecewise-defined: the square wave (alternating between 1 and -1), the sawtooth wave (linear ramp with periodic jumps), and the triangle wave (piecewise linear, continuous). Each produces a distinctive timbre.

# Examples

**Example 1** (p. 118): $g(x) = x$ for $x \leq 1$, $g(x) = 1$ for $x > 1$: continuous piecewise function (pieces match at $x = 1$).

**Example 2** (p. 128): Square wave: $s(t) = 1$ for $0 \leq t < \pi$, $s(t) = -1$ for $\pi \leq t < 2\pi$: discontinuous at $t = 0$ and $t = \pi$.

**Example 3** (Exercise 10, p. 137): Triangle wave: $r(t) = \frac{2}{\pi}t - 1$ for $0 \leq t < \pi$, $r(t) = -\frac{2}{\pi}t + 3$ for $\pi \leq t < 2\pi$: piecewise linear and continuous.

**Example 4** (Exercise 9, p. 136): Sawtooth wave: $q(t) = \frac{1}{\pi}t - 1$ on $[0, 2\pi)$: a single piece but discontinuous when extended periodically.

# Relationships

## Enables
- **Square Wave Fourier Analysis** -- The square wave is piecewise-defined
- **Fourier Series** -- Piecewise functions with finitely many discontinuities have Fourier series

## Related
- **Continuity and Discontinuity** -- Piecewise functions may have discontinuities at boundaries
- **Periodic Functions** -- Piecewise functions can be extended periodically

# Common Errors

- **Error**: Assuming all piecewise functions have discontinuities
  **Correction**: The triangle wave is piecewise linear but continuous everywhere; pieces can match at boundaries

# Common Confusions

- **Confusion**: Thinking piecewise means discontinuous
  **Clarification**: "Piecewise" describes how the function is defined, not whether it is continuous; continuity depends on whether pieces agree at junction points

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 118-120 and exercises pp. 136-137.

# Verification Notes

- Definition source: Presented through examples on pp. 118-119
- Confidence rationale: Multiple examples with clear explanations
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: triangle wave example, continuity clarification
