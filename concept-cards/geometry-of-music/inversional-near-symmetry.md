---
# === CORE IDENTIFICATION ===
concept: Inversional Near-Symmetry
slug: inversional-near-symmetry

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
section: "2.9.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "near-I-symmetry"
  - "near-inversional invariance"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inversion
  - near-symmetry
extends:
  - near-symmetry
related:
  - efficient-voice-leading
contrasts_with:
  - transpositional-near-symmetry
  - permutational-near-symmetry

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is inversional near-symmetry?"
  - "When can a chord be connected to its inversion by efficient voice leading?"
---

# Quick Definition
A chord is nearly inversionally symmetrical when it is close to a chord that is unchanged by some inversion, enabling efficient voice leading between the chord and its inversion.

# Core Definition
A chord A is near-I-symmetrical when there is a small voice leading from A to a chord S that is exactly inversionally symmetrical (i.e., I(S) = S for some inversion I). Inversionally symmetrical chords have their notes arranged symmetrically around an "axis of symmetry" crossing the pitch-class circle at two antipodal points. For example, {C, D, E} is inversionally symmetrical around D (and Ab). The argument: since A is close to S, and I(S) = S, then I(A) is also close to I(S) = S. Both A and I(A) are close to S, hence connectable by efficient voice leading. This explains why inversionally related chords (like half-diminished and dominant seventh) can often be connected by small voice leadings.

# Prerequisites
- **inversion** — The transformation under consideration
- **near-symmetry** — The general principle being applied

# Key Properties
1. Chord is close to one with an axis of inversional symmetry
2. Enables efficient voice leading to the chord's inversion
3. Inversionally symmetrical chords have notes symmetric around an axis
4. The axis consists of two antipodal points; notes may be placed on axis points or in symmetric pairs
5. Half-diminished and dominant seventh chords are near the inversionally symmetrical diminished seventh

# Construction / Recognition
## To Construct/Create:
1. Start with an inversionally symmetrical chord
2. Slightly perturb one or more notes
3. The result is near-I-symmetrical
## To Identify/Recognize:
1. Check if the chord is close to one with an axis of symmetry
2. Major/minor triads, half-diminished/dominant seventh chords are paradigmatic examples

# Context & Application
Inversional near-symmetry explains voice-leading relationships like the connection between half-diminished and dominant seventh chords (central to Wagner's Tristan) and the connection between major and minor triads. Both pairs of chords are inversionally related and both are near the inversionally symmetrical diminished seventh or diminished triad.

# Examples
**Example 1** (p. 75-76, Fig 2.9.7): {C, D#, E} is near the inversionally symmetrical {C, D, E}. Inversion around D maps {C, D, E} to itself. Small voice leading: (C, D#, E) -> (C, Db, E), connecting inversionally related chords.

**Example 2** (p. 76, Fig 2.9.8): F half-diminished is near the diminished seventh (which is I-symmetrical). The efficient voice leading from F half-diminished to E dominant seventh — the opening of Tristan — exploits this near-I-symmetry.

# Relationships
## Builds Upon
- **near-symmetry** — One of three types
- **inversion** — The relevant transformation
## Enables
- Understanding of voice leading between inversionally related chords (major/minor, half-dim/dom7)
## Related
- **efficient-voice-leading** — What near-I-symmetry enables
## Contrasts With
- **transpositional-near-symmetry** — Different type of near-symmetry
- **permutational-near-symmetry** — Different type

# Common Errors
- **Error**: Thinking inversional near-symmetry only applies to the I symmetry in the OPTIC sense
  **Correction**: It applies to pitch-space inversion (reflection), and the near-symmetry allows efficient voice leading between inversionally related chords

# Common Confusions
- **Confusion**: Thinking the Tristan chord progression is "just" chromatic voice leading
  **Clarification**: The specific efficiency of this voice leading is explained by the near-I-symmetry of half-diminished and dominant seventh chords (both near the diminished seventh)

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.9.2, pages 74-76.

# Verification Notes
- Definition source: Direct from Section 2.9.2
- Confidence rationale: High — detailed argument with the Tristan example
- Cross-reference status: Verified; the Tristan analysis is reprised in Chapter 8
