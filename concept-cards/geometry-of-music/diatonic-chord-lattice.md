---
concept: Diatonic Chord Lattice
slug: diatonic-chord-lattice

category: geometric-theory
subcategory: chord-space
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Functional Harmony"
chapter_number: 7
pdf_page: 273
section: "7.5"

extraction_confidence: high

aliases:
  - "chord cube"
  - "diatonic three-note lattice"

prerequisites:
  - circle-of-thirds-harmonic
  - two-lattices
extends:
  - circle-of-thirds-harmonic
related:
  - figured-bass-pedagogy
contrasts_with: []

answers_questions:
  - "How are suspensions represented geometrically?"
  - "What three-note diatonic sonorities can resolve to a triad by stepwise descent?"
---

# Quick Definition
A three-dimensional cubic lattice containing all diatonic three-note sonorities that can resolve to a triad by one or two descending steps — triads on the central spine, suspension chords and incomplete seventh chords on surrounding paths.

# Core Definition
The diatonic chord lattice is produced by scrambling the voice leadings on the circle of thirds into a three-dimensional structure. The triadic circle of thirds runs through the center in a zigzag fashion (CEG-CEA-CFA-...). Nontriadic chords form a second circle winding around the first (CFG-DFG-DEG-DEA-..., repeating every three chords at the interval of a descending third). The lattice contains all three-note diatonic sonorities that can resolve to a triad by either a single or double suspension. This provides waystations allowing composers to break large melodic motions into smaller steps — instead of moving directly from one triad to another, composers can use nonharmonic tones to smooth out the journey.

# Prerequisites
- The circle of thirds and its voice-leading properties
- The structural analogy between chord and scale lattices

# Key Properties
1. Triads lie on the central spine of the lattice
2. Fourth chords and incomplete seventh chords occupy peripheral positions
3. Every nontriadic chord resolves to a triad by one or two descending steps
4. The lattice visualizes suspension patterns and their resolutions
5. It does not include diatonic clusters or multisets

# Construction / Recognition
## To Construct/Create:
1. Start with the circle of thirds: CEG-CEA-CFA-CFD-...
2. Between each pair of adjacent triads, scramble the voice leadings
3. This produces cubes containing four triads and four nontriadic chords
4. Stack the cubes to produce the full three-dimensional lattice

## To Identify/Recognize:
1. Any three-note diatonic sonority that is not a triad belongs to the lattice if it can resolve by stepwise descent
2. Suspensions and incomplete seventh chords occupy positions adjacent to their resolution triads

# Context & Application
The chord lattice provides a visual map for understanding suspension patterns. Four ways of using suspensions to decorate descending first-inversion triads can be read directly from the lattice as different paths through the cube. It connects to figured-bass pedagogy, since the right-hand gestures described in C. P. E. Bach's treatise correspond to specific movements on the lattice.

# Examples
**Example 1** (Fig. 7.5.5, p. 273): The full diatonic chord lattice with triadic and nontriadic circles identified.

**Example 2** (Fig. 7.5.8, p. 275): Four paths through the cube producing different suspension patterns over descending first-inversion triads: 7-6 suspension, interposed root position triads, interposed second-inversion triads, and double suspensions.

**Example 3** (Fig. 7.5.9, p. 275): Philidor's "Art of Modulation" alternating 7-6 and 4-3 suspensions (paths a and b), and Grieg's "From Holberg's Time" using double suspension (path d).

# Relationships
## Builds Upon
- **circle-of-thirds-harmonic** — The lattice extends the circle into three dimensions
- **two-lattices** — The chord lattice parallels the scale lattice
## Enables
- **figured-bass-pedagogy** — The lattice encodes the practical gestures taught in figured-bass tradition

# Common Errors
- **Error**: Thinking the lattice contains all possible three-note chords
  **Correction**: It contains only those that resolve to a triad by stepwise descent; clusters and multisets are excluded

# Common Confusions
- **Confusion**: Confusing the diatonic chord lattice with chromatic three-note chord space
  **Clarification**: The diatonic lattice contains only diatonic sonorities; chromatic chord space (Chapter 3) is continuous

# Source Reference
Chapter 7: Functional Harmony, Section 7.5, pages 273-276, Figures 7.5.4-7.5.9.

# Verification Notes
- Definition source: Directly from Section 7.5 with multiple figures
- Confidence rationale: High — explicitly constructed and applied to musical examples
- Cross-reference status: Verified against the scale lattice from Chapter 4
