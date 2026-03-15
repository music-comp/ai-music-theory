---
# === CORE IDENTIFICATION ===
concept: Key Distance
slug: key-distance

# === CLASSIFICATION ===
category: harmony
subcategory: modulation
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 205
section: "5.8.2"

# === CONFIDENCE ===
extraction_confidence: medium-high

# === VARIANTS ===
aliases:
  - "tonal distance"
  - "distance between keys"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - macroharmony
  - generalized-theory-of-keys
extends: []
related:
  - modulation-as-circulation-change
  - nineteenth-century-chromaticism
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do we measure distance between keys?"
  - "What is key distance?"
  - "How does modulation work geometrically?"
---

# Quick Definition
Key distance can be measured geometrically as voice-leading distance between scales in seven-dimensional scale space, capturing the intuition that closely related keys share most of their pitch classes while distant keys differ by many notes.

# Core Definition
In traditional theory, key distance is measured by the number of shared pitch classes between scales (the circle of fifths measures this for diatonic keys). Tymoczko generalizes this using the voice-leading spaces of Chapter 3: scales are points in a high-dimensional space, and the distance between them is the total voice-leading motion required to transform one into another. For diatonic scales, adjacent keys on the circle of fifths differ by one note (one semitone of voice-leading motion). For the generalized key space of twentieth-century music — encompassing diatonic, acoustic, harmonic minor, harmonic major, octatonic, and other scales — distances can be measured in the same geometric framework. A chromatic chord like Schumann's A7 can be "close" by chromatic voice leading (semitone connections to neighboring chords) yet "distant" by key association (A7 suggests D major/minor, far from Ab major). This duality of nearness is central to understanding chromatic music.

# Prerequisites
- **macroharmony** — Keys are macroharmonies with centers
- **generalized-theory-of-keys** — The expanded key concept

# Key Properties
1. Traditional measure: number of shared pitch classes (circle of fifths)
2. Geometric measure: voice-leading distance in scale space
3. Adjacent diatonic keys differ by one semitone of voice-leading motion
4. Chromatic music exploits the discrepancy between chord distance and key distance
5. In the generalized key space, distance includes both pitch-class overlap and scale-type similarity
6. Modulations can be visualized as paths on the scale lattice

# Construction / Recognition
## To Construct/Create:
1. Identify two keys (scale + center)
2. Count the minimum total semitone motion needed to transform one scale into the other
3. Alternatively, count shared pitch classes (more shared = closer)
## To Identify/Recognize:
1. Keys sharing many pitch classes are close (e.g., C major and G major share 6 of 7 notes)
2. Keys sharing few pitch classes are distant (e.g., C major and F# major share only 2 of 7 notes)
3. On the scale lattice, adjacent keys are connected by single-step voice leadings

# Context & Application
Key distance is essential for understanding modulation in all periods. Bach's modulation from D major to A major (Figure 6.4.4) is a single step on the scale lattice — close keys. Schumann's A7 chord in "Chopin" (Section 6.5) suggests D major, which is "distant" from the tonic Ab major by key measure, even though the chord itself is "close" by voice-leading measure. This duality — melodic closeness vs. tonal distance — is central to the expressive power of chromatic music. In the twentieth century, key distance expands to include distances between different scale types (diatonic to acoustic, diatonic to octatonic, etc.).

# Examples
**Example 1** (p. 232, Figure 6.4.4): Bach's modulations D major -> A major -> B minor -> D major, shown as minimal steps on the scale lattice.

**Example 2** (p. 234): Schumann's "Chopin" — A7 chord is chromatically close to F minor and Eb7 (semitone voice leading) but tonally distant (D major vs. Ab major).

# Relationships
## Builds Upon
- **macroharmony** — Keys are macroharmonies
- **generalized-theory-of-keys** — The expanded framework for key distance
## Enables
- Analysis of modulation paths
## Related
- **modulation-as-circulation-change** — Modulation introduces new pitch classes proportional to key distance
- **nineteenth-century-chromaticism** — Exploits chord/key distance discrepancy
## Contrasts With
- None specifically

# Common Errors
- **Error**: Using only the circle of fifths to measure key distance
  **Correction**: The circle of fifths works for diatonic-to-diatonic distance, but the generalized framework handles distance between different scale types

# Common Confusions
- **Confusion**: Conflating chord distance with key distance
  **Clarification**: A chord can be chromatically near its neighbors (small voice-leading distance) while suggesting a key that is tonally distant — this duality drives chromatic expressivity

# Source Reference
Chapter 5, Section 5.8.2 (generalized key space); Chapter 6, Sections 6.4-6.5 (modulation and chromatic distance).

# Verification Notes
- Definition source: Synthesized from Chapters 5 and 6 discussion
- Confidence rationale: Medium-high — the concept is discussed across sections but not given a single focal definition
- Cross-reference status: Chapter 7 develops key distance more fully
