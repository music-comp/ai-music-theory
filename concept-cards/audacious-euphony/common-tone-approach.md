---
concept: Common-Tone Approach
slug: common-tone-approach

category: voice-leading
subcategory: distance-metrics
tier: foundational

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Mapping the Triadic Universe"
chapter_number: 1
pdf_page: 19
section: "Three Ways to Calculate Triadic Distance"

extraction_confidence: high

aliases:
  - "common-tone distance"
  - "pitch-class intersection metric"

prerequisites:
  - harmonic-distance
extends:
  - triadic-distance
related:
  - root-interval-approach
  - voice-leading-approach
  - voice-leading-work
contrasts_with:
  - root-interval-approach

answers_questions:
  - "What is the common-tone approach to measuring triadic distance?"
  - "What distinguishes common-tone distance from voice-leading distance?"
---

# Quick Definition
A method for calculating triadic distance based on the number of pitch classes shared between two triads: more common tones indicates closer relationship.

# Core Definition
The common-tone approach measures triadic proximity by the cardinality of pitch-class intersection between two triads. Two consonant triads can share 0, 1, or 2 pitch classes. Under this approach, triads sharing 2 tones are most closely related, those sharing 1 are intermediate, and those sharing 0 are most distant. This metric was dominant in nineteenth-century harmonic theory and "operates independently of diatonic collection membership or root relations" (synthesized from Ch. 1, pp. 23-25).

# Prerequisites
- **harmonic-distance** — The common-tone approach is one specific way of measuring harmonic distance

# Key Properties
1. Only three possible values (0, 1, or 2 shared pitch classes)
2. Coarser than voice-leading metric (3 values vs. 6)
3. Can conflict with diatonic distance: C to d minor shares 0 tones (diatonically close but common-tone distant)
4. Maximum common-tone retention (2) does not guarantee minimum voice-leading work
5. Nineteenth-century theorists increasingly privileged this metric over root-interval distance

# Construction / Recognition
For triads X and Y:
- |X intersection Y| = 2: closest (e.g., C major and a minor share C and E)
- |X intersection Y| = 1: intermediate (e.g., C major and E major share E)
- |X intersection Y| = 0: most distant (e.g., C major and f# minor share nothing)

# Context & Application
The common-tone approach was dominant in nineteenth-century harmonic theory. Galeazzi (1796) rated progressions based on common tones, Krause (1827) stated that "most closely related consonant triads are those that have two notes in common," Marx (1837) argued common tones provide "a more distinct tie" than mere diatonic co-occurrence, and both Hauptmann (1853) and Helmholtz (1877) developed common-tone-based theories of harmonic proximity (pp. 23-25).

# Examples
**Example 1** (p. 23): Galeazzi (1796) rated C major to E major as "regular and good" (1 common tone) while rating C major to d minor as "very irregular and poor" (0 common tones).

**Example 2** (p. 28): The Schubert B-flat Sonata recapitulation has a common-tone total of 6, "above average for quartet of triads."

# Relationships
## Builds Upon
- **harmonic-distance** — One specific approach to measuring harmonic distance
- **triadic-distance** — One of three metrics discussed

## Enables
- **minimal-work-relation** — Defined partly by maximum common-tone retention (2 shared tones)

## Related
- **voice-leading-approach** — A finer-grained alternative that also foregrounds pitch-class connection
- **root-interval-approach** — An alternative metric based on root distance

## Contrasts With
- **root-interval-approach** — Can produce contradictory judgments (C-E close by common tone, distant by root)

# Common Errors
- **Error**: Assuming maximum common-tone retention guarantees minimum voice-leading work
  **Correction**: Two triads sharing 2 tones may differ in voice-leading work (P = 1 unit, R = 2 units)

# Common Confusions
- **Confusion**: Thinking common tones must be held in the same register
  **Clarification**: Common tones are counted as pitch classes; octave equivalence applies

# Source Reference
Chapter 1: Mapping the Triadic Universe, pp. 23-25.

# Verification Notes
- Re-extracted from v2 card; preserved: Galeazzi, Krause, Marx, Hauptmann, Helmholtz citations
- Confidence: HIGH — the approach is explicitly defined with historical citations
