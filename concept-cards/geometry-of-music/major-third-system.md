---
# === CORE IDENTIFICATION ===
concept: Major-Third System and Minor-Third System
slug: major-third-system

# === CLASSIFICATION ===
category: voice-leading
subcategory: chord-relationships
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 113
section: "3.10"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "triadic major-third relations"
  - "seventh-chord minor-third relations"
  - "major-third and minor-third root motion"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - near-symmetry
  - nearly-even-chords
  - three-note-chord-space
extends: []
related:
  - voice-leading-lattices
  - harmonic-consistency-and-efficient-voice-leading
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why do triads favor major-third root motion in chromatic contexts?"
  - "Why do seventh chords favor minor-third root motion?"
  - "What geometric fact unifies these two patterns?"
---

# Quick Definition
Nearly even triads can be linked by maximally efficient voice leading when their roots are a major third apart (the "major-third system"), while nearly even seventh chords favor minor-third root motion (the "minor-third system"). These preferences arise from the chords' proximity to augmented triads (3-fold symmetry) and diminished sevenths (4-fold symmetry), respectively.

# Core Definition
The major-third system describes the fact that major and minor triads can be connected to their major-third transpositions by pure contrary voice leading — the most efficient possible non-parallel connection. This is because triads are near augmented triads, which have 3-fold symmetry (12/3=4 semitones = major third). Similarly, the minor-third system describes the fact that dominant seventh, half-diminished seventh, and other nearly even four-note chords connect most efficiently to their minor-third transpositions, because they are near diminished seventh chords with 4-fold symmetry (12/4=3 semitones = minor third). By adding a small parallel component to the pure contrary voice leading, composers can also reach perfect-fourth transpositions efficiently, explaining the prevalence of fifth-based (or fourth-based) root motion in tonal music.

# Prerequisites
- Near symmetry and its relationship to transposition intervals
- Three-note chord space and the central lattice

# Key Properties
1. Triads: pure contrary motion connects major-third transpositions; small parallel addition gives perfect-fourth transposition
2. Seventh chords: pure contrary motion connects minor-third transpositions; small parallel addition gives major-second or tritone transpositions
3. The switch from triads to sevenths characteristically accompanies a switch in root-motion pattern
4. The perfect fourth/fifth uniquely appears in both systems, explaining its universality in tonal music

# Construction / Recognition
## To Identify:
1. Note the chord types in a passage
2. Observe the root motion intervals
3. Major-third or fifth root motion with triads = major-third system
4. Minor-third or tritone root motion with sevenths = minor-third system
5. A switch from one system to the other often accompanies a change in chord type

# Context & Application
The major-third and minor-third systems underwrite three major compositional practices: (1) maximally efficient voice leading between successive chords, (2) chord substitution (e.g., jazz tritone substitution), and (3) descending sequences combining efficient voice leading with small parallel descent. These patterns are found across the entire extended common practice, from Schubert and Chopin through Nirvana and the Beatles.

# Examples
**Example 1** (p. 117-118): Nirvana's "Heart-Shaped Box" uses major-third root motion for its triads (A-F-D) but switches to minor-third motion (F -> D7) when a seventh chord appears (Figure 3.10.4).
**Example 2** (p. 118): The Beatles' "Glass Onion" juxtaposes major-third-related triads in the verse and minor-third-related dominant sevenths in the chorus.
**Example 3** (p. 118): Schumann's "Chopin" from Carnaval alternates between the major-third system (F minor triad to A dominant seventh) and the minor-third system (A7 to Eb7) — Figure 3.10.5.

# Relationships
## Builds Upon
- **near-symmetry** — The geometric explanation for these systems
- **nearly-even-chords** — The chord types that participate
## Enables
- Analysis of chromatic harmony in Chapters 6, 8
## Related
- **voice-leading-lattices** — The cubic lattices encode these relationships
- **harmonic-consistency-and-efficient-voice-leading** — These systems are the primary solution

# Common Errors
- **Error**: Thinking major-third relations are only relevant to chromatic music
  **Correction**: While most prominent in chromatic harmony, major-third relations underlie many diatonic patterns as well (e.g., iii -> vi in C major is a major-third root motion)

# Common Confusions
- **Confusion**: Why is the perfect fourth/fifth special?
  **Clarification**: It is the only interval appearing in both the major-third and minor-third systems (Figure 3.10.8), making it universally useful regardless of chord type

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.10, pages 113-121.

# Verification Notes
- Definition source: Synthesized from Section 3.10 and Figure 3.10.8
- Confidence rationale: High — the central analytical result of Section 3.10
- Cross-reference status: Verified against Nirvana, Beatles, Schumann, and Schubert examples
