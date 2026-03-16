---
concept: Detuning
slug: detuning

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Detuning"

extraction_confidence: high

aliases:
  - synthesizer detuning
  - cent offset tuning

prerequisites:
  - n-chromatic-scale
  - non-standard-chromatic-scales
extends: []
related:
  - n-tone-row-chart
contrasts_with: []

answers_questions:
  - "How can a synthesizer be used to play in a non-standard chromatic scale?"
  - "How are detuning offsets calculated for an n-chromatic scale?"
---

# Quick Definition

The practice of adjusting individual notes on a synthesizer by a specified number of cents from their standard 12-chromatic tuning, enabling performance in non-standard chromatic scales.

# Core Definition

Detuning modifies the pitch of individual keyboard notes by a specified offset in cents from their standard equal-temperament values. To play an n-chromatic scale on a keyboard, each key is detuned so that adjacent used keys are separated by exactly 1200/n cents. The detuning offset for each key is calculated cumulatively from a reference pitch (Wright, p. 74).

# Prerequisites

- **N-chromatic scale** — Understanding the target tuning is needed to compute offsets
- **Non-standard chromatic scales** — Detuning is the practical method for realizing these scales

# Key Properties

1. Detuning is measured in cents relative to standard equal temperament
2. Offsets accumulate: each successive key's offset depends on all previous intervals
3. Only a subset of keyboard keys is used when n < 12
4. When n divides 12 (n = 1, 2, 3, 4, 6), no detuning is needed
5. The required offset for key k equals k * (1200/n) minus the standard tuning distance from the reference

# Construction / Recognition

## To Detune for an n-Chromatic Scale
1. Choose a starting note and compute the n-chromatic unit: 1200/n cents
2. For each subsequent key to be used, compute the cumulative ideal interval from the starting note
3. Subtract the standard tuning interval to get the required detuning offset
4. Apply the offset on the synthesizer
5. Use only the detuned keys for performance

# Context & Application

Many synthesizers allow individual note detuning in cents, making it possible to explore the sound of non-standard chromatic scales. This is the practical gateway to microtonal music, transforming a standard keyboard into an instrument capable of playing in any equal temperament with n <= 12 (using a subset of keys).

# Examples

**Example 1** (p. 74): 5-chromatic scale from G (unit = 240 cents):
- G: reference (0 cents offset)
- A: default 200 cents, need 240, detune +40 cents
- B: default 400 cents, need 480, detune +80 cents
- C: default 500 cents, need 720, detune +220 cents
- D: default 700 cents, need 960, detune +260 cents

**Example 2** (Ch. 7, p. 93): 7-chromatic scale from C (unit ~ 171.43 cents):
- D: detune -28.57 cents
- E: detune -57.14 cents
- F: detune +114.29 cents
- G: detune +85.71 cents
- A: detune +57.14 cents
- B: detune +28.57 cents

**Example 3** (p. 74): n = 4 (unit = 300 cents): no detuning needed; use G, Bb, Db, E.

# Relationships

## Builds Upon
- **N-chromatic scale** — Detuning realizes n-chromatic scales on keyboard instruments
- **Non-standard chromatic scales** — Detuning is the primary method for accessing these scales

## Enables
- **N-tone row chart** — Composing with non-standard row charts requires detuned instruments

# Common Errors

- **Error**: Detuning each key by the same fixed amount
  **Correction**: Offsets accumulate differently for each key because standard keyboard intervals vary (semitones vs. whole steps)

# Common Confusions

- **Confusion**: Thinking detuning is relative to just intonation
  **Clarification**: Detuning is measured relative to standard 12-tone equal temperament

- **Confusion**: Believing all keyboard keys must be retuned
  **Clarification**: When n < 12, only a subset of keys is used and retuned; unused keys retain their standard pitch

# Source Reference

Chapter 6: "Chromatic Scales," p. 74 (detuning section). See also Chapter 7, p. 93, for the 7-tone detuning example.

# Verification Notes

- Definition source: Direct from Wright, p. 74
- Confidence rationale: High — explicit worked examples with numerical offsets
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: both detuning examples (n=5 and n=7), n=4 no-detuning case
