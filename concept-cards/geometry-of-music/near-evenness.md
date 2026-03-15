---
# === CORE IDENTIFICATION ===
concept: Near Evenness
slug: near-evenness

# === CLASSIFICATION ===
category: geometric-theory
subcategory: chord-properties
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.10"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "nearly even chord"
  - "approximately even division"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class-space
  - near-symmetry
extends:
  - near-symmetry
related:
  - acoustic-consonance
  - overdetermination-principle
  - efficient-voice-leading
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is near evenness?"
  - "How do acoustic consonance and efficient voice leading relate?"
  - "How do the five components constrain each other?"
---

# Quick Definition
A chord is nearly even when it divides the pitch-class circle into arcs of approximately equal size, placing it close to a perfectly even (and hence transpositionally symmetrical) chord.

# Core Definition
Near evenness is a special case of near-transpositional symmetry. A chord that divides the pitch-class circle nearly evenly is close to a chord that divides it perfectly evenly (which is transpositionally symmetrical). By the general near-symmetry argument, such chords can be connected to their transpositions by efficient voice leading. The critical insight is that acoustic consonance implies near evenness: the most consonant chords (whose notes have frequencies in simple ratios) divide the frequency-space octave evenly, which maps to nearly even division of the pitch-space octave under the logarithmic transformation. This is the bridge connecting acoustic consonance to efficient voice leading.

# Prerequisites
- **pitch-class-space** — Near evenness is defined on the pitch-class circle
- **near-symmetry** — Near evenness is a special case

# Key Properties
1. Nearly even chords divide the circle into approximately equal arcs
2. Special case of near-transpositional symmetry
3. Consonant chords (major triad, dominant seventh, etc.) are nearly even
4. Consonance implies near evenness (via Fourier analysis of harmonic sounds)
5. Near evenness implies efficient voice leading to transpositions
6. Therefore: consonance implies efficient voice leading

# Construction / Recognition
## To Construct/Create:
1. Divide the pitch-class circle into n approximately equal arcs
2. Place notes at the arc boundaries
## To Identify/Recognize:
1. Compute arc lengths between adjacent notes on the circle
2. If the arcs are approximately equal (differ by at most 1-2 semitones), the chord is nearly even

# Context & Application
Near evenness connects the two pillars of Western tonal music: consonance and counterpoint. Figure 2.10.3 lists the most consonant chords of various sizes — perfect fifth, major/minor triads, dominant seventh, pentatonic scale, diatonic scale — and all of them divide the octave nearly evenly. This is the mathematical foundation for the "overdetermination principle" (the remarkable coincidence that consonant chords are also the best chords for counterpoint).

# Examples
**Example 1** (p. 80-81, Fig 2.10.2): The major triad in frequency space {330, 440, 550} divides the frequency-space octave perfectly evenly. In pitch space, this becomes a nearly even division of approximately 5, 4, and 3 semitones.

**Example 2** (p. 81, Fig 2.10.3): Table of nearly even consonant chords of various sizes: {C, G} (perfect fifth), {C, E, G} (major triad), {C, E, G, Bb} (dominant seventh), up to {C, D, E, F, G, A, B} (diatonic scale).

# Relationships
## Builds Upon
- **near-symmetry** — Near evenness is a special case of near-T-symmetry
- **pitch-class-space** — Defined on the pitch-class circle
## Enables
- **overdetermination-principle** — Consonance implies near evenness implies efficient voice leading
## Related
- **acoustic-consonance** — Consonance implies near evenness
- **efficient-voice-leading** — Near evenness enables efficient voice leading
## Contrasts With
- No direct contrast within this source

# Common Errors
- **Error**: Thinking near evenness IS consonance
  **Correction**: Near evenness is a necessary consequence of consonance for small chords, but not the same thing. Some nearly even chords are not particularly consonant.

# Common Confusions
- **Confusion**: Thinking equal temperament is required for near evenness
  **Clarification**: Near evenness is defined in continuous pitch-class space and applies to any tuning system

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.10, pages 79-82.

# Verification Notes
- Definition source: Direct from Section 2.10
- Confidence rationale: High — precisely defined with mathematical argument
- Cross-reference status: Verified; used throughout the book
