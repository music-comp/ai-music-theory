---
# === CORE IDENTIFICATION ===
concept: "N Transformation (Nebenverwandt)"
slug: n-transformation

# === CLASSIFICATION ===
category: transformations
subcategory: weitzmann-transformations
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Weitzmann Regions"
chapter_number: 4
pdf_page: 79
section: "Weitzmann Transformations and N/R Cycles"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Nebenverwandt transformation"
  - "L-inverse"
  - "L-prime"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - consonant-triad
  - nebenverwandt
extends: []
related:
  - r-transformation
  - s-transformation
  - l-transformation
  - nr-cycle
  - weitzmann-transformations
  - w-group
contrasts_with:
  - l-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes L, P, R, N, and S transformations?"
  - "What is the N transformation?"
  - "How does N relate to L?"
---

# Quick Definition
The Nebenverwandt transformation (N) takes any triad to its nebenverwandt partner, defined as the inversion about the root of a major triad or the fifth of a minor triad, moving two voices by semitone in contrary motion.

# Core Definition
"The nebenverwandt transformation (abbreviated N) takes any triad to its nebenverwandt chord, which Weitzmann defines as the inversion about the root of a major triad, or about the fifth of a minor triad" (p. 80). N maps C major to f minor and vice versa. Morris 1998 proposes the label L' (L-inverse) for N, "whose semitonally moving and stationary voices swap those of L" (p. 80). N requires 2 units of voice-leading work.

# Prerequisites
- **Consonant triad**: N operates on consonant triads
- **Nebenverwandt**: The theoretical concept that N formalizes as a transformation

# Key Properties
1. N maps a major triad to the minor triad a perfect fourth above its root (e.g., C major -> f minor)
2. N maps a minor triad to the major triad a perfect fifth above its fifth (e.g., f minor -> C major)
3. Two voices move by semitone in contrary motion; one voice is stationary
4. N requires 2 units of voice-leading work
5. N is an involution: N(N(X)) = X
6. N is the L-inverse (L'): it swaps L's moving and stationary voices
7. N preserves Weitzmann region and shifts hexatonic region

# Construction / Recognition
Voice-leading action:
- C major {C, E, G} -> f minor {F, Ab, C}: E moves to F (up semitone), G moves to Ab (up semitone), C retained
- f minor {F, Ab, C} -> C major {C, E, G}: F moves to E (down semitone), Ab moves to G (down semitone), C retained

Note: the two moving voices go in contrary motion when described as moving from the third/fifth of the major triad to the root/third of the minor triad, but in the same direction when both are described as semitonal motions.

# Context & Application
N connects a major tonic to its minor subdominant (or a minor tonic to its major dominant) -- a classical syntactic progression. The N/R alternation generates the complete Weitzmann region cycle. Weitzmann conceived of it as a key relation; Oettingen (1866) first interpreted it transformationally.

# Examples
**Schubert, Fourth Symphony, mm. 86-106** (p. 81): N/R chain where "each major triad is prolonged as a local tonic, which each minor triad serves as subdominant."

**Schubert, Ninth Symphony trombone solo** (pp. 81-82): "Complete N/R cycle with dominant sevenths attached to major triads."

**Classical syntax** (p. 80): N = R + LP (the motion from minor tonic to major dominant can be executed via relative major).

# Relationships
## Builds Upon
- nebenverwandt: N formalizes the nebenverwandt key relation as a voice-leading operation

## Enables
- nr-cycle: N alternating with R generates the complete Weitzmann region
- double-agent-complex: N connects interior to exterior triads in the complex

## Related
- r-transformation: Together with N, generates the N/R cycle
- s-transformation: The third W-group transformation alongside N and R
- w-group: N is a member of the W-group

## Contrasts With
- l-transformation: N swaps L's moving and stationary voices; Morris calls N "L-inverse"

# Common Errors
- **Error**: Equating N with functional dominant-tonic motion
  **Correction**: N describes a voice-leading relationship, not a tonal function; the functional interpretation depends on context

# Common Confusions
- **Confusion**: N and L are the same because both involve semitonal motion with mode change
  **Clarification**: N and L swap which voices move: in L (C major -> e minor) the root moves; in N (C major -> f minor) the third and fifth move

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 4: Weitzmann Regions, pp. 79-81.

# Verification Notes
Re-extracted from v2 card; preserved: L-inverse label (Morris 1998), Hauptmann definition, Oettingen transformational interpretation. High confidence: core transformation with precise definition.
