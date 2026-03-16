---
concept: Pitch and Frequency
slug: pitch-and-frequency

category: pitch-and-intervals
subcategory: frequency
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Pitch"

extraction_confidence: high

aliases:
  - "pitch"
  - "frequency"
  - "hertz"

prerequisites:
  - sets-and-number-systems
extends: []
related:
  - note-notation-and-the-staff
  - keyboard-layout
  - musical-intervals
  - octave-equivalence
contrasts_with: []

answers_questions:
  - "What is a pitch and how does it relate to frequency?"
  - "What is the mathematical model for the set of all pitches?"
  - "What is the standard tuning reference pitch?"
---

# Quick Definition

Pitch is the perceived quality of a musical tone determined by the frequency of its sound wave vibration, measured in hertz (cycles per second).

# Core Definition

A musical tone results from a regular vibration transmitted through air as a sound wave. The pitch of the tone is the frequency of the vibration, measured in hertz (Hz), named after Heinrich Hertz (1857-1894). The set of all pitches is placed in one-to-one correspondence with $\mathbb{R}^+$, where a positive real number $x$ corresponds to frequency $x$ Hz (Wright, pp. 17-18).

# Prerequisites

- **Sets and Number Systems** — Pitches are identified with $\mathbb{R}^+$

# Key Properties

1. Pitch is identified with frequency, a continuous quantity in $\mathbb{R}^+$
2. Standard tuning: A above middle C ($A_4$) = 440 Hz
3. Range of human audibility: approximately 20 Hz to 20,000 Hz
4. The mathematical model allows any $x \in \mathbb{R}^+$, not just discrete keyboard pitches
5. Vertical position on a musical staff indicates pitch

# Construction / Recognition

## To determine pitch:

1. Identify the vibrating source
2. Measure the frequency of vibration in cycles per second (Hz)
3. The pitch corresponds to this frequency value in $\mathbb{R}^+$

# Context & Application

The identification of pitches with $\mathbb{R}^+$ means pitch is a continuous quantity, even though musical practice uses discrete pitches (keyboard notes). This identification enables the use of real analysis, functions, and transformations on the set of pitches. In equal temperament, the frequency doubles with each octave, so adjacent semitones are related by the factor $2^{1/12}$.

# Examples

- $A_4 = 440$ Hz (standard tuning reference pitch) (p. 17)
- Human audible range: approximately 20 Hz to 20,000 Hz (p. 17)
- Middle C ($C_4$) is approximately 261.63 Hz in equal temperament
- The pitch continuum $\mathbb{R}^+$ includes all frequencies, not just those of keyboard notes

# Relationships

## Builds Upon
- **Sets and Number Systems** — Pitches correspond to $\mathbb{R}^+$

## Enables
- **Musical Intervals** — Intervals measure the distance between pitches
- **Octave Equivalence** — Defined in terms of frequency ratios
- **Note Notation and the Staff** — Staff notation represents specific pitches

## Related
- **Keyboard Layout** — Keyboard notes sample the continuous pitch space

# Common Errors

- **Error**: Conflating pitch (frequency) with volume (amplitude)
  **Correction**: Hertz measures cycles per second (pitch), not wave amplitude (loudness)

# Common Confusions

- **Confusion**: Thinking pitch and note are the same thing
  **Clarification**: A note is a notational symbol on a staff that specifies a pitch; pitches exist that are not represented by standard notes (between adjacent keyboard keys)
- **Confusion**: Believing the set of pitches is discrete
  **Clarification**: There are pitches between adjacent keyboard notes; the set of pitches is $\mathbb{R}^+$, not a discrete set

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Pitch" section, pp. 17-18 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 17-18
- Confidence rationale: High — explicit definition with named standard (440 Hz)
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch vs. note distinction, $\mathbb{R}^+$ emphasis, equal temperament ratio
