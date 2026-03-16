---
concept: GIS from Simply Transitive Group
slug: gis-from-simply-transitive-group

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
  - "Theorem 7.1.1"
  - "simply transitive group theorem"

prerequisites:
  - simply-transitive-group
  - generalized-interval-system
  - anti-isomorphism
extends:
  - generalized-interval-system
related:
  - intervals-as-transpositions
  - group-of-transpositions
contrasts_with: []

answers_questions:
  - "How does a GIS relate to a simply transitive group?"
  - "Can any simply transitive group generate a GIS?"
  - "What is the formal equivalence between GIS structure and simply transitive groups?"
---

# Quick Definition
Any simply transitive group of operations on a set can be used to construct a GIS, where the group operations become the formal transpositions and the interval function is defined by which operation maps one element to another.

# Core Definition
Lewin proves in Theorem 7.1.1 that given a family S and a simply transitive group STRANS on S, there exists a GIS having S for its space and STRANS for its group of transpositions. The construction proceeds by: (1) creating an index family IVLS in 1-to-1 correspondence with STRANS, (2) defining a group structure on IVLS via ij = k when (OP_i)(OP_j) = OP_k, making IVLS anti-isomorphic to STRANS, and (3) defining int(r, s) as the unique i such that OP_i(r) = s. The resulting (S, IVLS, int) satisfies both GIS conditions (Lewin, 7.1.1, pp. 157-158).

# Prerequisites
- **Simply transitive group** — The theorem's hypothesis requires this property
- **Generalized interval system** — Must understand GIS conditions A and B to verify the construction
- **Anti-isomorphism** — IVLS is anti-isomorphic (not isomorphic) to STRANS due to composition order

# Key Properties
1. IVLS is anti-isomorphic to STRANS as a group
2. int(r, s) = the unique i in IVLS such that OP_i(r) = s
3. Condition A (int(r,t) = int(r,s)int(s,t)) follows from the group structure of IVLS
4. Condition B (given s and i, unique t with int(s,t) = i) follows from simple transitivity
5. T_i = OP_i for every i in IVLS (transpositions are the group operations)

# Construction / Recognition
## To Construct:
1. Start with set S and simply transitive group STRANS on S
2. Create index family IVLS in bijection with STRANS: i corresponds to OP_i
3. Define group operation on IVLS: ij = k when (OP_i)(OP_j) = OP_k
4. Define int(r, s) = i where OP_i(r) = s (unique by simple transitivity)
5. Verify (S, IVLS, int) is a GIS
## To Recognize:
1. Identify the space S and the simply transitive group
2. Check that every pair (s, t) determines a unique group element
3. Verify the interval function satisfies both GIS conditions

# Context & Application
This theorem establishes the formal equivalence between GIS structure and simply transitive group structure. Lewin notes that "all the work we have done with GIS structures since chapter 2 can be regarded as a special branch of transformational theory, namely that branch in which we study a space S and a simply transitive group STRANS of operations on S" (7.1.2, p. 158). From a strictly mathematical viewpoint, GIS structure could have been deferred until after a general exploration of transformations.

# Examples
**Example 1** (p. 157): The twelve pitch-class transpositions form a simply transitive group on pitch classes. This generates the familiar GIS where IVLS = Z_12, int(C, G) = 7 corresponds to T_7, and T_i = OP_i for all i.

**Example 2** (Appendix B): STRANS1 on the octatonic collection generates GIS1 = (S, IVLS1, int1), where applying any of the eight operations to a member of S "amounts formally precisely to 'transposing' the given s by a suitable corresponding interval of IVLS1."

# Relationships
## Builds Upon
- **Generalized interval system** — The theorem constructs a GIS from a simply transitive group
- **Simply transitive group** — The required input for the construction
## Enables
- **Intervals as transpositions** — The equivalence makes intervals and transpositions interchangeable
- **GIS1 octatonic** — Constructed via this theorem from STRANS1
- **GIS2 octatonic** — Constructed via this theorem from STRANS2
## Related
- **Anti-isomorphism** — IVLS is anti-isomorphic to STRANS
## Contrasts With
- (none)

# Common Errors
- **Error**: Assuming IVLS is isomorphic to STRANS
  **Correction**: IVLS is anti-isomorphic to STRANS because of how composition order works in right vs. left orthography
- **Error**: Attempting to construct a GIS from a non-simply-transitive group
  **Correction**: The theorem requires simple transitivity; without it, int(r,s) may not be well-defined

# Common Confusions
- **Confusion**: Thinking this construction only works for commutative groups
  **Clarification**: The theorem applies to any simply transitive group, commutative or not (the octatonic examples are non-commutative)
- **Confusion**: Believing the GIS constructed is unique
  **Clarification**: The construction depends on the choice of STRANS; different simply transitive groups on the same set yield different GIS structures (as with GIS1 and GIS2 on the octatonic set)

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.1.1, pages 157-158. Full proof of Conditions A and B.

# Verification Notes
- Definition source: Direct from theorem statement and proof in 7.1.1
- Confidence rationale: Explicit theorem with complete proof in source
- Re-extraction notes: Re-extracted from v2 card; preserved: core construction steps, anti-isomorphism point
