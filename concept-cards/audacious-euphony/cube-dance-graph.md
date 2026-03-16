---
# === CORE IDENTIFICATION ===
concept: Cube Dance Graph
slug: cube-dance-graph

# === CLASSIFICATION ===
category: representations
subcategory: voice-leading graph
tier: advanced

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 103
section: "How Hexatonic and Weitzmann Regions Interact"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Cube Dance"
  - "Douthett's Cube Dance"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - hexatonic-cycle
  - weitzmann-region
  - augmented-triad-as-switching-station
  - hexatonic-weitzmann-interaction
extends:
  - hexatonic-weitzmann-figure-ground
related:
  - connected-tonnetz
  - voice-leading-zones
  - cube-dance-vs-tonnetz
contrasts_with:
  - connected-tonnetz

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Cube Dance graph?"
  - "How do I navigate the Cube Dance graph?"
  - "What must I know before understanding the Cube Dance?"
---

# Quick Definition
A graph created by Jack Douthett (1992) that represents all twenty-four consonant triads and four augmented triads as a unified voice-leading space, where every distance can be interpreted as representing voice-leading size, and directional motion correlates consistently with melodic direction.

# Core Definition
Cube Dance is a "true" model of voice-leading distance between triads: "every distance can be interpreted as representing voice-leading size" (Tymoczko 2009b, p. 271, cited p. 103). The graph contains four hexatonic cycles (portrayed by circuits of unbroken lines) and four Weitzmann regions (portrayed by broken-line "water bugs"). The augmented triads occupy cardinal points, with minor triads clockwise and major triads counterclockwise from each augmented center. Clockwise motion denotes upshifting; counterclockwise motion denotes downshifting. Published in Douthett and Steinbach 1998.

# Prerequisites
- **Hexatonic cycle**: The four six-triad cycles connected by L and P form the unbroken-line circuits
- **Weitzmann region**: The four six-triad clusters around augmented triads form the broken-line water bugs
- **Augmented triad as hub**: The four augmented triads occupy cardinal points connecting the regions
- **Hexatonic-Weitzmann interaction**: Understanding why both systems must combine for full connectivity

# Key Properties
1. 28 nodes: 24 consonant triads + 4 augmented triads
2. Edges connect triads related by single semitonal displacement
3. Hexatonic cycles appear as 6-node circuits (unbroken lines)
4. Weitzmann regions appear as 6-consonant-triad clusters with augmented centers (broken lines)
5. Clockwise = upshifting; counterclockwise = downshifting
6. Cyclic closure is visually explicit (unlike the Tonnetz)
7. Augmented triads have explicit nodal positions (unlike some other representations)

# Construction / Recognition
1. Place four augmented triads (CEG#, C#FA, DG-Bb, EbGB) at cardinal points
2. Connect each to its six Weitzmann-region consonant triads (broken lines)
3. Connect consonant triads sharing hexatonic regions into six-node cycles (unbroken lines)
4. Minor triads clockwise from each augmented triad; major triads counterclockwise
5. The result shows twelve voice-leading zones as radial positions (figure 5.24)

# Context & Application
Cube Dance unifies hexatonic and Weitzmann structures into a single connected space, enabling analysis of typical nineteenth-century passages that cross regional boundaries. It is the primary representation for tracking voice-leading trajectories, cyclic closure, and zonal patterns. It is preferred over the Tonnetz when cyclic closure matters more than pitch-class tracking.

# Examples
- **Figure 5.3** (p. 104): Jack Douthett's Cube Dance with hexatonic cycles and Weitzmann water bugs
- **Figure 5.4** (p. 105): Schubert *Die Zauberharfe* Overture charted on Cube Dance, showing consistent downshifting and arc of motion
- **Figure 5.11** (p. 112): Brahms Symphony No. 2, 1st mvt. charted on Cube Dance, showing clockwise L/R chain
- **Figure 5.23** (p. 120): Liszt *Lelio* Fantasy charted on Cube Dance, showing cyclic closure explicitly
- **Figure 6.12** (p. 146): Liszt Kyrie departure-return scheme on Cube Dance

# Relationships
## Builds Upon
- Hexatonic-Weitzmann figure-ground relation (adds structure to the hexatonic pools)
## Enables
- Voice-leading zones (radial positions on Cube Dance)
- Tracking chromatic sequences and transformational substitutions
- Departure-return and upshift scripts
## Related
- Connected Tonnetz (alternative representation of same space)
## Contrasts With
- Connected Tonnetz (Cube Dance makes cyclic closure explicit but loses pitch-class detail)

# Common Errors
- **Error**: Assuming Cube Dance is a three-dimensional cube
  **Correction**: It is a planar graph whose visual appearance resembles dancing cubes; the name comes from its geometric structure
- **Error**: Thinking every edge represents the same transformation
  **Correction**: Edges represent single semitonal displacement, which can be L, P, N, R, S, or the relation to an augmented triad

# Common Confusions
- **Confusion**: Cube Dance vs. Tonnetz -- which is better?
  **Clarification**: Neither is universally superior. Cube Dance excels at cyclic closure and true voice-leading distance; Tonnetz excels at pitch-class tracking, common-tone tracing, and historical connections
- **Confusion**: Thinking Cube Dance shows sequential structure
  **Clarification**: Cube Dance tracks voice-leading trajectory but not the specific sequential pattern; any random selection from each T4-related trio would look equally orderly

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, pp. 103-106, Figure 5.3. Originally created by Jack Douthett (1992), published in Douthett and Steinbach 1998.

# Verification Notes
Re-extracted from v2 cards (cube-dance-graph.md and cube-dance.md); preserved: directional conventions, figure references, Tymoczko quote. High confidence -- central representation of the book, extensively discussed.
