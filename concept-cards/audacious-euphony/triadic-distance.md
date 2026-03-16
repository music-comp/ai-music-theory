---
concept: Triadic Distance
slug: triadic-distance

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
  - "triadic proximity"

prerequisites:
  - harmonic-distance
extends:
  - harmonic-distance
related:
  - root-interval-approach
  - common-tone-approach
  - voice-leading-approach
  - voice-leading-work
contrasts_with: []

answers_questions:
  - "How can triadic distance be measured?"
  - "Why do different distance metrics yield different results?"
  - "What distinguishes common-tone distance from voice-leading distance?"
---

# Quick Definition
The measurement of relatedness between two consonant triads, which can be calculated using root-interval, common-tone, or voice-leading metrics that may yield conflicting proximity judgments.

# Core Definition
Triadic distance quantifies the proximity or remoteness between two consonant triads. Chapter 1 identifies three historical approaches: root-interval distance (based on the circle of fifths), common-tone distance (based on shared pitch classes), and voice-leading distance (based on aggregate semitonal displacement). These metrics "are not equivalent and can produce contradictory proximity judgments" (synthesized from Ch. 1, pp. 19-31). The classical tradition privileges root-interval distance, while nineteenth-century theory increasingly valued common-tone and voice-leading metrics.

# Prerequisites
- **harmonic-distance** — Triadic distance is the specific application of harmonic distance to consonant triads

# Key Properties
1. Three distinct metrics yield different proximity judgments for the same triadic pair
2. Root-interval: 0-6 values (fifths apart); common-tone: 0-2 values; voice-leading: 1-6 values
3. Voice-leading provides the finest gradations (6 distinct values vs. 3 for common-tone)
4. The metrics can conflict dramatically: C to E major is root-distant (4 fifths) but voice-leading close (2 units)
5. No single metric is universally "correct"; each captures different musical intuitions

# Construction / Recognition
For triads X and Y:

| Metric | Formula | Range | Example (C maj to G maj) |
|--------|---------|-------|--------------------------|
| Root-interval | Fifths between roots | 0-6 | 1 (close) |
| Common-tone | Shared pitch classes | 0-2 | 1 (medium) |
| Voice-leading | Total semitonal motion | 1-6 | 4 (distant) |

Conflicting example - C major to c minor:
- Root-interval: 0 (identical root)
- Common-tone: 2 (close)
- Voice-leading: 1 (very close)

Conflicting example - C major to E major:
- Root-interval: 4 fifths (distant)
- Common-tone: 1 (medium)
- Voice-leading: 2 (close)

# Context & Application
The comparison of distance metrics motivates the book's central argument. The Schubert B-flat Sonata passage (mm. 217-256) illustrates how a progression can be diatonically disjunct but voice-leading conjunct: "only 1 pair shares a diatonic collection" yet the voice-leading total is "toward the lower/more conjunct end of range" (Ch. 1, Table 1.1).

# Examples
**Example 1** (Ch. 1, Table 1.1): Schubert's B-flat Sonata recapitulation (Bb - Gb - f# - A - Bb): common-tone total of 6 (above average), voice-leading total of 14 units (conjunct), but diatonic coherence is very low.

**Example 2** (pp. 23-25): Galeazzi (1796) rated C major to E major as "regular and good" based on the shared common tone E, while rating C major to d minor as "very irregular and poor" because they share no common tones.

# Relationships
## Builds Upon
- **harmonic-distance** — Triadic distance is the systematic investigation introduced in Ch. 1

## Enables
- **voice-leading-work** — The specific metric central to pan-triadic analysis
- **minimal-work-relation** — Defined by the minimum possible voice-leading distance

## Related
- **root-interval-approach** — One of the three distance metrics
- **common-tone-approach** — Another of the three metrics
- **voice-leading-approach** — The metric privileged in this book

## Contrasts With
No direct contrasts; the concept encompasses multiple approaches.

# Common Errors
- **Error**: Using only one metric to assess triadic relatedness
  **Correction**: Different metrics illuminate different aspects; analysts should consider multiple metrics

# Common Confusions
- **Confusion**: Assuming "close" and "distant" are absolute properties of triadic pairs
  **Clarification**: These judgments are relative to the chosen metric
- **Confusion**: Believing the "law of least motion" combines common-tone and voice-leading approaches into one
  **Clarification**: The two approaches can diverge; maximum common-tone retention does not guarantee minimum voice-leading work

# Source Reference
Chapter 1: Mapping the Triadic Universe, pp. 19-31. Table 1.1 on p. 28.

# Verification Notes
- Re-extracted from v2 card; preserved: the table comparing metrics, the Schubert and Galeazzi examples
- Confidence: HIGH — the three metrics are explicitly presented and compared in Ch. 1
