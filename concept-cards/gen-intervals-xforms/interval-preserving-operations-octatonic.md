---
# === CORE IDENTIFICATION ===
concept: "Interval-Preserving Operations (Octatonic)"
slug: interval-preserving-operations-octatonic

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: octatonic-structures
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
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - gis1-octatonic
  - gis2-octatonic
  - dual-simply-transitive-groups
extends:
  - interval-preserving-operation
related:
  - strans1-group
  - strans2-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes transposition from interval-preserving operations in a non-commutative GIS?"
---

# Quick Definition
In GIS1, the interval-preserving operations are exactly the STRANS2 members; in GIS2, they are exactly the STRANS1 members. Each group of interval-preserving operations is precisely the dual simply transitive group of the corresponding transposition group.

# Core Definition
"As it turns out, the members of STRANS2 are exactly the interval-preserving operations for GIS1. Every member of STRANS2 commutes with every member of STRANS1. In fact, the members of STRANS2 are precisely those transformations on S that commute with every member of STRANS1" (Lewin, Appendix B, p. 252). Dually: "The interval-preserving operations for GIS2 are exactly the members of STRANS1; those are in fact precisely the transformations on S that commute with every member of STRANS2."

# Prerequisites
- **GIS1 octatonic** — One of the two GIS structures
- **GIS2 octatonic** — The other GIS structure
- **Dual simply transitive groups** — The theoretical foundation for the swap

# Key Properties
1. In GIS1: STRANS2 = interval-preserving, STRANS1 = transpositions
2. In GIS2: STRANS1 = interval-preserving, STRANS2 = transpositions
3. The roles swap symmetrically between the two GIS structures
4. Interval-preserving operations commute with all transpositions (of the same GIS)
5. In a commutative GIS, transpositions and interval-preserving operations would be the same group

# Construction / Recognition
## To Construct:
1. Given a GIS, identify the transposition group
2. Find all operations commuting with every transposition
3. These are the interval-preserving operations
## To Recognize:
1. Operations that preserve the interval function: int(f(s), f(t)) = int(s, t) for all s, t

# Context & Application
The octatonic case illustrates why transposition and interval-preservation are genuinely different in non-commutative GIS, unlike the familiar commutative case where they coincide. This distinction is fundamental to understanding non-commutative GIS structures.

# Examples
**Example 1** (p. 252): In GIS1, applying any STRANS2 member f to elements s, t preserves their GIS1-interval: int1(f(s), f(t)) = int1(s, t).

# Relationships
## Builds Upon
- **Dual simply transitive groups** — Provides the theoretical framework
## Related
- **STRANS1 group** — Interval-preserving in GIS2, transpositions in GIS1
- **STRANS2 group** — Interval-preserving in GIS1, transpositions in GIS2

# Common Errors
- **Error**: Assuming transpositions and interval-preserving operations are always the same
  **Correction**: They coincide only in commutative GIS; in non-commutative GIS they are distinct groups

# Common Confusions
- **Confusion**: Thinking interval-preservation is a weaker property than transposition
  **Clarification**: Both groups are simply transitive; interval-preserving operations are exactly as structured as transpositions

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 252.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly stated with dual structure
- Re-extraction notes: Re-extracted from v2 card; preserved: commutation property, role-swapping, commutative special case
