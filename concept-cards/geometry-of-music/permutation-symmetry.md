---
concept: Permutation Symmetry (P)
slug: permutation-symmetry

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
  - "P symmetry"
  - "reordering"
  - "voice permutation"

prerequisites:
  - basic-musical-object
extends: []
related:
  - optic-symmetries
  - chord
  - voice-leading-vs-chord-progression
  - permutational-near-symmetry
contrasts_with: []

answers_questions:
  - "What are OPTIC symmetries?"
  - "What is a voice leading vs. a chord progression?"
---

# Quick Definition
The P (Permutation) symmetry allows reordering the voices of a musical object — changing which voice is assigned to which note — without changing the object's harmonic identity.

# Core Definition
The permutation symmetry operation reorders the elements of a basic musical object, changing which instrumental voice plays which note. It transforms (C4, E4, G4) into (E4, G4, C4), (G4, C4, E4), etc. When applied to progressions, permutation can be uniform (same reordering of both chords) or individual (different reorderings). Uniform permutation preserves voice identity and yields voice leadings; individual permutation destroys voice identity and contributes to chord progressions. Permutational symmetry in the near-symmetry sense (having duplicate notes) enables efficient voice leading from a chord to itself.

# Prerequisites
- **basic-musical-object** — The object being reordered

# Key Properties
1. Reorders voices without changing pitch content
2. Transforms ordered objects into unordered collections
3. Uniform application preserves voice identity (voice leadings)
4. Individual application destroys voice identity (chord progressions)
5. Permutational symmetry = having duplicate notes (multiset with repeats)
6. One of Rameau's three implicit chord-defining operations

# Construction / Recognition
## To Construct/Create:
1. Take a basic musical object (ordered pitch series)
2. Rearrange the elements in any order
## To Identify/Recognize:
1. Two objects contain the same pitches but in different orders
2. Same notes assigned to different voices

# Context & Application
Permutation is central to the distinction between voice leadings and chord progressions. When we care about which voice goes where (voice leading), we apply P uniformly. When we care only about the succession of harmonies (chord progression), we apply P individually. Near-permutational symmetry (chords close to having duplicate notes, i.e., clusters) enables efficient voice leading to a chord's own reorderings.

# Examples
**Example 1** (p. 54): (C4, E4, G4) and (E4, G4, C4) are related by permutation — same pitches, different voice assignments.

**Example 2** (p. 59, Fig 2.5.1): Uniform permutation of a progression moves all voices to different staves in the same way; individual permutation applies different reorderings to each chord.

# Relationships
## Builds Upon
- **basic-musical-object** — Permutation reorders basic objects
## Enables
- **chord** — O + P + C = chord (unordered set of pitch classes)
- **voice-leading-vs-chord-progression** — The uniform/individual distinction is key
## Related
- **optic-symmetries** — P is the second of five OPTIC operations
- **permutational-near-symmetry** — Near-P-symmetry enables voice leading to self
## Contrasts With
- No direct contrast within this source

# Common Errors
- **Error**: Thinking permutation changes what notes are present
  **Correction**: Permutation only changes which voice plays which note, not what notes exist

# Common Confusions
- **Confusion**: Confusing uniform and individual permutation
  **Clarification**: Uniform permutation (same reordering of both chords) preserves voice identity; individual permutation (different reorderings) destroys it

# Source Reference
Chapter 2: Harmony and Voice Leading, Sections 2.4-2.5, pages 54-60.

# Verification Notes
- Definition source: Direct from Section 2.4, Figure 2.4.4, and Section 2.9.3
- Confidence rationale: High — explicitly defined in the OPTIC table
- Cross-reference status: Verified; key to the voice-leading vs. chord-progression distinction
