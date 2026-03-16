---
concept: Spiral Diagram Derivation
slug: spiral-diagram-derivation

category: fundamentals
subcategory: geometric-models
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 2: Deriving the Spiral Diagrams"
chapter_number: null
pdf_page: 545
section: null

extraction_confidence: high

aliases:
  - helical derivation
  - chord-space subspace derivation

prerequisites:
  - voice-leading-geometry
  - double-transposition
extends:
  - scalar-spiral-diagram
related:
  - spiral-construction-recipe
  - quadruple-hierarchy
  - collectional-hierarchy-as-synthesis
contrasts_with: []

answers_questions:
  - "How do you construct a spiral diagram for a given scale?"
  - "What are the two mathematical derivations of spiral diagrams?"
  - "Why do spiral diagrams come in three flavors?"
---

# Quick Definition

The two mathematical methods for constructing spiral diagrams: as the union of interleaved helices (one per chord mode) viewed from above, or as subspaces (line segments) within the higher-dimensional geometrical space containing all possible n-note chords.

# Core Definition

**Method 1 (union of helices):** Starting from an n-note chord in an o-note scale, chromatic transposition traces a helix in a three-dimensional space where the vertical axis represents pitch sum ("center of gravity"). Transposition along the chord (t1) adds o to the pitch sum, creating a new interleaved helix. For an n-note chord, n interleaved helices represent the n modes. "Viewed from above," these produce the spiral diagram (p. 545).

**Method 2 (subspace of chord space):** The spiral diagrams are line segments in higher-dimensional chord space, determined by equations that define a region containing one point per chord (pp. 547-550). Points on the left boundary relate by t1 to points on the right; "moving off the right boundary requires us to apply t1 to every point in the space" (p. 550). Three flavors result: when n divides o (diagonal voice leading), when n is coprime to o (basic voice leading), and mixed cases (p. 553).

# Prerequisites

- **Voice-leading geometry** -- The higher-dimensional spaces within which spiral diagrams are subspaces
- **Double transposition** -- The combination of big-T and little-t transposition that spiral diagrams represent

# Key Properties

1. Spiral diagrams represent all possible combinations of big-T and little-t transposition for any chord in any scale
2. An n-note chord produces n interleaved helices (one per mode)
3. Transposition by octave adds o*n to the pitch sum; transposition along the chord adds o
4. The diagrams are topological (like subway maps) rather than metric (like topographical maps)
5. Apparent self-intersections are artifacts of projection from higher dimensions
6. Three flavors: n divides o (diagonal voice leading), n coprime to o (basic voice leading), mixed cases

# Construction / Recognition

## To Derive a Spiral Diagram (Helix Method):
1. Choose a chord (n notes) in a scale (o notes)
2. Place the chord in 3D space with vertical axis = pitch sum
3. Trace chromatic transposition to form a helix returning after one octave per voice
4. Apply t1 to create interleaved helices for each mode
5. View from above to project the spiral diagram onto 2D
6. Replace "teleportation" between helices with continuous loops

## To Derive from Chord Space:
1. Construct the n-dimensional space of all n-note chords
2. Identify the region containing one representative per chord (using the equations in Figure A2.9)
3. Locate the line segments containing all transpositions of the chosen chord type
4. Note that left-boundary points relate by t1 to right-boundary points
5. Connect segments to form the spiral diagram

# Context & Application

The spiral diagrams are the book's central geometrical tool. They reveal how hierarchically nested transpositions combine, which combinations nearly counteract each other (producing efficient voice leading), and why enharmonic respelling is necessary when modulatory schemas are repeated to return to their starting key. The derivation shows that "geometrical boundaries can be identified with specific musical transformations" (p. 548), connecting geometry to musically significant operations.

# Examples

**Example 1** (pp. 545-546): 2-in-12 chromatic diagram -- double helix of perfect fifths and fourths, with t1 adding 12 to pitch sum; (C4, G4) sums to 127, while (G4, C5) sums to 139.

**Example 2** (pp. 549-550): Two-note chord space derived as a rotated square, with spiral lines for major thirds (y = x + 4) and minor sixths (y = x + 8).

**Example 3** (p. 553): Three flavors illustrated -- 3-in-12 (coprime, basic voice leading), 4-in-12 (divisible, diagonal voice leading), and general cases with both.

# Relationships

## Builds Upon
- **Voice-leading geometry** -- Spiral diagrams are subspaces of the higher-dimensional chord spaces
- **Double transposition** -- The fundamental operation represented by the diagrams

## Enables
- **Spiral construction recipe** -- Practical application of the derivation
- **Collectional hierarchy as synthesis** -- The spiral diagrams are the key tool for this synthesis

## Related
- **Quadruple hierarchy** -- The hierarchical structure that spiral diagrams visualize
- **Scalar spiral diagram** -- Application to specific scales

## Contrasts With
- None listed

# Common Errors

- **Error**: Treating distances on the spiral diagram as proportional to voice-leading distance
  **Correction**: The diagrams are topological, not metric; nearly even chords produce small radial distances, but exact sizes depend on intervallic structure

- **Error**: Assuming the spiral curves actually intersect
  **Correction**: Self-intersection is an artifact of 2D projection; the helices never intersect in the full 3D (or higher-dimensional) space

# Common Confusions

- **Confusion**: Thinking there is only one kind of spiral diagram
  **Clarification**: Three flavors exist depending on whether n divides o, is coprime to o, or shares a common factor

- **Confusion**: Confusing the two derivation methods as competing
  **Clarification**: Both methods produce the same diagrams; the helix method is more intuitive, the chord-space method connects to the formal geometry

# Source Reference

Appendix 2: "Deriving the Spiral Diagrams," pp. 545-554. Key figures: A2.1-A2.12.

# Verification Notes

- Definition source: Direct from pp. 545-553; both methods explicitly presented
- Confidence rationale: HIGH -- mathematical derivations presented with full detail
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: the three-flavor classification, the pitch-sum calculations, the topological vs. metric distinction
