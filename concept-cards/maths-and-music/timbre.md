---
# === CORE IDENTIFICATION ===
concept: Timbre
slug: timbre

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: acoustics
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Timbre"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "tone quality"
  - "tone color"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - timbre-as-harmonic-amplitudes
  - fourier-series
  - formants
  - pure-tone
  - harmonics-and-overtones
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is timbre?"
  - "What makes a violin sound different from a flute?"
---

# Quick Definition

Timbre is the quality of a musical tone that distinguishes different instruments or voices playing the same pitch -- what makes a violin sound different from a flute, a trombone, or a human vowel.

# Core Definition

"The term *timbre* refers to the quality or distinguishing properties of a musical tone other than its pitch, i.e., that which enables one to distinguish between a violin, a trombone, a flute, the vowel o, or the vowel e, even though the tones have the same pitch" (Wright, Ch. 10, p. 118).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Timbre is what distinguishes instruments playing the same pitch
2. Mathematically, timbre is determined by the relative amplitudes $d_1, d_2, d_3, \ldots$
3. Phase shifts do not affect timbre -- only amplitudes matter
4. Timbre is independent of pitch and loudness
5. Each instrument has a characteristic amplitude profile

# Construction / Recognition

## To analyze timbre:
1. Decompose the waveform into its Fourier series
2. Compute the amplitudes $d_k = \sqrt{A_k^2 + B_k^2}$ for each harmonic
3. The sequence $\{d_k\}$ determines the timbre
4. Phase shifts $\{\beta_k\}$ can be ignored for timbre analysis

# Context & Application

Timbre enables listeners to distinguish instruments and voices even when all produce the same pitch. Each instrument has a characteristic pattern of harmonic amplitudes, often shaped by formants (resonant frequency ranges determined by the instrument's physical structure).

# Examples

**Example 1** (p. 118): A violin, trombone, flute, and two vowels can be distinguished by timbre alone, even at the same pitch.

**Example 2** (p. 125): A pure sine wave (tuning fork) has the simplest timbre -- a "nondescript hum."

**Example 3** (p. 132): A square wave has odd harmonics only, giving a hollow timbre resembling a clarinet.

# Relationships

## Enables
- **Timbre as Harmonic Amplitudes** -- The mathematical characterization of timbre
- **Formants** -- Frequency-dependent amplitude shaping

## Related
- **Fourier Series** -- The mathematical tool for analyzing timbre
- **Pure Tone** -- The simplest timbre (one harmonic)
- **Harmonics and Overtones** -- The components that determine timbre

# Common Errors

- **Error**: Describing timbre using only subjective terms ("bright," "warm") without reference to harmonic content
  **Correction**: Timbre can be precisely characterized by the amplitudes of the harmonics in the Fourier decomposition

# Common Confusions

- **Confusion**: Thinking waveform shape directly determines timbre
  **Clarification**: Two waveforms can look very different but sound identical if they have the same harmonic amplitudes but different phase shifts; the ear is insensitive to phase

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 118 and 127.

# Verification Notes

- Definition source: Direct quote from p. 118
- Confidence rationale: Explicit definition at the chapter opening
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: phase insensitivity insight, instrument examples
