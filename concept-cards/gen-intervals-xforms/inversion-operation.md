---
concept: Inversion Operation
slug: inversion-operation

category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.5 Inversions"

extraction_confidence: high

aliases:
  - "I_u^v"
  - "u/v inversion"
  - "inversion about u and v"

prerequisites:
  - generalized-interval-system
  - label-function
  - transposition-operation
  - interval-preserving-operation
extends:
  - generalized-interval-system
related:
  - interval-reversing-operation
  - petinv-group
  - commutative-gis-inversion-properties
contrasts_with:
  - transposition-operation
  - interval-preserving-operation

answers_questions:
  - "What is an inversion operation in a GIS?"
  - "How do commutative and non-commutative GIS structures differ?"
---

# Quick Definition
The u/v inversion I_u^v maps each element s to an element "balanced" about u and v: the interval from v to I_u^v(s) equals the interval from s to u.

# Core Definition
Given any u, v in S, the operation I_u^v is defined by int(v, I_u^v(s)) = int(s, u) for all s (Definition 3.5.1). Using LABEL with i = LABEL(v), j = LABEL(u): LABEL(I_u^v(s)) = i * LABEL(s)^{-1} * j (Theorem 3.5.2). The operation I_u^v maps u to v and v to u; its inverse is I_v^u (Corollary 3.5.9). In commutative GIS, I_u^v = I_v^u always; in non-commutative GIS this holds only when int(v, u) is central (Lewin, pp. 82-89).

# Prerequisites
- **Generalized Interval System** — Inversion is defined within a GIS
- **LABEL Function** — The LABEL formula for inversion is essential for computation
- **Transposition Operation** — Understanding how inversions combine with transpositions (Theorem 3.5.6)
- **Interval-Preserving Operation** — Understanding how inversions combine with P operations (Theorem 3.5.7)

# Key Properties
1. int(v, I_u^v(s)) = int(s, u) for all s
2. LABEL(I_u^v(s)) = i * LABEL(s)^{-1} * j where i = LABEL(v), j = LABEL(u)
3. I_u^v(u) = v and I_u^v(v) = u
4. (I_u^v)^{-1} = I_v^u (Corollary 3.5.9)
5. T_n I_u^v = I_x^v where x = T_n(u) (Theorem 3.5.6A)
6. I_u^v T_n = I_u^w where w = T_n^{-1}(v) (Theorem 3.5.6B)
7. I_u^v I_x^w = P_{im^{-1}} T_{k^{-1}j} (Theorem 3.5.8)
8. In commutative GIS: I = I^{-1}, IT = T^{-1}I (Corollary 3.5.10)

# Construction / Recognition
## To Construct:
1. Choose u, v in S
2. For each s, compute i = int(s, u)
3. Find the unique t with int(v, t) = i
4. Set I_u^v(s) = t

## To Recognize:
1. The transformation maps u to v and v to u
2. For any s, int(v, image) = int(s, u)
3. The LABEL formula has the pattern i * x^{-1} * j

# Context & Application
Inversion generalizes the familiar pitch-class operation. For pitch classes, I_C^C maps each note to its "mirror image" across C. The general definition allows inversion about two different points u and v. In commutative GIS, many pairs (u, v) yield the same operation; in non-commutative GIS, I_u^v = I_{u'}^{v'} only when u' = u and v' = v (Note 4.1.7H).

# Examples
**Example 1** (p. 83): In 12-tone pitch-class GIS:
- I_C^C maps E to A-flat, G to F
- I_C^C = I_{F#}^{F#} = I_D^{Bb} (multiple equivalent representations)

**Example 2** (p. 84): Determining when I_u^v = I_x^w:
- Commutative GIS: iff w = I_u^v(x)
- Non-commutative GIS: iff w = I_u^v(x) AND int(x, u) is central (Theorem 3.5.3)

# Relationships
## Builds Upon
- **Transposition Operation** — inversions combine with transpositions (Theorem 3.5.6)
- **Interval-Preserving Operation** — inversions combine with P operations (Theorem 3.5.7)

## Enables
- **PETINV Group** — inversions plus PETEY form the full canonical group
- **Interval-Reversing Operation** — in commutative GIS, inversions are precisely the interval-reversing operations

## Related
- **Commutative GIS Inversion Properties** — special simplifications in the commutative case

## Contrasts With
- **Transposition Operation** — transposition right-multiplies labels; inversion inverts-and-sandwiches labels
- **Interval-Preserving Operation** — preserves all intervals; inversion reverses them (in commutative case)

# Common Errors
- **Error**: Assuming I_u^v = I_v^u in all GIS
  **Correction**: This holds only in commutative GIS or when int(v, u) is central (Corollary 3.5.4)

- **Error**: Confusing int(v, I(s)) = int(s, u) with int(I(s), v) = int(u, s)
  **Correction**: The defining formula places v and s on opposite sides. The interval is FROM v TO the image, and FROM the original TO u.

# Common Confusions
- **Confusion**: Thinking inversion must be about a "center" (single point)
  **Clarification**: GIS inversion involves two points u and v. In the familiar pitch-class case, C/C inversion and F#/F# inversion happen to be the same operation, but the formalism allows u and v to differ.

- **Confusion**: Assuming inversion is always an involution (I^2 = identity)
  **Clarification**: In commutative GIS, I_u^v is indeed self-inverse. In non-commutative GIS, (I_u^v)^{-1} = I_v^u which may differ from I_u^v.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definition 3.5.1 through Corollary 3.5.10, pp. 82-89.

# Verification Notes
- Definition source: direct from Definition 3.5.1
- Confidence rationale: high — explicit definition with extensive theorems
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, equivalence conditions, visualization reference (Figure 3.7)
