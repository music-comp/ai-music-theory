---
# === CORE IDENTIFICATION ===
concept: Minor Triad
slug: minor-triad

# === CLASSIFICATION ===
category: chord-theory
subcategory: triads
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Minor Chord"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - minor chord

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-types-and-interval-sequences
extends: []
related:
  - triads
  - minor-seventh-chord
contrasts_with:
  - major-triad
  - diminished-triad

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes a major triad from a minor triad mathematically?"
  - "How do I construct a minor triad using interval sequences?"
---

# Quick Definition

The minor triad is a three-note chord defined by the interval sequence (3, 4, 5) in semitones, consisting of a root, a minor third above the root, and a perfect fifth above the root.

# Core Definition

The minor triad is defined by the sequence of modular intervals (3, 4, 5) in Z_12:

root --(3)--> third --(4)--> fifth --(5)--> (root)

Like the major triad, this sequence has no non-trivial cyclic symmetries -- its three cyclic permutations (3,4,5), (4,5,3), and (5,3,4) are all distinct -- so the root, third, and fifth are uniquely determined from the pitch class content alone (Wright, pp. 46-47).

# Prerequisites

- **Chord Types and Interval Sequences** -- The minor triad is defined by a specific interval sequence in Z_12

# Key Properties

1. Contains exactly three distinct note classes
2. Interval sequence: (3, 4, 5) in semitones, summing to 12
3. Root to third: minor third (3 semitones)
4. Root to fifth: perfect fifth (7 semitones = 3 + 4)
5. No non-trivial cyclic symmetries, so root is uniquely identifiable
6. Labeled with suffix "m" (e.g., "Am" denotes A minor triad)

# Construction / Recognition

## To Construct a Minor Triad

1. Choose a root note
2. Go up 3 semitones (minor third) to find the third
3. Go up 4 more semitones (major third) from the third to find the fifth
4. Verify: root to fifth spans 7 semitones (perfect fifth)

## To Recognize a Minor Triad

1. Reduce all notes to note classes (modulo octave)
2. Remove duplicates
3. Verify exactly three distinct note classes
4. Compute the cyclic interval sequence
5. Check that it matches (3, 4, 5)

# Context & Application

The minor triad differs from the major triad by having a minor third (3 semitones) rather than a major third (4 semitones) between root and third, while maintaining the same perfect fifth (7 semitones) between root and fifth. It conveys a darker or more somber quality compared to the major triad. In the major mode, diatonic minor triads occur on scale degrees II, III, and VI.

# Examples

**Example 1** (p. 47): Several minor chords are shown in musical notation with the interval sequence (3, 4, 5).

**Example 2**: A minor triad: A-C-E (root A, third C, fifth E) with intervals (3, 4, 5).

**Example 3**: D minor triad: D-F-A (root D, third F, fifth A). Labeled "Dm."

# Relationships

## Builds Upon

- **Chord Types and Interval Sequences** -- The minor triad is a specific interval sequence

## Enables

- **Minor Seventh Chord** -- The minor seventh chord (3, 4, 3, 2) contains the minor triad with same root, third, and fifth

## Related

- **Triads** -- The minor triad is one of four standard triad types

## Contrasts With

- **Major Triad** -- Interval sequence (4, 3, 5); the first two intervals are swapped compared to the minor triad
- **Diminished Triad** -- Interval sequence (3, 3, 6); shares the minor third from root but has a diminished fifth

# Common Errors

- **Error**: Confusing the interval sequence (3, 4, 5) with (3, 3, 6)
  **Correction**: (3, 4, 5) is the minor triad with a perfect fifth; (3, 3, 6) is the diminished triad with a diminished fifth (tritone)

# Common Confusions

- **Confusion**: Using "minor chord" when "minor triad" is needed for precision
  **Clarification**: "Minor triad" distinguishes the three-note chord from the minor seventh chord (3, 4, 3, 2), which also contains the word "minor"
- **Confusion**: Thinking major and minor triads differ in their fifth
  **Clarification**: Both have a perfect fifth (7 semitones from root); only the third differs (4 vs. 3 semitones from root)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 46-47.

# Verification Notes

- Definition source: Direct from pp. 46-47, explicit definition with interval sequence diagram
- Confidence rationale: High -- explicitly defined with musical notation examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: observation about major/minor sharing perfect fifth, note about terminology precision
