---
concept: Musical Intervals
slug: musical-intervals

category: pitch-and-intervals
subcategory: frequency
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Musical Intervals"

extraction_confidence: high

aliases:
  - "named intervals"
  - "keyboard intervals"
  - "tempered intervals"

prerequisites:
  - pitch-and-frequency
  - keyboard-layout
extends: []
related:
  - octave-equivalence
  - diatonic-and-chromatic-scales
  - note-classes
contrasts_with:
  - interval-notation

answers_questions:
  - "What is an interval in mathematical terms?"
  - "What are the standard named intervals and their semitone values?"
  - "What is the difference between positive and negative intervals?"
---

# Quick Definition

The "distance" between two pitches, measured in semitones in equal temperament, with standard names for each interval size from minor second through octave and beyond.

# Core Definition

The interval between two notes is the "distance" between their associated pitches. In equal temperament, the interval between any two adjacent keys (white or black) is the same and is called a *semitone*. An octave equals 12 semitones. Intervals are positive (upward) or negative (downward). Intervals between notes on the abstract infinite keyboard are called *keyboard intervals* or *tempered intervals*, to distinguish them from just or Pythagorean intervals introduced later (Wright, pp. 19-20).

# Prerequisites

- **Pitch and Frequency** — Intervals measure the distance between pitches
- **Keyboard Layout** — Adjacent keys define the semitone unit

# Key Properties

1. The semitone is the fundamental unit — the interval between any two adjacent keys
2. An octave equals 12 semitones
3. Intervals can be positive (upward) or negative (downward)
4. The term "interval" in music differs from mathematical interval notation $[a,b]$
5. Later chapters refine the definition using frequency ratios

# Construction / Recognition

## Standard interval names and semitone values:

| Interval | Semitones |
|----------|-----------|
| Minor second (half step) | 1 |
| Major second (whole step/tone) | 2 |
| Minor third | 3 |
| Major third | 4 |
| Perfect fourth | 5 |
| Tritone | 6 |
| Perfect fifth | 7 |
| Minor sixth / Augmented fifth | 8 |
| Major sixth | 9 |
| Minor seventh / Augmented sixth | 10 |
| Major seventh | 11 |
| Octave | 12 |
| Minor ninth | 13 |
| Ninth | 14 |

# Context & Application

In equal temperament, intervals are integer multiples of the semitone, making them elements of $\mathbb{Z}$ (when signed). The term "keyboard intervals" or "tempered intervals" refers specifically to equal-temperament intervals. Wright notes that small modifications (just and Pythagorean intervals) will be discussed later, so the keyboard intervals are a first approximation.

# Examples

- The interval from $C_4$ to $E_3$ is "down a minor sixth" or "negative a minor sixth" (8 semitones downward) (p. 20)
- A step (major second) equals 2 semitones; a half step (minor second) equals 1 semitone (p. 19)
- The Pythagorean major third (discussed later) is slightly larger than the keyboard major third of 4 semitones (p. 20)

# Relationships

## Builds Upon
- **Pitch and Frequency** — Intervals measure differences between pitches
- **Keyboard Layout** — Adjacent keys define the semitone

## Enables
- **Octave Equivalence** — An octave is defined as 12 semitones
- **Diatonic and Chromatic Scales** — Scales are defined by their interval patterns

## Contrasts With
- **Interval Notation** — Mathematical intervals ($[a,b] \subset \mathbb{R}$) vs. musical intervals (semitone distances); Wright explicitly flags this distinction (p. 19)

# Common Errors

- **Error**: Confusing musical intervals with mathematical intervals $[a,b]$
  **Correction**: Wright explicitly warns these are different uses of the word "interval"

# Common Confusions

- **Confusion**: Thinking "keyboard intervals" are the only kind of interval
  **Clarification**: Just and Pythagorean intervals (introduced later) differ slightly from keyboard (tempered) intervals
- **Confusion**: Assuming all intervals with the same semitone count have the same name
  **Clarification**: Some have multiple names: minor sixth and augmented fifth both equal 8 semitones

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Musical Intervals" section, pp. 19-20 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 19-20
- Confidence rationale: High — explicit definition with complete interval table
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete interval table, signed interval convention, distinction from mathematical intervals
