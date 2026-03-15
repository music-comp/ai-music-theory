---
# === CORE IDENTIFICATION ===
concept: Octave Symmetry (O)
slug: octave-symmetry

# === CLASSIFICATION ===
category: geometric-theory
subcategory: symmetry
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "octave equivalence"
  - "O symmetry"
  - "registral inversion"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-musical-object
  - pitch-space
extends: []
related:
  - optic-symmetries
  - pitch-class-space
contrasts_with:
  - transposition-symmetry

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are OPTIC symmetries?"
  - "What is octave equivalence?"
---

# Quick Definition
The O (Octave) symmetry allows moving any single note of a musical object into a different octave without changing the object's essential harmonic identity.

# Core Definition
The octave symmetry operation moves any individual note in a musical object up or down by one or more octaves (multiples of 12 semitones). This is the operation that allows (C4, E4, G4) and (C4, E5, G4) to be considered harmonically equivalent — they are the same "chord" in different voicings. Applying O to all notes produces pitch-class space from pitch space. The O symmetry is distinct from the T (transposition) symmetry: O moves one note at a time by an octave, while T moves all notes simultaneously by the same amount. O corresponds to what traditional theory calls "registral inversion" or "chord voicing."

# Prerequisites
- **basic-musical-object** — The object to which the operation is applied
- **pitch-space** — The space in which octave shifts occur

# Key Properties
1. Moves any single note by a multiple of 12 semitones
2. Preserves pitch-class content while changing register
3. Different from transposition (which moves all notes uniformly)
4. Generates pitch-class space from pitch space
5. Traditional terms: "registral inversion," "chord voicing," "open vs. close position"

# Construction / Recognition
## To Construct/Create:
1. Take a basic musical object
2. Move any note up or down by 12 (or 24, 36, etc.) semitones
## To Identify/Recognize:
1. Two objects have the same notes but in different octaves
2. Same pitch-class content, different registral arrangement

# Context & Application
The O symmetry is one of Rameau's three implicit operations that define a "chord." Combined with P and C, it gives the standard notion of chord (unordered set of pitch classes). It is the most basic and universally accepted of the OPTIC symmetries.

# Examples
**Example 1** (p. 54-55): Transforming (C4, E4, G4) to (C4, E5, G4) by shifting E up an octave — same chord, different voicing.

# Relationships
## Builds Upon
- **basic-musical-object** — Applied to basic objects
- **pitch-space** — Operates within pitch space
## Enables
- **pitch-class-space** — Created by systematically applying O
- **chord** — O + P + C = chord
## Related
- **optic-symmetries** — O is the first of five OPTIC operations
## Contrasts With
- **transposition-symmetry** — T moves all notes; O moves one note by an octave

# Common Errors
- **Error**: Confusing octave shift (O) with transposition (T)
  **Correction**: O moves one note by an octave; T moves all notes by the same amount

# Common Confusions
- **Confusion**: Thinking O and T by 12 semitones are the same
  **Clarification**: T12 moves all notes up an octave (changing register uniformly); O can move different notes to different octaves

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 53-59.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.4
- Confidence rationale: High — explicitly defined in the OPTIC table
- Cross-reference status: Verified; used throughout the book
