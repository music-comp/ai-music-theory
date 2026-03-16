---
concept: Inversion Properties in Commutative GIS
slug: commutative-gis-inversion-properties

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
  - "commutative inversion simplifications"

prerequisites:
  - inversion-operation
  - transposition-operation
  - commutative-vs-noncommutative-gis
extends:
  - inversion-operation
related:
  - interval-reversing-operation
contrasts_with: []

answers_questions:
  - "What special properties do inversions have in commutative GIS?"
  - "How do commutative and non-commutative GIS structures differ?"
---

# Quick Definition
In a commutative GIS, inversions have special simplifying properties: every inversion is self-inverse (I^{-1} = I), inversion conjugates transposition to its inverse (IT = T^{-1}I), and I_u^v always equals I_v^u.

# Core Definition
Corollary 3.5.10 states: In a commutative GIS, for any transposition T and any inversion I: (A) I^{-1} = I (every inversion is an involution), and (B) IT = T^{-1}I (inversion conjugates transposition to its inverse). These follow from I_u^v = I_v^u (Corollary 3.5.5) and (I_u^v)^{-1} = I_v^u (Corollary 3.5.9) in the commutative case (Lewin, Corollary 3.5.10, p. 89).

# Prerequisites
- **Inversion Operation** — The operation whose properties are simplified
- **Transposition Operation** — Interacts with inversion via IT = T^{-1}I
- **Commutative vs. Non-Commutative GIS** — These properties hold only in the commutative case

# Key Properties
1. I_u^v = I_v^u for all u, v (Corollary 3.5.5)
2. I^{-1} = I (every inversion is self-inverse/involutory)
3. IT = T^{-1}I (inversion conjugates transposition to its inverse)
4. Equivalently: TIT = I, or TI = IT^{-1}
5. I_u^v = I_x^w iff w = I_u^v(x) (simplified from non-commutative condition)

# Construction / Recognition
## To Recognize:
1. Verify that IVLS is commutative
2. Then all five properties above automatically hold

# Context & Application
These properties are familiar from standard pitch-class theory. Property (B) explains why IT_n = T_{-n}I in pitch-class analysis: the transposition index negates when passing through an inversion. These properties fail in non-commutative GIS (like the time-span GIS), where inversions are not self-inverse and IT is not T^{-1}I.

# Examples
**Example 1** (p. 89): In 12-tone pitch-class GIS:
- I_0(x) = -x (mod 12) is self-inverse: I_0(I_0(x)) = -(-x) = x
- I_0 T_5 = T_7 I_0 = T_{-5} I_0

**Example 2**: Row operations in twelve-tone theory:
- Applying I twice returns the original row
- The commutator relation TIT = I explains standard row-class arithmetic

# Relationships
## Builds Upon
- **Inversion Operation** — these are special properties of inversions in the commutative case
- **Transposition Operation** — the IT = T^{-1}I formula involves transpositions

## Enables
- Standard pitch-class arithmetic and row operations

## Related
- **Interval-Reversing Operation** — in commutative GIS, inversions are also interval-reversing

# Common Errors
- **Error**: Applying IT = T^{-1}I in a non-commutative GIS
  **Correction**: This formula is specific to commutative GIS. In non-commutative GIS, use Theorem 3.5.6 directly.

# Common Confusions
- **Confusion**: Thinking I^{-1} = I means I is the identity
  **Clarification**: I^{-1} = I means I * I = identity (involution), not I = identity. Inversions are non-trivial self-inverse operations.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Corollary 3.5.10, p. 89.

# Verification Notes
- Definition source: direct from Corollary 3.5.10
- Confidence rationale: high — explicit corollary with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, row-operation application, involution clarification
