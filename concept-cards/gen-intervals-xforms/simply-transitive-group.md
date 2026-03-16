---
concept: Simply Transitive Group
slug: simply-transitive-group

category: transformation-theory
subcategory: simply-transitive-groups
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
section: "7.1.1"

extraction_confidence: high

aliases:
  - STRANS
  - "simply transitive action"
  - "regular group action"

prerequisites:
  - group
  - operation
  - musical-space-s
extends: []
related:
  - gis-from-simply-transitive-group
  - intervals-as-transpositions
  - group-of-transpositions
contrasts_with:
  - non-intervallic-transformations

answers_questions:
  - "What is a simply transitive group?"
  - "How does simple transitivity relate to GIS structure?"
  - "Why must a GIS transposition group be simply transitive?"
---

# Quick Definition
A group of operations STRANS on a set S is simply transitive when for any two elements s and t in S, there exists exactly one operation in STRANS that maps s to t.

# Core Definition
"The group STRANS of operations on S is simply transitive when the following condition is satisfied: Given any elements s and t of S, then there exists a unique member OP of STRANS such that OP(s) = t" (Lewin, 7.1.1, p. 157). This property combines transitivity (some operation always exists) with simplicity (that operation is unique). The group of transpositions in any GIS is simply transitive on the space of that GIS.

# Prerequisites
- **Group** — Simple transitivity is a property of a group acting on a set
- **Operation** — The group elements must be operations (functions) on the space
- **Musical space (S)** — The set on which the group acts

# Key Properties
1. For all s, t in S: there exists a unique OP in STRANS such that OP(s) = t
2. |STRANS| = |S| (the group and set have the same cardinality)
3. The action is both free (no non-identity element fixes any point) and transitive (any element can be reached from any other)
4. The group of transpositions in any GIS is simply transitive on the GIS space
5. Conversely, any simply transitive group on a set gives rise to a GIS structure (Theorem 7.1.1)

# Construction / Recognition
## To Construct:
1. Start with a set S and a group of operations on S
2. Verify transitivity: for any s, t in S, some operation maps s to t
3. Verify uniqueness: no two distinct operations map the same s to the same t
## To Recognize:
1. Given a group action, check that no non-identity element has a fixed point
2. Check that the group and set have the same cardinality
3. Verify any element can be mapped to any other element

# Context & Application
Simply transitive groups provide the formal bridge between interval-based thinking and transformation-based thinking. Lewin shows in Theorem 7.1.1 that the entire notion of a GIS can be developed from a family S and a simply transitive group STRANS on S. This equivalence means that "all the work we have done with GIS structures since chapter 2 can be regarded as a special branch of transformational theory" (7.1.2, p. 158).

# Examples
**Example 1** (p. 157): In a standard pitch-class GIS, given any two pitch classes s and t, there is exactly one transposition T_i that maps s to t. The twelve transposition operations form a simply transitive group on the twelve pitch classes.

**Example 2** (Appendix B, p. 251): STRANS1, the group of eight operations {RO, R3, R6, R9, K, L, M, N} on the octatonic collection, is simply transitive: "Given members s and t of S, there is a unique OP among the eight cited operations on S, satisfying OP(s) = t."

# Relationships
## Builds Upon
- **Group** — Simple transitivity is a special property of group actions
## Enables
- **GIS from simply transitive group** — Any simply transitive group generates a GIS (Theorem 7.1.1)
- **Intervals as transpositions** — The equivalence between intervals and transpositions depends on simple transitivity
## Related
- **Group of transpositions** — The transposition group of any GIS is simply transitive
- **Dual simply transitive groups** — Non-commutative simply transitive groups have dual groups
## Contrasts With
- **Non-intervallic transformations** — Groups containing both MED powers and PAR cannot be simply transitive

# Common Errors
- **Error**: Assuming every transitive group action is simply transitive
  **Correction**: Simple transitivity requires both existence AND uniqueness; a transitive action may have multiple operations mapping s to t
- **Error**: Confusing "simply transitive" with "simple group"
  **Correction**: "Simply transitive" describes a group action, not the algebraic structure of the group itself

# Common Confusions
- **Confusion**: Believing that only commutative groups can be simply transitive
  **Clarification**: Non-commutative groups can also be simply transitive (e.g., STRANS1 on the octatonic collection)
- **Confusion**: Thinking the full group of pitch-class operations (including inversions) is simply transitive
  **Clarification**: The 24-element group of T and I operations on 12 pitch classes is not simply transitive (two operations map C to C#: T_1 and some I)

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.1.1, pages 157-158.

# Verification Notes
- Definition source: Direct quotation from 7.1.1
- Confidence rationale: Explicit formal definition with proof in source
- Re-extraction notes: Re-extracted from v2 card; preserved: core definition, examples, common confusions about simple vs. simply transitive
