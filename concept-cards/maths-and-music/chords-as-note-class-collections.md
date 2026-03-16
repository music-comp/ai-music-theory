---
# === CORE IDENTIFICATION ===
concept: Chords as Note Class Collections
slug: chords-as-note-class-collections

# === CLASSIFICATION ===
category: chord-theory
subcategory: chord-notation
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - chord as pitch class set

# === TYPED RELATIONSHIPS ===
prerequisites:
  - harmony
extends: []
related:
  - chord-types-and-interval-sequences
  - voicing
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What defines a chord mathematically?"
  - "Why can a chord be played in different octaves and still be the same chord?"
---

# Quick Definition

A chord is a collection of note classes (usually three or more) sounded simultaneously, defined by its constituent pitch classes rather than by specific octave placements of the notes.

# Core Definition

A chord is defined by its note classes and the modular intervals (elements of Z_12) between them, rather than by specific pitches. Since chords are defined by note classes, any note in a chord may be displaced and/or doubled by one or more octaves without changing the chord's identity. The chord's type is determined by the sequence of intervals modulo octave between the notes (Wright, p. 44).

# Prerequisites

- **Harmony** -- Chords are the basic building blocks of harmony; understanding harmony provides the context for why chords matter

# Key Properties

1. A chord is a collection of note classes, not specific pitches
2. Notes may be displaced by one or more octaves without changing the chord
3. Notes may be doubled at the octave without changing the chord's identity
4. Chord type is determined by the sequence of modular intervals in Z_12
5. Usually consists of three or more note classes

# Construction / Recognition

## To Identify a Chord

1. List all pitches sounding simultaneously
2. Reduce each pitch to its note class (modulo octave)
3. Remove duplicate note classes (from doublings)
4. Determine the ordered sequence of intervals between successive note classes in Z_12
5. The resulting interval sequence identifies the chord type

# Context & Application

The abstraction from specific pitches to note classes is fundamental to harmonic analysis. It allows musicians to speak of chord identity independent of register. A pianist might play C3, E4, and G4, or C4, G4, and E5 -- both are the same chord (C major) in different voicings. This abstraction relies on the quotient map from Z (pitches in semitones) to Z_12 (note classes modulo octave).

# Examples

**Example 1** (p. 44): A chord is "a collection of notes, usually three or more, sounded simultaneously."

**Example 2** (p. 46): The major chord can appear in various voicings -- any of the notes may be "displaced and/or doubled by the interval of one or more octaves" and the chord remains a major chord.

**Example 3** (p. 46): The C major chord consists of note classes {C, E, G}, regardless of whether it is voiced as C3-E3-G3, E3-G3-C4, or C2-G2-E3-C4.

# Relationships

## Builds Upon

- **Harmony** -- Chords are the fundamental units of harmony

## Enables

- **Chord Types and Interval Sequences** -- Once chords are defined as note class collections, their types can be classified by interval sequences
- **Voicing** -- Voicing describes the specific realization of an abstract chord

## Related

- **Cyclic Permutations and Root Identification** -- Root identification depends on the abstract note class structure

# Common Errors

- **Error**: Treating a chord as a fixed set of specific pitches rather than note classes
  **Correction**: Always reduce to note classes modulo octave when identifying a chord

# Common Confusions

- **Confusion**: Thinking that doubling a note at the octave adds a new note class to the chord
  **Clarification**: Doubling does not change the chord's identity; it only affects voicing
- **Confusion**: Believing that the number of notes sounded equals the number of note classes
  **Clarification**: A five-note voicing may contain only three distinct note classes (a triad with doublings)

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 44-46.

# Verification Notes

- Definition source: Direct from p. 44 and elaborated on p. 46
- Confidence rationale: High -- explicit definition in source text
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: mathematical context about Z -> Z_12 quotient, examples of octave displacement
