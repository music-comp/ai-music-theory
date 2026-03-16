---
concept: Voicing
slug: voicing

category: chord-theory
subcategory: chord-notation
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Voicing"

extraction_confidence: high

aliases: []

prerequisites:
  - chords-as-note-class-collections
extends: []
related:
  - chord-spelling
  - cyclic-permutations-and-root-identification
contrasts_with: []

answers_questions:
  - "What is voicing?"
  - "Does changing the voicing change the chord?"
  - "Can the root be a note other than the lowest note?"
---

# Quick Definition

Voicing refers to the particular way a chord is written or realized -- the specific notes (as opposed to note classes) chosen, including their octave placement and any doublings.

# Core Definition

A voicing of a chord is a specific selection of pitches (not merely note classes) that realizes the chord. Since chords are defined by note classes modulo octave, any note in a chord may be displaced by one or more octaves and/or doubled. Different voicings of the same chord may place different chord tones as the lowest note, spread the notes across different octaves, and double certain note classes. The root need not be the bottom note (Wright, p. 46).

# Prerequisites

- **Chords as Note Class Collections** -- Voicing is the concrete realization of an abstract chord defined by note classes

# Key Properties

1. Voicing operates at the level of specific pitches (Z), not note classes (Z_12)
2. The passage from voicing to chord type is the quotient map Z -> Z_12
3. Multiple pitches in a voicing may map to the same note class (octave doublings)
4. The root need not be the lowest-sounding note
5. Regardless of voicing, there is no ambiguity about which note is the root, third, or fifth (for chords with discernible roots)
6. Different voicings create different sonic textures and affect voice leading

# Construction / Recognition

## To Determine a Chord from Its Voicing

1. List all specific pitches in the voicing
2. Reduce each to its note class (modulo 12)
3. Remove duplicate note classes
4. Determine the interval sequence to identify chord type
5. Identify root, third, fifth (and seventh if applicable) from the sequence

# Context & Application

Voicing is a crucial aspect of arranging and composition. Wright notes that "the root need not be the bottom note" -- in one example, the fifth of a major chord is the lowest note in the voicing. But because the interval sequence (4, 3, 5) has no non-trivial cyclic symmetries, the root remains identifiable regardless of voicing (Wright, p. 46).

# Examples

**Example 1** (p. 46): Several voicings of a major chord are shown, including one where "the lowest note is the fifth of the major chord."

**Example 2**: C major can be voiced as C3-E3-G3, E3-G3-C4, G3-C4-E4, C3-G3-E4-C5, etc. All are the C major chord.

# Relationships

## Builds Upon

- **Chords as Note Class Collections** -- Voicing is the realization of an abstract chord

## Enables

- **Chord Spelling** -- Spelling concerns the written notation of a voicing

## Related

- **Cyclic Permutations and Root Identification** -- The root remains identifiable regardless of voicing when the interval sequence is asymmetric

# Common Errors

- **Error**: Assuming the lowest note in a voicing is always the root
  **Correction**: The root is determined by the interval sequence structure, not by register placement

# Common Confusions

- **Confusion**: Thinking that changing the voicing changes the chord type
  **Clarification**: Voicing affects the sonic texture but not the chord's identity; a C major chord in any voicing is still C major
- **Confusion**: Conflating voicing with inversion
  **Clarification**: Voicing encompasses all aspects of pitch selection; inversion is a specific aspect relating to which note is in the bass

# Source Reference

Chapter 3: "Harmony and Related Numerology," p. 46.

# Verification Notes

- Definition source: Direct from p. 46
- Confidence rationale: High -- explicitly defined with examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: mathematical context about Z vs Z_12, example of fifth as lowest note
