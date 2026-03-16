---
concept: Chord Progression
slug: chord-progression

category: harmony
subcategory: definition
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.5"

extraction_confidence: high

aliases:
  - "harmonic progression"
  - "individual OPC class"

prerequisites:
  - chord
extends: []
related:
  - voice-leading-vs-chord-progression
  - optic-symmetries
contrasts_with:
  - voice-leading-in-pitch-space
  - voice-leading-in-pitch-class-space

answers_questions:
  - "What is a voice leading vs. a chord progression?"
  - "What is a chord progression?"
---

# Quick Definition
A chord progression is a succession of chords with no implied mapping between their notes — it describes what harmonies occur but not how individual voices move between them.

# Core Definition
A chord progression, as Tymoczko formally defines it, is a sequence of unordered pitch-class sets with no voice-mapping information. It arises from individual (not uniform) application of the O, P, and C symmetries, which destroy the identity of individual voices by applying different transformations to each chord. The notation uses a double arrow: {C, E, G, Bb} => {E, G#, B} indicates a succession from a C dominant seventh to an E major triad, but says nothing about which note moves to which. Chord progressions describe the harmonic dimension of music while ignoring the contrapuntal dimension.

# Prerequisites
- **chord** — Chord progressions are sequences of chords

# Key Properties
1. Sequence of chords (unordered pitch-class sets) with no voice mappings
2. Uses double arrow notation: =>
3. Arises from individual application of OPC symmetries
4. Focuses on harmony, ignores counterpoint
5. Multiple different voice leadings can "realize" the same chord progression
6. Can also be defined in pitch space (pairs of unordered pitch sets)

# Construction / Recognition
## To Construct/Create:
1. Specify a sequence of chords: {C, E, G} => {C, F, A}
2. Do not specify voice-to-voice mappings
## To Identify/Recognize:
1. A progression states what chords occur but not how voices move
2. No specific voice assignments or motion paths indicated

# Context & Application
Chord progressions are the standard way of describing harmonic motion in music. Roman numeral analysis (I-IV-V-I) describes chord progressions. However, Tymoczko emphasizes that chord progressions are less informative than voice leadings — a single chord progression can be realized by many different voice leadings, and the contrapuntal choices matter. Much of the book argues for supplementing chord-progression analysis with voice-leading analysis.

# Examples
**Example 1** (p. 62): {C, E, G, Bb} => {E, G#, B}, or C7 => E — the first chord is C dominant seventh, the second is E major, but no voice mappings are specified.

**Example 2** (p. 63, Fig 2.5.5): A chord progression shown as two snapshots of points on the pitch-class circle, with no lines connecting them.

# Relationships
## Builds Upon
- **chord** — Progressions are sequences of chords
## Enables
- Harmonic analysis (Roman numerals, jazz chord symbols, etc.)
## Related
- **voice-leading-vs-chord-progression** — The fundamental distinction
- **optic-symmetries** — Chord progressions arise from individual OPC
## Contrasts With
- **voice-leading-in-pitch-space** — Voice leadings specify how voices move
- **voice-leading-in-pitch-class-space** — Voice leadings specify voice paths

# Common Errors
- **Error**: Assuming a chord progression implies a specific voice leading
  **Correction**: Many different voice leadings can realize the same chord progression

# Common Confusions
- **Confusion**: Confusing chord progressions with voice leadings
  **Clarification**: "C major to F major" is a chord progression; "(C, E, G) -> (C, F, A)" with specific voice mappings is a voice leading

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.5, pages 62-63.

# Verification Notes
- Definition source: Direct from Section 2.5 with formal characterization
- Confidence rationale: High — precisely defined and contrasted with voice leadings
- Cross-reference status: Verified; fundamental distinction throughout the book
