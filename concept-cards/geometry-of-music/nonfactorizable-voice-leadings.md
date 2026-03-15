---
# === CORE IDENTIFICATION ===
concept: Nonfactorizable Voice Leadings
slug: nonfactorizable-voice-leadings

# === CLASSIFICATION ===
category: voice-leading
subcategory: four-voice-techniques
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Functional Harmony"
chapter_number: 7
pdf_page: 254
section: "7.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "nonfactorizable four-voice voice leadings"
  - "split-merge voice leadings"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - efficient-voice-leading
extends: []
related:
  - three-plus-one-schema
  - figured-bass-pedagogy
contrasts_with:
  - three-plus-one-schema

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the nine types of nonfactorizable four-voice triadic voice leadings?"
  - "How do nonfactorizable voice leadings complement the 3+1 schema?"
  - "What near-symmetries of the triad do nonfactorizable voice leadings exploit?"
---

# Quick Definition
A nonfactorizable four-voice triadic voice leading is one where no voice can be eliminated without creating an incomplete chord — there are exactly 9 basic types, each following a "split-merge" pattern where one note splits into two while two notes merge onto one.

# Core Definition
A four-voice triadic voice leading is nonfactorizable when eliminating any single voice would create an incomplete triad. This occurs only when the voice leading follows a specific schema: one note in the first chord "splits" into two adjacent notes in the second chord, while two notes in the first chord "merge" onto the remaining note in the second. There are 3 x 3 = 9 basic possibilities, depending on whether the splitting and merged-upon notes are the root, third, or fifth of their respective triads. For example, a chord with doubled root can map to a chord with doubled root (exploiting the triad's proximity to the tritone), or to a chord with doubled third, and so on. Together with the 3+1 schema, nonfactorizable voice leadings generate *all* strongly crossing-free four-voice triadic voice leadings, including all those in which each note moves to its nearest neighbor.

# Prerequisites
- Understanding of efficient voice leading and the concept of crossing-free voice leadings
- The 3+1 schema from Chapter 6

# Key Properties
1. Exactly 9 basic types (3 choices for splitting note x 3 choices for merge target)
2. In each, one note splits into two and two notes converge onto one
3. They exploit various near-symmetries of the triad: proximity to tritone, diminished seventh, and quadruple unison
4. Together with the 3+1 schema, they produce ALL strongly crossing-free four-voice triadic voice leadings
5. In keyboard style, they correspond to moving from a complete triad to an incomplete chord in the right hand

# Construction / Recognition
## To Construct/Create:
1. Start with a four-voice triad (one note doubled)
2. The doubled note "merges" — both copies move to a single note in the next chord
3. One of the other notes "splits" — it sends voices to two different notes in the next chord
4. The result is another four-voice triad with one note doubled

## To Identify/Recognize:
1. In a four-voice passage, check if removing any voice creates an incomplete triad
2. If no voice can be removed without incompleteness, the voice leading is nonfactorizable
3. Look for the split-merge pattern: one note fanning out, two notes converging

# Context & Application
Nonfactorizable voice leadings, combined with the 3+1 schema, account for a substantial proportion of four-voice triadic voice leadings from Dufay to Bach. In Palestrina, they account for more than 20% of voice leadings; in Lassus, only about 5%. This suggests composer-by-composer variation rather than broad historical change, reinforcing the claim of continuity between Renaissance and functional tonal practices. In keyboard pedagogy, these correspond to simple physical gestures: the right hand moves from a close-position triad to an incomplete chord while the left hand completes the second triad.

# Examples
**Example 1** (Fig. 7.2.10, p. 254): Two nonfactorizable voice leadings — the first maps doubled root to doubled root, the second maps doubled root to doubled third.

**Example 2** (Fig. 7.2.12, p. 255): Statistical data showing that the 3+1 and nonfactorizable schemas together account for 70-95% of four-voice triadic voice leadings across composers from Dufay to Bach.

**Example 3** (Fig. 7.2.13, p. 256): Nonfactorizable voice leadings in keyboard style — close-position triad in right hand moves to an incomplete chord while bass completes the triad.

# Relationships
## Builds Upon
- **efficient-voice-leading** — Nonfactorizable voice leadings are efficient, crossing-free connections
## Enables
- **figured-bass-pedagogy** — These schemas are embedded in traditional figured-bass teaching
- Complete inventory of four-voice triadic voice leading
## Related
- **three-plus-one-schema** — Together they generate all strongly crossing-free four-voice triadic voice leadings
## Contrasts With
- **three-plus-one-schema** — In the 3+1 schema, three voices move between complete triads and one moves independently; in nonfactorizable, the structure is split-merge

# Common Errors
- **Error**: Assuming nonfactorizable means "unusual" or "rare"
  **Correction**: They account for a significant portion of Renaissance and baroque voice leading (up to 20% in Palestrina)

# Common Confusions
- **Confusion**: Thinking every keyboard voice leading that moves from complete to incomplete triad is nonfactorizable
  **Clarification**: Not every such movement is nonfactorizable — some are factorizable (e.g., (C3, E4, G4, C5) to (F3, C4, A4, C5) is factorizable)

# Source Reference
Chapter 7: Functional Harmony, Section 7.2, pages 254-256, Figures 7.2.10-7.2.13. See also Appendix F for exercises.

# Verification Notes
- Definition source: Directly from Section 7.2 with formal definition and enumeration
- Confidence rationale: High — major theoretical contribution with precise mathematical characterization
- Cross-reference status: Verified against Appendix F exercises and statistical data
