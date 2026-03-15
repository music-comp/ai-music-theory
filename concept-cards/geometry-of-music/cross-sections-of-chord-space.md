---
# === CORE IDENTIFICATION ===
concept: Cross Sections of Chord Space
slug: cross-sections-of-chord-space

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
pdf_page: 100
section: "3.7-3.8"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "vertical cross sections"
  - "horizontal slices"
  - "sum-class sections"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - decomposition-into-parallel-contrary
  - two-note-chord-space
extends: []
related:
  - three-note-chord-space
  - individual-t-relatedness
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are cross sections of chord space?"
  - "Why are cross sections useful for analyzing voice leading?"
  - "What is the relationship between cross sections and chord types?"
---

# Quick Definition
Vertical (in 2D) or horizontal (in 3D) slices through chord space that contain all chords whose pitch classes sum to the same value. Every chord type appears in every cross section, making them useful for studying the purely contrary component of voice leading.

# Core Definition
A cross section of chord space is a lower-dimensional slice containing chords whose pitch classes sum to a fixed value. In two-note chord space, cross sections are vertical line segments; in three-note chord space, they are horizontal triangles. Every cross section contains precisely the same set of chord *types* (transpositional set classes), though not all types appear in twelve-tone equal-tempered form in every section. Cross sections are musically significant because restricting attention to a cross section is equivalent to studying only the purely contrary component of voice leading, abstracting away from parallel motion (transposition). Line segments within a cross section represent the contrary components of individually T-related voice leadings. Labels like "01" for minor second, "04" for major third, etc., can be used to mark chord types abstractly within any cross section.

# Prerequisites
- Decomposition of voice leading into parallel and contrary components
- The concept that chords on a vertical/horizontal line sum to the same value

# Key Properties
1. Every cross section contains every chord type (transpositional set class)
2. Not all chord types appear in equal-tempered form in every cross section
3. Chords within a cross section can be linked by pure contrary motion
4. Cross sections are redundant: each chord type appears twice (e.g., 04 and 08 both represent major thirds)
5. Line segments in the cross section represent collections of individually T-related voice leadings
6. In 3D, triangular cross sections contain chords that are transpositionally related by major third (due to 4+4+4=0)

# Construction / Recognition
## To Construct (2D):
1. Choose a value for the pitch-class sum
2. Draw the vertical line through all dyads summing to that value
3. Label points by interval type (01, 02, ..., 06)
## To Construct (3D):
1. Choose a sum value
2. Draw the horizontal triangular slice
3. Chords related by major-third transposition (adding 4+4+4=0) appear at 120-degree rotations

# Context & Application
Cross sections provide a powerful tool for reducing the dimensionality of voice-leading analysis. Instead of working in the full n-dimensional chord space, analysts can often restrict attention to a cross section, which is one dimension lower and much easier to visualize. This technique is essential for studying three-note and larger chord spaces, where full visualization is impossible. The cross sections also connect to the abstract representation of chord-type space.

# Examples
**Example 1** (p. 100-101): In two-note space, the sum-0 cross section contains {C, C}, {C#, B}, {D, Bb}, {Eb, A}, {E, Ab}, {F, G}, {F#, F#} — all summing to 0 (Figure 3.7.2).
**Example 2** (p. 107-108): In three-note space, horizontal triangular slices contain chords summing to the same value, with 120-degree rotation corresponding to major-third transposition (Figure 3.8.4).
**Example 3** (p. 108): Pure contrary voice leadings between three-note chords — where ascending motion exactly balances descending — lie entirely within a triangular cross section.

# Relationships
## Builds Upon
- **decomposition-into-parallel-contrary** — Cross sections isolate the contrary component
- **two-note-chord-space** — The simplest space with cross sections
## Enables
- **individual-t-relatedness** — Voice leadings within a cross section are T-related
## Related
- **three-note-chord-space** — Where cross sections are triangles

# Common Errors
- **Error**: Assuming every equal-tempered chord type appears in every cross section
  **Correction**: While every chord *type* is represented, some types may appear only in non-equal-tempered forms in a particular cross section

# Common Confusions
- **Confusion**: Why the redundancy?
  **Clarification**: The cross section is redundant because the chord {04} (major third) can also be labeled {08} (its octave complement). Removing this redundancy produces a "set class" space, but with mathematical complications.

# Source Reference
Chapter 3: A Geometry of Chords, Sections 3.7-3.8, pages 100-109.

# Verification Notes
- Definition source: From Sections 3.7 and 3.8, especially the discussion of abstract cross-section labels
- Confidence rationale: High — detailed mathematical treatment with figures
- Cross-reference status: Verified across 2D and 3D cases
