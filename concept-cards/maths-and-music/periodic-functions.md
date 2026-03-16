---
# === CORE IDENTIFICATION ===
concept: Periodic Functions
slug: periodic-functions

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
section: "Periodic Functions"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "periodic function"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - frequency-and-period
  - geometric-transformations-on-periodic-functions
  - fourier-series
  - vibrations-and-sound-waves
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a periodic function?"
  - "How do periodic functions relate to musical tones?"
---

# Quick Definition

A periodic function repeats its values at regular intervals. The smallest such interval is the period $P$, and the function's behavior is completely determined by its values on any half-open interval of length $P$.

# Core Definition

"A function $f(x)$ whose domain is all of $\mathbb{R}$ is called *periodic* if there is a positive number $P$ such that for all $x \in \mathbb{R}$, $f(x + P) = f(x)$. [...] The number $P$ is called the *period* of the function" (Wright, Ch. 10, p. 120). The function is determined by its values on $[0, P)$.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. $f(x + P) = f(x)$ for all $x \in \mathbb{R}$
2. The function is completely determined by its values on $[0, P)$
3. Any function on $[0, P)$ can be uniquely extended to a periodic function on $\mathbb{R}$
4. If $f$ has period $P$, it also has period $nP$ for any positive integer $n$
5. The frequency is $F = 1/P$ (cycles per unit time)

# Construction / Recognition

## To extend a function from [0, P) to R by periodicity:
1. Given $f(x)$ defined on $[0, P)$
2. For any $x \in \mathbb{R}$, find integer $n$ such that $x \in [nP, (n+1)P)$
3. Set $g(x) = f(x - nP)$
4. The resulting $g$ is periodic with period $P$

# Context & Application

Sound waves are periodic functions of time: the repeating pattern of air pressure variation is what the ear perceives as a sustained musical tone. The period $P$ (in seconds) determines the pitch, with frequency $F = 1/P$ Hz. The shape of one period determines the timbre.

# Examples

**Example 1** (p. 120): $\sin x$ and $\cos x$ are periodic of period $2\pi$.

**Example 2**: A vibrating string producing A4 (440 Hz) has period $P = 1/440$ seconds.

**Example 3** (p. 128): The square wave alternating between 1 and -1 every $\pi$ units has period $2\pi$.

# Relationships

## Enables
- **Frequency and Period** -- Frequency is defined as $1/P$
- **Geometric Transformations on Periodic Functions** -- Shifting and stretching periodic functions
- **Fourier Series** -- Decomposes periodic functions into sine and cosine components

## Related
- **Vibrations and Sound Waves** -- Physical vibrations are periodic

# Common Errors

- **Error**: Assuming the period is always the smallest positive value satisfying $f(x+P) = f(x)$
  **Correction**: The formal definition allows any such $P$ to be called "a period"; if $f$ has period $P$, it also has period $2P$, $3P$, etc.

# Common Confusions

- **Confusion**: Thinking the function must be continuous to be periodic
  **Clarification**: Periodic functions may have discontinuities (like the square wave); the Fourier theorem allows finitely many per period

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Periodic Functions" section, p. 120.

# Verification Notes

- Definition source: Direct quote from p. 120
- Confidence rationale: Explicit formal definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: extension by periodicity procedure, period multiplicity note
