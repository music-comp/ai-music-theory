---
concept: Effect of Horizontal Stretching on Pitch
slug: effect-of-horizontal-stretching-on-pitch

category: harmonics-and-timbre
subcategory: periodic-functions
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Effect of Horizontal Stretching on Pitch"

extraction_confidence: high

aliases: []

prerequisites:
  - geometric-transformations-on-periodic-functions
  - frequency-and-period
extends:
  - geometric-transformations-on-periodic-functions
related:
  - formants
  - chipmunk-effect
  - timbre
contrasts_with: []

answers_questions:
  - "How do I tune a waveform to a specific pitch?"
  - "What happens to pitch when a recording is sped up or slowed down?"
---

# Quick Definition

Compressing a waveform horizontally by factor $c$ multiplies its frequency by $c$. To produce a tone of desired frequency $r$ Hz from a function of period $P$, use $y = f(rPt)$.

# Core Definition

"We want $r = cF = c/P$, which gives $c = rP$. Thus the function $y = f(rPt)$ represents a tone having frequency $r$ cycles per second, i.e., $r$ Hz" (Wright, Ch. 10, p. 125). The transformation $f(t) \to f(ct)$ compresses the time axis by factor $c$, making the period $P/c$ and the frequency $cF$.

# Prerequisites

- **Geometric Transformations on Periodic Functions** -- General theory of transformations
- **Frequency and Period** -- The reciprocal relationship $F = 1/P$

# Key Properties

1. $f(ct)$ has period $P/c$ and frequency $cF$
2. To achieve frequency $r$ from period $P$: set $c = rP$, giving $y = f(rPt)$
3. This is a linear relationship between compression factor and resulting frequency
4. Speeding up a recording is equivalent to horizontal compression

# Construction / Recognition

## To tune a waveform to frequency r Hz:
1. Determine the original period $P$
2. Compute $c = rP$ (the compression factor)
3. Use $y = f(rPt)$
4. Verify: period = $P/(rP) = 1/r$, frequency = $r$ Hz

# Context & Application

This technique shows how to tune any waveform to any desired pitch. It also explains the chipmunk effect: speeding up a recording multiplies all frequencies by the same factor, shifting formants and producing unnatural sound.

# Examples

**Example 1** (p. 125): $\sin t$ has period $2\pi$. For A4 (440 Hz): $y = \sin(440 \cdot 2\pi \cdot t) = \sin(880\pi t)$. The resulting tone is "a nondescript hum, very similar to the tone produced by a tuning fork."

**Example 2**: Doubling playback speed ($c = 2$): frequency doubles, pitch rises one octave.

**Example 3**: Halving playback speed ($c = 1/2$): frequency halves, pitch drops one octave.

# Relationships

## Builds Upon
- **Geometric Transformations on Periodic Functions** -- Horizontal compression as a special case

## Enables
- **Chipmunk Effect** -- Speeding up recordings shifts all frequencies including formants

## Related
- **Formants** -- Horizontal stretching shifts formants, distorting timbre
- **Timbre** -- Timbre is affected when formants shift

# Common Errors

- **Error**: Confusing $f(ct)$ with $f(t/c)$
  **Correction**: $f(ct)$ compresses (speeds up, raises pitch); $f(t/c)$ stretches (slows down, lowers pitch)

# Common Confusions

- **Confusion**: Thinking speeding up a recording only changes pitch
  **Clarification**: It also shifts all formants, which is why it sounds unnatural (chipmunk effect) rather than like a higher-pitched version of the same instrument

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Effect of Horizontal Stretching on Pitch" section, pp. 124-125.

# Verification Notes

- Definition source: Direct quote from p. 125
- Confidence rationale: Explicit formula with worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: sin(880*pi*t) example, chipmunk effect connection
