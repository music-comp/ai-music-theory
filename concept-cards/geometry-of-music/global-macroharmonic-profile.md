---
concept: Global Macroharmonic Profile
slug: global-macroharmonic-profile

category: analysis
subcategory: macroharmony
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 182
section: "5.5"

extraction_confidence: high

aliases:
  - "macroharmonic profile"
  - "collection distribution"

prerequisites:
  - macroharmony
  - pitch-class-circulation
extends: []
related:
  - macroharmonic-consistency
  - chromaticism-quantification
contrasts_with:
  - pitch-class-circulation

answers_questions:
  - "How can we identify which macroharmonies a piece uses?"
  - "How do we measure macroharmonic consistency?"
  - "What distinguishes Schoenberg's chromaticism from Coltrane's?"
---

# Quick Definition
A global macroharmonic profile is a bar graph showing the relative frequency of different five-to-eight-note collections in a piece, revealing which macroharmonies the music emphasizes and thus measuring macroharmonic consistency.

# Core Definition
Where pitch-class circulation graphs show how fast pitch classes pass by, global macroharmonic profiles show what the macroharmonies actually are. To construct a profile, one exhaustively tabulates all n-note chord types (for various n) found in consecutive windows throughout a piece. The resulting bar graph has set classes on the x-axis and their frequency of occurrence on the y-axis. A peaked profile — with one or a few collections strongly emphasized — indicates high macroharmonic consistency (the piece clearly articulates specific scales). A flat profile — with many collections appearing at similar frequencies — indicates low consistency (no particular collection dominates). This tool complements pitch-class circulation graphs by answering a different question: not "how fast?" but "what?"

# Prerequisites
- **macroharmony** — Understanding the concept being profiled
- **pitch-class-circulation** — Understanding the complementary measurement tool

# Key Properties
1. Tabulates frequency of occurrence of each n-note set class in a piece
2. A peaked graph indicates macroharmonic consistency (the piece uses particular collections)
3. A flat graph indicates macroharmonic inconsistency (many collections used equally)
4. The specific peaks identify which collections dominate (diatonic, octatonic, etc.)
5. Can be computed for various collection sizes (6-note, 7-note, 8-note)

# Construction / Recognition
## To Construct/Create:
1. Linearize simultaneous attacks (random arpeggiation)
2. For each note position i and collection size n, identify the n-note set class beginning at note i
3. Count occurrences of each set class
4. Plot set classes on x-axis and frequency on y-axis
## To Identify/Recognize:
1. A highly peaked profile signals clear macroharmonic consistency
2. A flat profile signals macroharmonic inconsistency
3. The location of peaks identifies the dominant collections

# Context & Application
Global macroharmonic profiles are essential for distinguishing musical styles that may have identical circulation rates. Tymoczko's analysis of the Rite of Spring (Figure 5.5.5) reveals a large-scale macroharmonic trajectory: the introduction is chromatic (no dominant collection), the "Dance of the Adolescents" emphasizes harmonic and melodic minor, the "Ritual of Abduction" is 18% octatonic, and "Spring Rounds" is 53% diatonic. This shows Stravinsky's piece progressing from chromaticism to diatonicism, mediated by nondiatonic scales. Similarly, Shostakovich's Preludes and Fugues are shown to be "profoundly and almost stubbornly diatonic" — more so than Bach's minor-key works.

# Examples
**Example 1** (p. 183-184, Figure 5.5.2): Seven-note collection profiles for Schoenberg's Op. 11 vs. Coltrane's "Giant Steps": Coltrane's is highly peaked at the diatonic scale; Schoenberg's is relatively flat across all set classes.

**Example 2** (p. 185-186, Figure 5.5.5): The Rite of Spring's first four sections show distinct macroharmonic profiles: Introduction (no dominant collection), Dance of the Adolescents (harmonic minor 18%, melodic minor 16%), Ritual of Abduction (octatonic 18%), Spring Rounds (diatonic 53%).

**Example 3** (p. 187, Figure 5.5.6): Shostakovich's Preludes and Fugues: the diatonic scale is the most common seven-note collection in all but four of the 48 pieces, and in some it is the only macroharmony. Minor-key pieces are 56% diatonic vs. Bach's 20%.

# Relationships
## Builds Upon
- **macroharmony** — Profiles measure macroharmonic content
- **pitch-class-circulation** — Complementary measurement tool
## Enables
- **macroharmonic-consistency** — Profiles directly measure consistency
- **chromaticism-quantification** — One dimension of the chromaticism framework
## Related
- None additional
## Contrasts With
- **pitch-class-circulation** — Profiles show WHAT collections; circulation graphs show HOW FAST

# Common Errors
- **Error**: Using only circulation graphs to assess chromaticism
  **Correction**: Circulation graphs cannot distinguish rapidly modulating tonal music from atonal music; macroharmonic profiles are needed for this distinction

# Common Confusions
- **Confusion**: Thinking a peaked profile means the music stays in one key
  **Clarification**: Coltrane's "Giant Steps" has a strongly peaked diatonic profile despite modulating extremely rapidly — the peaks reflect which collection TYPE dominates, not stasis in a single key

# Source Reference
Chapter 5: Macroharmony and Centricity, Section 5.5, pages 182-187, Figures 5.5.1-5.5.6.

# Verification Notes
- Definition source: Section 5.5, with detailed construction and multiple examples
- Confidence rationale: High — novel analytical tool with extensive illustration
- Cross-reference status: Applied in Chapters 9 and 10 analyses
