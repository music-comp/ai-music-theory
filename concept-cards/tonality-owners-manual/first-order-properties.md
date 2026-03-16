---
# === CORE IDENTIFICATION ===
concept: First-Order Properties
slug: first-order-properties

# === CLASSIFICATION ===
category: harmony
subcategory: functional-harmony
tier: foundational

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "The Origins of Functional Tonality"
chapter_number: 6
pdf_page: 257
section: "The logical structure of protofunctionality"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - progression tendencies
  - transition probabilities

# === TYPED RELATIONSHIPS ===
prerequisites:
  - zeroth-order-properties
extends: []
related:
  - tendency-histogram
  - protofunctionality
contrasts_with:
  - zeroth-order-properties

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are first-order properties in harmonic analysis?"
  - "How do you distinguish genuine harmonic tendencies from zeroth-order effects?"
---

# Quick Definition

The likelihood that one chord will progress to another: for example, the probability that V will go to I. Theories of functionality often highlight first-order properties, associating "dominantness" with a chord's tendency to move to the tonic.

# Core Definition

First-order properties describe the likelihood that one chord will progress to another (p. 258). They are context-dependent, contrasting with zeroth-order properties. Theories of functionality often associate "dominantness" with a chord's first-order tendency to move to tonic (p. 259). However, first-order tendencies can arise from zeroth-order preferences combined with melodic constraints (p. 261). To isolate genuine tendencies, Tymoczko calculates the difference between a chord's zeroth-order probability and its first-order probability after a specific chord -- revealing whether V is particularly likely after IV, beyond what we'd expect from IV and V being individually common (p. 266).

# Prerequisites

- **Zeroth-order properties** — First-order properties are measured relative to zeroth-order baseline

# Key Properties

1. Context-dependent: probability of chord Y following chord X
2. Include both forward-looking (V tends to I) and backward-looking (I tends to be preceded by V) tendencies
3. Dominants are "constrained toward the future"; tonics "constrained toward the past" (p. 278)
4. Must be distinguished from zeroth-order effects
5. The "tendency" metric: first-order probability minus zeroth-order probability

# Construction / Recognition

## To Measure First-Order Properties:
1. Count all two-chord progressions in a corpus
2. Calculate the probability of each successor chord after each predecessor
3. Subtract the zeroth-order probability to isolate genuine "tendency"
4. Positive values indicate the progression is favored; negative values indicate suppression

# Context & Application

First-order tendencies reveal the gradual strengthening of functional relationships. V-to-I tendency increases from Josquin through Beethoven (Fig. 6.6.5). Similarly, ii-to-V tendency eventually matches V-to-I attraction (p. 294).

# Examples

**Example 1** (p. 266, Fig. 6.2.3): The "tendency" of IV-V and IV-I progressions in Dalza and the frottola.

**Example 2** (p. 294, Fig. 6.6.5): The "tendency" of V-to-I and ii-to-V, increasing over centuries.

# Relationships

## Builds Upon
- **Zeroth-order properties** — Baseline against which tendencies are measured

## Enables
- **Tendency histogram** — Visual representation of first-order properties

## Related
- **Protofunctionality** — First-order tendencies help define proto-functional behavior

## Contrasts With
- **Zeroth-order properties** — Context-independent chord frequencies

# Common Errors

- **Error**: Treating a common progression as having a strong "tendency" without controlling for zeroth-order probability
  **Correction**: Some common progressions (like IV-I in Palestrina) are common mainly because IV and I are individually popular (p. 279)

# Common Confusions

- **Confusion**: Thinking first-order properties are more fundamental than zeroth-order
  **Clarification**: Tymoczko argues zeroth-order (vocabulary) is primary; first-order tendencies can arise from vocabulary + melodic constraints

# Source Reference

Chapter 6: The Origins of Functional Tonality, sections 1-2, pp. 258-267; section 6, pp. 293-296.

# Verification Notes

- Definition source: Direct from pp. 258-259, 266
- Confidence rationale: Explicitly defined with clear statistical method
- Cross-reference status: Verified against zeroth-order-properties, tendency-histogram
- Re-extraction notes: Re-extracted from v2 card; preserved: all content derivable from source
