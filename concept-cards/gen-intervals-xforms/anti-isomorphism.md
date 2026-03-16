---
concept: Anti-Isomorphism
slug: anti-isomorphism

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
  - "anti-homomorphism"
  - "order-reversing isomorphism"

prerequisites:
  - group-of-transpositions
  - transposition-operation
extends: []
related:
  - group-of-interval-preserving-operations
  - left-vs-right-group-operations
contrasts_with:
  - group-of-interval-preserving-operations

answers_questions:
  - "Why does composing two transpositions reverse the order of intervals?"
  - "What is the algebraic relationship between IVLS and TNSPS?"
  - "How does anti-isomorphism differ from isomorphism?"
---

# Quick Definition
An anti-isomorphism is a bijective map between groups that reverses the order of multiplication: f(ab) = f(b)f(a). The map from intervals to transpositions, f(i) = T_i, is an anti-isomorphism from IVLS onto TNSPS.

# Core Definition
A function f: G -> H between groups is an **anti-isomorphism** if (1) f is bijective (one-to-one and onto), and (2) f(ab) = f(b)f(a) for all a, b in G. Theorem 3.4.2 establishes that the map f(i) = T_i from IVLS to the group of transpositions TNSPS is an anti-isomorphism, yielding the composition formula T_i T_j = T_{ji} (not T_{ij}).

# Prerequisites
- **Transposition operation (T_i)** — The operations whose group structure is characterized by the anti-isomorphism
- **Group of transpositions (TNSPS)** — The codomain of the anti-isomorphism

# Key Properties
1. The composition formula T_i T_j = T_{ji} reverses the subscript order
2. In commutative groups, anti-isomorphism coincides with isomorphism since ab = ba
3. The anti-isomorphism contrasts with the isomorphism f(i) = P_i from IVLS to PSVS, where P_i P_j = P_{ij}
4. The reversal arises because int(s, T_i(T_j(s))) = j * i = ji

# Construction / Recognition
## To Construct:
1. Define the map f: IVLS -> TNSPS by f(i) = T_i
2. Verify bijectivity: T_i = T_j implies i = j, and every transposition is some T_i
3. Verify the reversal property: f(ij) = T_{ij}, but f(i)f(j) = T_i T_j = T_{ji}
## To Recognize:
1. A group homomorphism that reverses multiplication order
2. The composition of transpositions yields a subscript product in reversed order

# Context & Application
The anti-isomorphism explains why care is needed when composing transpositions: "transpose by i then transpose by j" yields T_{ji}, not T_{ij}. In commutative groups (like pitch-class intervals mod 12), the distinction vanishes since ij = ji. In non-commutative groups (like the time-span interval group), the reversal is musically and computationally significant.

# Examples
**Example 1** (p. 78): In the commutative pitch-class GIS, T_5 T_3 = T_{3+5} = T_8 = T_{5+3}, so the reversal is invisible.

**Example 2**: In the non-commutative time-span GIS, the reversal matters: T_{(i,p)} T_{(j,q)} = T_{(j,q)(i,p)} = T_{(j+qi, qp)}, which generally differs from T_{(i,p)(j,q)} = T_{(i+pj, pq)}.

**Contrast with P operations**: P_i P_j = P_{ij} (isomorphism, preserves order) vs. T_i T_j = T_{ji} (anti-isomorphism, reverses order).

# Relationships
## Builds Upon
- **Transposition operation** — The individual maps whose group structure is described
## Enables
- **Group of transpositions** — The anti-isomorphism establishes TNSPS as a group
- **PETEY group** — Understanding how T and P interact requires knowing their distinct relationships to IVLS
## Related
- **Group of interval-preserving operations** — PSVS is isomorphic (not anti-isomorphic) to IVLS
## Contrasts With
- **Group of interval-preserving operations** — Isomorphism (P_i P_j = P_{ij}) vs. anti-isomorphism (T_i T_j = T_{ji})

# Common Errors
- **Error**: Assuming T_i T_j = T_{ij}
  **Correction**: The correct formula is T_i T_j = T_{ji}; the subscript product is reversed

# Common Confusions
- **Confusion**: "Anti-" means the map is somehow defective or opposite
  **Clarification**: Anti-isomorphism is a perfectly valid structural correspondence; it simply reverses multiplication order
- **Confusion**: In commutative settings, anti-isomorphism and isomorphism seem identical, leading students to miss the distinction
  **Clarification**: The distinction becomes critical in non-commutative GIS structures like the time-span GIS

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.2, pages 77-79.

# Verification Notes
- Definition source: Direct from Theorem 3.4.2
- Confidence rationale: High -- theorem and proof are explicitly stated
- Re-extraction notes: Re-extracted from v2 card; preserved: contrast with P operations, time-span examples, compositional reading order discussion
