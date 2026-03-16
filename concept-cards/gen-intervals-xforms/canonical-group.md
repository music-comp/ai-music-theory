---
concept: Canonical Group
slug: canonical-group

category: generalized-set-theory
subcategory: canonical-groups
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.2.1"

extraction_confidence: high

aliases:
  - CANON
  - canonical operations

prerequisites:
  - generalized-interval-system
  - group
  - operation
extends: []
related:
  - canonical-equivalence
  - set-class
  - emb-function
  - interval-preserving-operation
contrasts_with: []

answers_questions:
  - "What is a canonical group?"
  - "How do canonical groups relate to set equivalence?"
  - "What distinguishes canonical equivalence from general set equivalence?"
---

# Quick Definition
A canonical group (CANON) is a designated group of operations on the space S that determines which sets are considered "equivalent" for analytical purposes. The choice of canonical group is context-dependent and reflects analytical priorities.

# Core Definition
Definition 5.2.1: "In certain connections we shall fix a group of operations on S and call it 'the canonical group.' It will be denoted CANON. Sets X and X' will be called 'canonically equivalent' if there exists some canonical operation A such that X' = A(X)" (Lewin, p. 136). The canonical equivalence relation is reflexive (X = IDENT(X)), symmetric (if X' = A(X) then X = A^{-1}(X')), and transitive (if X' = A(X) and X'' = B(X') then X'' = (BA)(X)).

# Prerequisites
- **Generalized Interval System** — Canonical groups typically involve operations from a GIS
- **Group** — CANON must satisfy group axioms: identity, inverses, closure under composition
- **Operation** — Members of CANON must be 1-to-1 and onto S

# Key Properties
1. CANON is a group of operations on S (closed, has identity, has inverses)
2. Different choices of CANON yield different notions of equivalence
3. No GIS structure is formally required — just a family S and a group of operations
4. Generally, interval-preserving operations should be included in CANON (Theorem 5.1.5 justifies this)
5. In a commutative GIS, interval-preserving operations = transpositions
6. The choice of CANON determines set classes, EMB values, and all derived constructs

# Construction / Recognition
## To Construct:
1. Choose a family S of musical objects
2. Identify operations on S that preserve the structural features you consider important
3. Verify these operations form a group (closure, identity, inverses)
4. Designate this group as CANON

## To Recognize:
1. A group of operations explicitly designated as determining set equivalence
2. Context-specific: the same S may support multiple valid canonical groups

# Context & Application
In Forte's atonal set theory, CANON typically includes transpositions and inversions, making major and minor triads equivalent. If CANON contains only transpositions, major and minor triads are distinct set classes. Other systems include circle-of-fifths transformations (M5/M7) or other operations. Lewin emphasizes that "it can be a tricky business to decide for any particular theoretical exercise just which operations on S are to be allowed into CANON" (p. 137).

# Examples
**Example 1** (p. 136): X = {C, E, G}. With CANON = {T_0, ..., T_11} (transpositions only): /X/ = {major triads}, 12 sets. With CANON = {T_i, I_j} (transpositions and inversions): /X/ = {all harmonic triads}, 24 sets.

**Example 2** (pp. 111-112): For the non-commutative GIS of time spans, CANON = interval-preserving operations (not transpositions), because these preserve the relationship between dyad structure and interval structure.

# Relationships
## Builds Upon
- **Group** — CANON must be a group of operations
- **Interval-Preserving Operation** — Generally included as minimum canonical operations

## Enables
- **Canonical Equivalence** — Defined via CANON
- **Set Class** — Equivalence classes under CANON
- **EMB Function** — Depends critically on the choice of CANON
- **K-Relation Generalized** — Uses CANON to define Forte's K/Kh relations

## Related
- **Canonical Groups (Octatonic)** — Specific example of CANON choice for octatonic sets

# Common Errors
- **Error**: Assuming there is one "correct" canonical group
  **Correction**: The choice depends on analytical goals; different CANON choices produce different but potentially valid analyses

# Common Confusions
- **Confusion**: Thinking CANON must include transpositions
  **Clarification**: In non-commutative GIS, transpositions may not preserve internal intervallic structure; interval-preserving operations are the more fundamental choice

# Source Reference
Chapter 5: Generalized Set Theory (1), Definition 5.2.1, pp. 135-137.

# Verification Notes
- Definition source: Direct from Definition 5.2.1
- Confidence rationale: Explicit definition with extensive discussion
- Re-extraction notes: Re-extracted from v2 card; preserved: X={C,E,G} example, commutative/non-commutative distinction. Added v3.1 structure, time-span CANON example, competency questions.
