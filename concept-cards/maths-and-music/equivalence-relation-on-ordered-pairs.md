---
# === CORE IDENTIFICATION ===
concept: Equivalence Relation on Ordered Pairs
slug: equivalence-relation-on-ordered-pairs

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: ratios
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "The Equivalence Relation of Ratios"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - ratio equivalence relation
  - ratio as equivalence class

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - interval-as-frequency-ratio
  - multiplicative-composition-of-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is a ratio formally defined as an equivalence class?"
  - "Why can different pairs of numbers represent the same ratio?"
---

# Quick Definition

The ratio of two positive real numbers is formally defined as an equivalence class of ordered pairs, where two pairs (a, b) and (a', b') are equivalent when a/b = a'/b'.

# Core Definition

On the set (R+)^2 of ordered pairs of positive reals, define a relation declaring (a, b) ~ (a', b') if a/b = a'/b', equivalently if a'b = ab'. This is an equivalence relation. The equivalence class of (a, b) is denoted (a : b) or a : b, called the ratio of a and b. The set of all equivalence classes is denoted (R+ : R+). The function phi: (R+ : R+) -> R+ defined by phi((a:b)) = a/b is well-defined, one-to-one, and onto, establishing a bijection between ratio classes and positive reals (Wright, pp. 58-59, formula 4.1).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Two pairs (a, b) and (a', b') are equivalent iff a/b = a'/b' iff a'b = ab'
2. Each equivalence class contains infinitely many pairs (e.g., (2:3) = (4:6) = (1/2 : 3/4))
3. The bijection phi identifies each ratio class with a unique positive real number
4. This construction parallels the standard construction of rational numbers from integer pairs
5. The abstraction is essential because musical intervals operate on pairs of frequencies

# Construction / Recognition

## To Determine if Two Pairs Represent the Same Ratio

1. Given pairs (a, b) and (a', b')
2. Compute a/b and a'/b'
3. If the results are equal, the pairs are equivalent (same ratio)
4. Alternatively, check if a'b = ab' (cross-multiplication)

# Context & Application

Since pitches are identified with positive real numbers (frequencies), this equivalence relation applies directly to pairs of pitches (f_2, f_1). The equivalence class f_2 : f_1 captures the interval between them. Different pairs of notes can produce the same interval: (440, 220) and (880, 440) both belong to the class 2:1 (the octave). This formal foundation ensures that interval ratios are well-defined independently of specific frequency values (Wright, pp. 58-59).

# Examples

**Example 1** (p. 58): (2:3) = (4:6) = (1/2 : 3/4) -- all represent the same ratio.

**Example 2**: (440 : 220) = (880 : 440) = (2:1) -- the octave, regardless of specific frequencies.

**Example 3**: The bijection phi maps (3:2) to 3/2 = 1.5, the ratio associated with a nearly perfect fifth.

# Relationships

## Enables

- **Interval as Frequency Ratio** -- Ratio equivalence classes are the formal definition of intervals

## Related

- **Multiplicative Composition of Intervals** -- Multiplication of ratios is well-defined on equivalence classes

# Common Errors

- **Error**: Treating a ratio as a single ordered pair rather than an equivalence class
  **Correction**: A ratio is a class; many pairs represent the same ratio

# Common Confusions

- **Confusion**: Thinking the notation a:b represents a specific pair
  **Clarification**: a:b represents the entire equivalence class; a/b represents the corresponding real number
- **Confusion**: Assuming ratios must be rational numbers
  **Clarification**: This is a ratio of real numbers, not just integers; irrational ratios like sqrt(2):1 are perfectly valid

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 58-59. Formula 4.1 defines the bijection phi.

# Verification Notes

- Definition source: Direct from pp. 58-59
- Confidence rationale: High -- formal mathematical definition with explicit bijection
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: parallel to rational number construction, irrational ratio note, formula 4.1 reference
