---
# === CORE IDENTIFICATION ===
concept: Permutational Near-Symmetry
slug: permutational-near-symmetry

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
section: "2.9.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "near-P-symmetry"
  - "near-cluster"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - permutation-symmetry
  - near-symmetry
  - multiset
extends:
  - near-symmetry
related:
  - efficient-voice-leading
contrasts_with:
  - transpositional-near-symmetry
  - inversional-near-symmetry

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is permutational near-symmetry?"
  - "How does permutation relate to efficient voice leading?"
---

# Quick Definition
A chord is nearly permutationally symmetrical when it is close to a chord with duplicate notes (a "multiset cluster"), enabling efficient voice leading from the chord to itself — i.e., an efficient rearrangement of its own notes among voices.

# Core Definition
A chord (treated as a multiset) is permutationally symmetrical if it contains multiple copies of the same pitch class, like {C, C, C}. Such chords are trivially unchanged by permutation of their notes. Near-P-symmetry means being close to such a chord: {B, C, Db} is near {C, C, C} since (B, C, Db) -> (C, C, C) is small. By the standard near-symmetry argument: A is close to the P-symmetrical S, so P(A) is close to P(S) = S, hence A and P(A) are close. The result is an efficient voice leading from the chord to itself with notes rearranged: (B, C, Db) -> (C, Db, B). This means clustered chords can be efficiently "rotated" among voices.

# Prerequisites
- **permutation-symmetry** — The transformation under consideration
- **near-symmetry** — The general principle being applied
- **multiset** — Permutationally symmetrical chords are multisets with duplicates

# Key Properties
1. Chord is close to one with duplicate pitch classes (a cluster)
2. Enables efficient voice leading from the chord to a permuted version of itself
3. "Permutation symmetry" = having duplicate notes
4. Analogous to T and I near-symmetry but for a different operation
5. Chromatic clusters {B, C, Db} are the paradigmatic example

# Construction / Recognition
## To Construct/Create:
1. Start with a chord containing duplicate notes (e.g., {C, C, C})
2. Slightly spread the notes apart
3. The result is near-P-symmetrical
## To Identify/Recognize:
1. Check if the chord's notes are clustered close together
2. Chromatic clusters and near-unisons indicate near-P-symmetry

# Context & Application
Permutational near-symmetry may seem surprising alongside the more familiar T and I symmetries, but it is an equally valid manifestation of the same general principle. It is particularly relevant for cluster-based music (Ligeti, Lutoslawski) and for understanding how voice-leading possibilities arise from a chord's proximity to "boring" multisets. The voice leading (C, C, E, G) -> (A, C, F, F) in common-practice music exploits permutational symmetry alongside transpositional symmetry.

# Examples
**Example 1** (p. 76-77, Fig 2.9.9): {B, C, Db} is near {C, C, C}. The voice leading (B, C, Db) -> (C, C, C) -> (C, Db, B) gives, upon removing the middle chord, an efficient voice leading (B, C, Db) -> (C, Db, B) from the chord to a permuted version of itself.

# Relationships
## Builds Upon
- **near-symmetry** — One of three types
- **permutation-symmetry** — The relevant transformation
- **multiset** — The symmetrical objects are multisets with duplicates
## Enables
- Understanding of voice leading in cluster-based music
## Related
- **efficient-voice-leading** — What near-P-symmetry enables
## Contrasts With
- **transpositional-near-symmetry** — Enables voice leading to transpositions
- **inversional-near-symmetry** — Enables voice leading to inversions

# Common Errors
- **Error**: Thinking permutational symmetry is musically trivial
  **Correction**: It has real musical consequences, enabling efficient self-voice-leadings and voice redistribution

# Common Confusions
- **Confusion**: Why place permutation alongside transposition and inversion?
  **Clarification**: All three are symmetry operations on musical objects; the near-symmetry argument works identically for all three, and they are the only three needed to explain all efficient voice leading between similar chords

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.9.3, pages 76-79.

# Verification Notes
- Definition source: Direct from Section 2.9.3
- Confidence rationale: High — detailed argument with explicit parallels to T and I
- Cross-reference status: Verified; contributes to the completeness of the near-symmetry theory
