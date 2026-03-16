---
# === CORE IDENTIFICATION ===
concept: Composition of Inversion Operations
slug: inversion-composition

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: inversion-theory
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 88
section: "3.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "inversion product"
  - "composition of inversions"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inversion-operation
  - interval-preserving-operation
  - transposition-operation
extends: []
related:
  - petey-group
  - petinv-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What operation results from composing two inversions?"
  - "What is the inverse of an inversion operation?"
  - "Why does the composition of two inversions yield a 'direct' transformation?"
---

# Quick Definition
The composition of two inversion operations yields an operation of the form PT (interval-preserving followed by transposition). This establishes that inversions, combined with transpositions and interval-preserving operations, form a closed group.

# Core Definition
**Theorem 3.5.8**: Fix ref, and let the LABELs of v, u, w, and x be respectively i, j, k, and m. Then:

$$I_u^v I_x^w = P_{im^{-1}} T_{k^{-1}j}$$

The composition of two inversions is an interval-preserving operation (with parameter im^{-1}) followed by a transposition (by interval k^{-1}j). **Corollary 3.5.9**: I_v^u is the inverse operation to I_u^v.

# Prerequisites
- **Inversion operation (I_u^v)** — The operations being composed
- **Interval-preserving operation (P_i)** — One component of the result
- **Transposition operation (T_i)** — The other component of the result

# Key Properties
1. Two inversions compose to give a PT operation (not another inversion)
2. The inverse of I_u^v is I_v^u (Corollary 3.5.9)
3. In commutative GIS, I^{-1} = I (inversions are self-inverse, Corollary 3.5.10(A))
4. In commutative GIS, IT = T^{-1}I (Corollary 3.5.10(B))

# Construction / Recognition
## To Construct:
1. Fix ref and compute LABELs: i = LABEL(v), j = LABEL(u), k = LABEL(w), m = LABEL(x)
2. The composition I_u^v I_x^w equals P_{im^{-1}} T_{k^{-1}j}
## To Recognize:
1. Two successive inversions yield a "direct" (non-inverting) transformation
2. The result decomposes into an interval-preserving operation and a transposition

# Context & Application
This theorem is fundamental for understanding the group structure of transformations in a GIS. It shows that the set of inversions is not itself a group (since the composition of two inversions is not an inversion), but that inversions together with transpositions and interval-preserving operations do form a group (PETINV, Theorem 3.5.11).

# Examples
**Example 1** (pp. 88-89): Proof of Corollary 3.5.9: Take x = v, w = u in Theorem 3.5.8. Then m = i and k = j, so im^{-1} = e and k^{-1}j = e. Thus I_u^v I_v^u = P_e T_e = identity.

**Example 2**: In the commutative pitch-class GIS with ref = C: I_0 I_6 computes as a transposition T_6 (since P = T in commutative case). Generally, I_a I_b = T_{b-a}.

**Example 3**: In commutative GIS, Corollary 3.5.10(A) shows I^{-1} = I (inversions are involutions), contrasting with the non-commutative case where (I_s^t)^{-1} = I_t^s may differ from I_s^t.

# Relationships
## Builds Upon
- **Inversion operation** — The operations being composed
- **Interval-preserving operation** — Component of the composition result
- **Transposition operation** — Component of the composition result
## Enables
- **PETINV group** — The closure result (Theorem 3.5.11) depends on this composition formula
- **PETEY group** — The "direct" transformations that inversions compose into

# Common Errors
- **Error**: Assuming the composition of two inversions is another inversion
  **Correction**: Two inversions compose to give a PT operation, not an inversion

# Common Confusions
- **Confusion**: The inverse of I_u^v is I_u^v itself
  **Clarification**: The inverse is I_v^u; in commutative GIS I_u^v = I_v^u so inversions are self-inverse, but in non-commutative GIS these may differ
- **Confusion**: The formula involves both P and T redundantly in commutative GIS
  **Clarification**: Even when P = T (commutative case), the formula structure reveals the two separate algebraic roles

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.8 and Corollaries 3.5.9-3.5.10, pages 88-89.

# Verification Notes
- Definition source: Direct from Theorem 3.5.8
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: proof details, commutative corollaries, pitch-class calculation examples
