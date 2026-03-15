---
# === CORE IDENTIFICATION ===
concept: Half-Diminished to Dominant Seventh Voice Leadings
slug: half-diminished-to-dominant-voice-leadings

# === CLASSIFICATION ===
category: voice-leading
subcategory: chromatic-voice-leading
tier: advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Chromaticism"
chapter_number: 8
pdf_page: 311
section: "8.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Tristan chord resolutions"
  - "iiø7-V7 voice leadings"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - efficient-voice-leading
  - tesseract-model-dominant-sevenths
extends: []
related:
  - tristan-chord-analysis
  - generalized-augmented-sixths
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How many efficient voice leadings exist from a half-diminished seventh to dominant seventh chords?"
  - "How are these voice leadings organized in four-note chord space?"
---

# Quick Definition
From any half-diminished seventh, there are eight most-efficient voice leadings to the twelve dominant sevenths: seven require only two semitones of total motion, one requires four, organized in pairs by root-to-root, root-to-third, root-to-fifth, and root-to-seventh mappings.

# Core Definition
Figure 8.6.1 catalogs the most efficient four-voice voice leading from Fø7 to each of the twelve dominant sevenths. Seven voice leadings involve just two semitones of total motion, one involves four, and the remaining four involve six. Setting aside the distant ones, the eight closest can be grouped into pairs: two move root to root (Fø7->E7 and Fø7->F7), two move root to third (Fø7->Db7 and Fø7->D7), two move root to fifth (Fø7->Bb7 and Fø7->B7), and two move root to seventh (Fø7->Ab7 and Fø7->G7). In each pair, one involves predominantly descending motion and the other predominantly ascending. When moving to the lower four dominant sevenths, one can pass through F diminished; when moving to the upper four, one passes through minor sevenths and French sixths.

# Prerequisites
- Efficient voice leading and the crossing-avoidance principle
- Four-note chord space geometry

# Key Properties
1. Eight most efficient voice leadings from any half-diminished seventh
2. Organized by root-to-X mapping (root, third, fifth, seventh)
3. Each pair contains one predominantly descending, one ascending
4. Two total semitones for seven of the eight; four semitones for one
5. Intermediate chords (diminished seventh, minor seventh, French sixth) lie between
6. Wagner uses all eight types in the Tristan opera

# Construction / Recognition
## To Construct/Create:
1. Start with a half-diminished seventh (e.g., Fø7 = F, Ab, Cb, Eb)
2. For each of the 12 dominant sevenths, find the most efficient crossing-free voice leading
3. Group by which note the half-diminished root maps to
4. For analysis, classify any half-diminished-to-dominant progression using this scheme

## To Identify/Recognize:
1. A half-diminished seventh resolving to a dominant seventh
2. Identify where the root moves (to the dominant's root, third, fifth, or seventh)
3. Check total voice-leading distance (most efficient ones total 2-4 semitones)

# Context & Application
This catalog is the analytical backbone of the Tristan analysis. It provides a complete inventory of efficient resolutions, allowing systematic comparison of different resolutions across the opera. It also connects to jazz practice, where ii-half-dim-V7 progressions are fundamental.

# Examples
**Example 1** (Fig. 8.6.1, p. 312): Complete catalog of most efficient voice leadings from Fø7 to all twelve dominant sevenths.

**Example 2** (Fig. 8.6.2, p. 313): The eight nearest dominant sevenths arranged in four-note chord space, showing the spatial relationships.

# Relationships
## Builds Upon
- **efficient-voice-leading** — The voice-leading efficiency criterion
- **tesseract-model-dominant-sevenths** — The space in which these are located
## Enables
- **tristan-chord-analysis** — The catalog of resolutions Wagner uses

# Common Errors
- **Error**: Expecting only the standard iiø7-V7 resolution to be efficient
  **Correction**: There are seven equally efficient alternatives and one nearly as efficient

# Common Confusions
- **Confusion**: Thinking the "root to root" mapping always produces the most familiar sound
  **Clarification**: The four mapping types produce four distinct but equally efficient resolution characters

# Source Reference
Chapter 8: Chromaticism, Section 8.6, pages 311-313, Figures 8.6.1-8.6.2.

# Verification Notes
- Definition source: Directly from Section 8.6 with systematic catalog
- Confidence rationale: High — explicitly enumerated with figure
- Cross-reference status: Verified against Wagner opera examples
