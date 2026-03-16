---
# === CORE IDENTIFICATION ===
concept: Augmented Triad
slug: augmented-triad

# === CLASSIFICATION ===
category: chord-theory
subcategory: triads
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Diminished and Augmented Chords"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - augmented chord
  - aug chord

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-types-and-interval-sequences
  - cyclic-permutations-and-root-identification
extends: []
related:
  - triads
  - diminished-seventh-chord
contrasts_with:
  - major-triad
  - diminished-triad

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an augmented triad?"
  - "Why does an augmented triad have no discernible root?"
  - "How many distinct augmented triads exist?"
---

# Quick Definition

The augmented triad is a three-note chord defined by the interval sequence (4, 4, 4) in semitones, consisting of three notes each separated by a major third. Its complete cyclic symmetry means it has no discernible root.

# Core Definition

The augmented triad is defined by the sequence of modular intervals (4, 4, 4) in Z_12. Every cyclic permutation of this sequence yields the same sequence (4, 4, 4). Consequently, the augmented triad has complete cyclic symmetry and no discernible root -- any of its three notes could equally serve as root (Wright, p. 47).

# Prerequisites

- **Chord Types and Interval Sequences** -- Must understand interval sequences to see how (4, 4, 4) defines this chord
- **Cyclic Permutations and Root Identification** -- The augmented triad is the key triad example of full cyclic symmetry

# Key Properties

1. Contains exactly three distinct note classes
2. Interval sequence: (4, 4, 4) in semitones, summing to 12
3. Every adjacent pair of notes is separated by a major third (4 semitones)
4. Complete cyclic symmetry (Z_3 symmetry group) -- no discernible root
5. Divides the octave into three equal parts
6. Only 4 distinct augmented triads exist (12 / 3 = 4)
7. Labeled with suffix "aug" or "+" (e.g., Caug or C+)

# Construction / Recognition

## To Construct an Augmented Triad

1. Choose any note as the starting point
2. Go up 4 semitones (major third)
3. Go up 4 more semitones (another major third)
4. The final note is 4 semitones below the starting note (completing the cycle)

## To Recognize an Augmented Triad

1. Reduce to note classes and verify exactly three
2. Compute the interval sequence
3. Check that all three intervals equal 4

# Context & Application

The augmented triad creates a sense of ambiguity and suspension due to its symmetric structure. When labeling is needed for chords with no discernible root, the root is conventionally assigned to the lowest note in the voicing. Context may override this convention -- the text notes a case where a chord might be labeled E aug even though it is spelled as C aug (Wright, pp. 52-53).

# Examples

**Example 1** (p. 47): Musical notation examples of augmented chords are shown.

**Example 2**: C augmented: C-E-G#. The same pitch classes could equally be labeled E aug (E-G#-C) or Ab aug (Ab-C-E).

**Example 3** (p. 52): A chord spelled as C aug might be labeled E aug in context, with the sharp applied to allow the augmented fifth to be written diatonically.

# Relationships

## Builds Upon

- **Chord Types and Interval Sequences** -- The augmented triad is defined by the fully symmetric sequence (4, 4, 4)

## Enables

- **Chord Spelling** -- For augmented triads, correct spelling identifies the intended root

## Related

- **Triads** -- One of four standard triad types
- **Diminished Seventh Chord** -- Also has full cyclic symmetry (with 4 notes)

## Contrasts With

- **Major Triad** -- (4, 3, 5): asymmetric, with a discernible root; the augmented triad replaces the minor third and perfect fourth with two major thirds
- **Diminished Triad** -- (3, 3, 6): also has repeated intervals but retains a discernible root

# Common Errors

- **Error**: Stating there are 12 distinct augmented triads
  **Correction**: There are only 4, since the symmetry means each set of 3 pitch classes can be named three ways

# Common Confusions

- **Confusion**: Thinking the augmented triad has an inherent root determined by its interval structure
  **Clarification**: The full cyclic symmetry means the "root" is assigned by voicing or context, not by the interval structure

# Source Reference

Chapter 3: "Harmony and Related Numerology," p. 47. Spelling conventions discussed on pp. 52-53.

# Verification Notes

- Definition source: Direct from p. 47
- Confidence rationale: High -- explicitly defined with discussion of symmetry
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: count of 4 distinct augmented triads, Z_3 symmetry group observation, contextual labeling example from p. 52
