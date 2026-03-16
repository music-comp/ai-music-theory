---
concept: Set Class
slug: set-class

category: harmony
subcategory: classification
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
  - "OPTIC class"
  - "Tn/TnI-type"

prerequisites:
  - transpositional-set-class
  - inversion-symmetry
extends:
  - transpositional-set-class
related:
  - optic-symmetries
  - harmonic-consistency
contrasts_with:
  - transpositional-set-class

answers_questions:
  - "What is a set class?"
  - "What are OPTIC symmetries?"
---

# Quick Definition
A set class groups together all chords related by any combination of the five OPTIC symmetries — including both transposition and inversion — so that major and minor triads, for instance, belong to the same set class.

# Core Definition
A set class is an equivalence class formed by all five OPTIC symmetry operations: O, P, T, I, and C. Two objects belong to the same set class if they can be transformed into each other (or into some third chord) by any sequence of octave shifts, permutations, transpositions, inversions, and cardinality changes. Members share the same sequence of arc lengths on the pitch-class circle, though the sequence may proceed clockwise or counterclockwise. Set classes are central to twentieth-century music theory, where composers like Schoenberg and his followers treated inversionally related chords as equivalent.

# Prerequisites
- **transpositional-set-class** — Set class adds inversional equivalence to chord type
- **inversion-symmetry** — The additional operation distinguishing set classes from chord types

# Key Properties
1. OPTIC equivalence class — all five symmetries applied
2. Members share arc-length sequences, clockwise or counterclockwise
3. Major and minor triads belong to the same set class
4. Maximal abstraction within the OPTIC framework
5. Central to atonal set theory (Forte, Babbitt, etc.)

# Construction / Recognition
## To Construct/Create:
1. Take a chord
2. Generate all transpositions and inversions
3. The resulting collection is the set class
## To Identify/Recognize:
1. Compute arc lengths between adjacent notes on the circle
2. Two chords belong to the same set class if they share the same arc lengths (in either direction)

# Context & Application
Set classes represent the most abstract level of chord classification in the OPTIC framework. They are particularly useful in analyzing twentieth-century music where composers often treat inversionally related chords as equivalent. In Tymoczko's approach, set classes are just one of 32 possible OPTIC combinations, and not always the most appropriate level of analysis.

# Examples
**Example 1** (p. 57, Fig 2.4.5): C major {C, E, G} divides the circle into 4, 3, 5 clockwise; C minor {C, Eb, G} divides it into 4, 3, 5 counterclockwise. Same set class, different chord types.

**Example 2** (p. 56-58, Fig 2.4.6): A half-diminished chord can be transformed via OPTIC operations into the Tristan chord, then into its inversion (dominant seventh), demonstrating set-class equivalence.

# Relationships
## Builds Upon
- **transpositional-set-class** — Set class adds I to OPTC
- **inversion-symmetry** — The I operation
## Enables
- Analysis of atonal and post-tonal music
## Related
- **optic-symmetries** — OPTIC = set class
- **harmonic-consistency** — Can be defined at the set-class level
## Contrasts With
- **transpositional-set-class** — Chord type does not include inversional equivalence

# Common Errors
- **Error**: Assuming set class is always the most appropriate level of analysis
  **Correction**: Different musical contexts call for different OPTIC combinations; sometimes chord type (OPTC) is more appropriate

# Common Confusions
- **Confusion**: Thinking set class = chord
  **Clarification**: A chord is a specific unordered pitch-class set; a set class is a family of chords related by transposition and inversion

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 56-58.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figures 2.4.4-2.4.7
- Confidence rationale: High — precisely defined as OPTIC equivalence class
- Cross-reference status: Verified; used throughout the book
