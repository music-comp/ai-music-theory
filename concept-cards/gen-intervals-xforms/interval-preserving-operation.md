---
concept: Interval-Preserving Operation
slug: interval-preserving-operation

category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.4 Transpositions and Interval-Preserving Operations"

extraction_confidence: high

aliases:
  - "Pi"
  - "P_i"
  - "interval-preserving transformation"

prerequisites:
  - generalized-interval-system
  - label-function
  - reference-point
extends:
  - generalized-interval-system
related:
  - transposition-operation
  - group-of-interval-preserving-operations
  - inversion-operation
contrasts_with:
  - transposition-operation

answers_questions:
  - "What is an interval-preserving operation in a GIS?"
  - "How do commutative and non-commutative GIS structures differ?"
  - "What distinguishes the LABEL function from the int function?"
---

# Quick Definition
An interval-preserving operation is a transformation on a GIS space that maintains all intervallic relationships: int(X(s), X(t)) = int(s, t) for all s and t.

# Core Definition
A transformation X on the space S of a GIS is interval-preserving if int(X(s), X(t)) = int(s, t) for all s and t (Definition 3.4.6). The interval-preserving transformations are precisely the operations P_i defined by LABEL(P_i(s)) = i * LABEL(s), and they form a group isomorphic (not anti-isomorphic) to IVLS under f(i) = P_i, with P_i P_j = P_{ij} (Lewin, Definitions 3.4.4, 3.4.6 and Theorems 3.4.5, 3.4.7, pp. 79-81).

# Prerequisites
- **Generalized Interval System** — Interval-preserving operations are defined within a GIS
- **LABEL Function** — P_i is defined via left-multiplication of labels
- **Reference Point** — The specific P_i depends on ref, but the family does not (Theorem 3.4.7)

# Key Properties
1. int(P_i(s), P_i(t)) = int(s, t) for all s, t (interval preservation)
2. LABEL(P_i(s)) = i * LABEL(s) (left-multiplication of labels)
3. Isomorphism: P_i P_j = P_{ij} (composition preserves interval order)
4. The family of interval-preserving operations is independent of the choice of ref
5. Every transposition commutes with every interval-preserving operation (Theorem 3.4.10)
6. T_i = P_i iff i is central in IVLS (Theorem 3.4.8)

# Construction / Recognition
## To Construct:
1. Fix ref in S, choose interval i
2. For each s, set P_i(s) = the unique element with LABEL = i * LABEL(s)
3. Equivalently: int(ref, P_i(s)) = i * int(ref, s)

## To Recognize:
1. Verify int(X(s), X(t)) = int(s, t) for all s, t
2. The transformation preserves all intervallic distances

# Context & Application
Interval-preserving operations generalize isometries. Unlike transpositions (which right-multiply labels), interval-preserving operations left-multiply labels. In commutative GIS, these coincide; in non-commutative GIS, they are fundamentally different families. This distinction is central to understanding non-commutative GIS structures.

# Examples
**Example 1** (p. 80): In the commutative 12-tone pitch-class GIS, T_i = P_i for all i, so transpositions and interval-preserving operations are the same.

**Example 2**: In the non-commutative time-span GIS:
- P_{(h,u)}(a, x) = (h + ua, ux) — first scales by u, then shifts by h
- T_{(i,p)}(a, x) = (a + ix, px) — different operation
- These are distinct for all (h,u) and (i,p) except (0,1)

# Relationships
## Builds Upon
- **Generalized Interval System** — interval preservation is defined in terms of the int function
- **LABEL Function** — P_i defined via LABEL left-multiplication

## Enables
- **Group of Interval-Preserving Operations** — these form a group isomorphic to IVLS
- **PETEY Group** — one of the two generating families

## Related
- **Inversion Operation** — inversions combine with P via Theorem 3.5.7

## Contrasts With
- **Transposition Operation** — T_i right-multiplies labels (anti-isomorphism); P_i left-multiplies (isomorphism)

# Common Errors
- **Error**: Assuming P_i is independent of ref
  **Correction**: The specific operation labeled "P_i" depends on ref. However, the family of all interval-preserving operations is the same for any ref (Theorem 3.4.7).

- **Error**: Assuming P_i P_j = P_{ji}
  **Correction**: The composition is P_i P_j = P_{ij} (isomorphism, preserving order), unlike transpositions which reverse order.

# Common Confusions
- **Confusion**: Conflating interval-preserving with transposition
  **Clarification**: In commutative GIS they are identical. In non-commutative GIS, they are distinct: P_i preserves intervals by definition; T_i generally does not.

- **Confusion**: Thinking interval-preserving operations must fix some element
  **Clarification**: P_i maps ref to the element with LABEL = i * e = i. Generally P_i moves all elements (unless i = e).

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.4.4, 3.4.6 and Theorems 3.4.5, 3.4.7, 3.4.8, pp. 79-82.

# Verification Notes
- Definition source: direct from Definitions 3.4.4, 3.4.6
- Confidence rationale: high — explicit definitions with proofs
- Re-extraction notes: Re-extracted from v2 card; preserved: time-span example, distinction from transposition, note about ref-independence of the family
