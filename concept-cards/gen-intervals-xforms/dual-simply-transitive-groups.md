---
# === CORE IDENTIFICATION ===
concept: Dual Simply Transitive Groups
slug: dual-simply-transitive-groups

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: non-commutative-gis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: null
pdf_page: 282
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "STRANS and STRANS'"
  - "commutant group"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - simply-transitive-group
  - gis-from-simply-transitive-group
extends:
  - simply-transitive-group
related:
  - strans1-group
  - strans2-group
  - gis1-octatonic
  - gis2-octatonic
  - interval-preserving-operations-octatonic
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are dual simply transitive groups?"
  - "How does a GIS relate to a simply transitive group?"
  - "What distinguishes transposition from interval-preserving operations in a non-commutative GIS?"
---

# Quick Definition
Given a simply transitive group STRANS on a set S, the dual group STRANS' consists of all operations on S commuting with every member of STRANS. STRANS' is itself simply transitive, and the duality is symmetric: STRANS consists of all operations commuting with every member of STRANS'.

# Core Definition
"Consider the family STRANS' of transformations f on S such that f commutes with every member of the given group STRANS. It can be proved that STRANS' is itself a simply transitive group of operations on S, and that every transformation A which commutes with every member of STRANS' is (already) a member of the given group STRANS" (Lewin, Appendix B, p. 252). In a GIS with STRANS as transpositions, STRANS' members are the interval-preserving operations. Dually, in a GIS with STRANS' as transpositions, STRANS members are interval-preserving. "If STRANS is commutative, then STRANS' will be precisely STRANS itself" (p. 252).

# Prerequisites
- **Simply transitive group** — Both STRANS and STRANS' must be simply transitive
- **GIS from simply transitive group** — The construction that generates GIS from each group

# Key Properties
1. STRANS' = {f : S -> S | fg = gf for all g in STRANS}
2. STRANS' is simply transitive on S
3. STRANS = {f : S -> S | fg = gf for all g in STRANS'} (symmetric duality)
4. GIS with STRANS transpositions: STRANS' = interval-preserving operations
5. GIS with STRANS' transpositions: STRANS = interval-preserving operations
6. If STRANS is commutative (abelian): STRANS' = STRANS (trivial duality)
7. Non-commutative case: STRANS' is genuinely different from STRANS

# Construction / Recognition
## To Construct:
1. Given STRANS, find all operations commuting with every member
2. These form STRANS'
3. Verify STRANS' is simply transitive
## To Recognize:
1. Two groups on the same set where each consists of all operations commuting with the other
2. Roles of transposition and interval-preservation swap between the two associated GIS structures

# Context & Application
The duality theorem explains why, in the familiar chromatic GIS (commutative Z_12 transpositions), the transpositions and interval-preserving operations are the same group. In non-commutative contexts (like the octatonic collection), the two groups differ, yielding genuinely different GIS structures on the same set.

# Examples
**Example 1** (pp. 251-252): STRANS1 and STRANS2 on the octatonic set: STRANS2 = all ops commuting with everything in STRANS1; STRANS1 = all ops commuting with everything in STRANS2.

**Example 2** (p. 252): In the commutative case (standard Z_12 pitch-class GIS), STRANS' = STRANS: transpositions are their own interval-preserving operations.

# Relationships
## Builds Upon
- **Simply transitive group** — Both groups must be simply transitive
- **GIS from simply transitive group** — Each group generates a GIS
## Enables
- **GIS1 octatonic** — STRANS1 transpositions, STRANS2 interval-preserving
- **GIS2 octatonic** — STRANS2 transpositions, STRANS1 interval-preserving
## Related
- **STRANS1 group** — One side of the octatonic duality
- **STRANS2 group** — Other side of the octatonic duality
- **Interval-preserving operations (octatonic)** — The dual group provides these

# Common Errors
- **Error**: Assuming the dual group is always different from the original
  **Correction**: In commutative groups, STRANS' = STRANS (the duality is trivial)

# Common Confusions
- **Confusion**: Thinking one group is more "correct" than its dual
  **Clarification**: The duality is symmetric; neither group has priority
- **Confusion**: Confusing "dual" in this sense with other mathematical uses of "dual"
  **Clarification**: Here "dual" means the commutant group, defined by mutual commutativity

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 252.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly stated theorem with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: commutative special case, symmetric duality, INJ preservation property
