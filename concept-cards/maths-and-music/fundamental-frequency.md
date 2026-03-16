---
# === CORE IDENTIFICATION ===
concept: Fundamental Frequency
slug: fundamental-frequency

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: fourier-analysis
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Harmonics and Overtones"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "first harmonic"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - frequency-and-period
  - fourier-series
extends:
  - frequency-and-period
related:
  - harmonics-and-overtones
  - overtone-series
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the fundamental frequency of a tone?"
  - "How does the fundamental relate to harmonics?"
---

# Quick Definition

The fundamental frequency $F = 1/P$ is the lowest frequency component of a periodic tone, equal to the reciprocal of the period. It determines the perceived pitch, while higher harmonics ($2F, 3F, \ldots$) determine the timbre.

# Core Definition

For a periodic function $g(t)$ with period $P$, the fundamental frequency is $F = 1/P$. In the Fourier series $g(t) = C + \sum d_k\sin(2\pi Fkt + \beta_k)$, the term with $k = 1$ is the first harmonic (fundamental), having frequency $F$. All other harmonics have frequencies that are integer multiples of $F$ (Wright, Ch. 10, pp. 126-127).

# Prerequisites

- **Frequency and Period** -- $F = 1/P$
- **Fourier Series** -- The fundamental appears as the $k = 1$ term

# Key Properties

1. $F = 1/P$ where $P$ is the period
2. All harmonics have frequencies $kF$ ($k = 1, 2, 3, \ldots$)
3. The fundamental is the GCD of all harmonic frequencies
4. The fundamental determines the perceived pitch
5. The fundamental may be weak or absent, yet the pitch is still perceived

# Construction / Recognition

## To determine the fundamental frequency:
1. Identify the period $P$ of the function
2. Compute $F = 1/P$
3. Verify: all harmonic frequencies $kF$ are integer multiples of $F$

# Context & Application

A440 tuning means the fundamental frequency of A4 is 440 Hz. Even if the fundamental is weak or absent, the ear can often infer the pitch from higher harmonics -- the "missing fundamental" phenomenon.

# Examples

**Example 1** (p. 127): Starting from $F_2$ as fundamental, the sequence of harmonics approximates $F_2, F_3, C_4, F_4, A_4, C_5, \ldots$ on the keyboard.

**Example 2**: A4: fundamental 440 Hz, period $1/440$ seconds.

**Example 3**: $\sin(880\pi t)$: frequency $880\pi/(2\pi) = 440$ Hz.

# Relationships

## Builds Upon
- **Frequency and Period** -- $F = 1/P$

## Enables
- **Harmonics and Overtones** -- All harmonics are multiples of the fundamental
- **Overtone Series** -- The sequence $F, 2F, 3F, \ldots$

# Common Errors

- **Error**: Assuming the fundamental is always the loudest harmonic
  **Correction**: The fundamental need not be the strongest harmonic; in some instruments, higher harmonics dominate

# Common Confusions

- **Confusion**: Confusing fundamental frequency with the lowest audible frequency
  **Clarification**: The fundamental is specific to a particular tone; different notes have different fundamentals

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Harmonics and Overtones" section, pp. 126-127.

# Verification Notes

- Definition source: Synthesized from pp. 126-127
- Confidence rationale: Clear implicit definition from the Fourier series treatment
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: missing fundamental mention, F2 harmonic sequence
