---
# === CORE IDENTIFICATION ===
concept: Odd Harmonics Only
slug: odd-harmonics-only

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
section: "Example: the Square Wave"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - fourier-coefficients
  - square-wave-fourier-analysis
extends:
  - square-wave-fourier-analysis
related:
  - timbre-as-harmonic-amplitudes
  - harmonics-and-overtones
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does it mean for a waveform to have only odd harmonics?"
  - "Why does the square wave sound like a clarinet?"
---

# Quick Definition

Some waveforms contain only odd-numbered harmonics ($1, 3, 5, 7, \ldots$) with all even harmonics absent. This produces a distinctive hollow or nasal timbre. The square wave and the clarinet are the primary examples.

# Core Definition

A periodic function has "only odd harmonics" when $d_k = 0$ for all even $k$. The Fourier series contains terms only at frequencies $F, 3F, 5F, 7F, \ldots$. "For reasons rooted in physics of sound, the clarinet also has only odd harmonics, which explains the faint resemblance of its sound to that of the square wave" (Wright, Ch. 10, p. 132).

# Prerequisites

- **Fourier Coefficients** -- Understanding which coefficients vanish
- **Square Wave Fourier Analysis** -- The primary worked example

# Key Properties

1. $A_k = B_k = 0$ (hence $d_k = 0$) for all even $k$
2. Only frequencies $F, 3F, 5F, 7F, \ldots$ are present
3. Produces a hollow or nasal quality
4. The square wave has amplitudes $d_k = 4/(k\pi)$ for odd $k$
5. The triangle wave also has only odd harmonics, but with amplitudes $\propto 1/k^2$ (softer)

# Construction / Recognition

## Why even harmonics vanish for the square wave:
1. For even $k$: $\sin(kt)$ has the same shape on $[0, \pi]$ and $[\pi, 2\pi]$
2. Multiplying by $s(t)$ (which flips sign at $\pi$) produces equal and opposite contributions
3. The integral cancels to zero

# Context & Application

The clarinet's odd-harmonic dominance comes from the physics of its cylindrical bore (closed at one end by the reed). Open-ended instruments (flute) and conical bore instruments (oboe, saxophone) produce both odd and even harmonics. The triangle wave's softer sound (compared to square wave) comes from its amplitudes decreasing as $1/k^2$ rather than $1/k$.

# Examples

**Example 1** (p. 132): Square wave: $d_k = 4/(k\pi)$ for odd $k$, $d_k = 0$ for even $k$.

**Example 2** (Exercise 10, p. 137): Triangle wave: amplitudes $\propto 1/k^2$ for odd $k$, zero for even $k$ (softer because higher harmonics drop off faster).

**Example 3** (Exercise 9, p. 136): Sawtooth wave: $d_k = 2/(k\pi)$ for ALL $k$ -- contrast with odd-harmonics-only waveforms.

# Relationships

## Builds Upon
- **Square Wave Fourier Analysis** -- The primary example

## Related
- **Timbre as Harmonic Amplitudes** -- The distinctive hollow timbre
- **Harmonics and Overtones** -- Which harmonics are present

# Common Errors

- **Error**: Assuming "odd harmonics only" means the pitch is lower or sounds an octave different
  **Correction**: The fundamental ($k = 1$, odd) is still present; the pitch is unchanged. Only the timbre is affected.

# Common Confusions

- **Confusion**: Thinking waveforms with only even harmonics exist as a common category
  **Clarification**: A waveform with only even harmonics would actually have period $P/2$, effectively having fundamental at $2F$ -- it would be heard as a tone one octave higher with all harmonics

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 130-132. Triangle wave in exercises, p. 137.

# Verification Notes

- Definition source: Synthesized from square wave analysis and exercises
- Confidence rationale: Clear from the worked example and exercises
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: clarinet physics connection, triangle vs. square wave comparison, even-harmonics-only clarification
