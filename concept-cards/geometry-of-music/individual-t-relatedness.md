---
concept: Individual T-Relatedness
slug: individual-t-relatedness

category: voice-leading
subcategory: equivalence
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 102
section: "3.7"

extraction_confidence: high

aliases:
  - "individually T-related voice leadings"
  - "individual transposition equivalence"

prerequisites:
  - decomposition-into-parallel-contrary
  - cross-sections-of-chord-space
extends: []
related:
  - voice-leading-lattices
contrasts_with: []

answers_questions:
  - "When are two voice leadings 'individually T-related'?"
  - "What geometric operation relates individually T-related voice leadings?"
---

# Quick Definition
Two voice leadings are individually T-related when one can be obtained from the other by transposing individual voices by different amounts, preserving the relative motion among voices while changing only the absolute pitch level. They share the same purely contrary component in chord space.

# Core Definition
Individual T-relatedness (introduced in Chapter 2, Section 2.6) means that the same pattern of relative voice motions occurs between differently transposed chords. Geometrically, individually T-related voice leadings share the same purely contrary component but differ in their parallel component. In chord space, they project onto the same line segment within any vertical cross section. The operation of "individual transposition" alters the horizontal component of a voice leading while leaving the vertical component unchanged. This means that individually T-related voice leadings are essentially "the same" when viewed from the perspective of the cross section — they represent the same contrapuntal relationship, merely transposed to different pitch levels.

# Prerequisites
- Decomposition of voice leading into parallel and contrary components
- Cross sections of chord space

# Key Properties
1. Individually T-related voice leadings share the same contrary component
2. They differ only in their parallel (transpositional) component
3. They project onto the same line segment in any cross section
4. They represent the same basic contrapuntal relationship at different pitch levels

# Construction / Recognition
## To Identify:
1. Decompose two voice leadings into their contrary components
2. If the contrary components are identical, the voice leadings are individually T-related

# Context & Application
Individual T-relatedness is essential for the book's analytical method because it allows us to recognize the same contrapuntal pattern at different transpositional levels. When we restrict attention to a cross section of chord space, we are effectively grouping all individually T-related voice leadings together and studying their shared structure.

# Examples
**Example 1** (p. 102): (E, B) -> (F, Bb) and (G, D) -> (F#, B) are individually T-related: both move their voices by semitone in contrary motion, but at different pitch levels. Both project onto the same line segment (0,7) -> (0,5) in the cross section (Figure 3.7.4).

# Relationships
## Builds Upon
- **decomposition-into-parallel-contrary** — The decomposition that reveals the shared component
- **cross-sections-of-chord-space** — Where individually T-related voice leadings coincide
## Enables
- **voice-leading-lattices** — Lattice edges represent classes of individually T-related voice leadings

# Common Errors
- **Error**: Thinking individually T-related voice leadings must involve the same chord types
  **Correction**: They must involve the same *interval types* but not necessarily the same specific chords

# Common Confusions
- **Confusion**: Confusing individual T-relatedness with ordinary transposition
  **Clarification**: Ordinary transposition moves all voices by the same amount; individual transposition can move different voices by different amounts

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.7, pages 102-103.

# Verification Notes
- Definition source: From Section 3.7 and cross-referenced with Chapter 2 Section 2.6
- Confidence rationale: High — precisely defined with geometric interpretation
- Cross-reference status: Verified against Figure 3.7.4
