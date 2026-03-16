---
concept: Nearly Even Chords
slug: nearly-even-chords

category: geometric-theory
subcategory: chord-structure
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 98
section: "3.6, 3.9-3.10"

extraction_confidence: high

aliases:
  - "approximately even chords"
  - "near interval cycles"
  - "maximally even chords"

prerequisites:
  - two-note-chord-space
  - harmonic-consistency-and-efficient-voice-leading
extends: []
related:
  - even-vs-uneven-dyads
  - clustered-chords
  - three-note-chord-space
  - voice-leading-lattices
  - goldilocks-principle
contrasts_with:
  - clustered-chords

answers_questions:
  - "What are 'nearly even' chords and why are they important?"
  - "Why do major and minor triads permit efficient voice leading?"
  - "What is the connection between chord evenness and the geometry of chord space?"
---

# Quick Definition
Chords that divide the octave approximately (but not perfectly) equally. They lie near the center of chord space and can be connected to their transpositions by efficient voice leading. Major triads, minor triads, dominant seventh chords, and diatonic scales are all nearly even.

# Core Definition
A nearly even chord is one that divides the pitch-class circle into approximately equal parts. In chord space, nearly even chords occupy positions near the center — close to the perfectly even chords (augmented triads, diminished sevenths, whole-tone scales) that lie exactly at the center. This central position is geometrically significant: it means that nearly even chords can be connected to certain transpositions by short, primarily contrary voice leadings that cross through the center of the space. The specific transpositions available depend on the chord's cardinality: nearly even two-note chords connect efficiently to tritone-related transpositions; three-note chords to major-third-related transpositions; four-note chords to minor-third-related and tritone-related transpositions. Nearly even chords are central to the book's thesis because they are overdetermined — independently favored for acoustic consonance, efficient voice leading, and (when used as scales) moderate variation under scalar transposition.

# Prerequisites
- Chord space geometry and the relationship between position and evenness
- Harmonic consistency and efficient voice leading as simultaneous constraints

# Key Properties
1. Divide the octave approximately equally
2. Lie near the center of chord space
3. Can be connected to certain transpositions by efficient (small) voice leading
4. Include the most familiar chords of Western music: major/minor triads, seventh chords
5. Are "near interval cycles" — circular sequences where all but one interval is the same
6. Perfectly even chords (augmented triads, diminished sevenths) are the limiting case
7. Slight unevenness is musically preferable — it allows for non-parallel efficient voice leading

# Construction / Recognition
## To Construct:
1. Divide the octave (12 semitones) into n approximately equal parts
2. Allow one interval to differ by one semitone from the others
3. Example: a major triad divides 12 into parts of 4+3+5, close to the perfectly even 4+4+4
## To Recognize:
1. The chord's intervals are all similar in size
2. The chord lies near the center of the appropriate chord space
3. The chord can be connected by small voice leading to transpositions at specific intervals

# Context & Application
Nearly even chords are arguably the most important concept in the book. They explain why Western music gravitates toward particular chord types: major and minor triads (nearly even 3-note chords), dominant and half-diminished sevenths (nearly even 4-note chords), and diatonic and pentatonic scales (nearly even 7- and 5-note chords) all share the property of near-evenness. This single geometric property accounts for their ability to support harmonic consistency combined with efficient voice leading — the fundamental contrapuntal challenge of tonal composition. The concept connects to the Goldilocks Principle: perfectly even chords produce only parallel motion; a little unevenness allows for independent voice motion.

# Examples
**Example 1** (p. 98): Perfect fifths (interval 7, close to the tritone's 6) are nearly even 2-note chords that connect to tritone-related transpositions by semitonal voice leading.
**Example 2** (p. 103-105): Major and minor triads are nearly even 3-note chords represented near the center of three-note chord space (Figure 3.8.2), where they can be linked to major-third-related transpositions by efficient voice leading.
**Example 3** (p. 119-120): Figure 3.10.8 shows that nearly even chords of every cardinality can be linked to their perfect-fourth transpositions by efficient voice leading.

# Relationships
## Builds Upon
- **two-note-chord-space** — Where near-evenness is first geometrically demonstrated
- **harmonic-consistency-and-efficient-voice-leading** — The problem that near-evenness solves
## Enables
- **voice-leading-lattices** — Lattices representing the voice-leading connections among nearly even chords
- **three-note-chord-space** — Where triads appear near the center
## Related
- **goldilocks-principle** — Near-evenness is the "just right" condition
- **even-vs-uneven-dyads** — The two-note version
## Contrasts With
- **clustered-chords** — The opposite extreme, also permitting efficient voice leading

# Common Errors
- **Error**: Equating "nearly even" with "perfectly even"
  **Correction**: Perfectly even chords (augmented triads, whole-tone scales) can only be connected to transpositions by parallel motion. Near-evenness introduces the slight asymmetry needed for independent voice motion.

# Common Confusions
- **Confusion**: Thinking near-evenness is only about consonance
  **Clarification**: Near-evenness is primarily a geometric property determining voice-leading efficiency. It correlates with consonance but is conceptually independent — the geometry, not the acoustics, explains the prevalence of these chords in contrapuntal music.

# Source Reference
Chapter 3: A Geometry of Chords, Sections 3.6 and 3.9-3.10, pages 98-99, 109-121.

# Verification Notes
- Definition source: Synthesized from Sections 3.6, 3.9, and 3.10
- Confidence rationale: High — the book's central thesis, discussed extensively
- Cross-reference status: Verified across multiple sections and against the Goldilocks Principle in Chapter 4
