---
# === CORE IDENTIFICATION ===
concept: Overtone Series
slug: overtone-series

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: fourier-analysis
tier: foundational

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
aliases:
  - "harmonic series (acoustics)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - fundamental-frequency
  - harmonics-and-overtones
extends:
  - harmonics-and-overtones
related:
  - keyboard-approximation-of-integer-ratios
  - integral-intervals
  - timbre-as-harmonic-amplitudes
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the overtone series?"
  - "What pitches make up the overtone series?"
---

# Quick Definition

For a given fundamental frequency $F$, the overtone series is the infinite sequence of pitches $F, 2F, 3F, 4F, 5F, \ldots$, formed by all positive integer multiples of the fundamental.

# Core Definition

"For a given fundamental frequency $F$, the infinite sequence of pitches $F, 2F, 3F, 4F, 5F, \ldots$ is called its *overtone series*" (Wright, Ch. 10, p. 127). Each member $kF$ is the $k$-th harmonic and forms the interval ratio $k:1$ with the fundamental.

# Prerequisites

- **Fundamental Frequency** -- The starting frequency $F$
- **Harmonics and Overtones** -- The individual components of the series

# Key Properties

1. The sequence is $\{kF : k = 1, 2, 3, \ldots\}$
2. Intervals between consecutive overtones narrow: ratio $(k+1)/k$ decreases
3. The overtone series contains all integer ratio intervals from the fundamental
4. The series connects to the integers-as-intervals analysis of Chapter 9

# Construction / Recognition

## The overtone series from a fundamental F:
1. $1F$ = fundamental (1st harmonic)
2. $2F$ = octave above (2nd harmonic)
3. $3F$ = octave + fifth (3rd harmonic, ~2 cents off tempered)
4. $4F$ = two octaves (4th harmonic)
5. $5F$ = two octaves + major third (5th harmonic, ~14 cents off)
6. Continue with $6F, 7F, \ldots$

# Context & Application

The overtone series is fundamental to Western harmony. The relative amplitudes $d_1, d_2, d_3, \ldots$ of the overtones determine timbre. The series also appears in the analysis of integer ratio chords and reinforced overtones in well-tuned chords.

# Examples

**Example 1** (p. 127): From $F_2$ as fundamental, the first 13 harmonics approximate: $F_2, F_3, C_4, F_4, A_4, C_5, (E_5^\flat), F_5, G_5, A_5, (\text{between } B_5^\flat \text{ and } B_5), C_6, (\text{between } C_6^\sharp \text{ and } D_6)$.

**Example 2**: The 7th harmonic ($7F$) is ~31 cents from the nearest keyboard note (poorly tempered).

**Example 3**: The intervals between consecutive harmonics narrow: $2F/F = 2$ (octave), $3F/2F = 3/2$ (fifth), $4F/3F = 4/3$ (fourth), $5F/4F = 5/4$ (major third).

# Relationships

## Builds Upon
- **Harmonics and Overtones** -- The overtone series lists all harmonics

## Enables
- **Timbre as Harmonic Amplitudes** -- The relative strengths of overtones determine timbre

## Related
- **Keyboard Approximation of Integer Ratios** -- How overtones map to the keyboard
- **Integral Intervals** -- Overtones are at integer ratios from the fundamental

# Common Errors

- **Error**: Confusing the harmonic series (acoustics) with the harmonic series (mathematics: $1 + 1/2 + 1/3 + \ldots$)
  **Correction**: The acoustic overtone series is the sequence of frequencies $F, 2F, 3F, \ldots$; the mathematical harmonic series is a divergent sum

# Common Confusions

- **Confusion**: Thinking all overtones are well-approximated by the keyboard
  **Clarification**: Only those involving small primes (2, 3, 5) are well-approximated; the 7th, 11th, and 13th harmonics are poorly represented

# Source Reference

Chapter 10: "Timbre and Periodic Functions," pp. 126-128. Also Chapter 9, pp. 110-116.

# Verification Notes

- Definition source: Direct quote from p. 127
- Confidence rationale: Explicit definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: F2 example, interval narrowing observation
