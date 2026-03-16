---
# === CORE IDENTIFICATION ===
concept: Chord Progressions and Circle of Fifths
slug: chord-progressions-and-circle-of-fifths

# === CLASSIFICATION ===
category: chord-theory
subcategory: null
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Progressions"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - circle of fifths progression
  - circle of fourths progression

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-labeling
extends: []
related:
  - functional-harmony
  - seventh-chord
  - half-diminished-seventh-chord
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a chord progression?"
  - "What is the circle of fifths?"
  - "What is the classical circle-of-fifths progression pattern?"
---

# Quick Definition

A chord progression is the movement from one chord to the next in time. The circle of fifths arranges all twelve chromatic pitch classes by intervals of a fifth, and counter-clockwise movement (up a fourth) around it is a fundamental classical progression pattern.

# Core Definition

A progression is the procedure from one chord to the next. The circle of fifths arranges the twelve chromatic pitch classes so that proceeding clockwise moves up a fifth (or equivalently, down a fourth), and proceeding counter-clockwise moves up a fourth (or down a fifth). Each chromatic scale tone occupies one and only one position on the circle. The classical circle-of-fifths progression moves counter-clockwise: each chord's root is a fourth (5 semitones) above the preceding chord's root (Wright, pp. 53-54).

# Prerequisites

- **Chord Labeling** -- Progressions are described using labeled chords with Roman numerals or note class names

# Key Properties

1. Clockwise on the circle: up a fifth (7 semitones), equivalently down a fourth
2. Counter-clockwise: up a fourth (5 semitones), equivalently down a fifth
3. Each of the 12 chromatic pitch classes occupies exactly one position
4. The classical progression pattern moves counter-clockwise (roots up a fourth)
5. gcd(7, 12) = 1 ensures the cycle visits all 12 pitch classes
6. Musical character is created by the way chords are organized and juxtaposed in time

# Construction / Recognition

## To Construct a Circle-of-Fifths Progression

1. Choose a target tonic chord (I)
2. Work backwards: the chord before I should have its root a fifth above I (= V)
3. Continue backwards: the chord before V has its root a fifth above V (= II)
4. Each step counter-clockwise adds another chord to the sequence
5. Common pattern: VI7 -> IIm -> V7 -> I

# Context & Application

The circle-of-fifths progression is one of the most common harmonic patterns in Western music. It creates a strong sense of harmonic momentum toward the tonic. The half-diminished seventh chord also often resolves around the circle of fifths. A certain amount of musical satisfaction is obtained merely from a pleasing or catchy sequence of progressions, giving rise to "musical cliches that are quite familiar to most listeners" (Wright, pp. 53-54).

# Examples

**Example 1** (p. 53): The circle of fifths is depicted as a clock diagram where each position is a chromatic pitch class.

**Example 2** (p. 54): Classical circle-of-fifths progression in the major mode: VI7 -> IIm -> V7 -> I.

**Example 3** (p. 54): In the key of F major, the progression I -> V7 -> I (F -> C7 -> F) is a simple tonic-dominant-tonic pattern.

# Relationships

## Builds Upon

- **Chord Labeling** -- Progressions are expressed using chord labels

## Enables

- **Functional Harmony** -- The circle of fifths is the structural backbone of functional harmony

## Related

- **Seventh Chord** -- V7 is the primary chord in circle-of-fifths resolutions
- **Half-Diminished Seventh Chord** -- Also resolves around the circle of fifths

# Common Errors

- **Error**: Moving clockwise on the circle and calling it "the classical circle-of-fifths progression"
  **Correction**: The classical progression moves counter-clockwise (roots go up a fourth)

# Common Confusions

- **Confusion**: Thinking the circle of fifths is about specific pitches rather than pitch classes
  **Clarification**: The circle operates modulo octave; it organizes pitch classes, not specific octave-positioned pitches
- **Confusion**: Believing all progressions must follow the circle of fifths
  **Clarification**: The circle of fifths describes one common pattern; many other progression types exist

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 53-55. Includes circle of fifths diagram and several progression examples.

# Verification Notes

- Definition source: Direct from pp. 53-54
- Confidence rationale: High -- explicit definition with diagram and examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: gcd(7,12)=1 observation, counter-clockwise direction clarification, Z_12 mathematical context
