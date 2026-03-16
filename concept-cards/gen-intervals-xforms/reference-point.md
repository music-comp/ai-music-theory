---
concept: Reference Point (ref)
slug: reference-point

category: generalized-interval-systems
subcategory: labeling-and-coordinates
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.1"

extraction_confidence: high

aliases:
  - "ref"
  - "referential member"
  - "referential element"

prerequisites:
  - generalized-interval-system
extends: []
related:
  - label-function
  - interval-preserving-operation
  - transposition-operation
contrasts_with: []

answers_questions:
  - "What is the role of a reference point in a GIS?"
  - "How does choosing a different ref affect labels?"
  - "Why is the choice of ref methodologically significant?"
---

# Quick Definition
A reference point (ref) is a fixed element in a GIS space S chosen to define the LABEL function via LABEL(s) = int(ref, s), establishing a coordinate system for the space.

# Core Definition
Given a GIS (S, IVLS, int), a **referential member** ref is any chosen element of S used to define the LABEL function: LABEL(s) = int(ref, s) (Definition 3.1.1). By Theorem 3.1.2, whatever the choice of ref, LABEL maps S one-to-one onto IVLS and satisfies int(s, t) = LABEL(s)^{-1}LABEL(t). The choice of ref determines how elements are labeled but does not affect the intrinsic intervallic structure.

# Prerequisites
- **Generalized interval system** — The structure within which ref is chosen

# Key Properties
1. LABEL(ref) = int(ref, ref) = e (the identity element of IVLS)
2. LABEL is a bijection from S onto IVLS (Theorem 3.1.2)
3. int(s, t) = LABEL(s)^{-1}LABEL(t) regardless of ref choice
4. Changing ref changes all labels: LABEL_{ref2}(s) = int(ref1, ref2) * LABEL_{ref1}(s)
5. The intervallic structure is independent of ref

# Construction / Recognition
## To Construct:
1. Choose any element ref in S
2. Define LABEL(s) = int(ref, s) for all s in S
3. Use LABEL to coordinate the space
## To Recognize:
1. An element of S serving as the "origin" for a labeling system
2. The element whose LABEL is the identity e

# Context & Application
The reference point corresponds to choosing an "origin" in musical space. Lewin raises aesthetic and methodological concerns about this choice (pp. 62-63): choosing C as reference for pitch classes privileges one pitch class over others a priori, and computations can be "muddled by the algebraic influence of irrelevant intervals" arising from irrelevant relations of ref to the objects under study. He suggests the music itself may project a referential element, and the choice should be context-sensitive.

# Examples
**Example 1** (p. 62): The familiar convention of labeling pitch classes by integers 0-11 uses C as ref. Then LABEL(C) = 0, LABEL(C#) = 1, ..., LABEL(B) = 11.

**Example 2** (p. 63): For string music, A might be a more methodologically natural ref (tuning reference). For a piece in E, using E as ref yields LABEL(E) = 0, centering the labeling on the tonic.

**Example 3**: Fixed-do vs. movable-do analogy: fixed-do uses a fixed ref (C), while movable-do uses a contextual ref (the local tonic).

# Relationships
## Builds Upon
- **Generalized interval system** — The structure within which ref is chosen
## Enables
- **LABEL function** — Defined by the choice of ref
- **Interval-preserving operation** — P_i depends on ref: LABEL(P_i(s)) = i * LABEL(s)
- **Transposition operation** — T_i is independent of ref but proved using LABELs
## Related
- **Interval function (int)** — The intrinsic structure that is independent of ref

# Common Errors
- **Error**: Assuming the choice of ref affects the GIS structure
  **Correction**: Changing ref changes labels but not intervals; the GIS is the same regardless of ref

# Common Confusions
- **Confusion**: The reference point is the same as a musical "tonic" or "center"
  **Clarification**: ref is a formal construct for computational convenience; tonics are perceptual/analytical concepts. They may coincide but are conceptually distinct
- **Confusion**: Different ref choices yield different GIS structures
  **Clarification**: Different ref choices yield different labeling systems within the same GIS

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Section 3.1, pages 62-63.

# Verification Notes
- Definition source: Direct from Definition 3.1.1 and Theorem 3.1.2
- Confidence rationale: High -- formally defined with explicit theorem
- Re-extraction notes: Re-extracted from v2 card; preserved: fixed/movable-do analogy, tuning reference example, methodological concerns about "muddled" computations
