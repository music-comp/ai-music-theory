---
concept: Pure Tone
slug: pure-tone

category: harmonics-and-timbre
subcategory: acoustics
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Effect of Horizontal Stretching on Pitch"

extraction_confidence: high

aliases:
  - "sine tone"

prerequisites:
  - sine-and-cosine-functions
  - vibrations-and-sound-waves
extends: []
related:
  - timbre
  - fourier-series
  - harmonics-and-overtones
contrasts_with:
  - square-wave-fourier-analysis

answers_questions:
  - "What is a pure tone?"
  - "What does a sine wave sound like?"
---

# Quick Definition

A pure tone is the sound produced by a single sine wave -- the simplest possible musical sound, containing only one frequency with no overtones. It resembles the sound of a tuning fork.

# Core Definition

"The tone given by a sine function [...] is sometimes called a 'pure tone'. It is a nondescript hum, very similar to the tone produced by a tuning fork" (Wright, Ch. 10, p. 125). In Fourier terms, a pure tone has $d_1 > 0$ and $d_k = 0$ for all $k \geq 2$.

# Prerequisites

- **Sine and Cosine Functions** -- A pure tone is a single sine wave
- **Vibrations and Sound Waves** -- A pure tone is a specific type of vibration

# Key Properties

1. Described by $y = d\sin(2\pi Ft + \beta)$ for amplitude $d$, frequency $F$, phase $\beta$
2. Only the first harmonic ($k = 1$) has non-zero amplitude
3. Sounds like a "nondescript hum"
4. The tuning fork produces an approximately pure tone
5. It is the fundamental building block of all complex tones

# Construction / Recognition

## To create a pure tone at frequency r Hz:
1. Use $y = \sin(2\pi r t)$
2. Or equivalently: $y = \sin(rPt)$ where $P = 2\pi$ is the period of $\sin t$

# Context & Application

Pure tones have pitch but minimal timbral character. Real musical instruments always produce complex tones with multiple harmonics, which gives them distinctive timbres. Pure tones serve as the "atoms" from which all complex sounds are built via Fourier superposition.

# Examples

**Example 1** (p. 125): $y = \sin(880\pi t)$: pure tone at A4 (440 Hz).

**Example 2**: A tuning fork struck gently: approximately pure.

**Example 3**: Electronic sine wave generator: exactly pure.

# Relationships

## Enables
- **Fourier Series** -- Pure tones are the building blocks of Fourier decomposition

## Related
- **Timbre** -- A pure tone has the simplest possible timbre
- **Harmonics and Overtones** -- A pure tone has no overtones

## Contrasts With
- **Square Wave Fourier Analysis** -- A square wave has many harmonics, unlike a pure tone

# Common Errors

- **Error**: Thinking pure tones are "better" or more musical than complex tones
  **Correction**: Pure tones are the simplest, but musical richness comes from the interaction of multiple harmonics

# Common Confusions

- **Confusion**: Thinking any single-frequency sound is a pure tone
  **Clarification**: A pure tone is specifically a sinusoidal vibration; other periodic waveforms at a single fundamental frequency still contain harmonics

# Source Reference

Chapter 10: "Timbre and Periodic Functions," p. 125.

# Verification Notes

- Definition source: Direct quote from p. 125
- Confidence rationale: Explicit description with clear characterization
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: "nondescript hum" quote, tuning fork comparison
