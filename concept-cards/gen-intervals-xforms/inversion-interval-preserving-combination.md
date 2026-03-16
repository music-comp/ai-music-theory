---
# === CORE IDENTIFICATION ===
concept: Combination of Inversion and Interval-Preserving Operations
slug: inversion-interval-preserving-combination

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
pdf_page: 87
section: "3.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inversion-operation
  - interval-preserving-operation
  - central-interval
extends: []
related:
  - inversion-transposition-combination
  - inversion-equivalence-conditions
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens when an interval-preserving operation is composed with an inversion?"
  - "Which parameter of the inversion moves under left vs. right composition with P?"
  - "When does an interval-preserving operation commute with an inversion?"
---

# Quick Definition
When an interval-preserving operation P and an inversion I_u^v are composed, the result is another inversion. P moves one parameter while leaving the other fixed, with the moving parameter depending on the order of composition.

# Core Definition
**Theorem 3.5.7**: For any interval-preserving operation P and any inversion I_u^v:
- (A): PI_u^v = I_u^w where w = P(v) -- P on the left moves v to P(v)
- (B): I_u^v P = I_x^v where x = P^{-1}(u) -- P on the right moves u to P^{-1}(u)
- (C): P commutes with I_u^v if and only if P = T_c for some transposition T_c such that c is central and cc = e

# Prerequisites
- **Inversion operation (I_u^v)** — One of the operations being composed
- **Interval-preserving operation (P)** — The other operation being composed
- **Central interval** — Governs the commutation condition

# Key Properties
1. Left composition PI_u^v moves v to P(v) while fixing u
2. Right composition I_u^v P moves u to P^{-1}(u) while fixing v
3. The moving parameter differs depending on composition order
4. Commutation requires P to be a transposition T_c with c central and involutory
5. The proof parallels Theorem 3.5.6 for transpositions, using LABEL manipulations

# Construction / Recognition
## To Construct:
1. To compute PI_u^v: apply I_u^v first, then P; result is I_u^{P(v)}
2. To compute I_u^v P: apply P first, then I_u^v; result is I_{P^{-1}(u)}^v
## To Recognize:
1. Composition of P and I always yields another inversion
2. Identify which parameter (u or v) has changed and by what transformation

# Context & Application
This theorem parallels Theorem 3.5.6 for transpositions. In commutative GIS where P = T, this reduces to the transposition theorem. In non-commutative GIS, it distinguishes the roles of P and T when composing with inversions. The commutation condition (C) shows that the P operations commuting with inversions are precisely the transpositions with central, self-inverse intervals -- the same condition as for T operations.

# Examples
**Example 1** (p. 87): In commutative pitch-class GIS (where P = T): P_5 I_C^C = I_C^F (v moves from C to P_5(C) = F); I_C^C P_5 = I_{Ab}^C (u moves from C to P_5^{-1}(C) = Ab).

**Example 2**: In non-commutative time-span GIS: P_{(h,u)} I_{(c,z)}^{(d,w)} = I_{(c,z)}^{P_{(h,u)}(d,w)}. The "u" parameter (c, z) stays fixed; the "v" parameter (d, w) is transformed to P_{(h,u)}(d, w) = (h + ud, uw).

**Example 3**: Commutation: P commutes with some inversion iff P is a transposition T_c with c central and cc = e. In the time-span GIS, only the identity satisfies this, so no non-trivial P commutes with any inversion.

# Relationships
## Builds Upon
- **Inversion operation** — One component of the composition
- **Interval-preserving operation** — The other component
## Enables
- **PETINV group** — Closure of inversions under composition with P and T operations
## Related
- **Inversion-transposition combination** — Parallel theorem (3.5.6) for T operations
## Contrasts With
- **Inversion-transposition combination** — T on the left moves u (T_n I_u^v = I_{T_n(u)}^v), while P on the left moves v (PI_u^v = I_u^{P(v)})

# Common Errors
- **Error**: Assuming PI_u^v moves the same parameter as TI_u^v
  **Correction**: PI_u^v moves v to P(v); TI_u^v moves u to T(u) -- opposite parameters

# Common Confusions
- **Confusion**: In commutative GIS where P = T, the distinction between these two theorems seems unnecessary
  **Clarification**: The distinction becomes critical in non-commutative GIS where P and T are genuinely different operations
- **Confusion**: The group-theoretic lemma about conjugates (j^{-1}nj is central iff n is central) seems unrelated
  **Clarification**: This lemma is essential for proving part (C) of the theorem

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.7, pages 87-88.

# Verification Notes
- Definition source: Direct from Theorem 3.5.7
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: P vs T parameter distinction, commutation condition, time-span examples
