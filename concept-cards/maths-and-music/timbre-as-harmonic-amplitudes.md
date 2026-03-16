---
# === CORE IDENTIFICATION ===
concept: Timbre as Harmonic Amplitudes
slug: timbre-as-harmonic-amplitudes

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: fourier-analysis
tier: intermediate

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
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - timbre
  - fourier-series
  - harmonics-and-overtones
extends:
  - timbre
related:
  - phase-shift-and-amplitude
  - square-wave-fourier-analysis
  - formants
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do Fourier coefficients determine timbre?"
  - "Why don't phase shifts affect the sound?"
---

# Quick Definition

The timbre of a sustained musical tone is determined solely by the relative sizes of the harmonic amplitudes $d_1, d_2, d_3, \ldots$, independent of the phase shifts $\beta_1, \beta_2, \beta_3, \ldots$. This is the central insight connecting Fourier analysis to music.

# Core Definition

"For a given pitch, it is the relative sizes of the (non-negative) amplitudes $d_1, d_2, d_3, \ldots$ that determines the timbre, or 'personality', of a sustained tone, allowing us to distinguish between different musical voices and instruments. We can think of $d_k$ as the 'weight' or 'degree of presence' of the $k^{\text{th}}$ harmonic in the sound represented by $g(t)$. The timbre of the tone seems to depend on this sequence alone, independent of the sequence of phase shifts $\beta_1, \beta_2, \beta_3, \ldots$" (Wright, Ch. 10, p. 127).

# Prerequisites

- **Timbre** -- The perceptual concept being characterized
- **Fourier Series** -- The mathematical decomposition
- **Harmonics and Overtones** -- The components with amplitudes $d_k$

# Key Properties

1. Timbre depends only on $\{d_k\}$ where $d_k = \sqrt{A_k^2 + B_k^2}$
2. Phase shifts $\{\beta_k\}$ affect graph shape but not perceived sound
3. Two functions with identical $\{d_k\}$ but different $\{\beta_k\}$ sound the same
4. The human auditory system effectively discards phase information
5. $d_k$ represents the "weight" of the $k$-th harmonic

# Construction / Recognition

## To characterize a tone's timbre:
1. Compute the Fourier series
2. For each harmonic $k$, compute $d_k = \sqrt{A_k^2 + B_k^2}$
3. The sequence $\{d_1, d_2, d_3, \ldots\}$ fully characterizes the timbre
4. Ignore the phase shifts -- they are musically irrelevant

# Context & Application

Different instruments playing the same pitch have different amplitude profiles. A clarinet has predominantly odd harmonics. A flute has a strong fundamental with weak upper harmonics. A trumpet has many strong harmonics. These profiles are what the ear uses to identify the instrument.

# Examples

**Example 1** (p. 127): Pure tone (tuning fork): $d_1 > 0$, $d_k = 0$ for $k \geq 2$ -- bland, featureless.

**Example 2** (p. 132): Square wave: $d_k = 4/(k\pi)$ for odd $k$, $d_k = 0$ for even $k$ -- hollow, clarinet-like.

**Example 3** (p. 127): Changing all phase shifts while keeping amplitudes fixed: graph changes completely, but sound is identical.

# Relationships

## Builds Upon
- **Timbre** -- This is the mathematical characterization
- **Fourier Series** -- The decomposition providing the amplitudes

## Enables
- **Understanding of instrument identity** -- Why instruments sound different

## Related
- **Phase Shift and Amplitude** -- Amplitude is the relevant parameter
- **Square Wave Fourier Analysis** -- Illustrates specific amplitude profile
- **Formants** -- Formants shape the amplitude profile

# Common Errors

- **Error**: Assuming waveform shape determines timbre
  **Correction**: Shape depends on both amplitudes and phases; timbre depends only on amplitudes

# Common Confusions

- **Confusion**: This seems counterintuitive -- how can two different-looking waves sound the same?
  **Clarification**: The ear performs something analogous to computing power spectra (discarding phase); oscilloscope displays are misleading for timbre analysis

# Source Reference

Chapter 10: "Timbre and Periodic Functions," p. 127.

# Verification Notes

- Definition source: Direct quote from p. 127
- Confidence rationale: Explicit statement with clear explanation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: "weight" terminology, square wave and pure tone examples
