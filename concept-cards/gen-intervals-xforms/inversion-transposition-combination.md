---
concept: Combination of Inversion and Transposition
slug: inversion-transposition-combination

category: generalized-interval-systems
subcategory: inversion-theory
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 86
section: "3.5"

extraction_confidence: high

aliases: []

prerequisites:
  - inversion-operation
  - transposition-operation
  - central-interval
  - involutory-elements
extends: []
related:
  - inversion-interval-preserving-combination
  - inversion-equivalence-conditions
contrasts_with: []

answers_questions:
  - "What happens when a transposition is composed with an inversion?"
  - "When does T_n commute with an inversion I_u^v?"
  - "Why is T_6 special in 12-tone pitch-class theory?"
---

# Quick Definition
When a transposition T_n and an inversion I_u^v are composed, the result is another inversion whose parameters are shifted by transposition. T_n commutes with I_u^v if and only if n is central and nn = e.

# Core Definition
**Theorem 3.5.6**: For any transposition T_n and any inversion I_u^v:
- (A): T_n I_u^v = I_x^v where x = T_n(u) -- T on the left moves u to T_n(u)
- (B): I_u^v T_n = I_u^w where w = T_n^{-1}(v) = T_{n^{-1}}(v) -- T on the right moves v to T_{n^{-1}}(v)
- (C): T_n commutes with I_u^v if and only if n is central and nn = e (n is involutory)

# Prerequisites
- **Inversion operation (I_u^v)** — One of the operations being composed
- **Transposition operation (T_n)** — The other operation being composed
- **Central interval** — Required for commutation
- **Involutory elements** — The self-inverse condition nn = e

# Key Properties
1. Left composition T_n I_u^v moves u to T_n(u) while fixing v
2. Right composition I_u^v T_n moves v to T_{n^{-1}}(v) while fixing u
3. Commutation requires both centrality and involutory conditions
4. Either T_n commutes with every inversion or with no inversion (part (C) is independent of u, v)

# Construction / Recognition
## To Construct:
1. To compute T_n I_u^v: result is I_{T_n(u)}^v
2. To compute I_u^v T_n: result is I_u^{T_{n^{-1}}(v)}
3. To check commutation: verify n is central and n^2 = e
## To Recognize:
1. Composition of T and I always yields another inversion
2. The parameter that moves depends on the order of composition

# Context & Application
These formulas generalize the familiar "T_n I = IT_n" relationships from pitch-class theory. In the 12-tone GIS, T_6 commutes with every inversion (since 6 + 6 = 0 mod 12 and 6 is central), while no other non-trivial transposition does. This explains the special role of the tritone in twelve-tone operations. The formulas are essential for understanding the PETINV group structure.

# Examples
**Example 1** (p. 87): In 12-tone pitch-class GIS: T_6 commutes with every inversion because 6 is central (trivially, since group is commutative) and 6 + 6 = 0 mod 12 (involutory). No other non-zero transposition has both properties.

**Example 2** (p. 86): T_5 I_C^C = I_{T_5(C)}^C = I_F^C. And I_C^C T_5 = I_C^{T_{-5}(C)} = I_C^G. These differ, confirming T_5 does not commute with I_C^C.

**Example 3**: In non-commutative time-span GIS: only the identity (0, 1) satisfies nn = e and is central, so no non-trivial transposition commutes with any inversion.

# Relationships
## Builds Upon
- **Inversion operation** — One component of the composition
- **Transposition operation** — The other component
- **Central interval** — Required for commutation
- **Involutory elements** — Required for commutation
## Enables
- **PETINV group** — The composition formulas establish closure
## Contrasts With
- **Inversion-interval-preserving combination** — P on the left moves v (PI_u^v = I_u^{P(v)}), while T on the left moves u (TI_u^v = I_{T(u)}^v)

# Common Errors
- **Error**: Assuming T_n I = I T_n in general
  **Correction**: T_n commutes with inversions only when n is central and involutory

# Common Confusions
- **Confusion**: T_n I_u^v = I_u^v T_n always holds because transposition and inversion "commute"
  **Clarification**: They commute only when n is both central and self-inverse; otherwise T_n moves different parameters on left vs. right
- **Confusion**: The asymmetry in the formulas (T on left moves u; T on right moves v with inverse)
  **Clarification**: This asymmetry is intrinsic to how transposition interacts with inversion parameters; note the inverse T_{n^{-1}} appearing in part (B)

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.6, pages 86-87.

# Verification Notes
- Definition source: Direct from Theorem 3.5.6
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: T_6 tritone example, time-span example, parameter movement asymmetry
