---
# === CORE IDENTIFICATION ===
concept: Tonnetz Representation
slug: tonnetz-representation

# === CLASSIFICATION ===
category: representations
subcategory: null
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Hexatonic Cycles"
chapter_number: 2
pdf_page: 35
section: "Hexatonic Progressions, Tonnetz Representations, and Triadic Transformations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Tonnetz"
  - "tonal network"
  - "tone net"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - hexatonic-cycle
  - consonant-triad
extends: []
related:
  - p-transformation
  - l-transformation
  - four-hexatonic-systems
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Tonnetz?"
  - "How does the Tonnetz represent hexatonic cycles?"
---

# Quick Definition
A planar graph ("tonal network") where points represent individual pitch classes, edges connect consonant intervals, and triangles represent consonant triads, enabling visualization of voice-leading paths through triadic space.

# Core Definition
The *Tonnetz* ("tonal network" in German) is "a planar figure that coordinates axes representing the consonant interval classes" (Cohn, p. 46). In the angled format used throughout the book: "perfect fifths rise from left to right along the horizontal axis, minor thirds rise from northwest to southeast, and major thirds from southwest to northeast" (p. 46). Triads appear as triangles: "Major triads extend upward, and minor triads subtend downward, from their shared perfect-fifth edge" (p. 46). "The *Tonnetz* was first presented by Leonhard Euler in 1739. It was revived by German harmonic theorists in the second half of the nineteenth century" (p. 47, footnote 10). The angled format was introduced by Hostinsky (1879) and adopted by late Riemann.

# Prerequisites
- **hexatonic-cycle** — The Tonnetz models hexatonic cycles as strips
- **consonant-triad** — Triads appear as triangles on the Tonnetz

# Key Properties
1. Points = individual pitch classes
2. Edges = consonant dyads (perfect fifth, major third, minor third)
3. Triangles = consonant triads (major pointing up, minor pointing down)
4. Internal edges between triangles = shared dyads
5. Hexatonic systems form vertical strips bounded by augmented triads
6. P transformation = vertical motion (0 degrees); L transformation = diagonal motion (120 degrees)
7. Suffers from a "Bering Strait flaw": identical points at top and bottom of a strip mask identities (p. 46)

# Construction / Recognition
Hexatonic strips on the Tonnetz:
1. Each hexatonic system forms a vertical strip
2. External boundaries of the strip are augmented triads
3. Interior tiled into triangles representing consonant triads
4. Clockwise cyclic motion = downward motion in Tonnetz strips
5. Double-labeling of nodes at enharmonic exchanges (following Harrison 2002b)

# Context & Application
The Tonnetz makes voice-leading relationships visually apparent. P appears as vertical motion, L as diagonal motion. The graphs reveal how hexatonic cycles trace paths through strips bounded by augmented triads. "These limitations are worth tolerating because of the many advantages that the triangularly tiled planar representations afford" (p. 46).

# Examples
**Example 1** (Fig. 2.9, pp. 46-47): Tonnetz models of Mozart K. 543, Haydn Symphony 98, Beethoven Op. 24, and Schubert Piano Trio Op. 100.

**Example 2** (Fig. 2.10, p. 47): A progression on a Tonnetz strip as incremental moves through a hexatonic cycle, showing "downward arrows, indicating chromatic-semitone descents, alternate with diagonal ones, indicating diatonic-semitone ascents."

**Example 3** (Fig. 2.11, p. 48): Four hexatonic passages from the nineteenth century (Brahms, Liszt, Schubert, Wagner) mapped on Tonnetz strips.

# Relationships
## Builds Upon
- **hexatonic-cycle** — The Tonnetz models hexatonic cycles as strips
- **consonant-triad** — Triads are the triangles on the Tonnetz

## Enables
No concepts within this scope depend specifically on the Tonnetz representation.

## Related
- **p-transformation** — Appears as vertical motion on the Tonnetz
- **l-transformation** — Appears as diagonal motion on the Tonnetz
- **four-hexatonic-systems** — Each system forms a strip on the Tonnetz

## Contrasts With
No direct contrasts within this source.

# Common Errors
- **Error**: Treating distances on the planar Tonnetz as true distances
  **Correction**: The planar format distorts distances due to the Bering Strait flaw; points at top and bottom of a strip are actually identical

# Common Confusions
- **Confusion**: Thinking the Tonnetz represents pitch space
  **Clarification**: It represents pitch-class space; octave equivalence is assumed
- **Confusion**: Confusing the angled format (Hostinsky/Riemann) with earlier rectangular formats
  **Clarification**: The angled format positions intervals at different angles; the rectangular format does not

# Source Reference
Chapter 2: Hexatonic Cycles, pp. 46-49. Figures 2.9-2.11. Historical note in footnote 10, p. 47.

# Verification Notes
- Re-extracted from v2 card; preserved: the Euler 1739 origin, the Hostinsky 1879 format, the Harrison double-labeling, the Bering Strait flaw, the axis orientation details
- Confidence: HIGH — the Tonnetz is extensively described with multiple figures
- Consolidated content from v2 card [tonnetz-strip]: No unique content to consolidate (strip structure, Bering Strait flaw, Harrison double-labeling, P/L motion directions, boundary augmented triads, and cylinder topology all already present)
