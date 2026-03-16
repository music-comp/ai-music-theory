---
concept: Combining Scalar and Chromatic Transposition
slug: combining-scalar-chromatic-transposition

category: voice-leading
subcategory: analysis
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 168
section: "4.10"

extraction_confidence: high

aliases:
  - "decomposition into scalar and chromatic"
  - "scalar-chromatic cancellation"

prerequisites:
  - interscalar-transposition
  - scalar-transposition
extends: []
related:
  - decomposition-into-parallel-contrary
  - harmonic-consistency-and-efficient-voice-leading
contrasts_with: []

answers_questions:
  - "How can efficient voice leadings be decomposed into scalar and chromatic components?"
  - "What does a composer 'know' when she knows all the efficient voice leadings between chord types?"
---

# Quick Definition
Any strongly crossing-free voice leading between chords of the same type can be decomposed into a scalar (or interscalar) transposition and a chromatic transposition that nearly cancel each other out. Efficient voice leadings occur precisely when these two components counterbalance.

# Core Definition
Section 4.10 shows that any strongly crossing-free voice leading from (say) C major to E major can be decomposed into: (1) a scalar transposition that moves each note down by some number of scale steps within the C major triad, and (2) a chromatic transposition that moves each note up by some number of semitones. The voice leading is efficient when these two components nearly cancel — for example, a one-step descending scalar transposition nearly neutralizes a four-semitone ascending chromatic transposition, leaving each voice close to where it started. This decomposition provides a surprising answer to the question "what does a composer know when she knows all efficient voice leadings between chord types?" Answer: she knows how to combine interscalar and chromatic transpositions. This reduces a seemingly complex skill to the combination of two simple operations.

# Prerequisites
- Interscalar transposition
- Scalar transposition (within a chord treated as a small scale)

# Key Properties
1. Every crossing-free voice leading = interscalar transposition + chromatic transposition
2. Efficiency = the two components nearly cancel out
3. As chromatic transposition increases, the counterbalancing scalar transposition increases
4. For n-note chords, only n interscalar templates exist; these combine with chromatic transpositions to generate all efficient voice leadings
5. Analogous to the decomposition into parallel and contrary motion (Section 3.7)

# Construction / Recognition
## To Decompose:
1. Identify the interscalar transposition component (which scale degree maps to which)
2. Identify the chromatic transposition component (how many semitones of uniform shift)
3. The efficient voice leadings are those where these components nearly cancel

# Context & Application
This is the culminating theoretical result of Part I. It explains how composers intuitively manage the search for efficient voice leadings: they learn a small number of interscalar templates and combine them with chromatic transpositions. The result is used extensively in Chapter 8 (analyzing Wagner's Tristan resolutions) and Chapter 10 (jazz voice leading). The parallel with the decomposition into parallel and contrary motion (Section 3.7) reveals a deep structural analogy between scalar and geometric perspectives.

# Examples
**Example 1** (p. 168-169): The minimal voice leading between C and E major triads combines a one-step descending scalar transposition with a four-semitone ascending chromatic transposition (Figure 4.10.1).
**Example 2** (p. 170): The four interscalar transpositions from C half-diminished to C dominant seventh combine with chromatic transpositions to generate all efficient voice leadings between half-diminished and dominant seventh chords (Figure 4.10.2).
**Example 3** (p. 171): The first four resolutions of the half-diminished seventh in Wagner's Tristan prelude: the first two use root-to-root interscalar transposition, the last two use root-to-third (Figure 4.10.3).

# Relationships
## Builds Upon
- **interscalar-transposition** — One component of the decomposition
- **scalar-transposition** — Treated as a special case of interscalar transposition
## Related
- **decomposition-into-parallel-contrary** — Analogous decomposition in geometric terms
- **harmonic-consistency-and-efficient-voice-leading** — This decomposition explains how the two constraints are satisfied

# Common Errors
- **Error**: Thinking composers consciously perform this decomposition
  **Correction**: The decomposition describes the structure of the knowledge composers possess, not necessarily their conscious reasoning process

# Common Confusions
- **Confusion**: How is this different from the parallel/contrary decomposition?
  **Clarification**: The parallel/contrary decomposition is geometric (horizontal + vertical in chord space). The scalar/chromatic decomposition is algebraic (scale-step motion + semitone motion). They are analogous but capture different aspects.

# Source Reference
Chapter 4: Scales, Section 4.10, pages 168-172.

# Verification Notes
- Definition source: Directly from Section 4.10
- Confidence rationale: High — the culminating theoretical synthesis of Part I
- Cross-reference status: Verified against Wagner Tristan example and the broader claims
