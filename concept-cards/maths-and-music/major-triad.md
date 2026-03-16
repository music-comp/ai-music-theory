---
# === CORE IDENTIFICATION ===
concept: Major Triad
slug: major-triad

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
section: "Major Chord"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - major chord

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-types-and-interval-sequences
extends: []
related:
  - triads
  - voicing
  - chord-labeling
contrasts_with:
  - minor-triad
  - diminished-triad
  - augmented-triad

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a major triad using interval sequences?"
  - "What distinguishes a major triad from a minor triad mathematically?"
  - "Why does a major triad have a unique root?"
---

# Quick Definition

The major triad is a three-note chord defined by the interval sequence (4, 3, 5) in semitones, consisting of a root, a major third above the root, and a perfect fifth above the root.

# Core Definition

The major triad is defined by the sequence of modular intervals (4, 3, 5) in Z_12 between successive note classes:

root --(4)--> third --(3)--> fifth --(5)--> (root)

The three notes are called root, third, and fifth respectively. The sequence (4, 3, 5) has no non-trivial cyclic symmetries -- its three cyclic permutations (4,3,5), (3,5,4), and (5,4,3) are all distinct -- so the root, third, and fifth are uniquely determined (Wright, pp. 45-46).

# Prerequisites

- **Chord Types and Interval Sequences** -- The major triad is defined by a specific interval sequence in Z_12; understanding how interval sequences classify chords is essential

# Key Properties

1. Contains exactly three distinct note classes
2. Interval sequence: (4, 3, 5) in semitones, summing to 12
3. Root to third: major third (4 semitones)
4. Root to fifth: perfect fifth (7 semitones = 4 + 3)
5. No non-trivial cyclic symmetries, so root is uniquely identifiable regardless of voicing
6. Labeled with no suffix (e.g., "C" denotes C major triad)

# Construction / Recognition

## To Construct a Major Triad

1. Choose a root note
2. Go up 4 semitones (major third) to find the third
3. Go up 3 more semitones (minor third) from the third to find the fifth
4. Verify: root to fifth spans 7 semitones (perfect fifth)

## To Recognize a Major Triad

1. Reduce all sounding notes to note classes (modulo octave)
2. Remove duplicates (octave doublings)
3. Verify exactly three distinct note classes remain
4. Compute the cyclic interval sequence
5. Check that it matches (4, 3, 5)

# Context & Application

The major triad has been a fundamental building block of Western harmony since the seventeenth century. It is the most basic chord in tonal music. The chord is labeled by its root with no suffix (e.g., "C" denotes C major, "V" in Roman numeral analysis). In the major mode, diatonic major triads occur on scale degrees I, IV, and V. The major triad also forms the basis for the seventh chord (4, 3, 3, 2) and the major seventh chord (4, 3, 4, 1).

# Examples

**Example 1** (p. 45): C major triad: C-E-G (root C, third E, fifth G).

**Example 2** (p. 45): F major triad: F-A-C (root F, third A, fifth C). The text identifies the root, third, and fifth explicitly.

**Example 3** (p. 46): Various voicings are shown, including one where the fifth is the lowest note. The root remains identifiable due to the asymmetric interval sequence.

# Relationships

## Builds Upon

- **Chord Types and Interval Sequences** -- The major triad is a specific instance of a chord type

## Enables

- **Seventh Chord** -- The seventh chord (4, 3, 3, 2) contains the major triad with same root, third, and fifth
- **Major Seventh Chord** -- The major seventh chord (4, 3, 4, 1) also contains the major triad

## Related

- **Triads** -- The major triad is one of four standard triad types
- **Voicing** -- Different voicings of the major triad are all identified as the same chord
- **Chord Labeling** -- The major triad uses no suffix in labeling

## Contrasts With

- **Minor Triad** -- Has interval sequence (3, 4, 5); differs in the first interval (minor third vs. major third from root)
- **Diminished Triad** -- Has interval sequence (3, 3, 6); differs in both third and fifth quality
- **Augmented Triad** -- Has interval sequence (4, 4, 4); differs in fifth quality and has no discernible root

# Common Errors

- **Error**: Stating that the interval from root to fifth is 5 semitones based on the sequence (4, 3, 5)
  **Correction**: The 5 in the sequence is the interval from the fifth back to the root; the interval from root to fifth is 4 + 3 = 7 semitones (a perfect fifth)
- **Error**: Assuming the root must be the lowest-sounding note
  **Correction**: The root is determined by the interval sequence structure, not by voicing; any chord tone can be the bass note

# Common Confusions

- **Confusion**: Using "major chord" and "major triad" interchangeably without care
  **Clarification**: "Major triad" is the precise term; "major chord" is sometimes used but can be ambiguous when other chords containing the major triad (like the seventh chord) are discussed
- **Confusion**: Thinking major and minor triads differ in their fifth
  **Clarification**: Both have a perfect fifth (7 semitones from root to fifth); only the third differs (4 vs. 3 semitones)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 45-46. The major chord is the first chord introduced, with musical notation examples and the interval sequence diagram.

# Verification Notes

- Definition source: Direct from pp. 45-46, explicit definition with interval sequence diagram
- Confidence rationale: High -- the source provides an explicit definition with diagram and examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: observation about the 5 in the interval sequence being fifth-to-root not root-to-fifth, note about major chord vs. major triad terminology
