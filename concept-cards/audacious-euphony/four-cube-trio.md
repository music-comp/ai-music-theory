---
concept: 4-Cube Trio
slug: four-cube-trio

category: neo-riemannian-theory
subcategory: voice-leading maps
tier: advanced

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Dissonance"
chapter_number: 7
pdf_page: 175
section: "4-Cube Trio"

extraction_confidence: high

aliases:
  - "Power Towers (original name)"

prerequisites:
  - boretz-region
  - boretz-spider
  - octatonic-region
  - tristan-genus
extends: []
related:
  - cube-dance
  - tristan-genus-voice-leading-zones
  - circumnavigation-tetrachordal
contrasts_with:
  - cube-dance

answers_questions:
  - "What is 4-Cube Trio?"
  - "How does the voice-leading map for seventh chords compare to Cube Dance for triads?"
---

# Quick Definition
4-Cube Trio is Jack Douthett's graph portraying the entire voice-leading system of Tristan-genus chords, diminished seventh chords, minor seventh chords, and French sixth chords -- the tetrachordal analogue to Cube Dance for triads.

# Core Definition
**4-Cube Trio** (Figure 7.16) "portrays the entire system of interlocking Boretz and octatonic regions. As in the triadic case, the spiders and pools are subject to figure-ground reversals" (p. 175). The graph contains three Boretz spiders at 2:00, 6:00, and 10:00, connected by octatonic bridges at 12:00, 4:00, and 8:00. "Both of these chord types [minor seventh and French sixth] appear simultaneously as octatonic bridges... Triangular nodes indicate minor seventh chords, and stars indicate French sixth chords. As both fulfill the bridging function independently, either can be removed without disconnecting the graph" (p. 175). Originally named "Power Towers" by Douthett (1993); renamed in Douthett & Steinbach 1998. Tymoczko (2011b, 106) independently rediscovered a version, showing it is "an accurate model of voice leading among its included chords."

# Prerequisites
- **Boretz region**: The spider structures within the graph
- **Boretz spider**: The subgraphs at hub positions
- **Octatonic region**: The bridge structures connecting spiders
- **Tristan genus**: The primary chord type populating the graph

# Key Properties
1. Contains all 24 Tristan-genus chords + 3 diminished sevenths + 12 minor sevenths + 6 French sixths
2. Tristan-genus chords occupy odd voice-leading zones (1, 3, 5, 7, 9, 11)
3. Diminished sevenths, minor sevenths, French sixths occupy even zones
4. Voice-leading distance = shortest path edge count (Tymoczko 2011b)
5. Links three four-dimensional cubes (tesseracts), hence the name
6. Subject to figure-ground reversal between Boretz and octatonic perspectives

# Construction / Recognition
Structure:
- Three Boretz spiders at 2:00, 6:00, 10:00 (centered on diminished seventh chords)
- Three octatonic bridge zones at 12:00, 4:00, 8:00
- At each bridge position: minor seventh chords (triangles) and French sixth chords (stars)
- Edges connect chords differing by single semitonal displacement
- Either bridge type can be removed without disconnecting the graph

# Context & Application
4-Cube Trio serves as the unified voice-leading space for tetrachordal analysis, enabling circumnavigation through transpositional sequences. Like Cube Dance, it is a true model of voice leading where edge distances represent most efficient voice leadings. Passages that foreground Boretz-region membership and passages that foreground octatonic regions represent different figure-ground perspectives on the same graph.

# Examples
- **Chopin e minor Prelude**: First phrase's circumnavigatory path wrapped about 4-Cube Trio (Figure 7.25, pp. 181-182)
- **Chopin Prelude Op. 45**: Extended downshifting roulade circumnavigating 4-Cube Trio multiple times (p. 177)
- **Tristan and Gotterdammerung passages**: Boretz-region motion within spiders, with octatonic bridges for interregional modulation (pp. 170-175)

# Relationships
## Builds Upon
- Boretz region and Boretz spider (the hub structures)
- Octatonic region (the bridge structures)
## Enables
- Circumnavigation analysis of tetrachordal passages
- Visual tracking of voice-leading trajectories through the seventh-chord universe
## Related
- Cube Dance (triadic analogue)
- Voice-leading zones (the organizing metric for positions on the graph)
## Contrasts With
- Cube Dance (triadic: 24 triads + 4 augmented triads; tetrachordal: 24 Tristan-genus + 3 diminished sevenths + bridges)

# Common Errors
- **Error**: Thinking minor seventh and French sixth chords are structurally necessary at every bridge position
  **Correction**: "Either can be removed without disconnecting the graph" -- they provide alternative bridges

# Common Confusions
- **Confusion**: Assuming 4-Cube Trio and Cube Dance represent the same harmonic space
  **Clarification**: They are independent systems for different chord cardinalities; chords in one system have no direct relationship to chords in the other

# Source Reference
Cohn, R. *Audacious Euphony*, Chapter 7: "Dissonance," pp. 174-176. See also Douthett & Steinbach 1998; Tymoczko 2011b, 106.

# Verification Notes
Re-extracted from v2 card; preserved: node types and their shapes, bridge removability. Fresh extraction adds Power Towers history, Tymoczko verification, figure-ground reversal concept, and direct quotations.
