---
# === CORE IDENTIFICATION ===
concept: Simple Period and Grand Period
slug: simple-period-and-grand-period

# === CLASSIFICATION ===
category: form
subcategory: sequences
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Repetition"
chapter_number: 4
pdf_page: 155
section: "Repetition reimagined"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - taneyev-arrow-notation
extends: []
related:
  - canonic-sequence
  - transpositional-sequence
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the difference between simple period and grand period in a sequence?"
---

# Quick Definition

The simple period is the minimal block of music sufficient to generate a sequence (one arrow application). The grand period is the time for all voices to cycle back to their initial melodic positions, all transposed by the same amount. With permutation, the grand period contains multiple simple periods.

# Core Definition

The simple period is "the length of the minimal block of music sufficient to generate the sequence, or the cells as I have defined them" (p. 157). The grand period is "the time it takes for all voices to cycle back to their initial melodic positions, all transposed by the same amount (not counting octave displacements)" (p. 157). Previous theorists focused on the grand period exclusively, "in large part this is because they have neglected permutation as a sequence-generating operation" (p. 157). In transpositional sequences, both coincide. In canonic sequences, the grand period is a multiple of the simple period.

# Prerequisites

- **Taneyev Arrow Notation** — Defines the arrow structure that determines both periods

# Key Properties

1. Simple period: minimal generating block (one arrow application)
2. Grand period: time for all voices to return to initial melodic positions
3. In transpositional sequences: simple = grand
4. In canonic sequences: grand contains multiple simple periods
5. Different voice groups can have different simple periods
6. Harmonically unstable sequences also have grand > simple typically

# Construction / Recognition

## To Determine:
1. Identify the arrow structure
2. Simple period = one application of the arrows
3. Grand period = number of applications until all voices have cycled through all roles

# Context & Application

"Three Blinde Mice" has simple period = 1 bar, grand period = 3 bars (three voices cycling). Bach's G-sharp minor fugue has simple period = 1 bar, grand period = 2 bars (Figure 4.1.3). Beethoven's Op. 10 no. 3 has different groups of voices with different simple periods (Figure 4.5.7). Recognizing the simple period allows analysts to identify minimal repeating structure even before voices complete a full cycle.

# Examples

**Example 1** (p. 157): "Three Blinde Mice" -- simple period = 1 bar, grand period = 3 bars.

**Example 2** (p. 157): Bach G-sharp minor fugue -- simple period = 1 bar, grand period = 2 bars.

# Relationships

## Builds Upon
- **Taneyev Arrow Notation** — The framework defining period structure

## Enables
- None specified

## Related
- **Canonic Sequence** — Where the distinction is most relevant
- **Transpositional Sequence** — Where simple = grand

## Contrasts With
- None specified

# Common Errors

- **Error**: Equating simple period with metric duration (one measure, one beat)
  **Correction**: The simple period is defined by the arrow structure, not by metric units

# Common Confusions

- **Confusion**: Thinking grand period is the "real" sequential unit
  **Clarification**: Previous theorists focused on the grand period, but the simple period is the minimal generating structure and often more analytically useful

# Source Reference

Chapter 4, Section 1, pp. 157-158.

# Verification Notes

- Definition source: Direct quotations from p. 157
- Confidence: HIGH — explicitly defined with clear examples
- Re-extracted from v2 card; preserved: "Three Blinde Mice" and Bach examples, different voice-group periods
