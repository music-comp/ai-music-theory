---
# === CORE IDENTIFICATION ===
concept: Geometric Transformations on Periodic Functions
slug: geometric-transformations-on-periodic-functions

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: periodic-functions
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Effect of Shifting and Stretching on Periodicity"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - periodic-functions
  - frequency-and-period
extends:
  - periodic-functions
related:
  - effect-of-horizontal-stretching-on-pitch
  - timbre
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do shifts and stretches affect the period of a function?"
  - "Which transformations change pitch and which do not?"
---

# Quick Definition

Shifting and stretching periodic functions changes their musical properties in predictable ways: vertical changes affect loudness, horizontal compression changes pitch, and horizontal shifts (delays) preserve both pitch and timbre.

# Core Definition

If $y = f(x)$ is periodic with period $P$, then: vertical shift $y = f(x) + c$ has period $P$; horizontal shift $y = f(x - c)$ has period $P$; vertical stretch $y = cf(x)$ has period $P$; horizontal stretch $y = f(x/c)$ has period $cP$. "So the effect of stretching horizontally by a factor of $c$ is to divide the frequency of $f(x)$ by $c$" (Wright, Ch. 10, p. 120).

# Prerequisites

- **Periodic Functions** -- Understanding periodicity
- **Frequency and Period** -- Understanding the frequency-period relationship

# Key Properties

1. Vertical shift $f(x) + c$: period unchanged (DC offset, no audible effect)
2. Horizontal shift $f(x - c)$: period unchanged (delay, no effect on pitch or timbre)
3. Vertical stretch $cf(x)$: period unchanged (amplitude/loudness change)
4. Horizontal stretch $f(x/c)$: period becomes $cP$, frequency divided by $c$
5. Only horizontal scaling changes the period (and hence pitch)

# Construction / Recognition

## Musical interpretation of each transformation:
1. **Horizontal shift** = delay: does not change pitch or timbre
2. **Vertical stretch** = amplitude change: adjusts loudness
3. **Vertical shift** = DC offset: no audible effect
4. **Horizontal compression by $c$** = pitch multiplication by $c$

# Context & Application

These transformations explain how to derive tones of any desired frequency from a prototype waveform, and why speeding up or slowing down recordings changes pitch (horizontal compression).

# Examples

**Example 1** (p. 125): To produce A4 (440 Hz) from $\sin t$ (period $2\pi$): compress horizontally by factor $880\pi$ to get $\sin(880\pi t)$.

**Example 2** (p. 124): Doubling playback speed ($c = 2$): frequency doubles, pitch rises one octave.

**Example 3** (p. 124): Halving playback speed ($c = 1/2$): frequency halves, pitch drops one octave.

# Relationships

## Builds Upon
- **Periodic Functions** -- Transformations act on periodic functions

## Enables
- **Effect of Horizontal Stretching on Pitch** -- The detailed pitch-change formula

## Related
- **Timbre** -- Vertical stretching affects loudness but not timbre

# Common Errors

- **Error**: Confusing horizontal stretch with horizontal compression
  **Correction**: Replacing $x$ by $cx$ (for $c > 1$) compresses horizontally, making period shorter and frequency higher. Replacing $x$ by $x/c$ stretches, making period longer and frequency lower.

# Common Confusions

- **Confusion**: Thinking vertical stretching changes the timbre
  **Clarification**: Vertical stretching changes amplitude (loudness) with "very little effect, if any, on the timbre of the tone" (Wright, p. 124)

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Effect of Shifting and Stretching on Periodicity" section, pp. 120-121.

# Verification Notes

- Definition source: Direct from pp. 120-121
- Confidence rationale: Explicit statements about each transformation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical interpretations, compression vs. stretch clarification
