---
# === CORE IDENTIFICATION ===
concept: Pitch-Class Circulation Graph
slug: pitch-class-circulation

# === CLASSIFICATION ===
category: analysis
subcategory: macroharmony
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 176
section: "5.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "pitch-class circulation"
  - "PC circulation graph"
  - "chromaticism graph"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - macroharmony
extends: []
related:
  - global-macroharmonic-profile
  - macroharmonic-consistency
  - pitch-class-circulation-rate
  - chromaticism-quantification
contrasts_with:
  - global-macroharmonic-profile

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can we quantify how 'chromatic' a piece is?"
  - "How fast do pitch classes circulate in a given piece?"
  - "How do different composers compare in chromaticism?"
---

# Quick Definition
A pitch-class circulation graph plots how many distinct pitch classes appear in windows of various sizes, providing a quantitative measure of how quickly a piece cycles through the available notes and thus how "chromatic" it is.

# Core Definition
A pitch-class circulation graph is constructed by examining every window of n consecutive note attacks in a piece and counting the average number of distinct pitch classes. The x-axis represents window size (number of consecutive notes) and the y-axis represents the average number of distinct pitch classes found. The graph rises from 1 (a single note) toward some plateau, with the shape revealing macroharmonic properties. A piece that stays within a 7-note diatonic scale will plateau near 7; a piece using all 12 chromatic notes will plateau near 12. The speed of the rise reveals how quickly the music exhausts its pitch vocabulary, while the slope after the initial rise indicates the rate of macroharmonic change. These are "very crude tools" — they do not reveal the character of the macroharmonies, and can be influenced by texture and tempo — but they provide a first quantitative grip on chromaticism.

# Prerequisites
- **macroharmony** — The graphs measure macroharmonic properties

# Key Properties
1. Constructed from sliding windows of n consecutive note attacks
2. The plateau height indicates macroharmony size (7 for diatonic, 12 for chromatic)
3. The plateau sharpness indicates macroharmonic stability (sharp = static, gradual = modulating)
4. Susceptible to tempo differences and textural features (tremolo, repeated notes)
5. Require "linearizing" simultaneous attacks into sequential order

# Construction / Recognition
## To Construct/Create:
1. Linearize the music by converting simultaneous attacks into a single stream (random ordering)
2. For each window size n from 1 to some maximum, examine every n-note window
3. Count distinct pitch classes in each window and compute the average
4. Plot window size (x) against average number of pitch classes (y)
## To Identify/Recognize:
1. Look for the characteristic shape: rapid rise followed by flattening
2. Compare the plateau level and slope to reference pieces
3. Note whether the curve levels off sharply (macroharmonically static) or gradually (modulating)

# Context & Application
Tymoczko uses these graphs to survey the chromaticism of Western music from Palestrina to Webern. Palestrina's curve rises quickly to about 7 (diatonic) and flattens sharply. Mozart, Beethoven, Brahms, and Wagner show curves rising quickly then leveling off gradually — reflecting modulation that introduces new pitch classes over time. Schoenberg and Webern rise quickly to 12 and flatten completely. The graphs confirm a music-historical truism: chromaticism increased from Palestrina through Wagner, with atonality representing a relatively natural endpoint. They also reveal that a typical 10-note classical excerpt contains roughly half the pitch classes of a typical 100-note excerpt — a log-linear relationship.

# Examples
**Example 1** (p. 177-178, Figures 5.3.1-5.3.2): Bach's F major two-part invention theme: every 3-note window contains on average 2.4 pitch classes, every 4-note window 2.9. The full graph shows rapid rise then gentle leveling characteristic of diatonic music with mild chromaticism.

**Example 2** (p. 178, Figure 5.3.2): Comparative graphs: Palestrina (bottom, leveling at ~7), Mozart/Beethoven/Brahms/Wagner (middle, gradually rising), Webern (top, sharp plateau at 12). The historical increase in chromaticism is clearly visible.

# Relationships
## Builds Upon
- **macroharmony** — Graphs measure macroharmonic properties
## Enables
- **chromaticism-quantification** — Primary tool for quantifying chromaticism
- **pitch-class-circulation-rate** — The rate of change is a key variable
## Related
- **global-macroharmonic-profile** — Complementary tool that identifies WHICH macroharmonies appear
- **macroharmonic-consistency** — Circulation rate alone cannot distinguish consistent from inconsistent macroharmony
## Contrasts With
- **global-macroharmonic-profile** — Circulation graphs show HOW FAST; profiles show WHAT

# Common Errors
- **Error**: Interpreting these graphs as definitive measures of chromaticism
  **Correction**: Tymoczko cautions they are "very crude tools" affected by tempo, texture, and the linearization of simultaneous attacks

# Common Confusions
- **Confusion**: Assuming high pitch-class circulation implies lack of macroharmonic consistency
  **Clarification**: Coltrane's "Giant Steps" has circulation as fast as Schoenberg's Op. 11, but Coltrane clearly articulates diatonic scales (rapid modulation, not atonality)

# Source Reference
Chapter 5: Macroharmony and Centricity, Section 5.3, pages 176-179, Figures 5.3.1-5.3.2.

# Verification Notes
- Definition source: Section 5.3, with detailed construction method and examples
- Confidence rationale: High — novel analytical tool with extensive illustration
- Cross-reference status: Used throughout Chapters 5, 9, and 10
