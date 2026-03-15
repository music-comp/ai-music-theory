---
# === CORE IDENTIFICATION ===
concept: Voice Leading vs. Chord Progression
slug: voice-leading-vs-chord-progression

# === CLASSIFICATION ===
category: voice-leading
subcategory: definition
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "counterpoint vs. harmony"
  - "voice mappings vs. chord succession"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading-in-pitch-space
  - chord-progression
  - permutation-symmetry
extends: []
related:
  - optic-symmetries
  - harmonic-consistency
  - conjunct-melodic-motion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a voice leading vs. a chord progression?"
  - "How do voice leadings and chord progressions differ?"
---

# Quick Definition
Voice leadings describe how individual voices move from one chord to the next (counterpoint); chord progressions describe what chords occur in sequence (harmony) — the same chord progression can be realized by many different voice leadings.

# Core Definition
This distinction is fundamental to Tymoczko's framework. Voice leadings arise from uniform application of the permutation symmetry to progressions, preserving voice identity: they specify which note in the first chord moves to which note in the second. Chord progressions arise from individual application of the permutation and cardinality-change symmetries, destroying voice identity: they specify only what chords occur. The voice leading (C, E, G) -> (C, F, A) specifies that C holds, E moves up to F, and G moves up to A. The chord progression {C, E, G} => {C, F, A} says only that a C major triad is followed by an F major triad, with no commitment about how voices move. Tymoczko uses single arrows for voice leadings and double arrows for chord progressions.

# Prerequisites
- **voice-leading-in-pitch-space** — Must understand what voice leadings are
- **chord-progression** — Must understand what chord progressions are
- **permutation-symmetry** — The symmetry operation whose application (uniform vs. individual) distinguishes them

# Key Properties
1. Voice leadings = uniform P; chord progressions = individual P + C
2. One chord progression can be realized by many voice leadings
3. Notation: single arrow for voice leadings, double arrow for chord progressions
4. Voice leadings capture contrapuntal information; chord progressions capture harmonic information
5. Western music theory has traditionally focused more on chord progressions; Tymoczko argues for equal attention to voice leadings

# Construction / Recognition
## To Construct/Create:
1. To create a voice leading: assign each voice a starting and ending note
2. To create a chord progression: specify a sequence of chords without voice assignments
## To Identify/Recognize:
1. Does the analysis specify which voice goes where? -> voice leading
2. Does it only specify what chords occur? -> chord progression

# Context & Application
This distinction underlies the entire book. Tymoczko argues that traditional music theory focuses too heavily on chord progressions (e.g., Roman numeral analysis) and insufficient on voice leadings. He shows that much of what makes music interesting lies in the specific voice-leading choices composers make, not just the chords they choose. The tension between harmonic consistency (chord progressions) and conjunct motion (voice leadings) is the central theme of the book.

# Examples
**Example 1** (p. 59-62): The progression {C, E, G, Bb} => {E, G#, B} is a chord progression. The voice leading (C, E, G, Bb) -> (B, E, G#, B) is one specific realization. Other voice leadings between the same chords would be different realizations.

**Example 2** (p. 63, Fig 2.5.6): Summary table showing that voice leadings in pitch space use uniform P, voice leadings in pitch-class space use uniform OP, and chord progressions use individual OPC.

# Relationships
## Builds Upon
- **voice-leading-in-pitch-space** — One side of the distinction
- **chord-progression** — The other side of the distinction
## Enables
- Understanding the two-dimensional coherence of Western music (harmony + counterpoint)
## Related
- **harmonic-consistency** — A constraint on chord progressions
- **conjunct-melodic-motion** — A constraint on voice leadings
## Contrasts With
- No direct contrast; this IS the contrast being defined

# Common Errors
- **Error**: Using "voice leading" and "chord progression" interchangeably
  **Correction**: They are fundamentally different: voice leadings specify voice mappings, chord progressions do not

# Common Confusions
- **Confusion**: Thinking each chord progression has a unique "correct" voice leading
  **Clarification**: Any chord progression can be realized by many different voice leadings; composers choose among them based on voice-leading quality

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.5, pages 59-63.

# Verification Notes
- Definition source: Direct from Section 2.5, with formal OPTIC characterization
- Confidence rationale: High — this is a central, explicitly argued distinction
- Cross-reference status: Verified; fundamental to the entire book
