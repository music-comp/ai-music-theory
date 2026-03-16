---
# === CORE IDENTIFICATION ===
concept: External Transformation
slug: external-transformation

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: injection-function
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - X-external transformation

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
extends: []
related:
  - internal-transformation
  - dispersive-transformation
  - progressive-transformation
contrasts_with:
  - internal-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an external transformation?"
---

# Quick Definition
An X-external transformation f has minimal INJ(X, X)(f) — it maps X largely outside itself. The semi-combinatorial hexachord property (INJ(X, X)(I) = 0) is a classic example.

# Core Definition
Section 6.4 (Lewin, p. 174): "An f such that INJ(X, X)(f) is minimal or at least relatively small, given the constraints of X and INSPECT, can be called X-external. Such an f maps X largely outside itself." The definition avoids mentioning complements, which may not be finite sets if S is infinite.

# Prerequisites
- **INJ Function** — External is defined via low INJ(X, X)(f) values

# Key Properties
1. f is X-external if INJ(X, X)(f) is near its minimum
2. X-internal followed by X-external tends to be X-external
3. When INJ(X, X)(f) = 0, f is maximally external
4. Semi-combinatorial hexachord: I mapping X to complement means INJ(X, X)(I) = 0

# Construction / Recognition
## To Identify:
1. Compute INJ(X, X)(f) for transformations of interest
2. Those with lowest values are X-external

## To Recognize:
1. A transformation that takes a chord entirely (or mostly) outside itself

# Context & Application
External transformations model "departure" from a harmony. The semi-combinatorial hexachord provides a canonical example: an inversion mapping X to its complement makes I maximally X-external. This is Babbitt's combinatoriality condition.

# Examples
**Example 1** (p. 174): If X is a semi-combinatorial hexachord and I is the inversion mapping X to complement(X), then INJ(X, X)(I) = 0 — I is maximally X-external.

# Relationships
## Builds Upon
- **INJ Function** — Defined through INJ values

## Related
- **Dispersive Transformation** — External concerns X vs. itself; dispersive concerns X vs. Y

## Contrasts With
- **Internal Transformation** — High vs. low INJ(X, X)(f)

# Common Errors
- **Error**: Confusing external with dispersive
  **Correction**: External: low INJ(X, X)(f). Dispersive: low INJ(X, Y)(f). Different set pairs.

# Common Confusions
- **Confusion**: Thinking external transformations are analytically unimportant
  **Clarification**: External transformations reveal important structural properties like combinatoriality

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.4, p. 174.

# Verification Notes
- Definition source: Direct from section 6.4
- Confidence rationale: Explicit definition
- Re-extraction notes: Re-extracted from v2 card; preserved: semi-combinatorial example. Added v3.1 structure.
