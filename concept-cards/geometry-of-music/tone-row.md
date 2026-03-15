---
# === CORE IDENTIFICATION ===
concept: Tone Row
slug: tone-row

# === CLASSIFICATION ===
category: harmony
subcategory: classification
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "ordered set of pitch classes"
  - "OC class"
  - "twelve-tone row"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-musical-object
  - octave-symmetry
  - cardinality-change-symmetry
extends:
  - basic-musical-object
related:
  - optic-symmetries
  - chord
contrasts_with:
  - chord

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a tone row in the OPTIC framework?"
  - "What are OPTIC symmetries?"
---

# Quick Definition
A tone row is an ordered sequence of pitch classes, where order matters but octave placement does not — defined by the OC symmetries in the OPTIC framework.

# Core Definition
A tone row, as defined by Schoenberg and formalized in the OPTIC framework, is an equivalence class formed by O (octave shift) and C (cardinality change) symmetries. The order of pitch classes is preserved (P is not applied), so (C, E, G) and (E, C, G) are different tone rows. Octave information is discarded (O is applied), so the specific register does not matter. Tone rows are particularly important for melody, where the order of notes is essential, and for twelve-tone serial composition, where a specific ordering of all twelve pitch classes governs the piece.

# Prerequisites
- **basic-musical-object** — Tone rows are formed from basic objects
- **octave-symmetry** — O discards octave information
- **cardinality-change-symmetry** — C allows note duplication

# Key Properties
1. OC equivalence class
2. Order is preserved (P is not applied)
3. Octave information is discarded
4. Particularly relevant for melody and serial composition
5. (C, E, G) and (E, C, G) are different tone rows but the same chord

# Construction / Recognition
## To Construct/Create:
1. Choose an ordered sequence of pitch classes
2. Octave placement is free (any register)
## To Identify/Recognize:
1. Check whether two sequences have the same pitch classes in the same order (ignoring octave)

# Context & Application
Tone rows are important in the Schoenbergian twelve-tone tradition and in any context where melodic order matters. The OPTIC framework reveals that tone rows represent a specific combination of symmetries — one of 32 possible combinations — rather than a fundamentally different kind of object from chords.

# Examples
**Example 1** (p. 58, Fig 2.4.7): Tone row = OC in the OPTIC table. Order matters: (C, E, G) is a different tone row from (E, G, C).

# Relationships
## Builds Upon
- **basic-musical-object** — Formed by applying OC
## Enables
- Twelve-tone serial analysis
## Related
- **optic-symmetries** — OC in the framework
## Contrasts With
- **chord** — Chords (OPC) discard order; tone rows (OC) preserve it

# Common Errors
- **Error**: Thinking tone rows can only have twelve notes
  **Correction**: While Schoenberg's twelve-tone rows use all 12 pitch classes, the OPTIC framework allows tone rows of any length

# Common Confusions
- **Confusion**: Confusing tone rows with melodies
  **Clarification**: A tone row specifies pitch-class order but not rhythm, register, or duration; a melody includes all these

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, page 58.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.7
- Confidence rationale: High — explicitly listed in the OPTIC table
- Cross-reference status: Verified; standard music-theoretical term
