---
# === CORE IDENTIFICATION ===
concept: Three-Note Chord Space
slug: three-note-chord-space

# === CLASSIFICATION ===
category: geometric-theory
subcategory: chord-spaces
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 103
section: "3.8"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "three-dimensional chord space"
  - "triangular prism chord space"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
  - mobius-strip-topology
extends:
  - two-note-chord-space
related:
  - nearly-even-chords
  - cross-sections-of-chord-space
  - higher-dimensional-chord-spaces
  - voice-leading-lattices
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the geometry of three-note chord space?"
  - "Where do major and minor triads lie in this space?"
  - "How does the three-note space generalize the Mobius strip?"
---

# Quick Definition
The space of all three-note chords is a three-dimensional triangular prism with a 120-degree twist, where augmented triads lie at the center, major and minor triads nearby, triple unisons on the edges, and voice leadings are generalized line segments in 3D.

# Core Definition
Three-note chord space is the three-dimensional analogue of the two-note Mobius strip. It is formed by taking a "tile" of three-dimensional ordered pitch space and identifying equivalent points. The result is a triangular prism whose top and bottom triangular faces are glued together with a 120-degree twist (analogous to the 180-degree twist of the Mobius strip). Augmented triads, which divide the octave into three perfectly equal parts, lie on the vertical line at the center. Major and minor triads (nearly even) are found near the center. Chords with duplicated notes are on the faces of the prism, and triple unisons (maximally uneven) are on the edges. Ascending parallel motion corresponds to vertical motion; the triangular faces are glued with a one-third turn. Horizontal cross sections are triangles containing chords that sum to the same value, with 120-degree rotation corresponding to major-third transposition.

# Prerequisites
- Two-note chord space and its Mobius strip topology
- The principle of folding periodic space to create quotient spaces

# Key Properties
1. Three-dimensional, shaped as a triangular prism
2. Top and bottom faces glued with 120-degree twist
3. Augmented triads at the exact center (perfectly even)
4. Major/minor triads near the center (nearly even)
5. Doubled notes on the faces; triple unisons on the edges
6. Horizontal cross sections are triangles (containing same-sum chords)
7. The space is continuous, containing all possible three-note chords including microtonal ones
8. Lines at the center form a lattice connecting major, minor, and augmented triads by single-semitone voice leading

# Construction / Recognition
## To Construct:
1. Use three dimensions, one per voice
2. Find the repeating tile of the wallpaper pattern
3. Glue the triangular faces with a 120-degree twist
## To Navigate:
1. Vertical motion = parallel motion in all three voices
2. Horizontal cross sections = pure contrary motion (sum preserved)
3. Reaching the top face means reappearing at the bottom, rotated 120 degrees

# Context & Application
Three-note chord space is where the book's geometric analysis begins to have direct relevance to Western tonal music, since triads are three-note chords. The cubic lattice at the center of this space (connecting augmented triads to major and minor triads by single-semitone voice leading) was first discovered by Douthett and Steinbach (1998) and is central to the analysis of chromatic harmony in Chapters 6 and 8. The space reveals why major-third-related triads can be connected by efficient voice leading, just as the Mobius strip revealed the efficiency of tritone-related dyads.

# Examples
**Example 1** (p. 104): Figure 3.8.2 depicts the prism, with augmented triads as cubes at the center, major triads as dark spheres, and minor triads as light spheres.
**Example 2** (p. 105-106): Four chromatic sequences from Brahms' C minor Piano Quartet traced on the central lattice, revealing that Brahms systematically explores 4 of the 6 possible descending semitonal voice-leading paths (Figure 3.8.3).
**Example 3** (p. 104): Ascending in parallel from {C,C,C} traces a path up the left edge, disappearing from the top face and reappearing on the bottom rotated 120 degrees.

# Relationships
## Builds Upon
- **two-note-chord-space** — The lower-dimensional analogue
- **mobius-strip-topology** — Generalized to 120-degree twist
## Enables
- **voice-leading-lattices** — The cubic lattice at the center
- **near-symmetry** — The slight asymmetry of triads near perfectly even augmented triads
## Related
- **nearly-even-chords** — Major/minor triads as nearly even three-note chords
- **cross-sections-of-chord-space** — Triangular cross sections
- **higher-dimensional-chord-spaces** — Further generalizations

# Common Errors
- **Error**: Thinking the 3D prism can be fully understood by analogy with the 2D Mobius strip
  **Correction**: While the principles are analogous, the 120-degree twist (vs. 180-degree) and the triangular cross sections introduce genuinely new geometric features

# Common Confusions
- **Confusion**: Why do major-third-related triads appear in the same cross section?
  **Clarification**: Transposing a three-note chord by 4 semitones adds 4+4+4=12=0 to the pitch-class sum, so the sum is unchanged. This is specific to 3-note chords and the major third.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.8, pages 103-109.

# Verification Notes
- Definition source: Directly from Section 3.8
- Confidence rationale: High — extensively described with multiple figures
- Cross-reference status: Verified against the lattice structures in Section 3.11
