---
concept: Chord Labeling
slug: chord-labeling

category: chord-theory
subcategory: chord-notation
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Chord Labeling"

extraction_confidence: high

aliases:
  - chord notation
  - chord symbols

prerequisites:
  - chord-types-and-interval-sequences
  - cyclic-permutations-and-root-identification
extends: []
related:
  - chord-spelling
  - functional-harmony
contrasts_with: []

answers_questions:
  - "How are chords labeled?"
  - "What do chord suffixes mean?"
  - "What is the difference between note-class and Roman numeral labeling?"
---

# Quick Definition

Chord labeling identifies a chord by its root (either a note class name or a Roman numeral scale degree) followed by a suffix indicating the chord type.

# Core Definition

A chord label consists of two parts: (1) the root identifier, which is either a specific note class (e.g., D, Bb) or a scale tone indicated by Roman numeral (possibly preceded by # or b, e.g., III, bVI), and (2) a suffix indicating chord type. The standard suffixes are:

- Major triad: no suffix
- Minor triad: m
- Augmented: aug or +
- Diminished: dim or ^0
- Seventh: ^7
- Minor seventh: m^7
- Major seventh: M^7
- Diminished seventh: ^o7
- Half-diminished seventh: ^ø7

For augmented or diminished seventh chords (which have no discernible root), the root is often declared to be the lowest note in the voicing (Wright, pp. 49-51).

# Prerequisites

- **Chord Types and Interval Sequences** -- Must know chord types to understand the suffix system
- **Cyclic Permutations and Root Identification** -- Must understand root identification to know when labeling requires convention vs. structure

# Key Properties

1. Two labeling systems: note-class (absolute) and Roman numeral (relative to key)
2. Note-class labels identify chords independently of key (e.g., G7, F#m)
3. Roman numeral labels identify chords relative to a tonic and mode (e.g., V7, IIm)
4. No suffix means major triad
5. Roman numeral labeling requires specifying the mode
6. An alternate system uses uppercase/lowercase to indicate major/minor thirds (e.g., iv for minor triad on scale degree 4)

# Construction / Recognition

## To Label a Chord

1. Identify the chord type from its interval sequence
2. Identify the root (from structure, or by convention for symmetric chords)
3. Choose a labeling system (note-class or Roman numeral)
4. For note-class: write the root note followed by the type suffix
5. For Roman numeral: write the scale degree numeral (with # or b if needed) followed by the suffix

# Context & Application

Two parallel labeling systems are in common use. Note-class labeling (e.g., G7, F#m) identifies chords absolutely. Roman numeral labeling (e.g., V7, IIm) identifies chords relative to a key, making it easy to see harmonic function and to transpose. The alternate system uses uppercase/lowercase to indicate major/minor quality: iv for a minor triad on scale degree 4, ii^0 for diminished on scale degree 2 (Wright, pp. 49-51).

# Examples

**Example 1** (p. 50): A C major triad is labeled "C" (no suffix), "V" in the key of F major, or "III" in A minor.

**Example 2** (p. 50): F#m7 is a minor seventh chord rooted on F#; in D major it would be IIIm7; in G minor it would be #VIIm7.

**Example 3** (p. 51): The alternate labeling system writes bb^7 for a minor seventh on Bb, and ii^0 for a diminished triad on scale degree 2.

**Example 4** (pp. 50-51): Musical notation examples show chords labeled by both note class and Roman numeral.

# Relationships

## Builds Upon

- **Chord Types and Interval Sequences** -- Labels encode chord type via suffix
- **Cyclic Permutations and Root Identification** -- Root identification is required for labeling

## Enables

- **Functional Harmony** -- Roman numeral labels reveal harmonic function

## Related

- **Chord Spelling** -- Spelling affects how labeled chords are notated
- **Chord Progressions and Circle of Fifths** -- Progressions are described using chord labels

# Common Errors

- **Error**: Omitting the suffix and assuming the chord type is obvious from context
  **Correction**: No suffix specifically means major triad; always include the suffix for other chord types

# Common Confusions

- **Confusion**: Thinking no suffix means "unspecified chord type"
  **Clarification**: No suffix specifically means major triad (e.g., "C" means C major triad)
- **Confusion**: Confusing Roman numeral labeling systems (some use case to indicate quality, others use suffixes)
  **Clarification**: Wright's primary system uses uppercase Roman numerals with suffixes; the alternate system uses case (uppercase = major third, lowercase = minor third)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 49-51. Alternate labeling on p. 51.

# Verification Notes

- Definition source: Direct from pp. 49-51
- Confidence rationale: High -- explicit list of suffixes and labeling examples
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete suffix list, alternate labeling system, examples with multiple key contexts
