---
# === CORE IDENTIFICATION ===
concept: Frequency and Period
slug: frequency-and-period

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
section: "Vibrations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "pitch and frequency"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - periodic-functions
extends:
  - periodic-functions
related:
  - vibrations-and-sound-waves
  - fundamental-frequency
  - effect-of-horizontal-stretching-on-pitch
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the relationship between frequency and period?"
  - "How does period determine pitch?"
---

# Quick Definition

The period $P$ of a vibration is the time for one complete cycle (in seconds). The frequency $F = 1/P$ is the number of cycles per second, measured in Hertz (Hz). Frequency determines the perceived pitch of a musical tone.

# Core Definition

"The frequency of the vibration, i.e., the number of repetitions of its pattern per unit of time, is $1/P$" (Wright, Ch. 10, p. 123). For a periodic function $f(t)$ with period $P$ measured in seconds, the frequency is $F = 1/P$ Hz. The relationship is reciprocal: longer period means lower frequency (lower pitch), shorter period means higher frequency (higher pitch).

# Prerequisites

- **Periodic Functions** -- Frequency is defined for periodic functions

# Key Properties

1. $F = 1/P$ and $P = 1/F$ (reciprocal relationship)
2. Frequency is measured in Hertz (Hz = cycles per second)
3. Doubling the frequency ($c = 2$) halves the period and raises pitch by one octave
4. Human hearing spans approximately 20 Hz to 20,000 Hz

# Construction / Recognition

## To determine frequency from a function:
1. Identify the period $P$ of the function
2. Compute $F = 1/P$
3. For $\sin(\alpha t)$: period is $2\pi/\alpha$, frequency is $\alpha/(2\pi)$ Hz

# Context & Application

Concert pitch A4 is standardized at 440 Hz, meaning the sound wave completes 440 cycles per second (period $\approx 0.00227$ seconds). The piano ranges from about 27.5 Hz (A0) to 4186 Hz (C8).

# Examples

**Example 1** (p. 125): $\sin(t)$ has period $2\pi$, frequency $1/(2\pi) \approx 0.159$ Hz (far below audibility).

**Example 2** (p. 125): $\sin(880\pi t)$ has period $2\pi/(880\pi) = 1/440$ seconds, giving A4 at 440 Hz.

**Example 3**: Middle C $\approx$ 262 Hz, period $\approx 0.00382$ seconds.

# Relationships

## Builds Upon
- **Periodic Functions** -- Frequency is defined for periodic functions

## Enables
- **Fundamental Frequency** -- The lowest frequency component of a tone
- **Effect of Horizontal Stretching on Pitch** -- Horizontal compression multiplies frequency

## Related
- **Vibrations and Sound Waves** -- Frequency determines pitch of vibrations

# Common Errors

- **Error**: Confusing angular frequency with frequency in Hz
  **Correction**: For $\sin(\alpha t)$, the frequency is $\alpha/(2\pi)$ Hz, not $\alpha$ Hz; the factor of $2\pi$ accounts for one full cycle of sine

# Common Confusions

- **Confusion**: Thinking higher frequency means louder
  **Clarification**: Frequency determines pitch, not loudness; loudness is determined by amplitude

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Vibrations" section, pp. 123-125.

# Verification Notes

- Definition source: Direct quote from p. 123
- Confidence rationale: Explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: angular frequency confusion, sin(880*pi*t) example
