---
concept: Transposition Symmetry (T)
slug: transposition-symmetry

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
  - "T symmetry"
  - "transpositional equivalence"

prerequisites:
  - basic-musical-object
  - transposition
extends:
  - transposition
related:
  - optic-symmetries
  - transpositional-set-class
  - transpositional-near-symmetry
contrasts_with:
  - inversion-symmetry

answers_questions:
  - "What are OPTIC symmetries?"
  - "What does transposition mean in both pitch and pitch-class spaces?"
---

# Quick Definition
The T (Transposition) symmetry groups together musical objects that can be transformed into one another by moving all notes in the same direction by the same amount, defining "chord type."

# Core Definition
The transposition symmetry operation, when used for classification, considers two musical objects equivalent if one can be obtained from the other by transposing all notes by the same amount. Adding T to the OPC symmetries (which define chords) yields OPTC, which defines "chord types" or transpositional set classes. Two chords belong to the same type if one can be rotated into the other in pitch-class space, meaning they share the same sequence of arc-length distances between adjacent notes. For example, all major chords divide the circle into arcs of 4, 3, and 5 semitones (reading clockwise from the root).

# Prerequisites
- **basic-musical-object** — The objects being classified
- **transposition** — The operation being used for classification

# Key Properties
1. Moves all notes uniformly by the same amount
2. Combined with OPC gives chord types (OPTC)
3. Two chords of the same type have identical arc-length patterns on the circle
4. Geometrically: rotation on the pitch-class circle
5. Distinct from O (which moves individual notes by octaves)
6. Can also be applied to progressions (uniformly or individually)

# Construction / Recognition
## To Construct/Create:
1. Take a musical object and add the same number to every pitch
2. The original and result belong to the same OPTC class
## To Identify/Recognize:
1. Two chords have the same intervals between successive notes
2. One can be rotated into the other on the pitch-class circle

# Context & Application
The T symmetry defines chord types — the families of transpositionally related chords that share the same interval structure. "Major chord," "minor seventh chord," "diminished triad" are all chord types (OPTC classes). When applied to progressions uniformly, T-related voice leadings represent "the same musical pattern at different pitch levels."

# Examples
**Example 1** (p. 55, Fig 2.4.3): All major chords are related by transposition: C major and D major both divide the pitch-class circle into arcs of 4, 3, and 5 semitones.

# Relationships
## Builds Upon
- **transposition** — The mathematical operation underlying this symmetry
- **basic-musical-object** — The objects being classified
## Enables
- **transpositional-set-class** — OPTC equivalence classes
## Related
- **optic-symmetries** — T is the third of five OPTIC operations
- **transpositional-near-symmetry** — Near-T-symmetry enables efficient voice leading
## Contrasts With
- **inversion-symmetry** — Adding I to OPTC gives the finer set class (OPTIC)

# Common Errors
- **Error**: Confusing T symmetry (applied to whole object) with O symmetry (applied to individual notes)
  **Correction**: T moves all notes by the same amount; O moves one note by an octave

# Common Confusions
- **Confusion**: Thinking T symmetry in the OPTIC context is the same as transposing in performance
  **Clarification**: As a symmetry operation, T defines equivalence classes; as a performance practice, transposition changes the key

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 55-56.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.4
- Confidence rationale: High — explicitly defined in the OPTIC table
- Cross-reference status: Verified; used throughout the book
