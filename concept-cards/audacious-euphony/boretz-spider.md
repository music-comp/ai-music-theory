---
concept: Boretz Spider
slug: boretz-spider

category: neo-riemannian-theory
subcategory: tetrachordal regions
tier: advanced

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Dissonance"
chapter_number: 7
pdf_page: 170
section: "Boretz spiders"

extraction_confidence: high

aliases: []

prerequisites:
  - boretz-region
  - boretz-group-transformations
extends: []
related:
  - weitzmann-water-bug
  - four-cube-trio
  - tristan-genus-voice-leading-zones
contrasts_with:
  - weitzmann-water-bug

answers_questions:
  - "What is a Boretz spider?"
  - "How does the Boretz spider model voice leading within a Boretz region?"
---

# Quick Definition
The Boretz spider is a graph representing voice-leading relationships within a single Boretz region, showing how eight Tristan-genus chords connect through two-unit voice-leading motions around a central diminished seventh chord -- analogous to the Weitzmann water bug for triads.

# Core Definition
The **Boretz spider** (Figure 7.10) models voice leading within a Boretz region. "As with the Weitzmann water bug, any two constituent chords are separated by two units of voice leading" (p. 171). The graph has eight peripheral nodes (4 V7, 4 ø7) surrounding a central diminished seventh. Same-species connections (V7 to V7 or ø7 to ø7) involve contrary semitonal motion, where "the resolution of the 'spoiler' into the diminished seventh body is offset by the displacement of a different voice out of that body, on the same flank" (p. 171). Opposite-species connections involve either two semitones distributed between two voices, or one voice moving by whole step.

# Prerequisites
- **Boretz region**: The regional structure the spider represents
- **Boretz-group transformations**: The operations labeling the spider's edges

# Key Properties
1. Eight nodes (4 V7 + 4 ø7) plus central diminished seventh (often virtual)
2. All edges represent exactly 2 voice-leading units
3. Same-species edges: contrary motion (spoiler resolves, new spoiler on same side)
4. Opposite-species edges: similar motion or whole-step motion
5. Back-and-forth motion across the spider toggles upshift with downshift

# Construction / Recognition
- Place the central diminished seventh at the hub
- Arrange four V7 chords (downward displacements) and four ø7 chords (upward displacements) around it
- Connect chords with edges labeled by Boretz-group transformations (S3(2), S3(4), S6, R*)
- Each edge represents a progression involving 2 units of voice-leading work

# Context & Application
"Back-and-forth motion across a Boretz spider, as across a Weitzmann water bug, toggles downshift with upshift, balancing between adjacent zones in voice-leading space" (p. 171). The spider is a subgraph of 4-Cube Trio. Circumnavigation of the full system requires leaving the spider via octatonic bridges.

# Examples
- **Tristan Prelude opening (chords 1-5)**: Motion across the Boretz spider centered on B-D-F-G#. "The first five chords alternately upshift and downshift, moving back and forth across the Boretz spider and balancing between adjacent zones in voice-leading space. Zone labels... indicate an alternation between zones 3 and 1" (p. 171)
- **Figure 7.10**: Models voice leading within the third Boretz region (D-F-Ab-B) (p. 171)
- **Brunnhilde's Immolation**: Similar spider-traversal pattern within the E-G-A#-C# region (pp. 173-174)

# Relationships
## Builds Upon
- Boretz region (the structure being represented)
## Enables
- Visual analysis of Tristan-genus progressions
- Understanding balanced vs. directed voice-leading trajectories
## Related
- Weitzmann water bug (triadic analogue, 6 nodes vs. 8)
- 4-Cube Trio (the larger graph containing three Boretz spiders)
## Contrasts With
- Weitzmann water bug (spider has 8 nodes vs. bug's 6, reflecting different cardinalities)

# Common Errors
- **Error**: Assuming the spider alone can model all seventh-chord progressions
  **Correction**: The spider represents only intra-regional motion; interregional motion requires octatonic bridges

# Common Confusions
- **Confusion**: Thinking the central diminished seventh must be present in the music
  **Clarification**: The center is often virtual -- it structures the region without sounding

# Source Reference
Cohn, R. *Audacious Euphony*, Chapter 7: "Dissonance," pp. 170-172. Tymoczko 2011b, 371 has a formally identical graph.

# Verification Notes
Re-extracted from v2 card; preserved: voice-leading types (same-species vs. opposite-species), transformation labels. Fresh extraction adds direct quotations on back-and-forth motion and upshift/downshift toggling.
