---
concept: Transposition and Interval Preservation
slug: transposition-and-interval-preservation

category: generalized-interval-systems
subcategory: group-structure
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 81
section: "3.4"

extraction_confidence: high

aliases: []

prerequisites:
  - transposition-operation
  - interval-preserving-operation
  - central-interval
extends: []
related:
  - group-of-transpositions
  - group-of-interval-preserving-operations
contrasts_with: []

answers_questions:
  - "When does a transposition preserve intervals?"
  - "Under what conditions does T_i equal P_i?"
  - "Why do transpositions in non-commutative GIS generally distort intervals?"
---

# Quick Definition
A transposition T_i preserves intervals if and only if the interval i is central (commutes with every interval in the group). In commutative GIS, all transpositions preserve intervals; in non-commutative GIS, typically only the identity does.

# Core Definition
**Theorem 3.4.8**: The following conditions are logically equivalent:
- (A): T_i preserves intervals (int(T_i(s), T_i(t)) = int(s, t) for all s, t)
- (B): For some choice of ref, T_i = P_i
- (C): For any choice of ref, T_i = P_i
- (D): i is central in IVLS (i commutes with every j in IVLS)

This establishes a fundamental dichotomy between commutative and non-commutative GIS.

# Prerequisites
- **Transposition operation (T_i)** — The operation whose interval-preserving property is characterized
- **Interval-preserving operation (P_i)** — The reference standard for interval preservation
- **Central interval** — The algebraic condition equivalent to interval preservation

# Key Properties
1. The four conditions (A)-(D) are equivalent
2. In commutative GIS: every T_i = P_i and every transposition preserves intervals
3. In non-commutative GIS: most T_i differ from P_i and distort intervals
4. The identity T_e always preserves intervals (since e is always central)
5. Theorem 3.4.7 provides the prerequisite: if T_i preserves intervals, then T_i = P_j for some j

# Construction / Recognition
## To Construct:
1. Determine whether i is central in IVLS
2. If yes, T_i preserves intervals and equals P_i for any ref
3. If no, T_i distorts intervals
## To Recognize:
1. A transposition that maps every pair of elements to a pair with the same interval
2. A transposition that coincides with some interval-preserving operation

# Context & Application
This theorem reveals a fundamental dichotomy. In commutative GIS (most familiar settings), every transposition preserves intervals -- this is the familiar situation where T_5 of an interval-class 3 dyad is still interval-class 3. In non-commutative GIS (like the time-span GIS of Chapter 4), most transpositions distort intervallic relationships, and transposition can even change the chronological ordering of events.

# Examples
**Example 1** (p. 81): In the commutative pitch-class GIS: every T_i = P_i. So T_5{C, E} = {F, A}, and int(C, E) = int(F, A) = 4.

**Example 2** (Corollary 3.4.9, p. 82): In a non-commutative GIS, there exists some i for which T_i does not preserve intervals, and hence T_i is not the same as P_i.

**Example 3** (Notes 4.1.7, p. 113): In the time-span GIS, only (0, 1) is central, so T_{(i,p)} does not preserve intervals for (i, p) different from (0, 1). Transposition can distort duration ratios and even chronological ordering.

# Relationships
## Builds Upon
- **Transposition operation** — The operations whose behavior is characterized
- **Interval-preserving operation** — The standard against which transpositions are measured
- **Central interval** — The equivalent algebraic condition
## Enables
- **Group of transpositions vs. group of interval-preserving operations** — Their relationship depends on this theorem
## Related
- **Non-commutative GIS** — Where the dichotomy becomes non-trivial

# Common Errors
- **Error**: Assuming all transpositions preserve intervals in any GIS
  **Correction**: Only T_i with central i preserves intervals; in non-commutative GIS this excludes most transpositions

# Common Confusions
- **Confusion**: If T_i does not preserve intervals, then the family of interval-preserving operations does not exist
  **Clarification**: The family PSVS of P_i operations always exists; it simply does not coincide with TNSPS in non-commutative GIS
- **Confusion**: The identity e is the only central element
  **Clarification**: In commutative groups all elements are central; the identity is merely the element that is always central regardless of group structure

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.8 and Corollary 3.4.9, pages 81-82.

# Verification Notes
- Definition source: Direct from Theorem 3.4.8
- Confidence rationale: High -- theorem with four equivalent conditions is explicitly stated and proved
- Re-extraction notes: Re-extracted from v2 card; preserved: four equivalent conditions, commutative/non-commutative dichotomy, time-span chronological distortion point
