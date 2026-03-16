---
concept: Simply Transitive Action
slug: simply-transitive-action

category: generalized-interval-systems
subcategory: group-actions
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Definition 2.3.1(B), Section 7.1.1"

extraction_confidence: high

aliases:
  - "simply transitive group action"
  - "STRANS"

prerequisites:
  - group
  - generalized-interval-system
extends:
  - group-action
related:
  - interval-group-ivls
  - musical-space-s
  - transposition
contrasts_with: []

answers_questions:
  - "What does it mean for a group to act simply transitively on a space?"
  - "How does simple transitivity relate to GIS Condition (B)?"
  - "How does simple transitivity connect GIS theory to transformation theory?"
---

# Quick Definition
A simply transitive group action means that for any two elements s and t in the space, there is exactly one group element (operation) that maps s to t. In a GIS, Condition (B) establishes that IVLS acts simply transitively on S.

# Core Definition
Condition (B) of the GIS definition states: "For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s" (Lewin, Definition 2.3.1, p. 47). In Chapter 7, Lewin makes the connection explicit: "The group STRANS of operations on S is simply transitive when the following condition is satisfied: Given any elements s and t of S, then there exists a unique member OP of STRANS such that OP(s) = t" (Section 7.1.1, p. 188).

# Prerequisites
- **Group** — The acting group must satisfy the group axioms
- **Generalized Interval System** — Simple transitivity is implicit in Condition (B)

# Key Properties
1. Simply transitive = free + transitive
2. Transitive: for any s, t in S, some group element maps s to t (every point is reachable)
3. Free (simple): if OP(s) = OP'(s) for some s, then OP = OP' (the element is unique)
4. Equivalently: for any fixed s, the map OP -> OP(s) is a bijection from the group to S
5. |S| = |IVLS| (same cardinality)
6. The group of transpositions is simply transitive on the space of any GIS

# Construction / Recognition
## To Construct:
1. Given a simply transitive group STRANS on S, define IVLS as an index family anti-isomorphic to STRANS
2. Define int(r, s) = i where OP_i is the unique operation with OP_i(r) = s
3. Verify Conditions (A) and (B) hold -- they follow from the construction
## To Recognize:
1. For every pair (s, t) in S, exactly one group element maps s to t
2. No group element other than the identity fixes any point

# Context & Application
Simple transitivity is the bridge between GIS structure and transformation theory. Lewin proves in Section 7.1.1 that "all the work we have done with GIS structures since chapter 2 can be regarded as a special branch of transformational theory, namely that branch in which we study a space S and a simply transitive group STRANS of operations on S." The GIS concept and the simply-transitive-group concept are mathematically equivalent: either can be constructed from the other.

# Examples
**Example 1**: Pitch-class space (Z_12) -- Fix reference C = 0. Every pitch class is reached by exactly one transposition: G = T_7(C), A = T_9(C), etc. The map i -> T_i(C) is a bijection from Z_12 to the 12 pitch classes.

**Example 2**: Chromatic pitch space (Z) -- Fix reference C4. Every pitch is reached by exactly one integer interval. The map i -> (C4 + i semitones) is a bijection from Z to chromatic pitches.

**Example 3** (Section 7.1.1, p. 188): Lewin proves the converse: "Let S be a family of objects and let STRANS be a simply transitive group of operations on S; then there exists a GIS having S for its space and STRANS for its group of transpositions."

# Relationships
## Builds Upon
- **Group** — The acting group provides the algebraic structure
## Enables
- **Generalized Interval System** — Simple transitivity of IVLS on S is equivalent to Condition (B)
- **Transformation theory** — GIS structure is subsumed into transformation theory via this equivalence
## Related
- **Transposition** — The transposition group is the canonical simply transitive group in a GIS
- **Interval group IVLS** — Acts simply transitively on S; anti-isomorphic to the transposition group

# Common Errors
- **Error**: Thinking simple transitivity is an additional requirement beyond Condition (B)
  **Correction**: Simple transitivity is exactly what Condition (B) establishes; they are equivalent formulations

# Common Confusions
- **Confusion**: Confusing "simply transitive" with "doubly transitive" or other group action types
  **Clarification**: "Simply" means each element of the group that acts on a point is unique (the action is free); "transitive" means every point is reachable. Together they give a perfect bijection between group elements and space points (once a reference is fixed).

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(B); Chapter 7: Transformation Graphs and Networks (1), Section 7.1.1, pages 47, 188-189.

# Verification Notes
- Definition source: Direct quotation from Definition 2.3.1(B) and Section 7.1.1
- Confidence rationale: Central concept connecting GIS theory to transformation theory
- Re-extraction notes: Re-extracted from v2 card; preserved: equivalence with Condition (B), Chapter 7 bridge to transformation theory
