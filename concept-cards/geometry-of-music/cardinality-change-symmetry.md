---
# === CORE IDENTIFICATION ===
concept: Cardinality Change Symmetry (C)
slug: cardinality-change-symmetry

# === CLASSIFICATION ===
category: geometric-theory
subcategory: symmetry
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
  - "C symmetry"
  - "note duplication"
  - "cardinality equivalence"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-musical-object
extends: []
related:
  - optic-symmetries
  - chord
  - multiset
contrasts_with:
  - multiset

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are OPTIC symmetries?"
  - "What is the difference between a chord and a multiset?"
---

# Quick Definition
The C (Cardinality change) symmetry allows adding or removing duplicate voices that double notes already present, so that {C, E, G} and {C, C, E, G, G} represent the same chord.

# Core Definition
The cardinality change symmetry operation permits adding a new voice that duplicates a note already in the musical object, or removing such duplications. This means (C4, E4, G4) and (C4, E4, G4, G4) are considered equivalent — both represent the C major chord. Cardinality change is one of Rameau's three implicit chord-defining operations (along with O and P). Without C, we would distinguish the multiset {C, C, E, G} (with two C's) from {C, E, G} (with one C); with C, both are simply "the C major chord."

# Prerequisites
- **basic-musical-object** — The object to which duplication is applied

# Key Properties
1. Adds or removes duplicate notes (note doublings)
2. One of the three operations defining "chord" (OPC)
3. Distinguishes chords from multisets: multisets (OP) track duplications; chords (OPC) do not
4. Mathematically somewhat delicate (see Callender, Quinn, and Tymoczko 2008)
5. Important for voice leading where larger chords exploit symmetries of smaller ones

# Construction / Recognition
## To Construct/Create:
1. Take a musical object and duplicate one or more of its notes in any octave
2. The result is equivalent under C symmetry
## To Identify/Recognize:
1. Two objects have the same pitch-class content but different numbers of notes
2. One has doublings that the other lacks

# Context & Application
Cardinality change is essential for defining chords as unordered sets of pitch classes (where only membership matters, not multiplicity). It also plays a role in voice leading: when a chord exploits the symmetry of a smaller chord (e.g., a three-note chord exploiting the symmetry of a two-note chord), extra voices with doublings are needed (Section 2.9.3).

# Examples
**Example 1** (p. 54-55): (C4, E4, G4) and (C4, E4, G4, G4) both represent C major. The second has an extra voice doubling G, but they are "the same chord" under C symmetry.

**Example 2** (p. 58, Fig 2.4.7): Chord = OPC; multiset = OP. The difference is C: chords ignore note multiplicity, multisets track it.

# Relationships
## Builds Upon
- **basic-musical-object** — Cardinality change applies to basic objects
## Enables
- **chord** — O + P + C = chord (unordered set of pitch classes)
## Related
- **optic-symmetries** — C is the fifth of five OPTIC operations
## Contrasts With
- **multiset** — Multisets (OP) track note multiplicity; chords (OPC) do not

# Common Errors
- **Error**: Thinking cardinality change allows adding arbitrary new notes
  **Correction**: Only notes already present in the object can be duplicated

# Common Confusions
- **Confusion**: Thinking cardinality change is musically trivial
  **Clarification**: It matters significantly for voice leading, where extra voices from doublings can exploit symmetries of smaller chords

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 54-58.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.4
- Confidence rationale: High — explicitly defined in the OPTIC table
- Cross-reference status: Verified; plays a role in voice-leading discussion (Section 2.9.3)
