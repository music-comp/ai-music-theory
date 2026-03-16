---
concept: Group of Interval-Preserving Operations (PSVS)
slug: group-of-interval-preserving-operations

category: generalized-interval-systems
subcategory: group-structure
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 79
section: "3.4"

extraction_confidence: high

aliases:
  - "PSVS"
  - "interval-preserving group"

prerequisites:
  - interval-preserving-operation
  - label-function
extends: []
related:
  - group-of-transpositions
  - anti-isomorphism
  - transposition-interval-preserving-commutativity
contrasts_with:
  - group-of-transpositions

answers_questions:
  - "How do interval-preserving operations compose?"
  - "What is the algebraic relationship between PSVS and IVLS?"
  - "How does PSVS differ from TNSPS structurally?"
---

# Quick Definition
The interval-preserving operations form a group (PSVS) that is isomorphic (not anti-isomorphic) to the interval group IVLS, with the composition formula P_i P_j = P_{ij} preserving multiplication order.

# Core Definition
**Theorem 3.4.5**: The transformations P_i form a group of operations on S that is isomorphic to IVLS under the map f(i) = P_i. The composition formula P_i P_j = P_{ij} holds, meaning the map preserves multiplication order -- in contrast to the anti-isomorphism between IVLS and TNSPS where T_i T_j = T_{ji}.

# Prerequisites
- **Interval-preserving operation (P_i)** — The individual operations that compose to form the group
- **LABEL function** — P_i is defined via LABEL(P_i(s)) = i * LABEL(s)

# Key Properties
1. Closure: P_i P_j = P_{ij} (the product is interval-preserving)
2. Identity: P_e = identity operation
3. Inverses: (P_i)^{-1} = P_{i^{-1}}
4. The map f(i) = P_i is an isomorphism: f(ij) = P_{ij} = P_i P_j = f(i)f(j)
5. In commutative GIS, PSVS = TNSPS; in non-commutative GIS, they are distinct

# Construction / Recognition
## To Construct:
1. Fix a reference point ref in S
2. For each interval i in IVLS, define P_i by LABEL(P_i(s)) = i * LABEL(s)
3. The collection of all P_i forms PSVS
## To Recognize:
1. A group of operations on S where composition preserves the multiplication order of IVLS
2. Operations that left-multiply labels (contrast with transpositions that right-multiply)

# Context & Application
PSVS captures all transformations that preserve intervallic relationships. The isomorphism with IVLS (versus anti-isomorphism for transpositions) reflects the different algebraic roles of left-multiplication (P) and right-multiplication (T) of labels. In commutative GIS, PSVS and TNSPS coincide. In non-commutative GIS, they are distinct groups with structurally different relationships to IVLS.

# Examples
**Example 1** (p. 79): Proof that P_i P_j = P_{ij}: LABEL(P_i(P_j(s))) = i * LABEL(P_j(s)) = i * (j * LABEL(s)) = (ij) * LABEL(s) = LABEL(P_{ij}(s)). So P_i P_j = P_{ij}.

**Example 2**: In the commutative pitch-class GIS, PSVS = TNSPS since P_5 P_3 = P_8 = P_{5+3} and T_5 T_3 = T_{3+5} = T_8.

**Example 3**: In the time-span GIS, P_{(h,u)} P_{(k,v)} = P_{(h,u)(k,v)} = P_{(h+uk, uv)}. The order is preserved, unlike T_{(h,u)} T_{(k,v)} = T_{(k,v)(h,u)} = T_{(k+vh, vu)}.

# Relationships
## Builds Upon
- **Interval-preserving operation** — The individual maps whose group structure is established
- **LABEL function** — The computational tool for proving composition formulas
## Enables
- **PETEY group** — PSVS is one of the two subgroups generating PETEY
- **Transposition-interval-preserving commutativity** — PSVS and TNSPS always commute
## Contrasts With
- **Group of transpositions (TNSPS)** — Anti-isomorphic to IVLS (T_i T_j = T_{ji}), while PSVS is isomorphic

# Common Errors
- **Error**: Assuming P_i P_j = P_{ji} by analogy with transpositions
  **Correction**: P operations preserve order: P_i P_j = P_{ij}

# Common Confusions
- **Confusion**: PSVS and TNSPS are always the same group
  **Clarification**: They coincide only in commutative GIS; in non-commutative GIS they are distinct
- **Confusion**: The specific P_i depends on ref, so the group must also depend on ref
  **Clarification**: While individual P_i operations depend on the choice of ref, the group PSVS as a whole is the same regardless of ref

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.5, pages 79-80.

# Verification Notes
- Definition source: Direct from Theorem 3.4.5
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: proof sketch, time-span example, PSVS vs TNSPS contrast, ref-dependence discussion
