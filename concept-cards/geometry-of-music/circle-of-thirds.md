---
concept: Circle of Thirds
slug: circle-of-thirds

category: geometric-theory
subcategory: voice-leading
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "The Extended Common Practice"
chapter_number: 6
pdf_page: 223
section: "6.3.2"

extraction_confidence: high

aliases:
  - "diatonic circle of thirds"
  - "triadic circle"

prerequisites:
  - renaissance-triadic-harmony
  - three-plus-one-voice-leading
extends: []
related:
  - fourth-progressions
  - extended-common-practice
contrasts_with: []

answers_questions:
  - "What is the circle of thirds?"
  - "How can we model all strongly crossing-free voice leadings between diatonic triads?"
  - "How is the circle of thirds analogous to the circle of fifths?"
---

# Quick Definition
The circle of thirds is a circular arrangement of the seven diatonic triads connected by single-step voice leadings, analogous to the diatonic circle of fifths but modeling three-voice counterpoint rather than key relationships.

# Core Definition
In three-note chord space, the seven diatonic triads form a "crooked chain" running through the center of the space, with the last chord connecting back to the first. This chain — the circle of thirds — links triads by single-step voice leadings: each clockwise step raises one voice by a diatonic step, and each counterclockwise step lowers one voice. The circle models ALL possible strongly crossing-free voice leadings between any two diatonic triads. To move from one triad to another, one simply counts the shortest path around the circle — clockwise (raising voices by step) or counterclockwise (lowering voices). Longer paths, including full rotations, generate all other strongly crossing-free voice leadings between the same two chords. For example, C major to F major requires two clockwise steps (raising two voices) or five counterclockwise steps (lowering five voices).

# Prerequisites
- **renaissance-triadic-harmony** — The triadic context
- **three-plus-one-voice-leading** — The circle models the upper-voice component

# Key Properties
1. Seven diatonic triads arranged in a circle: I-iii-V-vii°-ii-iv-vi-I
2. Clockwise steps raise one voice by step; counterclockwise steps lower one voice
3. Models ALL strongly crossing-free voice leadings between diatonic triads
4. Analogous to the circle of fifths but for triadic voice leading
5. The shortest path gives the most efficient voice leading
6. Longer paths (including full rotations) give other valid voice leadings
7. Applies to any diatonic system, not just major

# Construction / Recognition
## To Construct/Create:
1. Arrange diatonic triads in order of ascending thirds: C-E-G, E-G-B, G-B-D, B-D-F, D-F-A, F-A-C, A-C-E
2. Connect adjacent triads with lines representing single-step voice leadings
3. Close the circle by connecting A-C-E back to C-E-G
## To Identify/Recognize:
1. In a triadic progression, check whether the upper voices move by single steps
2. Count how many voices change and in which direction
3. Verify the voice leading is strongly crossing-free

# Context & Application
The circle of thirds is a powerful analytical and compositional tool. In the Josquin excerpt, 11 of 12 voice leadings take the shortest path along the circle. It also models contemporary guitar fingerings where the top three strings trace out paths on the circle. The circle provides an efficient representation of the combinatorial possibilities available to composers writing three-voice triadic counterpoint — far simpler than examining all possible voice leadings individually. It is located in three-note chord space, where the seven diatonic triads form the central chain.

# Examples
**Example 1** (p. 223-224, Figures 6.3.7-6.3.8): The circle of diatonic triads in three-note chord space. Two voice leadings from C major to F major: clockwise (two steps, raising G to A then E to F) or counterclockwise (five steps, lowering C to B, then B to A, then G to F, then E to D, then D to C).

**Example 2** (p. 224, Figure 6.3.9): The opening of Josquin's "Tu pauperum refugium" plotted on the circle, showing that the music takes the most direct path between successive triads.

**Example 3** (p. 225, Figure 6.3.10): Common guitar fingerings also trace shortest paths on the circle of thirds, demonstrating the schema's applicability across 500 years.

# Relationships
## Builds Upon
- **renaissance-triadic-harmony** — The triadic context
- **three-plus-one-voice-leading** — The circle models the upper-voice component
## Enables
- Efficient analysis of three-voice triadic counterpoint
- Understanding of voice-leading distances between triads
## Related
- **fourth-progressions** — Fourth progressions correspond to two steps on the circle
- **extended-common-practice** — The circle operates throughout the tradition
## Contrasts With
- None specifically

# Common Errors
- **Error**: Confusing the circle of thirds with the circle of fifths
  **Correction**: The circle of fifths arranges keys/scales by fifth relationships; the circle of thirds arranges triads by single-step voice leadings, which happen to be third-related in root

# Common Confusions
- **Confusion**: Thinking only the shortest path is musically valid
  **Clarification**: Any path around the circle, including multiple full rotations, produces a valid strongly crossing-free voice leading; the shortest path is merely the most efficient

# Source Reference
Chapter 6: The Extended Common Practice, Section 6.3.2, pages 223-225, Figures 6.3.7-6.3.10.

# Verification Notes
- Definition source: Section 6.3.2, with geometric visualization and multiple examples
- Confidence rationale: High — explicitly defined with formal geometric grounding
- Cross-reference status: Connected to three-note chord space discussion in Chapter 3
