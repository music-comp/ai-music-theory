---
concept: Chipmunk Effect
slug: chipmunk-effect

category: harmonics-and-timbre
subcategory: acoustics
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Formants"

extraction_confidence: high

aliases: []

prerequisites:
  - formants
  - effect-of-horizontal-stretching-on-pitch
extends: []
related:
  - geometric-transformations-on-periodic-functions
  - timbre-as-harmonic-amplitudes
contrasts_with: []

answers_questions:
  - "Why does speeding up a recording sound unnatural?"
  - "What is the chipmunk effect?"
---

# Quick Definition

The "chipmunk effect" is the unnatural sound produced when a recording is sped up, which raises all frequencies proportionally -- including formants -- destroying the natural timbre. Slowing down produces a similarly unnatural dark, muddy sound.

# Core Definition

"When a recorded tone is played at a different rate from which it was recorded, the sound wave is simply stretched or compressed over time, i.e., the frequency $F$ is changed, with all other parameters in (10.8) remaining unaltered. Thus the formants are not preserved, but rather shifted along with $F$" (Wright, Ch. 10, p. 134). "Speeding up recorded music produces the familiar 'chipmunk effect.' Music which is slowed down sounds dark and muddy. In either case the character of the music is changed in a rather comical way."

# Prerequisites

- **Formants** -- Understanding that formants are fixed frequency bands
- **Effect of Horizontal Stretching on Pitch** -- Time-scaling multiplies all frequencies

# Key Properties

1. Simple time-scaling $f(ct)$ multiplies ALL frequencies by $c$, including formants
2. The mathematical amplitudes $d_k$ remain unchanged, but the physical formant frequencies shift
3. The formant structure no longer matches the original source
4. Modern pitch-shifting can preserve formants -- "a great triumph in signal analysis technology"

# Construction / Recognition

## Why the chipmunk effect occurs:
1. Recording played at speed $c$: $g(t) \to g(ct)$
2. Fundamental changes: $F \to cF$
3. All harmonics change: $kF \to ckF$
4. Formant frequencies shift: center at $f_0 \to cf_0$
5. The shifted formants no longer match the natural resonance of the original instrument/voice

# Context & Application

The chipmunk effect demonstrates that timbre is not simply determined by the mathematical amplitudes $d_k$ in the Fourier series; it also depends on the formant structure. Modern studio technology can transpose pitch while preserving formants, which is sophisticated signal processing.

# Examples

**Example 1** (p. 134): Speeding up a male voice by factor 2: pitch rises an octave, formants shift up by an octave, producing cartoon-like sound.

**Example 2** (p. 134): Slowing down music by factor 1/2: pitch drops an octave, formants shift down, sound becomes muddy and unnatural.

**Example 3** (p. 134): Modern pitch-shifting: can change pitch while keeping formants intact, preserving natural character.

# Relationships

## Builds Upon
- **Formants** -- The fixed frequency bands that get shifted
- **Effect of Horizontal Stretching on Pitch** -- The mathematical transformation

## Related
- **Geometric Transformations on Periodic Functions** -- Time-scaling is a horizontal transformation
- **Timbre as Harmonic Amplitudes** -- Formant shifting changes the effective amplitudes

# Common Errors

- **Error**: Thinking speeding up a recording merely raises pitch
  **Correction**: It also raises all formant frequencies, which is why the sound becomes unnatural

# Common Confusions

- **Confusion**: Thinking the mathematical description $g(ct)$ fully explains what we hear
  **Clarification**: The Fourier amplitudes $d_k$ are preserved under time-scaling, but the formant structure (which depends on fixed physical resonances) is shifted, causing the unnatural sound

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 133-134.

# Verification Notes

- Definition source: Direct quotes from p. 134
- Confidence rationale: Explicit description with physical explanation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: modern pitch-shifting note, formant-shift mechanism
