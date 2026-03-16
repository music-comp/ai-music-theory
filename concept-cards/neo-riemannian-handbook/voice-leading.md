---
# === CORE IDENTIFICATION ===
concept: Voice Leading
slug: voice-leading

# === CLASSIFICATION ===
category: voice-leading
subcategory: foundational-concept
tier: foundational

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Dmitri Tymoczko"
chapter: "Dualism and the Beholder's Eye"
chapter_number: 8
pdf_page: null
section: "Sections 3-5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "voice-leading motion"
  - "part writing"
  - "contrapuntal motion"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - voice-leading-efficiency
  - inversional-symmetry
  - plr-transformations
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is voice leading in neo-Riemannian theory?"
  - "How do PLR transformations relate to voice-leading efficiency?"
  - "What is voice-leading parsimony?"
---

# Quick Definition

The horizontal motion of individual melodic lines (voices) as they move from chord to chord, formalized by Tymoczko as a mapping from pitches in one chord to pitches in another, with "efficient" or "parsimonious" voice leading minimizing the distances voices travel.

# Core Definition

A **voice leading** is a mapping from pitches in one chord to pitches in another, specifying which note moves to which. Tymoczko's notation: (C4, E4, G4) → (C4, Eb4, G4) indicates C moves to C, E moves to Eb, and G moves to G.

"By 'efficient' voice leading I mean, roughly, voice leading in which no voice moves very far" (Ch. 8). Voice leadings are formalized as equivalence classes of progressions under the individual symmetries of reordering and octave shift.

Voice leading is the domain where Riemann's dualistic insights prove most relevant to chromatic music: "the efficient voice leadings between members of any two set classes can always be grouped into inversionally related pairs" (Ch. 8).

# Prerequisites

This is a foundational concept with no strict prerequisites within this source.

# Key Properties

1. Voice leading maps individual pitches from one chord to corresponding pitches in another
2. Efficient voice leading minimizes total pitch motion
3. DVLS (Displacement Voice-Leading Size): Sum of semitones moved by all voices
4. AVLS (Average Voice-Leading Size): DVLS divided by number of voices
5. Semitonal voice leading: No voice moves more than one semitone (maximal efficiency)
6. Efficient voice leadings always come in inversionally related pairs

# Construction / Recognition

## Measuring Voice-Leading Efficiency
1. Assign each note in chord 1 to a note in chord 2
2. Calculate the distance (in semitones) each voice moves
3. Sum all distances = DVLS
4. Divide by number of voices = AVLS
5. Lower values = more efficient voice leading

## Neo-Riemannian PLR and Voice Leading
- **P** (Parallel): One voice moves by semitone (DVLS = 1)
- **L** (Leittonwechsel): One voice moves by semitone (DVLS = 1)
- **R** (Relative): One voice moves by whole tone (DVLS = 2)
- These represent the most efficient transformations between consonant triads

## Cataloging Semitonal Voice Leadings
Tymoczko catalogs all 16 semitonal voice leadings between consonant triads (where no voice moves more than a semitone). These can be grouped by:
- Inversional equivalence (pairs)
- Retrograde relationship (pairs)
- Individual transpositional equivalence (pairs)

# Context & Application

Tymoczko proposes that 19th-century chromaticism combines a "first practice" (diatonic functional harmony) with a "second practice" where "virtually any voice leading between familiar chords may be used, as long as it is efficient." Three principles (P1-P3) characterize this second practice:
- P1: If a sonority is acceptable, so is its inversion
- P2: Efficient voice leadings are desirable
- P3: Voice-leading prohibitions apply equally to ascending and descending motion

When all three hold, any acceptable voice leading will have an acceptable inversion -- explaining why inversional symmetry emerges in chromatic music without composers explicitly seeking it.

# Examples

**Wagner's Tarnhelm** (Ch. 8): (G#, B, D#) → (G, B, E). Total motion: G# down to G (1 semitone), D# up to E (1 semitone) = DVLS of 2. This is maximally efficient for connecting these two triads.

**Brahms Intermezzo Op. 76 No. 4** (Ch. 8): The piece "systematically explores the voice-leading possibilities of a few characteristic sonorities." The Tristan chord {F, G#, B, Eb} resolves three different ways, each using efficient voice leading.

**Major-third related triads**: C major to E major requires only two voices to move by semitone (C→B, G→G#) -- the most efficient voice leading between distinct major triads. This explains the frequency of major-third relations in chromatic music.

# Relationships

## Enables
- **Voice-leading efficiency** -- The concept of measuring and optimizing voice-leading motion
- **PLR transformations** -- Defined by their voice-leading properties
- **Maximally smooth cycles** -- Cycles that minimize voice-leading distance

## Related
- **Inversional symmetry** -- Efficient voice leadings form inversionally related pairs
- **Contrapuntal vs. harmonic dualism** -- Voice leading provides the contrapuntal perspective on dualism

# Common Errors

- **Error**: Confusing voice leading (how notes move) with chord progression (what chords result)
  **Correction**: The same chord progression can have multiple different voice leadings; voice leading specifies the specific mapping

- **Error**: Assuming that efficient voice leading implies functional relationship
  **Correction**: Efficient voice leading may connect functionally unrelated chords (e.g., C major to Ab major)

# Common Confusions

- **Confusion**: "Smooth" and "efficient" voice leading are identical
  **Clarification**: "Smooth" typically implies stepwise motion; "efficient" means minimal total distance (a voice could leap if total motion is still minimal)

- **Confusion**: Voice-leading inversion and chord inversion are the same
  **Clarification**: Voice-leading inversion (I-related voice leadings) is a pitch-space reflection; chord inversions (root position, first inversion) are voicing changes

# Source Reference

Chapter 8: Dmitri Tymoczko, "Dualism and the Beholder's Eye," in *The Oxford Handbook of Neo-Riemannian Music Theories*. Sections 3-5. Also references: Tymoczko, *A Geometry of Music* (2011).

# Verification Notes

- Definition: Directly from Tymoczko's formal treatment
- DVLS/AVLS: Referenced in Ch. 8 and defined in Tymoczko's other publications
- 16 semitonal voice leadings: Cataloged by Tymoczko in Ch. 8
- Confidence: HIGH -- voice leading is formally defined and extensively demonstrated
- Re-extracted from v2 card; preserved: DVLS/AVLS definitions, Tarnhelm example, Brahms Intermezzo analysis, three principles (P1-P3), contrapuntal vs. harmonic distinction
