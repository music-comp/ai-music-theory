---
# === CORE IDENTIFICATION ===
concept: Central Interval
slug: central-interval

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: group-structure
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 81
section: "3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "central element"
  - "element of the center"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
extends: []
related:
  - transposition-and-interval-preservation
  - inversion-equivalence-conditions
  - involutory-elements
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When does a transposition T_i preserve intervals?"
  - "When does T_i equal P_i?"
  - "What determines whether two inversions with different parameters are the same operation?"
---

# Quick Definition
A central interval is an element of IVLS that commutes with every other element in the group. Central intervals determine when transpositions preserve intervals and when inversions with different parameters are equivalent.

# Core Definition
An interval i in IVLS is **central** if ij = ji for every j in IVLS (Definition 1.8.2). The set of all central elements forms the **center** of the group, denoted Z(IVLS). Theorem 3.4.8 establishes that T_i preserves intervals if and only if i is central, and Theorem 3.5.3 shows that inversion equivalence I_u^v = I_x^w requires int(x, u) to be central.

# Prerequisites
- **Generalized interval system** — The framework in which centrality is defined

# Key Properties
1. The center Z(IVLS) is a subgroup of IVLS
2. In a commutative group, every element is central: Z(IVLS) = IVLS
3. T_i preserves intervals if and only if i is central (Theorem 3.4.8)
4. T_i = P_i (for any choice of ref) if and only if i is central (Theorem 3.4.8)
5. I_u^v = I_x^w requires int(x, u) to be central (Theorem 3.5.3)

# Construction / Recognition
## To Construct:
1. Given an interval i in IVLS, test whether ij = ji for all j in IVLS
2. If yes, i is central; the set of all such i forms the center
## To Recognize:
1. An interval whose transposition T_i preserves all intervallic relationships
2. An interval that commutes with every other interval under the group operation

# Context & Application
Central intervals are the "well-behaved" intervals whose transpositions act like familiar pitch-class transpositions. The center measures how commutative the group is: in a commutative group the center is the entire group; in highly non-commutative groups, only the identity may be central. In the time-span GIS (Chapter 4), only the identity (0, 1) is central, meaning no non-trivial transposition preserves intervals.

# Examples
**Example 1** (p. 82): In the commutative pitch-class group Z/12Z, all 12 intervals are central. Z(Z/12Z) = Z/12Z, so every T_i = P_i and every transposition preserves intervals.

**Example 2** (p. 113, Notes 4.1.7): In the time-span interval group, only (0, 1) is central. Proof: if (i, p)(j, q) = (j, q)(i, p) for all (j, q), then (i + pj, pq) = (j + qi, qp) for all j, q. This requires p = 1 and i = 0. Consequence: only T_{(0,1)} = identity preserves intervals.

# Relationships
## Builds Upon
- **Generalized interval system** — Centrality is defined within the interval group
## Enables
- **Transposition and interval preservation** — Centrality is the criterion for interval preservation
- **Inversion equivalence conditions** — Centrality constrains when inversions coincide
## Related
- **Involutory elements** — Both centrality and involutory properties are needed for T_n to commute with inversions

# Common Errors
- **Error**: Assuming all transpositions preserve intervals
  **Correction**: Only T_i with central i preserves intervals; in non-commutative GIS this excludes most transpositions

# Common Confusions
- **Confusion**: Central elements are the same as the identity
  **Clarification**: The identity is always central, but in commutative groups every element is central; centrality is a property of the group structure, not of individual elements in isolation
- **Confusion**: Centrality is a property of the GIS realization rather than the abstract group
  **Clarification**: Whether i is central depends solely on the group IVLS, not on any particular choice of ref or musical interpretation

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.8 and Corollary 3.4.9, pages 81-82.

# Verification Notes
- Definition source: Direct from Definition 1.8.2 and Theorem 3.4.8
- Confidence rationale: High -- explicitly defined and proved
- Re-extraction notes: Re-extracted from v2 card; preserved: time-span centrality proof, commutative/non-commutative distinction, role in inversion theory
