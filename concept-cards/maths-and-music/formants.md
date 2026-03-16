---
concept: Formants
slug: formants

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

aliases:
  - "resonant frequency band"

prerequisites:
  - harmonics-and-overtones
  - timbre-as-harmonic-amplitudes
extends: []
related:
  - vowel-formants
  - chipmunk-effect
  - effect-of-horizontal-stretching-on-pitch
contrasts_with: []

answers_questions:
  - "What are formants?"
  - "How do formants differ from harmonics?"
  - "Why does speeding up a recording distort the sound?"
---

# Quick Definition

Formants are fixed frequency ranges in which harmonics are amplified by an instrument's or voice's resonating chambers. Unlike harmonics (which move with pitch), formants stay in the same frequency range regardless of the note being played or sung.

# Core Definition

"These frequency ranges, called *formants*, depend only on the musical instrument being played or the human vowel sound being sung; they remain unchanged as the pitch $F$ varies. Thus each weight $d_k$ will change from note to note, depending on whether the $k^{\text{th}}$ harmonic lies within one of these formants" (Wright, Ch. 10, p. 132).

# Prerequisites

- **Harmonics and Overtones** -- Formants amplify specific harmonics
- **Timbre as Harmonic Amplitudes** -- Formants shape the amplitude profile

# Key Properties

1. Formants are fixed frequency bands, independent of fundamental pitch $F$
2. Harmonics falling within a formant receive larger amplitudes $d_k$
3. Different harmonics $k$ fall within formants for different pitches
4. Musical sounds typically have two or three formants
5. Formants are created by resonating chambers (instrument body, mouth, throat)
6. A formant has no effect if the fundamental lies above it

# Construction / Recognition

## How formants work:
1. For fundamental $F$, the $k$-th harmonic has frequency $kF$
2. If $kF$ falls within a formant band, $d_k$ is enhanced
3. As $F$ changes, different values of $k$ enter or leave the formant band
4. This is why timbre varies slightly with pitch

# Context & Application

Formants explain why instruments and vowels maintain their characteristic identity across pitches while sounding slightly different at different pitches. They also explain the chipmunk effect: speeding up a recording shifts formants, destroying the natural timbre.

# Examples

**Example 1** (p. 134): Clarinet formants: 1500-1700 Hz.

**Example 2** (p. 134): Trumpet formants: 1200-1400 Hz, and narrowly around 2500 Hz.

**Example 3** (p. 133): Human "oo" vowel: formants near 310, 870, and 2250 Hz.

**Example 4** (p. 134): If a soprano sings A5 (880 Hz) on "oo," the lowest formant (310 Hz) has no harmonics to amplify.

# Relationships

## Builds Upon
- **Harmonics and Overtones** -- Formants act on harmonics
- **Timbre as Harmonic Amplitudes** -- Formants shape the amplitude sequence

## Enables
- **Vowel Formants** -- Specific formant patterns for vowels
- **Chipmunk Effect** -- Explained by formant shifting

## Related
- **Effect of Horizontal Stretching on Pitch** -- Time-scaling shifts formants unnaturally

# Common Errors

- **Error**: Assuming formants move with pitch
  **Correction**: Formants are fixed frequency bands determined by physical resonating chambers; they stay put as pitch changes

# Common Confusions

- **Confusion**: Confusing formants with harmonics
  **Clarification**: Harmonics are integer multiples of $F$ and move with pitch. Formants are fixed frequency bands that stay put. A harmonic may or may not fall within a formant depending on the note being played.

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Formants" section, pp. 132-134.

# Verification Notes

- Definition source: Direct quote from p. 132
- Confidence rationale: Explicit definition with multiple examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: clarinet/trumpet formant values, soprano A5 example
