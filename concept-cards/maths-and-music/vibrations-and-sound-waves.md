---
concept: Vibrations and Sound Waves
slug: vibrations-and-sound-waves

category: harmonics-and-timbre
subcategory: acoustics
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Vibrations"

extraction_confidence: high

aliases:
  - "vibration"
  - "sound wave"

prerequisites:
  - periodic-functions
  - frequency-and-period
extends: []
related:
  - timbre
  - fourier-series
  - pure-tone
contrasts_with: []

answers_questions:
  - "What is a vibration in mathematical terms?"
  - "How does a vibration become a sound wave?"
  - "What conditions must a periodic function satisfy to model a vibration?"
---

# Quick Definition

A vibration is an oscillation with a repeating pattern of period $P$, producing a sound wave transmitted through air as contraction and expansion, perceived by the ear as a musical tone of frequency $F = 1/P$ Hz.

# Core Definition

"We will use the term *vibration* to mean an oscillation having a pattern which repeats every interval of $P$ units of time" (Wright, Ch. 10, p. 123). The vibration is given by a periodic function $y = f(t)$ where $y$ is the position at time $t$. "The vibration is transmitted through the air by contraction and expansion (This is called a sound wave.) and received by the human ear when the ear drum is set in motion" (p. 123). The brain interprets this as a musical tone of pitch $F = 1/P$ Hz.

# Prerequisites

- **Periodic Functions** -- A vibration is modeled by a periodic function
- **Frequency and Period** -- Pitch is determined by the frequency

# Key Properties

1. A vibration is a periodic function $y = f(t)$ representing position at time $t$
2. The function must satisfy: (a) finitely many discontinuities on $[0, P)$; (b) bounded
3. Discontinuities are interpreted as moments of very rapid position change
4. Pitch is determined by $F = 1/P$
5. Timbre is determined by the harmonic amplitudes in the Fourier decomposition

# Construction / Recognition

## Conditions for a periodic function to represent a vibration:
1. $f(t)$ is periodic with period $P \in \mathbb{R}^+$
2. $f$ has only finitely many discontinuities on $[0, P)$
3. $f$ is bounded: there exist $b, B \in \mathbb{R}$ such that $b < f(t) < B$ for all $t$

# Context & Application

Vibrating motion arises from violin strings, air columns inside trumpets, and human vocal cords. The mathematical model of vibrations as periodic functions, while not an exact representation, provides the foundation for understanding pitch, timbre, and the Fourier analysis of sound.

# Examples

**Example 1** (p. 125): A string vibrating at $P = 1/440$ seconds produces A4 (440 Hz).

**Example 2** (p. 125): $\sin(880\pi t)$ gives a pure tone at A4 since its period is $1/440$ seconds.

**Example 3**: A tuning fork produces a nearly pure sinusoidal vibration.

**Example 4**: A violin string produces a complex vibration with many harmonics.

# Relationships

## Builds Upon
- **Periodic Functions** -- Vibrations are periodic
- **Frequency and Period** -- Pitch = $1/P$

## Enables
- **Fourier Series** -- Vibrations can be decomposed into harmonics
- **Timbre** -- Harmonic content determines sound quality

## Related
- **Pure Tone** -- The simplest vibration (single sine wave)

# Common Errors

- **Error**: Assuming physical vibrations can have infinitely many discontinuities
  **Correction**: The Fourier conditions require only finitely many discontinuities per period

# Common Confusions

- **Confusion**: Confusing the vibration (mechanical motion) with the sound wave (pressure variation)
  **Clarification**: The vibration is the motion of an object; the sound wave is the resulting pressure variation transmitted through air. They have the same pattern but are different physical phenomena.

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Vibrations" section, pp. 123-124.

# Verification Notes

- Definition source: Direct quotes from pp. 123-124
- Confidence rationale: Explicit definitions with physical interpretation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: boundedness conditions, vibration vs. sound wave distinction
