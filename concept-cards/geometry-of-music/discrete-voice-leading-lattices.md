---
concept: Discrete Voice-Leading Lattices
slug: discrete-voice-leading-lattices

category: geometric-theory
subcategory: discrete-geometry
tier: advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Appendix C: Discrete Voice-Leading Lattices"
chapter_number: null
pdf_page: 430
section: "Appendix C"

extraction_confidence: high

aliases:
  - "voice-leading graphs"
  - "faithful lattices"

prerequisites:
  - chord-space-formal-construction
extends: []
related:
  - scale-lattice-twentieth-century
contrasts_with: []

answers_questions:
  - "When do discrete voice-leading graphs faithfully represent voice-leading distances?"
  - "What problems arise with graphs like the Tonnetz?"
  - "What five criteria ensure a lattice is reliable?"
---

# Quick Definition
Criteria for ensuring that discrete graphs of voice-leading relationships faithfully represent voice-leading distances, addressing the problem that many common graphs (including the Tonnetz) have local structure that does not generalize to global distances.

# Core Definition
Many discrete music-theoretical graphs have clear local structure but unreliable global structure. The Tonnetz places F major two steps from C major but F minor three steps away, even though F minor is closer by voice leading. The problem arises when graphs omit chords that divide the octave more evenly than those included. Five criteria ensure faithfulness: (1) every edge represents single-step voice leading; (2) the graph contains all interscalar transpositions between any two of its chords; (3) all chords have the same size; (4) paths representing interscalar transpositions involve no ascending-descending motion in the same voice; (5) no multisets. All lattices in Section 3.11 satisfy these criteria. Douthett and Steinbach's "Cube Dance" is virtually identical to the lattice at the center of three-note chord space.

# Prerequisites
- Understanding of chord space formal construction

# Key Properties
1. Local graph structure may not extend to global distances
2. The Tonnetz fails to faithfully represent voice-leading distances
3. Faithful graphs must include all chords at least as even as those of interest
4. Five explicit criteria for faithfulness
5. Geometry provides deeper understanding of reliable lattices
6. The continuous spaces of Chapter 3 subsume the discrete graphs

# Construction / Recognition
## To Construct/Create:
1. Select chord types of interest
2. Include all chord types dividing the octave at least as evenly
3. Connect all chords differing by single-step voice leading
4. Verify the five faithfulness criteria
## To Identify/Recognize:
1. Check whether the graph includes all interscalar transpositions
2. Verify no "shortcuts" through more-even chords are missing

# Context & Application
This appendix addresses widespread uncritical use of graphs like the Tonnetz whose global distances are meaningless for voice-leading purposes.

# Examples
**Example 1** (p. 431-432): The Tonnetz's failure -- F minor is three steps from C major despite being closer by voice leading.

**Example 2** (p. 434, Fig. C5): Douthett and Steinbach's "Cube Dance" is faithful because it includes augmented triads at the center.

# Relationships
## Builds Upon
- **chord-space-formal-construction** -- The continuous spaces of which lattices are subgraphs
## Enables
- Principled use of voice-leading graphs in analysis
## Related
- **scale-lattice-twentieth-century** -- Scale lattices follow the same principles

# Common Errors
- **Error**: Assuming any voice-leading graph faithfully represents distances
  **Correction**: Only graphs meeting the five criteria are reliable for global comparisons

# Source Reference
Appendix C: Discrete Voice-Leading Lattices, pages 430-435.

# Verification Notes
- Definition source: Five criteria explicitly stated
- Confidence rationale: High -- formal criteria with proofs
- Cross-reference status: Validates lattices used throughout the book
