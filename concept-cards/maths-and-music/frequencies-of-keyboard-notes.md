---
# === CORE IDENTIFICATION ===
concept: Frequencies of Keyboard Notes
slug: frequencies-of-keyboard-notes

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: acoustics
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Frequencies of Keyboard Notes"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - keyboard frequency calculation
  - A440 tuning

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semitone-ratio
extends:
  - semitone-ratio
related:
  - interval-as-frequency-ratio
  - multiplicative-composition-of-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I calculate the frequency of any keyboard note?"
  - "What is the frequency of A4 (concert pitch)?"
  - "Why are chromatic frequencies not equally spaced?"
---

# Quick Definition

Given a reference pitch (A4 = 440 Hz) and the semitone ratio 2^(1/12), the frequency of any keyboard note can be calculated by multiplying 440 by the appropriate power of 2^(1/12).

# Core Definition

If a note N has frequency f and an interval has ratio r, the note lying the interval r from N has frequency rf. Given the standard tuning A4 = 440 Hz, any note that is k semitones above A4 has frequency:

f = 440 * 2^(k/12)

For notes below A4, k is negative. More generally, given any note's frequency, applying an interval of ratio r yields a new frequency of rf (Wright, pp. 60-61).

# Prerequisites

- **Semitone Ratio** -- Must know that each semitone has ratio 2^(1/12) to compute keyboard frequencies

# Key Properties

1. Reference pitch: A4 = 440 Hz (international standard, "concert pitch")
2. Note k semitones above A4: f = 440 * 2^(k/12)
3. The frequency mapping is an exponential function of the chromatic pitch number
4. Chromatic frequencies are not equally spaced on a linear axis
5. The formula works for fractional semitone values too
6. Higher notes have larger frequency gaps between adjacent semitones

# Construction / Recognition

## To Calculate a Keyboard Frequency

1. Determine the number of semitones k from A4 to the target note (positive if above, negative if below)
2. Compute f = 440 * 2^(k/12)
3. Alternatively: find any known frequency, then apply the semitone ratio to reach the target

# Context & Application

Standard tuning (A4 = 440 Hz, also called A440 or concert pitch) is the international standard. From this single reference, the entire keyboard is tuned. The computed frequencies are the basis for electronic instrument tuning, synthesizer design, and acoustic instrument manufacturing. The non-equidistance of chromatic frequencies on a linear axis reflects the exponential nature of the mapping (Wright, pp. 60-61).

# Examples

**Example 1** (p. 60): A3 = 220 Hz (one octave below A4: 440 * 2^(-1) = 220).

**Example 2** (pp. 60-61): C#4 lies a major third above A3. Since A3 = 220 Hz and the major third has ratio 2^(1/3): f = 220 * 2^(1/3) ~ 277.18 Hz.

**Example 3**: A5 = 880 Hz (one octave above A4: 440 * 2^1 = 880).

**Example 4** (Exercise 2): The chromatic pitches from C4 to C5, when plotted on a number line, are not equidistant.

# Relationships

## Builds Upon

- **Semitone Ratio** -- The frequency formula is a direct application of r = 2^(k/12)

## Related

- **Interval as Frequency Ratio** -- Each keyboard interval corresponds to a frequency ratio
- **Multiplicative Composition of Intervals** -- Computing frequencies involves multiplying a reference by ratios

# Common Errors

- **Error**: Computing keyboard frequencies by adding equal increments to a reference frequency
  **Correction**: Frequencies are computed by multiplying by powers of the semitone ratio, not by adding

# Common Confusions

- **Confusion**: Thinking A4 = 440 Hz is a physical law
  **Clarification**: It is a convention (adopted as an international standard); historically and in some modern practices, other reference frequencies are used
- **Confusion**: Expecting equal frequency spacing between adjacent notes
  **Clarification**: Equal temperament means equal ratio spacing; the frequency differences between adjacent semitones increase as pitch rises

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 60-61.

# Verification Notes

- Definition source: Direct from pp. 60-61
- Confidence rationale: High -- explicit formula and worked example
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: C#4 calculation example, non-equidistance observation, A440 convention note
