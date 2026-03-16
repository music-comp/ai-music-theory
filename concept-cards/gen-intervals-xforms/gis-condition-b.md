---
# === CORE IDENTIFICATION ===
concept: GIS Condition B
slug: gis-condition-b

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: core-definitions
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.3.1(B)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - space completeness condition
  - existence and uniqueness condition

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - group
extends: []
related:
  - gis-condition-a
  - additive-duration-space
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a Generalized Interval System (GIS)?"
  - "How does the interval function int relate to the group IVLS?"
---

# Quick Definition

Condition (B) states that for any starting element s and any interval i, there exists a unique element t such that int(s, t) = i -- the space is "complete" and intervals act freely.

# Core Definition

"For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s, that is a unique t which satisfies the equation int(s, t) = i" (Lewin, Definition 2.3.1(B), p. 52). This guarantees that S is large enough to contain all theoretically conceivable elements. It also establishes a bijection between IVLS and S (for any fixed s), so |S| = |IVLS|.

# Prerequisites

- **Generalized Interval System** — Condition (B) is part of the GIS definition
- **Group** — IVLS must be a group for Condition (B) to make sense

# Key Properties

1. For all s in S and i in IVLS: exists unique t with int(s, t) = i
2. Requires BOTH existence AND uniqueness
3. Establishes a bijection from IVLS to S (for fixed s)
4. |S| = |IVLS| (same cardinality)
5. "Weak B" (existence without uniqueness) leads to equivalence classes

# Construction / Recognition

## To Verify:
1. For each s in S and each i in IVLS, check that some t exists with int(s, t) = i
2. Check that this t is unique

# Context & Application

Condition (B) forces the space S to be "theoretically complete." This may require extending practical spaces: chromatic pitch space must include supersonic and subsonic "pitches," Figure 2.2 extends infinitely, and time-point space extends indefinitely past and future. Lewin emphasizes: "We must conceive the formal space of a GIS as a space of theoretical potentialities, rather than as a compendium of musical practicalities."

# Examples

**Example 1** (p. 56): Example 2.2.5 (additive durations) FAILS Condition (B). For s = 3 units and i = -8, no duration t satisfies int(s, t) = -8, since t = -5 units is not a duration.

**Example 2**: Pitch-class space SATISFIES Condition (B): for any pitch class s and any i in {0,...,11}, exactly one pitch class t has int(s, t) = i.

**Example 3** (p. 52): Harmonic space (Figure 2.2) extends infinitely to satisfy Condition (B) -- every dominant/mediant combination must be reachable from every starting point.

**Example 4** (p. 53): "Weak B" discussion: replacing "unique" with "some" leads to equivalence classes. The quotient S/EQUIV then forms a GIS.

# Relationships

## Builds Upon
- **Group** — the interval group whose elements must all be realizable

## Related
- **GIS Condition A** — the other GIS condition
- **Additive Duration Space** — the key example where Condition (B) fails

# Common Errors

- **Error**: Checking only existence without checking uniqueness.
  **Correction**: Condition (B) requires BOTH: there must be a t, and it must be the only such t.

# Common Confusions

- **Confusion**: Thinking Condition (B) failure means the musical space is "wrong."
  **Clarification**: It means the space as defined cannot support a GIS structure. Modular reduction (as in Example 2.2.6) or space extension may provide a remedy.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(B), discussion pp. 52-53, 56.

# Verification Notes

- Definition source: direct from Definition 2.3.1(B)
- Confidence rationale: explicit condition with extended discussion of its meaning and failure cases
- Re-extracted from v2 card; preserved: additive duration failure example, "weak B" discussion, "theoretical potentialities" quote
