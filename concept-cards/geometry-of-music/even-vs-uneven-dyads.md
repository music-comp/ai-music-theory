---
# === CORE IDENTIFICATION ===
concept: Even vs Uneven Dyads
slug: even-vs-uneven-dyads

# === CLASSIFICATION ===
category: geometric-theory
subcategory: chord-structure
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 98
section: "3.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "chord evenness spectrum"
  - "even and uneven intervals"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
  - harmonic-consistency-and-efficient-voice-leading
extends: []
related:
  - nearly-even-chords
  - clustered-chords
  - tritone-as-midpoint
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does a dyad's evenness relate to its position in chord space?"
  - "Why can both very even and very uneven dyads be linked efficiently?"
---

# Quick Definition
In two-note chord space, even dyads (those that divide the octave nearly equally, like tritones and perfect fifths) lie near the center, while uneven dyads (like minor seconds) lie near the boundary. Both extremes permit efficient non-parallel voice leading, but for different geometric reasons.

# Core Definition
The evenness of a dyad — how equally it divides the octave — determines its vertical position in two-note chord space. Tritones (perfectly even, dividing the octave into two equal halves) lie exactly at the center. Perfect fifths and fourths (nearly even) lie close to the center. Minor and major seconds (very uneven) lie close to the mirror boundary. Even dyads reach their tritone transpositions efficiently by crossing the center of the strip (exploiting the twist), while uneven dyads reach nearby transpositions by bouncing off the mirror boundary. Dyads of intermediate evenness (like major thirds) have limited efficient voice-leading options.

# Prerequisites
- Two-note chord space layout
- Harmonic consistency and efficient voice leading

# Key Properties
1. Evenness = how equally the dyad divides the octave
2. Even dyads are near the center; uneven dyads are near the boundary
3. Even dyads link efficiently to tritone-related transpositions (via the twist)
4. Uneven dyads link efficiently to nearby transpositions (via mirror bounce)
5. Intermediate dyads have fewer efficient options
6. The tritone is perfectly even; the unison is perfectly uneven

# Construction / Recognition
## To Determine Evenness:
1. Measure the interval between the two pitch classes
2. Compare to 6 semitones (the tritone = perfect evenness)
3. Closer to 6 = more even; closer to 0 = more uneven

# Context & Application
The even/uneven distinction is the two-dimensional version of a principle that generalizes to all chord sizes. In higher dimensions, nearly even chords (major/minor triads, dominant sevenths) occupy positions near the center of chord space, while clustered chords are near the boundary. This positioning is what makes nearly even chords so useful in tonal music.

# Examples
**Example 1** (p. 98): Perfect fifths (nearly even) can be linked to their tritone transpositions by semitonal voice leading — a short vertical arrow crossing the center of the strip.
**Example 2** (p. 98): Major thirds lie farther from the center and can be linked only to their tritone transpositions by stepwise voice leading.
**Example 3** (p. 98-99): Minor seconds (very uneven) link to themselves by contrary motion bouncing off the mirror boundary.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space where evenness determines position
## Enables
- **nearly-even-chords** — The generalization to higher dimensions
## Related
- **tritone-as-midpoint** — The tritone as the perfectly even dyad at the center
- **clustered-chords** — The extremely uneven end of the spectrum

# Common Errors
- **Error**: Thinking only even dyads permit efficient voice leading
  **Correction**: Very uneven (clustered) dyads also permit efficient voice leading, but via a different mechanism (mirror bounce rather than center crossing)

# Common Confusions
- **Confusion**: Confusing "evenness" with "consonance"
  **Clarification**: Evenness measures how equally a chord divides the octave; consonance is an acoustic/perceptual property. They are related but not identical — the tritone is perfectly even but not traditionally consonant.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.6, pages 98-99.

# Verification Notes
- Definition source: From Section 3.6 and Figure 3.6.1
- Confidence rationale: High — directly described in geometric terms
- Cross-reference status: Verified against the higher-dimensional generalization in Section 3.10
