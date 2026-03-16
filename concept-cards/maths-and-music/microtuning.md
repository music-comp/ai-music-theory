---
concept: Microtuning
slug: microtuning

category: pitch-and-intervals
subcategory: acoustics
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Microtuning and Cents"

extraction_confidence: high

aliases:
  - microtonal tuning
  - alternative tuning

prerequisites:
  - cents
extends: []
related:
  - n-chromatic-units
  - semitone-ratio
  - interval-as-frequency-ratio
contrasts_with: []

answers_questions:
  - "What is microtuning?"
  - "Why is microtuning necessary?"
  - "What unit is used for microtuning specifications?"
---

# Quick Definition

Microtuning refers to tuning systems that alter the frequencies of notes in the equally tempered chromatic scale or add new notes to it, using intervals that cannot be expressed as integer multiples of semitones.

# Core Definition

Microtuning encompasses any tuning system that departs from 12-tone equal temperament (12-TET). This includes: (1) adjusting standard chromatic pitches by amounts smaller than a semitone, (2) adding pitches between the standard chromatic notes, and (3) using alternative equal temperaments with n != 12 divisions of the octave. "Mathematical tuning involves intervals which cannot be realized as integer multiples of semitones." The cent provides a sufficiently fine unit for specifying microtuning adjustments, since 1 cent is imperceptible and even 10 cents is difficult to perceive (Wright, pp. 61-62).

# Prerequisites

- **Cents** -- The cent is the standard unit for specifying microtuning adjustments

# Key Properties

1. Involves intervals that are not integer multiples of the semitone
2. The cent provides sufficient resolution for any practical microtuning specification
3. 1 cent is imperceptible; even 10 cents is difficult to perceive
4. Microtuning can involve just intonation ratios, historical temperaments, or novel pitch systems
5. Motivated by the mathematical impossibility of making all intervals pure in 12-TET

# Construction / Recognition

## To Specify a Microtuning Adjustment

1. Determine the desired interval ratio
2. Convert to cents: x = 1200 * log_2(r)
3. Express the deviation from the nearest equal-tempered pitch in cents
4. Apply the adjustment to the instrument or synthesizer

# Context & Application

Microtuning has gained renewed interest with electronic instruments and software synthesizers that allow precise frequency control. The term refers to systems that go beyond the standard 12-tone chromatic scale. The motivation comes from mathematical tuning theory, which reveals that certain important intervals (like the just major third at ratio 5/4) cannot be exactly represented in 12-TET (Wright, pp. 61-62).

# Examples

**Example 1** (p. 61): The just major third (ratio 5/4 = 386.31 cents) is about 14 cents flatter than the equal-tempered major third (400 cents).

**Example 2**: 19-tone equal temperament divides the octave into 19 equal parts.

**Example 3** (p. 61): Even 10 cents is difficult to perceive, so the cent provides more than adequate resolution.

# Relationships

## Builds Upon

- **Cents** -- Cents provide the measurement precision needed for microtuning

## Related

- **N-Chromatic Units** -- Alternative equal temperaments use different values of n
- **Semitone Ratio** -- Microtuning addresses limitations of the 12-TET semitone
- **Interval as Frequency Ratio** -- Microtuning works directly with frequency ratios

# Common Errors

- **Error**: Assuming microtuning means the instrument is "out of tune"
  **Correction**: Microtuning is deliberate, mathematically precise tuning that may be more consonant for specific intervals than 12-TET

# Common Confusions

- **Confusion**: Thinking all microtuning involves equal temperaments
  **Clarification**: Some microtuning uses just intonation ratios or other non-equal systems
- **Confusion**: Believing 12-TET is the "correct" tuning and all others are deviations
  **Clarification**: 12-TET is a compromise; microtuning can achieve intervals that are mathematically "purer" for specific purposes

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 61-62.

# Verification Notes

- Definition source: Direct from pp. 61-62
- Confidence rationale: High -- explicitly defined with motivation
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: imperceptibility thresholds, just major third comparison, 12-TET limitation motivation
