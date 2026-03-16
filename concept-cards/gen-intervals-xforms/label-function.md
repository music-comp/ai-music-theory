---
concept: LABEL Function
slug: label-function

category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.1 The LABEL Function"

extraction_confidence: high

aliases:
  - "LABEL"
  - "labeling function"
  - "ref-labeling"

prerequisites:
  - generalized-interval-system
  - interval-group-ivls
  - reference-point
extends:
  - interval-function-int
related:
  - transposition-operation
  - interval-preserving-operation
  - label-formula-for-intervals
contrasts_with:
  - interval-function-int

answers_questions:
  - "What is the LABEL function?"
  - "What distinguishes the LABEL function from the int function?"
  - "How does the choice of reference point affect labeling?"
---

# Quick Definition
The LABEL function maps elements of a GIS space S to elements of the interval group IVLS by measuring the interval from a fixed reference point to each element, providing a coordinate system for the space.

# Core Definition
Given a GIS (S, IVLS, int) and a fixed referential member "ref" of S, the function LABEL maps S into IVLS by the equation LABEL(s) = int(ref, s). Theorem 3.1.2 establishes that LABEL is a bijection (1-to-1 and onto) from S to IVLS, and satisfies int(s, t) = LABEL(s)^{-1} LABEL(t) (Lewin, Definition 3.1.1, p. 62).

# Prerequisites
- **Generalized Interval System** — The LABEL function is defined within a GIS (S, IVLS, int)
- **Interval Group (IVLS)** — LABEL maps into IVLS, so group structure must be understood
- **Reference Point** — The LABEL function depends on a chosen referential element ref in S

# Key Properties
1. LABEL(s) = int(ref, s) for every s in S
2. LABEL is 1-to-1: distinct elements receive distinct labels
3. LABEL is onto: every interval in IVLS is the label of some element
4. int(s, t) = LABEL(s)^{-1} LABEL(t) — intervals can be recovered from labels
5. The specific values of LABEL depend on the choice of ref
6. LABEL(ref) = e (the identity element of IVLS)

# Construction / Recognition
## To Construct:
1. Choose a GIS (S, IVLS, int)
2. Fix a referential element ref in S
3. For each s in S, compute LABEL(s) = int(ref, s)

## To Recognize:
1. Identify any function mapping S to IVLS via intervals from a fixed point
2. Verify the bijection property
3. Verify the interval-recovery formula int(s, t) = LABEL(s)^{-1} LABEL(t)

# Context & Application
The LABEL function generalizes the familiar convention of labeling pitch classes by integers from C: C=0, C#=1, ..., B=11 (mod 12). While computationally useful, LABEL introduces conceptual problems. There may be no musically adequate reason for privileging a particular ref. Why should C be referential rather than E or A? The labeling choice can also introduce irrelevant algebraic artifacts from the ref-to-s intervals when computing relationships among other elements.

These issues parallel fixed-do versus movable-do controversies in solfege pedagogy.

# Examples
**Example 1** (p. 62): In the twelve-tone pitch-class GIS with ref = C:
- LABEL(C) = 0, LABEL(C#) = 1, LABEL(D) = 2, ..., LABEL(B) = 11
- int(E, G) = LABEL(E)^{-1} LABEL(G) = (-4 + 7) = 3 (mod 12)

**Example 2** (p. 62): In the just-intonation GIS (example 2.1.5-2.1.6), elements are labeled by frequency ratios from a reference pitch, with the LABEL function providing coordinates on the game-board model.

# Relationships
## Builds Upon
- **Generalized Interval System** — LABEL is defined within any GIS
- **Reference Point** — LABEL requires fixing ref in S

## Enables
- **Transposition Operation** — LABEL(T_i(s)) = LABEL(s) * i (Theorem 3.4.3)
- **Interval-Preserving Operation** — LABEL(P_i(s)) = i * LABEL(s) (Definition 3.4.4)
- **Inversion Operation** — LABEL(I_u^v(s)) = i * LABEL(s)^{-1} * j (Theorem 3.5.2)

## Related
- **Label Formula for Intervals** — the recovery formula int(s,t) = LABEL(s)^{-1} LABEL(t)

## Contrasts With
- **Interval Function (int)** — int measures between any two elements; LABEL measures from a fixed ref

# Common Errors
- **Error**: Treating LABEL values as intrinsic properties of elements
  **Correction**: LABEL(s) depends entirely on the choice of ref; changing ref changes all labels

- **Error**: Computing intervals by subtracting labels in a non-commutative GIS
  **Correction**: The formula is int(s,t) = LABEL(s)^{-1} LABEL(t), using group inverse and product, not subtraction

# Common Confusions
- **Confusion**: Believing labels are the same as intervals
  **Clarification**: A label is an interval from ref to s; an interval int(s,t) relates two arbitrary elements. Labels are extrinsic (ref-dependent); intervals between pairs are intrinsic to the GIS.

- **Confusion**: Assuming LABEL is unique for a GIS
  **Clarification**: Each choice of ref produces a different LABEL function, though all yield the same intervals via the recovery formula.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definition 3.1.1 and Theorem 3.1.2, pp. 62-63.

# Verification Notes
- Definition source: direct from Definition 3.1.1
- Key properties: derived from Theorem 3.1.2 and surrounding discussion
- Confidence rationale: high — explicit definition with formal proof
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, just-intonation example, confusion about intrinsic vs extrinsic nature of labels
