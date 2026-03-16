---
# === CORE IDENTIFICATION ===
concept: Duration Proportion Space
slug: duration-proportion-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.2.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.2.3"
  - multiplicative duration space

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - function
extends: []
related:
  - duration-class-space
  - just-intonation-pitch-space
  - generalized-interval-system
contrasts_with:
  - additive-duration-space
  - time-point-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Duration proportion space is a GIS where durations are compared by their ratios (quotients), using a multiplicative interval group.

# Core Definition

"The musical space is a family of durations, each duration measuring a temporal span in time units. And int(s, t) is the quotient of the t and s measurements, t/s" (Lewin, Example 2.2.3, p. 53). The specific interval group depends on which proportions are allowed. If we allow basic proportions of 2 and 3, IVLS = {2^a * 3^b}; with 5 and 7 added, IVLS = {2^a * 3^b * 5^c * 7^d}; one could even use irrational factors.

# Prerequisites

- **Group** — IVLS is a multiplicative group of positive numbers
- **Function** — int(s, t) = t/s is the interval function

# Key Properties

1. S = durations (positive real numbers), extended to indefinitely short and long
2. IVLS = a multiplicative group of positive ratios (context-dependent)
3. int(s, t) = t/s (ratio of durations)
4. Identity: int(s, s) = 1
5. Durations can be identified with tempi (inverse relationship)

# Construction / Recognition

## To Construct:
1. Choose which basic proportions to allow (e.g., 2 and 3)
2. Generate the multiplicative group of ratios
3. Define int(s, t) = t/s

## To Recognize:
1. Intervals are ratios (multiplied together), not differences
2. The specific IVLS depends on which proportions the analysis allows

# Context & Application

This GIS models rhythmic proportions: "t is 3/4 the length of s" gives an interval. The framework applies to proportional notation, metric modulation, and tempo relationships. Durations identified with tempi allow inverse-proportion analysis. Carter, Nancarrow, and Ligeti used compositions involving such rhythmic proportions.

# Examples

**Example 1** (p. 53): If s = 4 units and t = 3 units, int(s, t) = 3/4.

**Example 2** (Section 2.4, p. 54): With basic proportions 2 and 3: IVLS = {2^a * 3^b}. With sqrt(2) and sqrt(3): IVLS = {2^(a/2) * 3^(b/2)}.

# Relationships

## Builds Upon
- **Group** — uses a multiplicative group of positive rationals (or reals)

## Enables
- **Duration-Class Space** — reducing this space by modulus M

## Related
- **Just Intonation Pitch Space** — structurally parallel (both use multiplicative groups of ratios)

## Contrasts With
- **Additive Duration Space** — quotients vs. differences
- **Time-Point Space** — measures lengths, not positions

# Common Errors

- **Error**: Adding intervals instead of multiplying them.
  **Correction**: Intervals are ratios; composition is by multiplication, not addition.

# Common Confusions

- **Confusion**: Thinking one specific IVLS applies to all duration analyses.
  **Clarification**: The choice of IVLS depends on which proportions the analyst wishes to allow.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.3, Section 2.4, pp. 53, 54.

# Verification Notes

- Definition source: direct from Example 2.2.3 and Section 2.4
- Confidence rationale: explicit example with multiple IVLS variants
- Re-extracted from v2 card; preserved: different IVLS options, tempo interpretation, composer references
