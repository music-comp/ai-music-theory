---
# === CORE IDENTIFICATION ===
concept: PETEY Group
slug: petey-group

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
section: "3.5 Inversions"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "PT group"
  - "group of direct transformations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transposition-operation
  - interval-preserving-operation
extends:
  - group-of-transpositions
  - group-of-interval-preserving-operations
related:
  - petinv-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the PETEY group?"
  - "How do transpositions and interval-preserving operations combine?"
---

# Quick Definition
PETEY is the group of all operations on a GIS space that can be expressed as PT, where P is an interval-preserving operation and T is a transposition. It combines the two fundamental families of "direct" (non-inverting) transformations.

# Core Definition
Theorem 3.5.11(A) defines PETEY as the family of all operations expressible as PT (P interval-preserving, T transposition) and proves it is a group. Closure follows because (PT)(P'T') = (PP')(TT') (using Theorem 3.4.10: every T commutes with every P). Inverses exist because (PT)^{-1} = P^{-1}T^{-1}, which is again in PETEY (Lewin, Theorem 3.5.11(A), pp. 89-90).

# Prerequisites
- **Transposition Operation** — One of the two generating families
- **Interval-Preserving Operation** — The other generating family

# Key Properties
1. PETEY = {PT : P is interval-preserving, T is transposition}
2. PETEY is closed: (PT)(P'T') = (PP')(TT') by Theorem 3.4.10
3. Inverses exist: (PT)^{-1} = P^{-1}T^{-1} is in PETEY
4. In commutative GIS: PETEY = TNSPS = PSVS (all three coincide)
5. In non-commutative GIS: PETEY is strictly larger than either TNSPS or PSVS
6. PT = TP for all P, T (Theorem 3.4.10)

# Construction / Recognition
## To Construct:
1. Take any interval-preserving operation P_i
2. Take any transposition T_j
3. Compose: P_i T_j (or equivalently T_j P_i) is in PETEY

## To Recognize:
1. The operation can be decomposed into an interval-preserving part and a transposition part

# Context & Application
PETEY represents all "direct" transformations — those not involving inversion. The name combines "P" and "T" into a pronounceable word. In commutative GIS, PETEY reduces to the familiar transposition group. In non-commutative GIS, PETEY has a richer structure that reflects the independent roles of transposition and interval preservation.

# Examples
**Example 1**: In commutative 12-tone GIS: PETEY = {T_0, T_1, ..., T_{11}} since T_i = P_i.

**Example 2**: In non-commutative time-span GIS: PETEY contains all compositions P_{(h,u)} T_{(i,p)}, forming a group larger than either the transposition group or the interval-preserving group alone.

# Relationships
## Builds Upon
- **Group of Transpositions** — TNSPS is a subgroup of PETEY
- **Group of Interval-Preserving Operations** — PSVS is a subgroup of PETEY

## Enables
- **PETINV Group** — PETEY plus inversions forms PETINV

# Common Errors
- **Error**: Assuming the order PT matters
  **Correction**: PT = TP by Theorem 3.4.10, so the order is irrelevant

# Common Confusions
- **Confusion**: Thinking PETEY is always larger than TNSPS
  **Clarification**: In commutative GIS, PETEY = TNSPS. The distinction arises only in non-commutative GIS.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.11(A), pp. 89-90.

# Verification Notes
- Definition source: direct from Theorem 3.5.11(A)
- Confidence rationale: high — explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: closure proof outline, commutative simplification
