---
concept: Chord Types and Interval Sequences
slug: chord-types-and-interval-sequences

category: chord-theory
subcategory: triads
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: null

extraction_confidence: high

aliases:
  - interval sequence
  - chord interval pattern

prerequisites:
  - chords-as-note-class-collections
extends: []
related:
  - major-triad
  - minor-triad
  - diminished-triad
  - augmented-triad
  - seventh-chord
  - cyclic-permutations-and-root-identification
contrasts_with: []

answers_questions:
  - "How is a chord type defined mathematically?"
  - "What interval sequences define the standard chord types?"
  - "How do I construct a major triad using interval sequences?"
---

# Quick Definition

A chord's type is determined by its sequence of modular intervals (measured in semitones as elements of Z_12) between successive note classes. Each standard chord type has a unique defining interval sequence that sums to 12.

# Core Definition

A chord type is defined by the ordered cyclic sequence of intervals (in semitones, modulo 12) between successive note classes. The standard chord types and their defining sequences are:

**Triads:** major (4, 3, 5), minor (3, 4, 5), diminished (3, 3, 6), augmented (4, 4, 4).

**Four-note chords:** seventh (4, 3, 3, 2), minor seventh (3, 4, 3, 2), major seventh (4, 3, 4, 1), diminished seventh (3, 3, 3, 3), half-diminished seventh (3, 3, 4, 2).

Each sequence sums to 12, the number of semitones in an octave (Wright, pp. 45-49).

# Prerequisites

- **Chords as Note Class Collections** -- Must understand that chords are defined by note classes and modular intervals to grasp how interval sequences classify chord types

# Key Properties

1. Each interval in the sequence measures the distance (in semitones) between successive note classes
2. The sum of all intervals in a sequence equals 12 (completing the octave in Z_12)
3. The sequence is cyclic: the last interval returns to the root
4. Whether the sequence has non-trivial cyclic symmetries determines whether the chord has a discernible root
5. Nine standard chord types cover the most musically significant partitions of 12 into 3 or 4 ordered parts

# Construction / Recognition

## To Determine a Chord Type

1. Identify the note classes in the chord (reduce to Z_12)
2. Order the note classes by ascending pitch within one octave
3. Calculate the semitone intervals between each successive pair
4. Include the interval from the last note class back to the first (completing the cycle)
5. Match the resulting sequence to the known chord type sequences

# Context & Application

Chord types form the vocabulary of harmony. The interval sequence tells a musician exactly what intervals to stack from the root. For example, a seventh chord (4, 3, 3, 2) means: start at the root, go up a major third to the third, up a minor third to the fifth, up another minor third to the seventh, and the remaining major second returns to the root. The mathematical formalism using Z_12 enables precise classification and comparison of all chord types (Wright, pp. 45-49).

# Examples

**Example 1** (pp. 45-46): The major triad has interval sequence (4, 3, 5): root to third is a major third (4 semitones), third to fifth is a minor third (3), fifth back to root is a perfect fourth (5).

**Example 2** (p. 47): The seventh chord (4, 3, 3, 2) contains the major triad (4, 3, 5) with the interval 5 split as 3 + 2 by the insertion of the seventh.

**Example 3** (pp. 47-49): The augmented triad (4, 4, 4) and diminished seventh (3, 3, 3, 3) have uniform interval sequences, giving them full cyclic symmetry and no discernible root.

# Relationships

## Builds Upon

- **Chords as Note Class Collections** -- Chord types classify the abstract note class structures

## Enables

- **Major Triad** -- Defined by the specific interval sequence (4, 3, 5)
- **Minor Triad** -- Defined by (3, 4, 5)
- **Seventh Chord** -- Defined by (4, 3, 3, 2)
- **Cyclic Permutations and Root Identification** -- Interval sequence symmetry determines root identifiability

## Related

- **Chord Labeling** -- Labels encode both root and chord type
- **Diminished Triad** -- Defined by (3, 3, 6)
- **Augmented Triad** -- Defined by (4, 4, 4)

# Common Errors

- **Error**: Measuring all intervals from the root rather than between successive note classes
  **Correction**: The intervals in the sequence are between consecutive note classes in cyclic order, not all measured from the root

# Common Confusions

- **Confusion**: Believing two chord types with some shared interval values are the same
  **Clarification**: The ordering of intervals matters; (4, 3, 5) and (3, 4, 5) define different chord types (major vs. minor)
- **Confusion**: Thinking the sequence can sum to something other than 12
  **Clarification**: The sum must always equal 12, since the intervals partition the octave in Z_12

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 45-49. All nine standard chord types are defined with their interval sequences.

# Verification Notes

- Definition source: Synthesized from the individual chord definitions across pp. 45-49
- Confidence rationale: High -- each chord type is explicitly defined with its interval sequence
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete list of all nine chord type sequences, observation that seventh chord splits the major triad's interval 5 as 3+2
