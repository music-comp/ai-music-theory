---
# === CORE IDENTIFICATION ===
concept: Minimal-Work Relation
slug: minimal-work-relation

# === CLASSIFICATION ===
category: voice-leading
subcategory: null
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Hexatonic Cycles"
chapter_number: 2
pdf_page: 35
section: "A Minimal-Work Model of the Triadic Universe"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "minimal work"
  - "single-semitone adjacency"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading-work
  - consonant-triad
extends:
  - voice-leading-approach
related:
  - single-semitonal-displacement
  - p-transformation
  - l-transformation
  - hexatonic-cycle
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the minimal-work relation?"
  - "What must I know before understanding hexatonic cycles?"
---

# Quick Definition
Two triads are in the minimal-work relation if motion between them involves the displacement of a single voice by semitone, representing the smallest possible voice-leading work between distinct consonant triads.

# Core Definition
"We will say that two triads are in the *minimal-work relation* if motion between them involves the displacement of a single voice by semitone" (Cohn, p. 36). Each triad is in this relation to exactly two triads of the opposite mode. "Each major triad is in the minimal-work relation with its parallel minor and with the minor triad whose root lies four semitones above it. For example, C major is in the specified relation with c minor and with e minor. Reciprocally, each minor triad is in the specified relation with its parallel major and with the major triad whose root lies four semitones below it" (p. 36).

# Prerequisites
- **voice-leading-work** — The minimal-work relation is defined as the minimum possible voice-leading work (1 unit)
- **consonant-triad** — The relation connects only consonant triads

# Key Properties
1. Voice-leading work = 1 semitone total displacement
2. Common tones = 2 (two of three pitch classes shared)
3. Only connects triads of opposite mode (major to minor or vice versa)
4. Each triad has exactly two minimal-work neighbors
5. Corresponds to the P and L transformations
6. Provides the foundation for constructing hexatonic cycles

# Construction / Recognition
## To find the minimal-work neighbors of a major triad:
1. Its parallel minor (P transformation): move the third down by semitone
2. The minor triad whose root is 4 semitones above (L transformation): move the root down by semitone

## To find the minimal-work neighbors of a minor triad:
1. Its parallel major (P transformation): move the third up by semitone
2. The major triad whose root is 4 semitones below (L transformation): move the fifth up by semitone

Example: C major connects to c minor (E->Eb) and e minor (C->B)
Example: c minor connects to C major (Eb->E) and Ab major (G->Ab)

# Context & Application
The minimal-work relation is the building block of hexatonic cycles and pan-triadic syntax. "Each consonant triad is thus situated in a chain of alternating major and minor triads" (p. 36), which closes into a six-element cycle after enharmonic identification.

# Examples
**Example 1** (p. 36): "C major is flanked by c minor and e minor, producing the three-element chain {c minor, C major, e minor}. That trio is nested within a five-element chain, {Ab major, {c minor, C major, e minor}, E major}."

**Example 2** (p. 36): "c minor can reach both C major and Ab major by a *single semitonal displacement*."

# Relationships
## Builds Upon
- **voice-leading-work** — Minimal-work is defined as 1 unit of voice-leading work
- **voice-leading-approach** — Applies the voice-leading metric at its minimum

## Enables
- **hexatonic-cycle** — Built by chaining minimal-work relations
- **single-semitonal-displacement** — Equivalent concept from the perspective of the moving voice
- **p-transformation** — One of the two transformations realizing the minimal-work relation
- **l-transformation** — The other transformation realizing the minimal-work relation

## Related
- **near-evenness** — The property of triads that makes minimal-work relations possible

## Contrasts With
No direct contrasts within this source.

# Common Errors
- **Error**: Trying to find a minimal-work connection between same-mode triads (e.g., C major to E major)
  **Correction**: The minimal-work relation only connects triads of opposite mode

# Common Confusions
- **Confusion**: Equating "minimal work" with "smooth voice leading" in general
  **Clarification**: Minimal-work specifically requires exactly one voice moving exactly one semitone; other smooth progressions may involve more displacement

# Source Reference
Chapter 2: Hexatonic Cycles, pp. 35-37.

# Verification Notes
- Re-extracted from v2 card; preserved: the chain-nesting description, the examples of minimal-work neighbors
- Definition based on direct quotation from p. 36
- Confidence: HIGH — explicitly defined with clear examples
