---
concept: Tesseract Model for Dominant Sevenths
slug: tesseract-model-dominant-sevenths

category: geometric-theory
subcategory: four-note-chord-space
tier: advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Chromaticism"
chapter_number: 8
pdf_page: 306
section: "8.5"

extraction_confidence: high

aliases:
  - "four-dimensional hypercube"
  - "hypercubic lattice for seventh chords"
  - "4D cube model"

prerequisites:
  - seventh-chord-voice-leading
extends: []
related:
  - chopin-open-form
  - chopin-e-minor-prelude
  - tristan-chord-analysis
  - cubic-lattice-three-note-space
  - minor-third-substitution-seventh-chords
contrasts_with:
  - cubic-lattice-three-note-space

answers_questions:
  - "What is the tesseract and how does it model four-note chord space?"
  - "How do Chopin and Wagner navigate this four-dimensional structure?"
  - "Why do minor-third and tritone-related seventh chords appear near each other?"
---

# Quick Definition
The tesseract (four-dimensional hypercube) lies at the center of four-note chromatic chord space, arranging dominant seventh chords so that minor-third- and tritone-related chords are adjacent — providing the geometric framework for understanding Chopin's, Wagner's, and jazz chromatic voice leading.

# Core Definition
At the center of four-note chromatic chord space lies a four-dimensional cubic lattice (tesseract). Each tesseract has four horizontal layers (1a, 1b, 1c, 1d). A dominant seventh chord sits at the apex (1a); lowering any of the three non-root voices by semitone produces chords on level 1b (minor sevenths and French sixths); further descent reaches half-diminished sevenths on level 1c; the fully diminished seventh sits at the base (1d). Adjacent tesseracts share a vertex, so the lattice continues downward. Chopin's pieces navigate this lattice by taking steps downward through each cube, with different routes producing different sequences. The lattice constrains which root motions are possible from semitonal descent: -1 (semitone), -4 (major third), -7 (fifth), and -10 (ascending major second) semitones — the formula -1 (mod 12/4). This explains why minor-third and tritone-related dominant sevenths are close, underpinning jazz tritone substitution.

# Prerequisites
- Seventh-chord voice leading and its descending character
- The concept of voice-leading geometry from Chapter 3

# Key Properties
1. The tesseract is a 4D hypercube — hard to visualize but mathematically precise
2. Four levels per cube: dominant 7th -> minor 7th/French 6th -> half-diminished 7th -> diminished 7th
3. Minor-third and tritone-related seventh chords are geometrically adjacent
4. Root motions from semitonal descent: -1, -4, -7, -10 semitones (mod 3)
5. Each cube contains chords from a single octatonic scale
6. The tesseract lattice is the seventh-chord analogue of the cubic lattice for triads

# Construction / Recognition
## To Construct/Create:
1. Place a dominant seventh at the top vertex
2. Create three vertices below by lowering third, fifth, or seventh by semitone each
3. Create three more by lowering pairs of these voices
4. Place the diminished seventh at the bottom (all three non-root voices lowered)
5. Connect to adjacent tesseracts by lowering the root

## To Identify/Recognize:
1. A passage of seventh chords connected by semitonal voice leading
2. Root motion by minor third, tritone, or semitone between dominant sevenths
3. Intermediate chords (diminished, half-diminished, French sixth) appearing between dominant sevenths
4. The passage can be mapped as a path through connected hypercubes

# Context & Application
The tesseract is the key to understanding nineteenth-century chromatic voice leading between seventh chords. Chopin's Mazurka and Prelude navigate it differently, Wagner's Tristan uses it for half-diminished-to-dominant voice leadings, and jazz tritone substitution exploits the adjacency of tritone-related chords. Mozart's Symphony No. 40 development section and Beethoven's Op. 54 also navigate this space, showing the geometry applies across centuries. The tesseract demonstrates that Chopin had intuitive understanding of four-dimensional geometry decades before mathematicians formalized it.

# Examples
**Example 1** (Fig. 8.5.4, p. 304): The cube representing one cycle of Chopin's Mazurka — G7 at apex, G diminished at base.

**Example 2** (Fig. 8.5.10, p. 309): The full tesseract lattice showing how connected hypercubes model the complete voice-leading space.

**Example 3** (Fig. 8.5.12, pp. 310): Mozart's Symphony No. 40 development section and Beethoven's Op. 54 as paths through the tesseract.

# Relationships
## Builds Upon
- **seventh-chord-voice-leading** — The descending voice-leading principle that defines the lattice
## Enables
- **chopin-open-form** — The Mazurka's paths through the tesseract
- **chopin-e-minor-prelude** — The Prelude's different paths
- **tristan-chord-analysis** — Wagner's voice leadings are also located on the tesseract
## Related
- **minor-third-substitution-seventh-chords** — Movement within a single tesseract
## Contrasts With
- **cubic-lattice-three-note-space** — Three-dimensional analogue for triads

# Common Errors
- **Error**: Thinking the tesseract requires understanding four spatial dimensions visually
  **Correction**: The tesseract can be understood as connected cubes (layers), each representing one semitonal descent

# Common Confusions
- **Confusion**: Conflating the tesseract with the diatonic chord lattice (Section 7.5)
  **Clarification**: The tesseract is a chromatic structure in four-note space; the diatonic chord lattice is a diatonic structure in three-note space

# Source Reference
Chapter 8: Chromaticism, Section 8.5, pages 306-311, Figures 8.5.4, 8.5.8, 8.5.10-8.5.13.

# Verification Notes
- Definition source: Directly from Section 8.5 with multiple geometric figures
- Confidence rationale: High — central geometric concept applied to multiple pieces
- Cross-reference status: Verified against Chapter 3 discussion of four-note chord space
