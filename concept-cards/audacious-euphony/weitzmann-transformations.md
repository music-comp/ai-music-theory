---
# === CORE IDENTIFICATION ===
concept: Weitzmann Transformations
slug: weitzmann-transformations

# === CLASSIFICATION ===
category: transformations
subcategory: transformation-groups
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
  - "W-group transformations (summary)"
  - "R, N, S transformations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - weitzmann-region
  - voice-leading-work
extends: []
related:
  - r-transformation
  - n-transformation
  - s-transformation
  - w-group
  - h-transformation
contrasts_with:
  - h-transformation
  - l-transformation
  - p-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the Weitzmann transformations?"
  - "What distinguishes L, P, R, N, and S transformations?"
  - "How do Weitzmann transformations relate to hexatonic transformations?"
---

# Quick Definition
The set of three transformations (R, N, S) that connect triads within a Weitzmann region, each requiring exactly two units of voice-leading work and contrasting with the hexatonic transformations (L, P, H).

# Core Definition
Weitzmann transformations are the three mode-reversing operations that preserve Weitzmann region membership: R (Relative) moves one voice by whole step; N (Nebenverwandt) moves two voices by semitone in contrary motion; S (Slide) moves two voices by semitone in parallel motion. All three require exactly 2 voice-leading units, "placing them at the second rank of parsimony (after L and P at 1 unit)" (cf. pp. 79-83). Additionally, LP and PL connect same-mode triads within a region via the hexatonic cycle mechanism but are "coextensive with the LP and PL transformations through a hexatonic cycle" even though they are now "routed through... an augmented [triad], not a minor one" (p. 79).

# Prerequisites
- **Weitzmann region**: The space within which these transformations operate
- **Voice-leading work**: Must understand semitonal measurement to compare transformation costs

# Key Properties
1. All three are mode-reversing (major to minor or vice versa)
2. All three require exactly 2 units of voice-leading work
3. All three preserve Weitzmann region membership
4. All three shift hexatonic region membership
5. All three are involutions (self-inverse)
6. R has two common tones but moves one voice by whole step
7. N has one common tone but moves two voices by semitone in contrary motion
8. S has one common tone (the third) but moves two voices by semitone in parallel motion

# Construction / Recognition
| Transformation | Common Tones | Moving Voices | Motion Type | Work |
|---|---|---|---|---|
| R (Relative) | 2 (major 3rd) | 1 voice, whole step | -- | 2 |
| N (Nebenverwandt) | 1 (root or 5th) | 2 voices, semitone | contrary | 2 |
| S (Slide) | 1 (3rd) | 2 voices, semitone | parallel | 2 |

# Context & Application
The Weitzmann transformations generate all mode-reversing progressions within a Weitzmann region. Combined with LP/PL (same-mode connections), they account for all five possible intra-regional progressions. The W-group (R, N, S) complements the H-group (L, P, H) in the larger transformational framework.

# Examples
**Figure 4.2** (p. 79): All five voice-leading connections from C major to the other triads in its region, demonstrating LP, PL, R, N, and S.

**Figure 4.6(b)** (p. 83): Transformational arrows on the Tonnetz showing these connections with labels.

# Relationships
## Builds Upon
- weitzmann-region: The space these transformations operate within

## Enables
- nr-cycle: N and R alternation generates the complete cycle
- double-agent-complex: N and R structure the complex; S connects its exterior triads

## Related
- r-transformation: Individual W-group member
- n-transformation: Individual W-group member
- s-transformation: Individual W-group member
- w-group: The algebraic group formed by these transformations

## Contrasts With
- h-transformation: H-group member (preserves hexatonic region)
- l-transformation: H-group member
- p-transformation: H-group member

# Common Errors
- **Error**: Including LP and PL as Weitzmann transformations
  **Correction**: LP and PL connect same-mode triads and are technically hexatonic compounds, though they function within Weitzmann regions

# Common Confusions
- **Confusion**: R is closer than N and S because it preserves more common tones
  **Clarification**: All three require the same voice-leading work (2 units); common-tone count does not determine distance

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 4: Weitzmann Regions, pp. 79-83.

# Verification Notes
Re-extracted from v2 card; preserved: comparison table, LP/PL distinction, second-rank parsimony characterization. High confidence: systematically presented in source.
