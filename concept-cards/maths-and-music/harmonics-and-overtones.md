---
concept: Harmonics and Overtones
slug: harmonics-and-overtones

category: harmonics-and-timbre
subcategory: fourier-analysis
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Harmonics and Overtones"

extraction_confidence: high

aliases:
  - "harmonic"
  - "overtone"
  - "partial"

prerequisites:
  - fourier-series
  - fundamental-frequency
extends:
  - fourier-series
related:
  - overtone-series
  - timbre-as-harmonic-amplitudes
  - overtone-singing
  - reinforced-overtone
contrasts_with: []

answers_questions:
  - "What are harmonics and overtones?"
  - "How do harmonics differ from overtones in numbering?"
---

# Quick Definition

The $k$-th harmonic of a tone with fundamental frequency $F$ is the component at frequency $kF$, with amplitude $d_k$ and phase shift $\beta_k$. The $k$-th overtone is the $(k+1)$-th harmonic. These numbering systems differ by one.

# Core Definition

"The $k^{\text{th}}$ summand $d_k\sin(2\pi Fkt + \beta_k)$ [...] is called the $k^{\text{th}}$ harmonic of the function $g(t)$. For $k \geq 1$ it is also called the $(k-1)^{\text{th}}$ overtone of $g(t)$. When isolated, this harmonic gives the pitch $kF$" (Wright, Ch. 10, p. 127). The amplitude $d_k = \sqrt{A_k^2 + B_k^2}$ and the phase shift $\beta_k$ are computed from the Fourier coefficients.

# Prerequisites

- **Fourier Series** -- Harmonics are the individual terms of the Fourier series
- **Fundamental Frequency** -- The fundamental is the first harmonic

# Key Properties

1. The $k$-th harmonic has frequency $kF$, amplitude $d_k$, and phase shift $\beta_k$
2. The $k$-th overtone is the $(k+1)$-th harmonic (off by one)
3. The 1st harmonic = fundamental = 0th overtone
4. Each harmonic is obtained from $\sin t$ by shifting, compressing, and stretching
5. Harmonics are generally not perceived individually as separate pitches
6. The totality of audible harmonics determines timbre

# Construction / Recognition

## Harmonic numbering:
1. 1st harmonic = fundamental = $F$ (0th overtone)
2. 2nd harmonic = 1st overtone = $2F$ (octave above)
3. 3rd harmonic = 2nd overtone = $3F$ (octave + fifth)
4. 4th harmonic = 3rd overtone = $4F$ (two octaves)
5. 5th harmonic = 4th overtone = $5F$ (two octaves + major third)

# Context & Application

Harmonics integrate into a single perceived tone whose timbre depends on the amplitudes. However, harmonics can become individually audible through overtone singing (manipulating vocal resonance) or as reinforced overtones in well-tuned chords. The sequence of harmonics corresponds to the integer ratios discussed in Chapter 9.

# Examples

**Example 1** (p. 127): Starting from $F_2$ as fundamental, the first 13 harmonics approximate on the keyboard: $F_2, F_3, C_4, F_4, A_4, C_5, (E_5^\flat), F_5, G_5, A_5, (\text{between } B_5^\flat \text{ and } B_5), C_6, (\text{between } C_6^\sharp \text{ and } D_6)$.

**Example 2**: 3rd harmonic of $C_3$: frequency $3F = G_4$ approximately (off by ~2 cents).

# Relationships

## Builds Upon
- **Fourier Series** -- Each harmonic is a term in the Fourier series
- **Fundamental Frequency** -- The reference frequency for all harmonics

## Enables
- **Timbre as Harmonic Amplitudes** -- Amplitudes determine timbre
- **Overtone Singing** -- Isolating individual harmonics
- **Reinforced Overtone** -- Harmonics shared between chord tones

## Related
- **Overtone Series** -- The sequence of all harmonics

# Common Errors

- **Error**: Saying "3rd harmonic" when meaning "3rd overtone"
  **Correction**: The 3rd harmonic is at $3F$; the 3rd overtone is at $4F$. Always clarify which numbering system you're using.

# Common Confusions

- **Confusion**: Thinking harmonics are heard as separate pitches
  **Clarification**: Harmonics are generally integrated into a single perceived tone; they determine timbre, not melody. Exceptions: overtone singing and reinforced overtones.

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Harmonics and Overtones" section, pp. 126-128.

# Verification Notes

- Definition source: Direct quote from p. 127
- Confidence rationale: Explicit definition with numbering examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: off-by-one numbering table, F2 harmonic sequence
