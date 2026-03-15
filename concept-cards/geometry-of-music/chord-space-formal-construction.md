---
# === CORE IDENTIFICATION ===
concept: Chord Space Formal Construction
slug: chord-space-formal-construction

# === CLASSIFICATION ===
category: geometric-theory
subcategory: formal-mathematics
tier: advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Appendix B: Chord Geometry: A More Technical Look"
chapter_number: appendix-b
pdf_page: 419
section: "Appendix B"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "T^n/S_n orbifold"
  - "chord space orbifold"
  - "fundamental domain for chords"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ordered-pitch-space
extends:
  - ordered-pitch-space
related:
  - voice-leading-lattices
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is the space of chords formally constructed?"
  - "What mathematical structure does chord space have?"
  - "How do we move from ordered pitch space to unordered chord space?"
---

# Quick Definition
The formal mathematical construction of n-note chord space as a prism whose simplicial faces are glued with a twist and whose remaining boundaries act as mirrors, symbolized as the orbifold T^n/S_n (the n-torus modulo the symmetric group on n elements).

# Core Definition
A chord of n pitch classes is represented by a point in n-dimensional space, determined by two sets of inequalities: (1) x1 <= x2 <= ... <= xn <= x1 + 12 (pitches in nondescending order spanning at most an octave), and (2) 0 <= x1 + x2 + ... + xn < 12 (sum in range 0 to 12). This fundamental domain is a prism whose cross-sections are simplices. To convert it into a proper quotient space: the sum-zero face is glued to the sum-twelve face with a cyclic twist (corresponding to scalar transposition by one step), and boundaries containing pitch duplications act as mirrors (voice crossings reflect back). The resulting orbifold, T^n/S_n, is the space of unordered sets of n pitch classes -- "something very much like the space of chords as musicians ordinarily conceive of them."

# Prerequisites
- Understanding of ordered pitch space

# Key Properties
1. Cross-sections are simplices (simplest possible shape in each dimension)
2. Top and bottom faces glued with cyclic twist (transposition)
3. Interior boundaries act as mirrors (pitch duplications)
4. In 2D: Mobius strip; in 3D: triangular prism with twist
5. Each cross-section contains n copies of each chord type (one per mode)
6. Vertices of each cross-section correspond to scales with one pitch class
7. Any ordered pitch sequence can be mapped into the fundamental domain

# Construction / Recognition
## To Construct/Create:
1. Convert pitches to pitch-class numbers (0 <= x < 12)
2. Order from low to high
3. If sum >= 12, subtract 12 from last note and move to front
4. Repeat until sum < 12
5. Plot resulting point in the fundamental domain
## To Identify/Recognize:
1. Points represent chords; line segments represent voice leadings
2. Mirror boundaries correspond to pitch duplications
3. The twist connects opposite faces of the prism

# Context & Application
This appendix provides the rigorous mathematical foundation for the geometric models used throughout the book. Understanding this construction enables readers to "work with the spaces directly, either by hand or using a computer."

# Examples
**Example 1** (p. 420-421, Figs. B1-B2): Constructing pitch-class space (circle) and the algorithm for projecting into the fundamental domain.

**Example 2** (p. 427, Fig. B6): Vertices of the zero-sum cross section in two through five dimensions.

# Relationships
## Builds Upon
- **ordered-pitch-space** -- The starting space before quotient operations
## Enables
- All geometric analyses in the book
## Related
- **voice-leading-lattices** -- Discrete subgraphs of the continuous space

# Common Errors
- **Error**: Confusing the fundamental domain with the quotient space
  **Correction**: The fundamental domain is ordinary Euclidean space; it becomes a quotient space only when boundary identifications are specified

# Source Reference
Appendix B: Chord Geometry: A More Technical Look, pages 419-429.

# Verification Notes
- Definition source: Rigorous mathematical construction in Appendix B
- Confidence rationale: High -- formal derivation with complete proofs
- Cross-reference status: Provides the foundation for Chapter 3's geometric models
