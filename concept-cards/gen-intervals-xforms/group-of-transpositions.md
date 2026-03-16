---
concept: Group of Transpositions (TNSPS)
slug: group-of-transpositions

category: generalized-interval-systems
subcategory: group-structure
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 77
section: "3.4"

extraction_confidence: high

aliases:
  - "TNSPS"
  - "transposition group"

prerequisites:
  - transposition-operation
extends: []
related:
  - anti-isomorphism
  - group-of-interval-preserving-operations
  - transposition-interval-preserving-commutativity
contrasts_with:
  - group-of-interval-preserving-operations

answers_questions:
  - "Do the transpositions form a group?"
  - "How do transpositions compose?"
  - "What is the algebraic relationship between the transposition group and the interval group?"
---

# Quick Definition
The transposition operations in a GIS form a group (TNSPS) that is anti-isomorphic to the interval group IVLS: the map f(i) = T_i reverses multiplication order, yielding T_i T_j = T_{ji}.

# Core Definition
**Theorem 3.4.2**: Each T_i is an operation (one-to-one and onto) on S. The transposition operations form a group of operations on S, anti-isomorphic to IVLS. Specifically, the map f(i) = T_i is an anti-isomorphism from IVLS onto TNSPS, giving the composition formula T_i T_j = T_{ji}.

# Prerequisites
- **Transposition operation (T_i)** — The individual operations that compose to form the group

# Key Properties
1. Closure: T_i T_j = T_{ji} (the composition is a transposition)
2. Identity: T_e = identity operation
3. Inverses: (T_i)^{-1} = T_{i^{-1}}
4. The map f(i) = T_i is an anti-isomorphism: f(ij) = T_{ij} but f(i)f(j) = T_i T_j = T_{ji}
5. In commutative IVLS, T_i T_j = T_{ij} = T_{ji}, and the anti-isomorphism is also an isomorphism

# Construction / Recognition
## To Construct:
1. For each interval i in IVLS, T_i is the unique operation satisfying int(s, T_i(s)) = i for all s
2. The collection {T_i : i in IVLS} forms TNSPS
## To Recognize:
1. A group of operations where composition reverses the subscript product
2. Operations defined by right-multiplication of labels: LABEL(T_i(s)) = LABEL(s) * i

# Context & Application
The anti-isomorphism explains why transposition composition seems to "reverse" interval order. In pitch-class theory with additive notation, this is invisible: T_5 T_3 = T_{3+5} = T_8 = T_{5+3}. In non-commutative groups, the reversal is significant: T_{(i,p)} T_{(j,q)} = T_{(j,q)(i,p)}, not T_{(i,p)(j,q)}.

# Examples
**Example 1** (p. 78): Proof that T_i T_j = T_{ji}: int(s, T_i(T_j(s))) = int(s, T_j(s)) * int(T_j(s), T_i(T_j(s))) = j * i = ji. So T_i T_j maps each s to the element lying interval ji from s, hence T_i T_j = T_{ji}.

**Example 2**: Pitch-class transpositions: T_5 T_3 = T_8 = T_3 T_5 (commutative). Inverses: (T_5)^{-1} = T_7 = T_{-5 mod 12}.

**Example 3**: Time-span transpositions: T_{(2,3)} T_{(4,5)} = T_{(4,5)(2,3)} = T_{(4+5*2, 5*3)} = T_{(14, 15)}.

# Relationships
## Builds Upon
- **Transposition operation** — The individual maps whose group structure is established
## Enables
- **PETEY group** — TNSPS is one of the two subgroups generating PETEY
- **Anti-isomorphism** — The key structural result about TNSPS
## Contrasts With
- **Group of interval-preserving operations (PSVS)** — Isomorphic to IVLS (order preserved), while TNSPS is anti-isomorphic (order reversed)

# Common Errors
- **Error**: Computing T_i T_j as T_{ij} instead of T_{ji}
  **Correction**: Transposition composition reverses subscript order; use T_i T_j = T_{ji}

# Common Confusions
- **Confusion**: In commutative settings, the anti-isomorphism is invisible, leading students to assume T_i T_j = T_{ij}
  **Clarification**: The formula is always T_i T_j = T_{ji}; in commutative groups this happens to equal T_{ij}
- **Confusion**: TNSPS and PSVS have the same relationship to IVLS
  **Clarification**: TNSPS is anti-isomorphic to IVLS while PSVS is isomorphic; the difference is right- vs. left-multiplication of labels

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.2, pages 77-79.

# Verification Notes
- Definition source: Direct from Theorem 3.4.2
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: proof idea, time-span examples, commutative/non-commutative distinction
