---
# === CORE IDENTIFICATION ===
concept: Voice Exchange
slug: voice-exchange

# === CLASSIFICATION ===
category: voice-leading
subcategory: geometric-voice-leading
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Line and Configuration"
chapter_number: 3
pdf_page: 96
section: "Voice exchanges"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "two-step voice exchange"
  - "five-step voice exchange"
  - "pairwise voice exchange"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-voice-leading
  - diatonic-dyad-circle
extends:
  - imperfect-consonance-system
related:
  - antiparallel-motion
  - contrary-motion-sequence
  - canonic-sequence
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a configuration in the context of voice leading?"
  - "What is the relationship between canonic sequences and voice exchange?"
  - "How do voice exchanges decompose voice leadings?"
---

# Quick Definition

A rearrangement of the notes within a single chord, where voices swap positions along equal and opposite paths that collectively sum to zero. Any voice leading can be decomposed into a crossing-free component (preserving spacing) plus a voice exchange.

# Core Definition

For diatonic thirds, there are two elementary voice exchanges: the small (two-step) exchange moving each voice by two scale steps (third down to root and root up to third), and the large (five-step) exchange moving each voice by five steps (third up to root and root down to third) (p. 103). More complicated voice leadings can be generated from these two, perhaps in combination with transpositions along the chord. For triads, there are three fundamental pairwise voice exchanges that swap adjacent pitch classes along equal and opposite paths: root-third, third-fifth, and root-fifth swaps (Figure 3.5.6, p. 124). "This factoring of voice leadings into voice exchanges and a crossing-free residue is a point of contact between Schenkerian theory and voice-leading geometry" (p. 103).

# Prerequisites

- **Basic Voice Leading** — Voice exchanges combine with basic voice leadings to produce antiparallel motion
- **Diatonic Dyad Circle** — Voice exchanges are represented as curved arrows on this diagram

# Key Properties

1. Paths collectively sum to zero (equal and opposite motions)
2. Two elementary voice exchanges for dyads: two-step (small) and five-step (large)
3. Three pairwise exchanges for triads: root-third, third-fifth, root-fifth
4. Combined with basic voice leading produces antiparallel (balanced contrary) motion
5. Contrary-motion sequences require voice exchanges (p. 165, fn. 11)
6. Voice crossings are fairly rare in Renaissance music and even rarer later (p. 124)

# Construction / Recognition

## To Construct (Dyadic):
1. Start with a dyad (e.g., C-E third)
2. Two-step exchange: move C up two steps to E, move E down two steps to C -> (E, C)
3. Five-step exchange: move C down five steps, move E up five steps -> produces the inverted arrangement

## To Recognize:
1. Look for two voices swapping their pitch classes
2. Verify the paths sum to zero (one up, the other down by complementary amounts)
3. In Schenkerian analysis, surface-level voice exchanges can indicate motion within a simpler background

# Context & Application

Voice exchanges are fundamental to functionally tonal composition. The two-step voice exchange applied separately from the basic voice leading produces wedge sequences where voices move apart in near contrary motion (Figure 3.2.3, p. 105). Beethoven's Op. 26 finale systematically deploys the basic voice leading, both voice exchanges, and their combinations -- "all the imperfect system's atomic moves" (p. 106). "Parallel motion and shifts between third and sixth generate all the moves on the expanded diagram" (p. 106). Schenkerian theory's surface-level voice exchanges can be understood as artifacts of voices moving within a simpler background (p. 135).

# Examples

**Example 1** (p. 104): Figure 3.2.1 shows curved arrows representing two-step (dotted) and five-step (solid) voice exchanges on the augmented dyadic diagram.

**Example 2** (p. 106): Beethoven's Op. 26 finale deploys all atomic moves of the imperfect system.

**Example 3** (p. 124): Figure 3.5.6 shows three basic pairwise voice crossings for triads, with factoring of a voice leading into crossing-free component plus voice exchanges.

# Relationships

## Builds Upon
- **Basic Voice Leading** — Voice exchanges combine with basic voice leadings
- **Diatonic Dyad Circle** — The geometric space where voice exchanges are represented

## Enables
- **Antiparallel Motion** — Produced by combining voice exchange with basic voice leading
- **Contrary-Motion Sequence** — Requires voice exchanges

## Related
- **Canonic Sequence** — Voice exchanges relate to permutation in sequential structure
- **Imperfect Consonance System** — Voice exchanges are atomic moves of this system

## Contrasts With
- None specified

# Common Errors

- **Error**: Assuming voice exchanges always produce audible voice crossings
  **Correction**: Voice exchange is a rearrangement of notes among voices that may or may not produce an audible crossing; the ear tends to filter out crossings, associating voices registrally (p. 124)

# Common Confusions

- **Confusion**: Thinking voice exchange and voice crossing are the same
  **Clarification**: Voice exchange is the abstract rearrangement operation; voice crossing is the registral event that may or may not result

- **Confusion**: Believing Schenkerian "voice exchanges" are a different concept
  **Clarification**: Schenkerian voice exchanges can be understood as artifacts of voices moving within a simpler background, connecting the two frameworks (p. 135)

# Source Reference

Chapter 3: Line and Configuration, Section 2, pp. 103-107, Figures 3.2.1-3.2.5; Section 5, pp. 124-125, Figure 3.5.6.

# Verification Notes

- Definition source: Direct from pp. 103-104
- Key Properties: All explicit in source
- Confidence: HIGH — clearly defined with geometric representation
- Re-extracted from v2 card; preserved: Schenkerian connection, distinction from voice crossing
