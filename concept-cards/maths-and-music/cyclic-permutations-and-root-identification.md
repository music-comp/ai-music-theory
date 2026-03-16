---
concept: Cyclic Permutations and Root Identification
slug: cyclic-permutations-and-root-identification

category: chord-theory
subcategory: triads
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Voicing"

extraction_confidence: high

aliases:
  - root discernibility

prerequisites:
  - cyclic-permutations
  - chord-types-and-interval-sequences
extends:
  - cyclic-permutations
related:
  - augmented-triad
  - diminished-seventh-chord
  - chord-labeling
contrasts_with: []

answers_questions:
  - "How do cyclic permutations determine whether a chord has a root?"
  - "Which chords have no discernible root?"
  - "Why is the root of a major chord identifiable regardless of voicing?"
---

# Quick Definition

A chord has a discernible root if and only if no non-trivial cyclic permutation of its interval sequence reproduces the original sequence. Chords with fully symmetric interval sequences (like augmented triads and diminished seventh chords) have no inherent root.

# Core Definition

Given a chord defined by a sequence of modular intervals (a_1, a_2, ..., a_k) in Z_12, a cyclic permutation is a sequence of the form (a_i, a_{i+1}, ..., a_k, a_1, ..., a_{i-1}). The chord has a discernible root if and only if the sequence has no non-trivial cyclic symmetries -- that is, no cyclic permutation other than the identity reproduces the original sequence. When the root is discernible, each note in the chord has a unique role (root, third, fifth, seventh, etc.) (Wright, pp. 46-49).

# Prerequisites

- **Cyclic Permutations** -- Must understand the mathematical concept of cyclic permutations
- **Chord Types and Interval Sequences** -- Must know how chords are defined by interval sequences

# Key Properties

1. A sequence of length k has at most k cyclic permutations
2. The number of distinct permutations divides k
3. If all k permutations are distinct (trivial symmetry group), the root is unique
4. If the sequence has full cyclic symmetry (all permutations identical), any note can serve as root
5. Standard chords with discernible roots: major, minor, diminished, seventh, minor seventh, major seventh, half-diminished seventh
6. Standard chords without discernible roots: augmented triad (4,4,4), diminished seventh (3,3,3,3)
7. For rootless chords, root is assigned by spelling convention or voicing

# Construction / Recognition

## To Determine if a Chord Has a Discernible Root

1. Write out the chord's interval sequence
2. Generate all cyclic permutations
3. Compare each permutation to the original
4. If all permutations are distinct: root is discernible
5. If any non-trivial permutation equals the original: root is not discernible

# Context & Application

Root identification is fundamental to chord labeling and harmonic analysis. The mathematical criterion is elegant: the interval sequence alone determines whether a unique root exists. For symmetric chords, the root must be assigned by convention -- typically the lowest note in the voicing or the note identified by the chord's spelling (Wright, pp. 46-49).

# Examples

**Example 1** (p. 46): Major triad (4, 3, 5): permutations (4,3,5), (3,5,4), (5,4,3) -- all distinct. Root is unique.

**Example 2** (p. 47): Augmented triad (4, 4, 4): all permutations are (4,4,4). No discernible root.

**Example 3** (pp. 48-49): Diminished seventh (3, 3, 3, 3): all permutations are identical. No discernible root.

**Example 4** (p. 48): Seventh chord (4, 3, 3, 2): four permutations (4,3,3,2), (3,3,2,4), (3,2,4,3), (2,4,3,3) -- all distinct. Root is unique.

**Example 5** (p. 48): Minor seventh (3, 4, 3, 2): despite containing two 3s, all four permutations are distinct. Root is unique.

# Relationships

## Builds Upon

- **Cyclic Permutations** -- This concept applies cyclic permutations to chord theory

## Enables

- **Chord Labeling** -- Root identification is required before a chord can be labeled
- **Chord Spelling** -- For rootless chords, spelling conventions assign the root

## Related

- **Augmented Triad** -- Primary triad example of full cyclic symmetry
- **Diminished Seventh Chord** -- Primary four-note example of full cyclic symmetry

# Common Errors

- **Error**: Concluding that a chord has no root because it contains repeated intervals
  **Correction**: Having repeated intervals does not automatically imply cyclic symmetry; check all cyclic permutations explicitly

# Common Confusions

- **Confusion**: Thinking the root is simply "the bottom note"
  **Clarification**: The root is a structural property of the interval sequence, determined mathematically; it may or may not be the lowest-sounding note in a voicing
- **Confusion**: Assuming rootless chords cannot be labeled
  **Clarification**: They can be labeled by assigning a root by convention (voicing or spelling)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 46-49. The principle is introduced with the major chord on p. 46 and applied to all subsequent chord types.

# Verification Notes

- Definition source: Synthesized from the repeated application of the principle across pp. 46-49
- Confidence rationale: High -- the principle is stated explicitly and applied consistently to every chord type
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: comprehensive list of which chords have/lack roots, minor seventh example showing repeated intervals don't imply symmetry
