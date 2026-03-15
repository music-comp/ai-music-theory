---
# === CORE IDENTIFICATION ===
concept: Chromaticism Quantification
slug: chromaticism-quantification

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
pdf_page: 182
section: "5.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "measuring chromaticism"
  - "chromatic vs. nonchromatic"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class-circulation
  - macroharmonic-consistency
extends:
  - macroharmony
related:
  - global-macroharmonic-profile
  - chromatic-tradition
  - scalar-tradition
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can we quantify how 'chromatic' a piece is?"
  - "What features distinguish 19th-century chromaticism from 20th-century scalar composition?"
  - "Is the term 'chromatic' adequate for describing musical style?"
---

# Quick Definition
The informal term "chromatic" actually involves at least two independent variables — rate of pitch-class circulation and degree of macroharmonic consistency — which together create a two-dimensional space of musical possibilities rather than a simple chromatic/non-chromatic binary.

# Core Definition
Tymoczko argues that the simple opposition between "chromatic" and "non-chromatic" music is inadequate because it conflates two independent properties. Rate of pitch-class circulation measures how fast a piece cycles through available notes. Macroharmonic consistency measures whether the piece emphasizes particular collections (like the diatonic scale). These two variables create a 2D space (Figure 5.5.4): high circulation + low consistency = very chromatic (Schoenberg); low circulation + high consistency = non-chromatic (Debussy); high circulation + high consistency = rapidly modulating but tonal (Coltrane); low circulation + low consistency = slowly chromatic without clear scales (Satie). This framework reveals that pieces previously lumped together as "chromatic" or "non-chromatic" may actually be quite different in their macroharmonic organization.

# Prerequisites
- **pitch-class-circulation** — One of the two independent variables
- **macroharmonic-consistency** — The other independent variable

# Key Properties
1. "Chromatic" is not a single-axis concept but involves at least two dimensions
2. Pieces with identical circulation rates can differ dramatically in macroharmonic consistency
3. The 2D space explains why Coltrane and Schoenberg sound so different despite similar circulation rates
4. Modulating tonal music and atonal music are distinguishable only by macroharmonic consistency, not circulation rate
5. The framework replaces "tonal/atonal" with more fine-grained categories

# Construction / Recognition
## To Construct/Create:
1. Compute the pitch-class circulation graph for a piece
2. Compute the global macroharmonic profile (identifying prominent collections)
3. Place the piece in the 2D space of circulation rate vs. macroharmonic consistency
## To Identify/Recognize:
1. Compare circulation graphs to reference pieces
2. Check whether the piece emphasizes particular collections (peaked profile) or uses many evenly (flat profile)
3. Locate the piece in the 2D classification space

# Context & Application
This framework resolves a persistent confusion in music theory. Schoenberg's Op. 11 No. 1 and Coltrane's "Giant Steps" solo have nearly identical rates of pitch-class circulation, yet one is atonal and the other is clearly tonal. The difference lies in macroharmonic consistency: Coltrane's graph of seven-note collections is highly peaked at the diatonic scale, while Schoenberg's is relatively flat. Similarly, Debussy's "La fille aux cheveux de lin" and Satie's "Theme of the Order" both have low circulation, but Debussy clearly articulates specific scales while Satie's accidentals appear sporadically without system.

# Examples
**Example 1** (p. 182-184, Figures 5.5.1-5.5.2): Schoenberg's Op. 11 and Coltrane's "Giant Steps" — identical pitch-class circulation rates but completely different macroharmonic profiles. Coltrane's is sharply peaked at the diatonic scale; Schoenberg's is flat across all set classes.

**Example 2** (p. 185, Figure 5.5.4): The four-quadrant classification: Schoenberg (high circulation, low consistency), Debussy (low circulation, high consistency), Coltrane (high circulation, high consistency), Satie (low circulation, low consistency).

# Relationships
## Builds Upon
- **pitch-class-circulation** — One axis of the 2D space
- **macroharmonic-consistency** — The other axis
## Enables
- **chromatic-tradition** — Understanding the trajectory of chromatic music
- **scalar-tradition** — Understanding the alternative to chromaticism
## Related
- **global-macroharmonic-profile** — The tool that reveals macroharmonic consistency
## Contrasts With
- None specifically

# Common Errors
- **Error**: Treating "chromatic" as a unidimensional concept
  **Correction**: At minimum, two independent variables (circulation rate and macroharmonic consistency) must be considered

# Common Confusions
- **Confusion**: Assuming fast pitch-class circulation means atonal music
  **Clarification**: Coltrane's "Giant Steps" circulates through pitch classes as fast as Schoenberg but is clearly tonal — the difference is macroharmonic consistency (clear diatonic regions vs. no clear collections)

# Source Reference
Chapter 5: Macroharmony and Centricity, Sections 5.3-5.5, pages 176-185, Figures 5.5.1-5.5.4.

# Verification Notes
- Definition source: Synthesized from Sections 5.3-5.5, especially Figure 5.5.4
- Confidence rationale: High — central analytical argument of the chapter
- Cross-reference status: Framework applied throughout Chapters 9 and 10
