---
concept: SLIDE Transformation
slug: slide-transformation

category: transformation-theory
subcategory: klang-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.1"

extraction_confidence: high

aliases: []

prerequisites:
  - klang-representation
extends: []
related:
  - lt-transformation
  - par-transformation
contrasts_with:
  - lt-transformation

answers_questions:
  - "How do I apply SLIDE to Klangs?"
---

# Quick Definition
A Klang transformation that preserves the third of a triad while changing its mode: (F, +)SLIDE = (F#, -) and (F#, -)SLIDE = (F, +), keeping pitch class A as the common tone. SLIDE is an involution.

# Core Definition
"We can also define a more exotic operation SLIDE that preserves the third of a triad while changing its mode: (F, +)SLIDE = (F#, -); (F#, -)SLIDE = (F, +)" (Lewin, 8.1.1, p. 177). The preserved element is the third of both triads: F major has third A, and F# minor has third A.

# Prerequisites
- **Klang representation** — SLIDE operates on Klangs

# Key Properties
1. (p, +)SLIDE = (p + 1, -) mod 12
2. (p, -)SLIDE = (p + 11, +) mod 12
3. SLIDE^2 = identity (involution)
4. The third of the triad is the common tone
5. Mode changes while root moves by semitone

# Construction / Recognition
## To Construct:
1. For major Klang (p, +): move root up a semitone, change to minor
2. For minor Klang (p, -): move root down a semitone, change to major
## To Recognize:
1. Two triads sharing only the third as common tone
2. Mode change with root motion by semitone

# Context & Application
SLIDE represents a chromatic, "exotic" relationship that appears in nineteenth-century practice. Lewin cites two examples from the repertoire.

# Examples
**Example 1** (p. 177): Beethoven's Eighth Symphony, last movement, mm. 379-91: "The F-major theme that begins on the note A, the third of the triad, is transformed... into F# minor, where it begins on the same A; the theme slides back into F major at measure 392."

**Example 2** (p. 177): Schubert's posthumous Bb-Major Piano Sonata, slow movement, mm. 103-110: "thematic material which we expect to hear in C# minor is presented in C major instead." SLIDE between (C, +) and (C#, -).

# Relationships
## Builds Upon
- **Klang representation** — SLIDE is defined on Klangs
## Related
- **LT transformation** — Both are involutory and change mode
- **PAR transformation** — Also changes mode; PAR preserves root, SLIDE preserves third
## Contrasts With
- **LT transformation** — LT preserves two common tones; SLIDE preserves only the third

# Common Errors
- **Error**: Thinking SLIDE preserves the root or fifth
  **Correction**: SLIDE preserves the third specifically; root moves by semitone

# Common Confusions
- **Confusion**: Assuming SLIDE is common in Classical diatonic harmony
  **Clarification**: SLIDE is a chromatic relationship more typical of Romantic practice

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, page 177.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicitly defined with repertoire examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Beethoven and Schubert examples
