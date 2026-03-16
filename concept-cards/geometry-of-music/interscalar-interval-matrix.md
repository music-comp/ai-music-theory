---
concept: Interscalar Interval Matrix
slug: interscalar-interval-matrix

category: geometric-theory
subcategory: formal-mathematics
tier: advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Appendix D: The Interscalar Interval Matrix"
chapter_number: null
pdf_page: 436
section: "Appendix D"

extraction_confidence: high

aliases:
  - "ISI matrix"
  - "scalar interval matrix"

prerequisites:
  - interscalar-transposition-twentieth-century
extends: []
related:
  - chord-space-formal-construction
contrasts_with: []

answers_questions:
  - "What is an interscalar interval matrix?"
  - "How can you find the most efficient voice leading between any two chord types?"
  - "How does the matrix relate to the geometry of chord space?"
---

# Quick Definition
A matrix representing all strongly crossing-free voice leadings between two chord types, where each row corresponds to a different interscalar transposition and any voice leading can be obtained by adding a constant (chromatic transposition) to some row.

# Core Definition
The interscalar interval matrix displays all possible interscalar transpositions between two chord types. Each row maps the notes of chord A to the notes of chord B in a different way (root to root, root to third, root to fifth, etc.), with the numbers showing how far each voice moves. Any strongly crossing-free voice leading between the two chord types can be obtained by adding a constant to some row -- combining interscalar transposition with chromatic transposition. For nearly even chords, the rows are close to constant values, meaning interscalar and chromatic transpositions nearly cancel, producing efficient voice leadings to all transpositions. The matrix thus provides a "road map" of chord space, displaying the most commonly traveled routes between any two points.

# Prerequisites
- Understanding of interscalar transposition

# Key Properties
1. Rows correspond to different interscalar transpositions
2. Adding a constant to a row combines interscalar + chromatic transposition
3. Every strongly crossing-free voice leading can be derived from the matrix
4. Near-even chords have rows close to constant values (enabling efficient voice leading everywhere)
5. The interscalar transposition = pure contrary component of a crossing-free voice leading
6. The chromatic transposition = pure parallel component
7. Provides practical computation without visualizing higher-dimensional geometry

# Construction / Recognition
## To Construct/Create:
1. Arrange both chords in ascending order spanning at most an octave
2. Map lowest note of A to lowest of B; record voice-leading distances (first row)
3. Transpose B upward by one step (move lowest note to top + octave)
4. Repeat mapping to get second row
5. Continue until B returns to original position
## To Identify/Recognize:
1. Look for a table where rows show voice-leading distances and adding constants produces new voice leadings

# Context & Application
The matrix provides a more intuitive alternative to higher-dimensional geometry for practical voice-leading computation, usable "by hand or with a computer." It is equivalent to the geometric approach but uses musical rather than purely geometric language.

# Examples
**Example 1** (p. 436, Fig. D1): The interscalar interval matrix from half-diminished to dominant seventh, with rows {0,1,1,0}, {4,4,4,2}, {7,7,6,6}, {10,9,10,9}.

**Example 2** (p. 439-440, Figs. D3-D4): The matrix from C diatonic to C acoustic scale, and the scalar interval matrix for the dominant seventh chord.

# Relationships
## Builds Upon
- **interscalar-transposition-twentieth-century** -- The musical concept formalized here
## Enables
- Practical computation of efficient voice leadings between any chord types
## Related
- **chord-space-formal-construction** -- The matrix is a "road map" of the continuous space

# Common Errors
- **Error**: Thinking the matrix depends on arbitrary choices of numbering or registration
  **Correction**: Different choices produce equivalent matrices (rows differ by constants of 12)

# Source Reference
Appendix D: The Interscalar Interval Matrix, pages 436-441.

# Verification Notes
- Definition source: Formally constructed with algorithm and multiple examples
- Confidence rationale: High -- complete mathematical treatment with worked examples
- Cross-reference status: Used in Chapter 9 analyses and connects to Chapter 3 geometry
