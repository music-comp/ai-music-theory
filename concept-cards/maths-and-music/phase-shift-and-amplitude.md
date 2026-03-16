---
concept: Phase Shift and Amplitude
slug: phase-shift-and-amplitude

category: harmonics-and-timbre
subcategory: periodic-functions
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Shifting and Stretching Sine and Cosine"

extraction_confidence: high

aliases: []

prerequisites:
  - general-sinusoidal-form
extends:
  - general-sinusoidal-form
related:
  - timbre-as-harmonic-amplitudes
  - fourier-coefficients
contrasts_with: []

answers_questions:
  - "What is the amplitude and phase shift of a sinusoidal function?"
  - "Does phase shift affect the sound of a tone?"
---

# Quick Definition

Amplitude $d = \sqrt{A^2 + B^2}$ determines loudness, while phase shift $\beta$ is a horizontal displacement that changes the waveform's shape but does not affect the perceived timbre. This insensitivity to phase is a key property of human hearing.

# Core Definition

For $h(x) = A\sin kx + B\cos kx = d\sin(kx + \beta)$: "The angle $\beta$ is called the *phase shift*, and the number $d \geq 0$ is the *amplitude*" (Wright, Ch. 10, p. 123). The point $(A/d, B/d)$ lies on the unit circle, determining $\beta$ uniquely.

# Prerequisites

- **General Sinusoidal Form** -- Phase shift and amplitude are defined through this equivalence

# Key Properties

1. Amplitude $d = \sqrt{A^2 + B^2}$ determines loudness
2. Phase shift $\beta$ determines horizontal displacement
3. Phase shifts affect the shape of the waveform graph but NOT the perceived sound
4. Timbre depends only on the amplitude sequence $\{d_k\}$, independent of $\{\beta_k\}$
5. The conversion from $(A, B)$ to $(d, \beta)$ is equivalent to Cartesian-to-polar coordinates

# Construction / Recognition

## To compute amplitude and phase shift:
1. Given $A$ and $B$ from the rectangular form
2. Amplitude: $d = \sqrt{A^2 + B^2}$
3. Normalize: $a = A/d$, $b = B/d$
4. Phase shift: $\beta$ such that $\cos\beta = a$ and $\sin\beta = b$
5. If $A = 0$: phase shift is $\pi/2$; if $B = 0$: phase shift is $0$

# Context & Application

The fact that phase shifts do not affect timbre is "counterintuitive but experimentally confirmed" (paraphrasing Wright). This means two waveforms that look completely different on an oscilloscope can sound identical if they have the same harmonic amplitudes. The ear effectively discards phase information.

# Examples

**Example 1** (p. 123): $h(x) = 3\sin x + 2\cos x$: $d = \sqrt{13}$, $\beta = \arcsin(2/\sqrt{13}) \approx 0.588$.

**Example 2**: If $A = 0$: $h(x) = B\cos kx = B\sin(kx + \pi/2)$, phase shift is $\pi/2$.

**Example 3**: If $B = 0$: $h(x) = A\sin kx$, phase shift is $0$.

# Relationships

## Builds Upon
- **General Sinusoidal Form** -- Amplitude and phase are the polar parameters

## Enables
- **Timbre as Harmonic Amplitudes** -- Timbre depends on amplitudes alone

## Related
- **Fourier Coefficients** -- Each harmonic has amplitude $d_k = \sqrt{A_k^2 + B_k^2}$

# Common Errors

- **Error**: Adding amplitudes $A + B$ to get the total amplitude
  **Correction**: The amplitude is $\sqrt{A^2 + B^2}$, not $A + B$

# Common Confusions

- **Confusion**: Assuming phase shifts must affect the sound because they affect the waveform shape
  **Clarification**: The human ear is insensitive to phase; timbre depends only on amplitudes. Two waveforms with identical $\{d_k\}$ but different $\{\beta_k\}$ sound the same despite looking different

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 122-123 and p. 127.

# Verification Notes

- Definition source: Direct quote from p. 123
- Confidence rationale: Explicit definition with worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: phase insensitivity insight, Cartesian-to-polar analogy
