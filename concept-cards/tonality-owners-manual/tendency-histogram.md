---
# === CORE IDENTIFICATION ===
concept: Tendency Histogram
slug: tendency-histogram

# === CLASSIFICATION ===
category: analysis
subcategory: corpus-analysis
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "The Origins of Functional Tonality"
chapter_number: 6
pdf_page: 257
section: "Harmony and polyphony"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - zeroth-order-properties
  - first-order-properties
extends:
  - first-order-properties
related:
  - harmony-and-polyphony
  - protofunctionality
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a tendency histogram?"
  - "How can you distinguish genuine harmonic tendencies from zeroth-order effects?"
---

# Quick Definition

A graph comparing a chord's zeroth-order frequency to its frequency before or after a particular chord, revealing whether specific progressions are favored, suppressed, or merely reflecting the popularity of individual chords.

# Core Definition

A tendency histogram compares a chord's zeroth-order frequencies to their frequencies either before or after a particular chord (p. 278). When the dotted line (first-order) lies above the solid line (zeroth-order), the progression happens more than its zeroth-order probability predicts -- a genuine tendency. When the lines nearly coincide, the progression's frequency merely reflects individual chord popularity. When first-order lies below zeroth-order, the progression is suppressed (p. 278).

# Prerequisites

- **Zeroth-order properties** — The baseline against which tendencies are measured
- **First-order properties** — The context-dependent probabilities being visualized

# Key Properties

1. Compares zeroth-order and first-order probabilities visually
2. Shows favored progressions (first-order > zeroth-order)
3. Shows neutral progressions (first-order = zeroth-order)
4. Shows suppressed progressions (first-order < zeroth-order)
5. Distinguishes genuine tendency from popularity effects

# Construction / Recognition

## To Create a Tendency Histogram:
1. Calculate zeroth-order frequency for each chord type
2. Calculate first-order frequency after (or before) a specific chord
3. Plot both on the same graph
4. Look for divergences indicating genuine tendencies or suppressions

# Context & Application

Tymoczko finds these "endlessly fascinating, encapsulating a wealth of musical information I know only tacitly" (p. 279). They reveal that some common progressions (like IV-I in Palestrina) are common mainly because of individual chord popularity, while others (like ii-vii-dim-6) represent genuine tendencies.

# Examples

**Example 1** (p. 278, Fig. 6.4.6): Tendency histograms for chords preceding and following vii-dim-6 in Palestrina's ionian masses -- showing strong ii-to-vii-dim-6 and vii-dim-6-to-I tendencies.

# Relationships

## Builds Upon
- **First-order properties** — Tendency histograms visualize first-order properties

## Enables
- No specific concepts enabled

## Related
- **Harmony and polyphony** — Tendency histograms reveal functional patterns in polyphonic music
- **Protofunctionality** — Tendency histograms can reveal proto-functional tendencies

## Contrasts With
- No direct contrasts

# Common Errors

- **Error**: Concluding a common progression has a strong "tendency" without checking zeroth-order baseline
  **Correction**: Compare first-order to zeroth-order; some common progressions merely reflect chord popularity

# Common Confusions

- **Confusion**: Thinking tendency histograms reveal only forward-looking tendencies
  **Clarification**: They can show both what follows a chord (forward-looking) and what precedes it (backward-looking)

# Source Reference

Chapter 6, section 4, pp. 278-279. See Figure 6.4.6.

# Verification Notes

- Definition source: Direct from pp. 278-279
- Confidence rationale: Explicitly named and described
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all content derivable from source
