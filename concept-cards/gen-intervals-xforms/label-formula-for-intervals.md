---
# === CORE IDENTIFICATION ===
concept: Label Formula for Intervals
slug: label-formula-for-intervals

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.1 The LABEL Function"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "interval from labels"
  - "LABEL recovery formula"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - label-function
  - interval-group-ivls
extends:
  - generalized-interval-system
related:
  - transposition-operation
  - interval-preserving-operation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can intervals be computed from LABEL values?"
  - "What distinguishes the LABEL function from the int function?"
---

# Quick Definition
The label formula for intervals states that int(s, t) = LABEL(s)^{-1} LABEL(t), allowing intervals between any two elements to be computed from their labels regardless of which referential element was chosen.

# Core Definition
Theorem 3.1.2 establishes that for any choice of ref, the interval between two elements s and t can be recovered from their labels: int(s, t) = LABEL(s)^{-1} LABEL(t). The derivation proceeds: LABEL(s)^{-1} LABEL(t) = int(ref, s)^{-1} int(ref, t) = int(s, ref) int(ref, t) = int(s, t), using GIS Condition (A) and Theorem 2.3.2 (Lewin, Theorem 3.1.2, p. 62).

# Prerequisites
- **LABEL Function** — The formula relates LABEL values to intervals
- **Interval Group (IVLS)** — The formula uses group inverse and product in IVLS

# Key Properties
1. int(s, t) = LABEL(s)^{-1} LABEL(t) for any ref
2. The formula uses group inverse on the left factor, not subtraction
3. In a commutative GIS, this simplifies to LABEL(t) * LABEL(s)^{-1}
4. The result is independent of which ref was chosen for LABEL
5. The formula generalizes the familiar computation of pitch-class intervals as differences of integer labels
6. The formula proves LABEL is bijective, since int can be recovered from LABELs and int determines the GIS

# Construction / Recognition
## To Construct:
1. Fix any ref in S
2. Compute LABEL(s) = int(ref, s) and LABEL(t) = int(ref, t)
3. Calculate LABEL(s)^{-1} in the group IVLS
4. Multiply: LABEL(s)^{-1} * LABEL(t) = int(s, t)

## To Recognize:
1. Any expression of the form x^{-1} y where x and y are labels of two elements yields the interval between those elements

# Context & Application
This formula is the primary computational tool when working within a LABEL system. It justifies the familiar practice of computing pitch-class intervals by subtraction (since in the commutative group Z/12Z, the group inverse of n is -n). It shows that while individual labels are ref-dependent, the intervals recovered from pairs of labels are always the same.

# Examples
**Example 1** (p. 62): In the 12-tone pitch-class GIS with ref = C:
- LABEL(E) = 4, LABEL(G) = 7
- int(E, G) = LABEL(E)^{-1} LABEL(G) = (-4) + 7 = 3 (mod 12)

**Example 2** (from old card): In the time-span GIS with ref = (0, 1):
- LABEL(2, 3) = (2, 3), LABEL(5, 6) = (5, 6)
- int((2, 3), (5, 6)) = (2, 3)^{-1} * (5, 6) = (-2/3, 1/3) * (5, 6) = (1, 2)
- Verification: int((2, 3), (5, 6)) = ((5-2)/3, 6/3) = (1, 2)

# Relationships
## Builds Upon
- **LABEL Function** — the formula uses LABEL values as inputs
- **Generalized Interval System** — the formula derives from GIS Conditions (A) and (B)

## Enables
- **Transposition Operation** — combined with LABEL(T_i(s)) = LABEL(s) * i
- **Interval-Preserving Operation** — combined with LABEL(P_i(s)) = i * LABEL(s)

## Related
- **Inversion Operation** — LABEL(I_u^v(s)) = i * LABEL(s)^{-1} * j uses a similar inverse-product structure

# Common Errors
- **Error**: Computing int(s,t) as LABEL(s) * LABEL(t)^{-1} (wrong order of inverse)
  **Correction**: The correct formula is LABEL(s)^{-1} * LABEL(t), with the inverse on the left factor

- **Error**: Using simple subtraction in a non-commutative GIS
  **Correction**: In non-commutative groups, the formula requires proper group inverse and left multiplication

# Common Confusions
- **Confusion**: Thinking the formula depends on which ref is chosen
  **Clarification**: Changing ref changes both labels, but the product LABEL(s)^{-1} LABEL(t) always yields the same int(s, t)

- **Confusion**: Believing this formula is specific to pitch-class spaces
  **Clarification**: The formula works universally in any GIS, including non-commutative ones like the time-span GIS

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.1.2, pp. 62-63.

# Verification Notes
- Definition source: direct from Theorem 3.1.2
- Confidence rationale: high — explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: time-span computation example, note about bijection proof
