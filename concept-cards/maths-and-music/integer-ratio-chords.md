---
concept: Integer Ratio Chords
slug: integer-ratio-chords

category: pitch-and-intervals
subcategory: integer-ratios
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: "Exercises"

extraction_confidence: medium

aliases: []

prerequisites:
  - integral-intervals
  - keyboard-approximation-of-integer-ratios
extends: []
related:
  - overtone-series
  - consonance-and-dissonance-from-integer-ratios
  - reinforced-overtone
contrasts_with: []

answers_questions:
  - "How can chords be described using integer ratios?"
  - "What familiar chords emerge from sequences of integer ratios?"
---

# Quick Definition

Chords can be identified by sequences of integer ratios (e.g., 2:3:4:5), specifying the frequency relationships between their notes. Sets of adjacent harmonics in the overtone series approximate familiar chord types, revealing the mathematical basis of harmony.

# Core Definition

A chord expressed as integer ratios $n_1:n_2:n_3:\ldots$ specifies the frequency relationships between its notes. The keyboard approximations of consecutive integers in the overtone series produce recognizable chord structures. The chapter exercises ask students to identify chords from integer ratio sequences (Wright, Ch. 9, Exercises 4-5, pp. 116-117).

# Prerequisites

- **Integral Intervals** -- Understanding integer ratio intervals
- **Keyboard Approximation of Integer Ratios** -- How ratios map to keyboard notes

# Key Properties

1. Each ratio pair gives an interval between adjacent chord tones
2. The ratio 4:5:6 gives a just-intoned major triad
3. The ratio 3:4:5 gives a just major chord in first inversion
4. Integer ratio chords represent just intonation, not equal temperament
5. Higher harmonics introduce increasingly exotic intervals

# Construction / Recognition

## To identify a chord from integer ratios:
1. Compute the interval between each pair of adjacent integers
2. Convert to cents: interval from $n_1$ to $n_2$ is $1200 \log_2(n_2/n_1)$
3. Find the nearest keyboard intervals
4. Identify the chord type from the interval sequence

# Context & Application

Integer ratio chords connect the harmonic series to chord theory. The exercises in Chapter 9 bridge abstract number theory and concrete harmonic structures, showing that familiar chords arise naturally from integer relationships.

# Examples

**Example 1** (Exercise 4, p. 116): 2:3:4:5 -- starting from a bottom note, this approximates a chord with a fifth, octave, and major third above.

**Example 2** (Exercise 4): 5:6:7:8 -- involves the problematic ratio 7, producing an unusual chord with a "natural seventh."

**Example 3** (Exercise 4): 10:12:15:18 -- the intervals can be analyzed by reducing ratios: 10:12 = 6:5 (minor third), 12:15 = 5:4 (major third), 15:18 = 6:5 (minor third).

**Example 4** (p. 115): The first 13 harmonics from $F_2$ produce a sequence containing major triads, dominant seventh-like structures, and increasingly exotic intervals.

# Relationships

## Builds Upon
- **Integral Intervals** -- Chords built from integer ratios
- **Keyboard Approximation of Integer Ratios** -- How the ratios map to keyboard

## Related
- **Overtone Series** -- Consecutive harmonics form integer ratio chords
- **Consonance and Dissonance from Integer Ratios** -- Chord quality depends on the integers involved
- **Reinforced Overtone** -- Common overtones in chord tones affect sonority

# Common Errors

- **Error**: Assuming integer ratio chords are the same as tempered chords
  **Correction**: The chord 4:5:6 is a pure major triad with a just major third (386 cents), differing from the tempered major third (400 cents) by 14 cents

# Common Confusions

- **Confusion**: Thinking each integer in the ratio must be a harmonic of the lowest note
  **Clarification**: The ratios specify frequency proportions between all chord tones, not necessarily starting from a harmonic series root

# Source Reference

Chapter 9: "The Integers as Intervals," Exercises 4-5, pp. 116-117.

# Verification Notes

- Definition source: Synthesized from exercises and the overtone series discussion
- Confidence rationale: Medium -- the concept is explored through exercises rather than a formal definition
- Uncertainties: Specific chord identifications are left as exercises
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: 10:12:15:18 ratio analysis
