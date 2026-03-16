---
# === CORE IDENTIFICATION ===
concept: Canonical Equivalence
slug: canonical-equivalence

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: canonical-groups
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - canonical equivalence relation
  - "X' is a form of X"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - canonical-group
  - set-in-gis
extends:
  - canonical-group
related:
  - set-class
  - emb-function
  - ifunc-interval-preserving
contrasts_with:
  - z-relation-generalized

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is canonical equivalence?"
  - "What distinguishes canonical equivalence from general set equivalence?"
  - "How do canonical groups relate to set equivalence?"
---

# Quick Definition
Two sets X and X' are canonically equivalent if one can be transformed into the other by some operation in the canonical group CANON. This relation partitions all sets into equivalence classes called set classes.

# Core Definition
Definition 5.2.1 (continued): "Sets X and X' will be called 'canonically equivalent' if there exists some canonical operation A such that X' = A(X)" (Lewin, p. 136). The relation is an equivalence relation: reflexive (X = IDENT(X)), symmetric (if X' = A(X) then X = A^{-1}(X')), and transitive (if X' = A(X) and X'' = B(X') then X'' = (BA)(X)). When CANON includes interval-preserving operations, canonical equivalence implies IFUNC(X, X) = IFUNC(X', X') (by Theorem 5.1.5).

# Prerequisites
- **Canonical Group** — Canonical equivalence is defined relative to a specific CANON
- **Set in a GIS** — The objects being compared are sets (finite subsets of S)

# Key Properties
1. Is an equivalence relation (reflexive, symmetric, transitive)
2. Depends entirely on the choice of CANON — not an absolute notion
3. If CANON contains interval-preserving operations, equivalent sets share internal intervallic structure
4. Partitions all sets into equivalence classes (set classes)
5. "X' is a form of X" means X' is canonically equivalent to X (Locution 5.2.3)
6. /X/ denotes the canonical equivalence class of X (Definition 5.2.2)

# Construction / Recognition
## To Determine Canonical Equivalence:
1. Fix the canonical group CANON
2. Check whether there exists any A in CANON such that X' = A(X)
3. If yes, X and X' are canonically equivalent

## To Recognize:
1. Two sets that can be related by some canonical operation
2. Sets sharing the same set-class label /X/

# Context & Application
Canonical equivalence generalizes the intuition that transposed or inverted forms of a set "sound the same" in terms of internal interval content. The key insight is that this notion is relative: different canonical groups produce different equivalence classes. Lewin uses the term "canonical" rather than simply "equivalent" to emphasize this relativity. The same set X may belong to different set classes under different choices of CANON.

# Examples
**Example 1** (p. 136): In pitch-class space with X = {C, E, G}:
- CANON = transpositions only: {C, E, G} ~ {D, F#, A} (both major triads), but {C, E, G} is NOT equivalent to {C, Eb, G} (major vs. minor)
- CANON = transpositions and inversions: {C, E, G} ~ {C, Eb, G} (both harmonic triads)

**Example 2** (p. 137): In the time-span GIS, CANON = interval-preserving operations. Y1 (a passage as first imagined), Y2 (as first performed), and Y3 (as played yesterday at a different tempo) are all approximately canonically equivalent — the formalism captures that these are "the same passage."

# Relationships
## Builds Upon
- **Canonical Group** — Equivalence is defined relative to CANON

## Enables
- **Set Class** — The equivalence classes under canonical equivalence
- **EMB Function** — EMB counts forms (canonically equivalent sets) embedded in a set
- **K-Relation Generalized** — Uses canonical forms to define subset relations

## Related
- **IFUNC Interval-Preserving** — Equivalent sets share IFUNC self-values when CANON includes P-operations

## Contrasts With
- **Z-Relation Generalized** — Z-related sets share IFUNC self-values without being canonically equivalent

# Common Errors
- **Error**: Treating canonical equivalence as absolute
  **Correction**: Always specify which CANON is in effect; equivalence is relative to this choice

# Common Confusions
- **Confusion**: Assuming all "reasonable" equivalences produce the same set classes
  **Clarification**: CANON = transpositions only vs. CANON = transpositions + inversions produce fundamentally different partitions of set space

# Source Reference
Chapter 5: Generalized Set Theory (1), Definitions 5.2.1-5.2.3, pp. 135-137.

# Verification Notes
- Definition source: Direct from Definition 5.2.1
- Confidence rationale: Explicit definition with proof of equivalence relation properties
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class examples, emphasis on relativity of equivalence. Added time-span example, v3.1 structure.
