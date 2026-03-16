---
# === CORE IDENTIFICATION ===
concept: Chord Spelling
slug: chord-spelling

# === CLASSIFICATION ===
category: chord-theory
subcategory: chord-notation
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Chord Spelling"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - enharmonic spelling

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chord-labeling
extends: []
related:
  - voicing
  - diminished-seventh-chord
  - augmented-triad
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is correct chord spelling?"
  - "Why does chord spelling matter?"
  - "How do you spell a chord correctly?"
---

# Quick Definition

Chord spelling refers to the choice of enharmonic representation for each note in a chord. Correct spelling follows rules relating each chord tone's underlying unaltered note class to the root's position on the diatonic scale.

# Core Definition

A "spelled note" differentiates between enharmonically equivalent notations (e.g., A# vs. Bb). The "underlying unaltered note class" is obtained by stripping away accidentals in the ambient key. Correct chord spelling requires:

- The third should be spelled so its underlying unaltered note class is **two** scale tone classes above the root's
- The fifth should be spelled so its underlying unaltered note class is **four** scale tone classes above the root's
- The seventh should be spelled so its underlying unaltered note class is **six** scale tone classes above the root's

(Wright, pp. 50-52)

# Prerequisites

- **Chord Labeling** -- Must understand chord labeling to know the root and chord type being spelled

# Key Properties

1. Spelling operates in the diatonic system (Z_7, seven scale tone classes) rather than the chromatic (Z_12)
2. The rules ensure chord tones maintain proper diatonic spacing
3. Correct spelling sometimes necessitates double flats, double sharps, or non-diatonic notes
4. For augmented and diminished seventh chords, correct spelling identifies the root
5. Spelling rules are followed less rigorously for diminished triads and symmetric chords
6. Chords are sometimes intentionally misspelled to make voice leading more natural

# Construction / Recognition

## To Spell a Chord Correctly

1. Identify the root's spelled note and its position in the diatonic scale
2. For the third: find the note two scale steps above the root; add accidentals as needed to achieve the correct chromatic interval
3. For the fifth: find the note four scale steps above the root; add accidentals as needed
4. For the seventh (if present): find the note six scale steps above the root; add accidentals as needed

# Context & Application

Correct spelling is important for readability and for identifying chord structure at a glance. For augmented and diminished seventh chords (which lack inherent roots), correct spelling identifies which note serves as root. The augmented sixth chord is enharmonically equivalent to the seventh chord but spells the seventh as an augmented sixth -- in a "dominant" role, it should always be spelled as a seventh chord (Wright, pp. 50-53).

# Examples

**Example 1** (p. 51): If the root is C#, the third must be E# (not F), even though they sound the same pitch.

**Example 2** (p. 51): D major triad misspelled: D-Gb-A (third misspelled). Correct: D-F#-A.

**Example 3** (p. 51): Eb7 misspelled: Eb-G-Bb-C# (both fifth and seventh misspelled). Correct: Eb-G-Bb-Db.

**Example 4** (p. 52): Co7 correctly spelled requires Bbb for the seventh, and the same enharmonic chord spelled as D#o7 uses different note names.

# Relationships

## Builds Upon

- **Chord Labeling** -- Spelling is the notation layer on top of labeling

## Enables

- **Augmented Triad** root identification through spelling
- **Diminished Seventh Chord** root identification through spelling

## Related

- **Voicing** -- Spelling concerns notation; voicing concerns pitch selection

# Common Errors

- **Error**: Spelling the third of C# major as F instead of E#
  **Correction**: The underlying unaltered note must be two scale steps above the root (C->D->E, so E# not F)

# Common Confusions

- **Confusion**: Thinking correct spelling is just a convention with no functional purpose
  **Clarification**: For symmetric chords, spelling is the only way to identify the intended root; for all chords, correct spelling reveals structure at a glance
- **Confusion**: Believing double sharps and double flats are always avoidable
  **Clarification**: Correct spelling sometimes requires them (e.g., Bbb for the seventh of Co7)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 50-53. Includes discussion of spelled notes, underlying unaltered notes, augmented sixth chords, and intentional misspelling.

# Verification Notes

- Definition source: Direct from pp. 50-52, explicit spelling rules
- Confidence rationale: High -- detailed rules and multiple examples in source
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_7 mathematical context, augmented sixth equivalence discussion, intentional misspelling note
