---
# === CORE IDENTIFICATION ===
concept: Enharmonic Equivalence
slug: enharmonic-equivalence

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: notation
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Accidentals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "enharmonic equivalents"
  - "enharmonic spelling"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - accidentals
  - equivalence-relations
extends:
  - equivalence-relations
related:
  - note-classes
  - key-signatures-and-the-circle-of-fifths
contrasts_with:
  - octave-equivalence

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does it mean for two notes to be enharmonically equivalent?"
  - "How does enharmonic equivalence differ from octave equivalence?"
  - "Which key pairs are enharmonically equivalent?"
---

# Quick Definition

The relationship between two differently named notes that produce the same pitch, such as F-sharp and G-flat.

# Core Definition

Two notes are enharmonically equivalent when they give the same pitch. This defines an equivalence relation on the set of note names (Wright, p. 21). In equal temperament, enharmonic equivalence is exact; in other tuning systems (just intonation, Pythagorean), nominally enharmonic notes may differ slightly in pitch.

# Prerequisites

- **Accidentals** — Enharmonic equivalence arises from different accidental spellings of the same pitch
- **Equivalence Relations** — Enharmonic equivalence is an equivalence relation

# Key Properties

1. Enharmonic equivalence is an equivalence relation (reflexive, symmetric, transitive)
2. In equal temperament, it is exact — the pitches are identical
3. In other tuning systems, nominally enharmonic notes may differ
4. The 7-letter naming system with accidentals generates more names than the 12 distinct pitches
5. Three pairs of keys are enharmonically equivalent: $D^\flat \sim C^\sharp$, $G^\flat \sim F^\sharp$, $C^\flat \sim B$

# Construction / Recognition

## To identify enharmonic equivalents:

1. Determine the pitch in semitones (from C)
2. List all note names that produce this pitch using different accidentals
3. All such names are enharmonically equivalent
4. The correct spelling depends on musical context (key, harmonic function)

# Context & Application

Enharmonic equivalence arises because the 7-letter naming system with sharps and flats generates more names than there are distinct pitches in equal temperament. The choice between enharmonic spellings depends on musical context: key signature, harmonic function, and voice leading. The two sequences of key signatures (sharps and flats) wrap against each other, yielding the three enharmonically equivalent key pairs.

# Examples

- $F^\sharp = G^\flat$ (same pitch, different names) (p. 21)
- $C^\flat_5 = B_4$ (same note, different notation) (p. 19)
- $B^\sharp_3 = C_4$ (p. 19)
- Enharmonically equivalent key pairs: $D^\flat / C^\sharp$, $G^\flat / F^\sharp$, $C^\flat / B$ (p. 23)

# Relationships

## Builds Upon
- **Accidentals** — Different accidentals on different letter names can produce the same pitch
- **Equivalence Relations** — Enharmonic equivalence satisfies all three properties

## Enables
- **Key Signatures and the Circle of Fifths** — The wrap-around of flat and sharp key sequences produces enharmonic key pairs

## Related
- **Note Classes** — Enharmonic equivalence reduces note names to 12 pitch classes

## Contrasts With
- **Octave Equivalence** — Octave equivalence identifies notes of the same name class across octaves; enharmonic equivalence identifies different names at the same pitch

# Common Errors

- **Error**: Assuming enharmonic equivalence holds in all tuning systems
  **Correction**: It is exact only in equal temperament; in just intonation or Pythagorean tuning, $F^\sharp$ and $G^\flat$ may differ

# Common Confusions

- **Confusion**: Thinking enharmonic spelling is arbitrary
  **Clarification**: In practice, the correct spelling depends on key and context: $F^\sharp$ is appropriate in G major, $G^\flat$ in $D^\flat$ major
- **Confusion**: Conflating enharmonic equivalence with octave equivalence
  **Clarification**: Enharmonic equivalence identifies different names at the same pitch; octave equivalence identifies the same name at different octaves — these are distinct equivalence relations

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Accidentals" section, p. 21 (PDF); "Key Signatures" section, p. 23 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 21
- Confidence rationale: High — explicit definition with clear examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: tuning system caveat, enharmonic key pairs, context-dependent spelling
