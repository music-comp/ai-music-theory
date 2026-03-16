---
concept: Set Class
slug: set-class

category: generalized-set-theory
subcategory: canonical-groups
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.2.2-5.2.3"

extraction_confidence: high

aliases:
  - /X/
  - "forms of X"
  - canonical equivalence class

prerequisites:
  - canonical-group
  - canonical-equivalence
extends:
  - canonical-equivalence
related:
  - emb-function
  - m-class-vector
  - z-relation-generalized
contrasts_with: []

answers_questions:
  - "What is a set class in generalized set theory?"
  - "How does the choice of canonical group affect set classes?"
---

# Quick Definition
A set class /X/ is the canonical equivalence class containing set X — the collection of all sets derivable from X by operations in the canonical group. "X' is a form of X" means X' is in /X/.

# Core Definition
Definition 5.2.2: "We shall write /X/ to denote the canonical equivalence-class containing the set X. /X/ will be called, for short, the 'set class of X'" (Lewin, p. 136). Locution 5.2.3: "'X' is a form of X' means that X' is canonically equivalent to X. /X/ may be referred to as 'the forms of X'" (p. 137). The set class depends on both X and CANON; different canonical groups produce different set classes from the same set.

# Prerequisites
- **Canonical Group** — Set classes are defined relative to CANON
- **Canonical Equivalence** — The equivalence relation that partitions sets into classes

# Key Properties
1. /X/ = {A(X) : A in CANON}
2. If X' is in /X/, then /X'/ = /X/ (same equivalence class)
3. |/X/| <= |CANON| (bounded by group size)
4. If X has symmetry (some non-identity A fixes X), then |/X/| < |CANON|
5. Different CANON choices yield different set classes from the same X
6. EMB(X', Y) = EMB(X, Y) for X' in /X/ — embedding number is well-defined on set classes

# Construction / Recognition
## To Construct /X/:
1. Fix the canonical group CANON
2. Apply every A in CANON to X
3. Collect the distinct results — this is /X/

## To Recognize:
1. Two sets belong to the same set class if one can be obtained from the other by a canonical operation

# Context & Application
Lewin acknowledges that "the term 'set class' will grate dreadfully on the ears of any mathematical logician" but adopts it as standard usage in atonal theory (p. 136). His earlier term "chord type" was more precise but loses intuitive meaning for non-pitch sets (rhythmic sets, timbral sets, etc.). The concept is fundamental to EMB, the M-class vector, and all comparative set-theoretic analysis.

# Examples
**Example 1** (p. 136): X = {C, E, G}. With CANON = transpositions: /X/ = {major triads} (12 sets). With CANON = transpositions + inversions: /X/ = Forte's 3-11 = {all harmonic triads} (24 sets).

**Example 2** (p. 136): The augmented triad {C, E, G#} has only 4 forms under transposition (symmetric under T_4), showing that |/X/| < |CANON| when X has symmetry.

# Relationships
## Builds Upon
- **Canonical Equivalence** — Set classes are the equivalence classes

## Enables
- **EMB Function** — Counts forms of one set class embedded in another set
- **M-Class Vector** — Lists EMB values for all M-element set classes
- **K-Relation Generalized** — Compares sets across set classes

## Related
- **Z-Relation Generalized** — Z-related sets belong to distinct set classes but share IFUNC self-values

# Common Errors
- **Error**: Forgetting that set class depends on CANON
  **Correction**: Always specify the canonical group; "set class" is meaningless without it

# Common Confusions
- **Confusion**: Thinking "set class" is a purely mathematical term
  **Clarification**: Lewin uses it as music-theoretic terminology (adapted from Forte), acknowledging it conflicts with mathematical logic usage

# Source Reference
Chapter 5: Generalized Set Theory (1), Definitions 5.2.2-5.2.3, pp. 136-137.

# Verification Notes
- Definition source: Direct from Definitions 5.2.2 and 5.2.3
- Confidence rationale: Explicit definitions in source
- Re-extraction notes: Re-extracted from v2 card; preserved: augmented triad symmetry example, Forte 3-11 reference. Added v3.1 structure.
