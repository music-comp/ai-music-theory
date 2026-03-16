---
concept: Multiset
slug: multiset

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
  - "multiset of pitch classes"
  - "OP class"

prerequisites:
  - basic-musical-object
  - octave-symmetry
  - permutation-symmetry
extends:
  - basic-musical-object
related:
  - optic-symmetries
  - chord
  - permutational-near-symmetry
contrasts_with:
  - chord

answers_questions:
  - "What is a multiset?"
  - "What is the difference between a chord and a multiset?"
---

# Quick Definition
A multiset is an unordered collection of pitch classes where the number of times each pitch class appears matters — {C, C, E, G} is distinct from {C, E, G}.

# Core Definition
A multiset of pitch classes is an equivalence class formed by two OPTIC symmetry operations: O and P (OP). Octave information and voice ordering are discarded, but the number of copies of each pitch class is preserved. Thus {C, C, E, G} (with two C's) is a different multiset from {C, E, G} (with one C). When we further apply C (cardinality change), multisets collapse into chords. Multisets are useful when note multiplicity matters — for instance, in voice-leading analysis where the number of voices sounding each pitch class is significant.

# Prerequisites
- **basic-musical-object** — Multisets are formed from basic objects
- **octave-symmetry** — O discards octave information
- **permutation-symmetry** — P discards ordering

# Key Properties
1. OP equivalence class
2. Unordered but tracks note multiplicity
3. {C, C, E, G} differs from {C, E, G}
4. More specific than chords (OPC), less specific than basic musical objects
5. Important for voice-leading analysis with specific voice counts

# Construction / Recognition
## To Construct/Create:
1. Reduce all pitches to pitch classes (apply O)
2. Discard ordering (apply P)
3. Keep track of duplicates
## To Identify/Recognize:
1. Same pitch-class content with same multiplicities = same multiset
2. Different multiplicities = different multisets

# Context & Application
Multisets matter in voice-leading contexts where the number of voices on each pitch class is significant. The concept also connects to permutational near-symmetry: a chord with duplicate pitch classes (like {C, C, C}) is permutationally symmetrical, and chords near such multisets can be connected to themselves by efficient voice leading.

# Examples
**Example 1** (p. 58, Fig 2.4.7): Multiset = OP in the OPTIC table. {C, C, E, G} contains two copies of C and is a different multiset from {C, E, G}.

# Relationships
## Builds Upon
- **basic-musical-object** — Formed by applying OP
## Enables
- **permutational-near-symmetry** — Multisets with duplicates are permutationally symmetrical
## Related
- **optic-symmetries** — OP in the framework
## Contrasts With
- **chord** — Chords (OPC) ignore multiplicity; multisets (OP) track it

# Common Errors
- **Error**: Treating multisets and chords as identical
  **Correction**: Multisets preserve multiplicity information; chords discard it

# Common Confusions
- **Confusion**: Thinking multisets are exotic or unusual
  **Clarification**: They arise naturally whenever multiple voices sound the same pitch class, which is common in actual music

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 58-59.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.7
- Confidence rationale: High — explicitly defined in the OPTIC framework
- Cross-reference status: Verified; used in Section 2.9.3 on permutational symmetry
