---
concept: OPTIC Symmetries
slug: optic-symmetries

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
  - "OPTIC operations"
  - "five symmetry operations"

prerequisites:
  - basic-musical-object
  - pitch-space
  - pitch-class-space
extends: []
related:
  - octave-symmetry
  - permutation-symmetry
  - transposition-symmetry
  - inversion-symmetry
  - cardinality-change-symmetry
  - chord
  - transpositional-set-class
  - set-class
contrasts_with: []

answers_questions:
  - "What are OPTIC symmetries?"
  - "How are chords, chord types, and set classes defined?"
---

# Quick Definition
OPTIC refers to five symmetry operations — Octave shift, Permutation, Transposition, Inversion, and Cardinality change — that can be applied to basic musical objects to form increasingly abstract musical categories.

# Core Definition
The OPTIC symmetries are five fundamental operations that preserve aspects of a musical object's identity while discarding other information. O (Octave shift) moves any note into a new octave. P (Permutation) reorders the voices. T (Transposition) moves all notes by the same amount. I (Inversion) turns the object upside-down. C (Cardinality change) adds or removes duplicate notes. Different combinations of these five operations yield different music-theoretical concepts: OPC gives chords, OPTC gives chord types (transpositional set classes), and OPTIC gives set classes. All 32 combinations of these symmetries represent potentially useful ways of classifying musical objects.

# Prerequisites
- **basic-musical-object** — The objects to which OPTIC operations are applied
- **pitch-space** — The space in which objects exist
- **pitch-class-space** — O symmetry creates pitch-class space

# Key Properties
1. Five operations: O, P, T, I, C
2. Each combination defines a different equivalence class (musical category)
3. Standard terms correspond to specific combinations (see Figure 2.4.7)
4. Musical classification = progressive discarding of information through symmetry
5. 32 possible combinations, each potentially useful
6. Can be applied uniformly or individually to progressions

# Construction / Recognition
## To Construct/Create:
1. Start with a basic musical object (ordered pitch series)
2. Choose which OPTIC operations to apply
3. The resulting equivalence class defines the musical category
## To Identify/Recognize:
1. Determine which operations relate two objects
2. The minimal set of operations needed defines the level of abstraction

# Context & Application
The OPTIC framework is one of Tymoczko's central contributions. It unifies many disparate music-theoretical concepts under a single systematic scheme and reveals that standard concepts like "chord" and "set class" are not arbitrary but correspond to natural combinations of symmetry operations. The framework also extends to progressions, where each symmetry can be applied uniformly (same operation to both chords) or individually (different operations to each chord), yielding voice leadings and chord progressions.

# Examples
**Example 1** (p. 56-58, Fig 2.4.6): Starting from (E4, G4, Bb4, D5), applying O gives (E3, G4, Bb3, D4), P reorders to (E3, Bb3, D4, G4), T transposes up by semitone to get the Tristan chord (F3, B3, D#4, G#4), I inverts to get (D4, G#3, E3, B2), and C adds voice doublings.

**Example 2** (p. 58, Fig 2.4.7): Term-to-symmetry mapping table:
- chord = OPC
- chord type / transpositional set class = OPTC
- set class = OPTIC
- multiset = OP
- tone row = OC

# Relationships
## Builds Upon
- **basic-musical-object** — The starting point for OPTIC classification
## Enables
- **chord** — OPC equivalence class
- **transpositional-set-class** — OPTC equivalence class
- **set-class** — OPTIC equivalence class
## Related
- **octave-symmetry** — The O in OPTIC
- **permutation-symmetry** — The P in OPTIC
- **transposition-symmetry** — The T in OPTIC
- **inversion-symmetry** — The I in OPTIC
- **cardinality-change-symmetry** — The C in OPTIC
## Contrasts With
- No direct contrast; this is the unifying framework

# Common Errors
- **Error**: Thinking OPTIC symmetries must all be applied together
  **Correction**: Different subsets yield different (and useful) musical categories

# Common Confusions
- **Confusion**: Confusing the T symmetry (transposition of the whole object) with O (octave shift of individual notes)
  **Clarification**: T moves all notes by the same amount; O moves a single note by an octave

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 53-59.

# Verification Notes
- Definition source: Direct from Section 2.4, including Figure 2.4.4 table and Figure 2.4.7
- Confidence rationale: High — this is a central, precisely defined framework
- Cross-reference status: Verified; used throughout the book, especially Chapter 3
