---
concept: Inversion Symmetry (I)
slug: inversion-symmetry

category: geometric-theory
subcategory: symmetry
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.4"

extraction_confidence: high

aliases:
  - "I symmetry"
  - "inversional equivalence"

prerequisites:
  - basic-musical-object
  - inversion
extends:
  - inversion
related:
  - optic-symmetries
  - set-class
  - inversional-near-symmetry
contrasts_with:
  - transposition-symmetry

answers_questions:
  - "What are OPTIC symmetries?"
  - "How does inversion relate to set classes?"
---

# Quick Definition
The I (Inversion) symmetry groups together musical objects that can be transformed into one another by turning pitch space "upside down," defining the additional equivalence that distinguishes set classes from chord types.

# Core Definition
The inversion symmetry operation considers two musical objects equivalent if one can be obtained by inverting the other (reflecting it in pitch or pitch-class space). Adding I to the OPTC symmetries (which define chord types) yields OPTIC, which defines set classes. Since inversionally related chords share the same sequence of arc lengths on the pitch-class circle (though potentially in opposite directions), they share many structural and sonic properties. For example, major and minor triads both divide the circle into arcs of 4, 3, and 5 semitones, just read in opposite directions.

# Prerequisites
- **basic-musical-object** — The objects being classified
- **inversion** — The mathematical operation underlying this symmetry

# Key Properties
1. Turns pitch space "upside down" via reflection
2. Combined with OPTC gives set classes (OPTIC)
3. Inversionally related chords share arc-length sequences (clockwise vs. counterclockwise)
4. Many twentieth-century composers treat inversionally related chords as equivalent
5. Major and minor triads are the canonical example

# Construction / Recognition
## To Construct/Create:
1. Take a musical object and apply the inversion formula: (x + y) - p
2. The original and result belong to the same OPTIC class
## To Identify/Recognize:
1. Two chords have the same arc lengths on the pitch-class circle, but in reverse order
2. One is a "mirror image" of the other

# Context & Application
The I symmetry is what distinguishes set classes from chord types. While all major chords belong to the same chord type (OPTC), major and minor chords together form a single set class (OPTIC). The musical justification is that inversionally related chords often sound similar — students readily confuse inversionally related three-note chords in ear-training tests.

# Examples
**Example 1** (p. 57, Fig 2.4.5): C major divides the circle into arcs of 4, 3, 5 clockwise from C; C minor divides it into 4, 3, 5 counterclockwise from G. Same arc lengths, different direction = same set class.

# Relationships
## Builds Upon
- **inversion** — The operation applied
- **basic-musical-object** — The objects classified
## Enables
- **set-class** — OPTIC equivalence class (requires I)
## Related
- **optic-symmetries** — I is the fourth of five OPTIC operations
- **inversional-near-symmetry** — Near-I-symmetry enables efficient voice leading between inversionally related chords
## Contrasts With
- **transposition-symmetry** — T without I gives chord types; adding I gives set classes

# Common Errors
- **Error**: Assuming inversionally related chords always sound the same
  **Correction**: They share many properties and sound similar, but are perceptibly different (e.g., major vs. minor)

# Common Confusions
- **Confusion**: Confusing I symmetry (pitch-space inversion/reflection) with O symmetry (registral inversion/octave shift)
  **Clarification**: I reflects the entire pitch space; O shifts individual notes by octaves. These are completely different operations sharing the unfortunate label "inversion"

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 56-57.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.4
- Confidence rationale: High — explicitly defined in the OPTIC table
- Cross-reference status: Verified; used throughout the book
