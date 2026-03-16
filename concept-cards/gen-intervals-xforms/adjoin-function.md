---
# === CORE IDENTIFICATION ===
concept: ADJOIN Function
slug: adjoin-function

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: embedding-functions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - set-class
extends:
  - emb-function
related:
  - sndw-function
  - cov-function
contrasts_with:
  - sndw-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the ADJOIN function?"
---

# Quick Definition
ADJOIN(X, Y, Z) counts forms of Y that are disjoint from X but can be combined with X to fit within some form of Z.

# Core Definition
Section 5.5 Notes (Lewin, p. 121): "ADJOIN(X, Y, Z) is the number of forms Y' of Y satisfying both (A) Y' is disjoint from X, and (B) there is some form of Z that includes both X and Y'." Unlike SNDW, ADJOIN can be computed from set classes: ADJOIN(/X/, /Y/, /Z/) is well-defined.

# Prerequisites
- **EMB Function** — ADJOIN uses embedding concepts
- **Set Class** — ADJOIN is well-defined on set classes (unlike SNDW for X and Z)

# Key Properties
1. ADJOIN(X, Y, Z) = |{Y' in /Y/ : X intersect Y' = empty AND exists Z' in /Z/ with X union Y' subset Z'}|
2. Can write ADJOIN(/X/, /Y/, /Z/) — fully set-class invariant
3. Counts "compatible additions" to X within Z-type contexts

# Construction / Recognition
## To Compute:
1. Enumerate forms Y' of Y
2. Check (A): Y' disjoint from X
3. Check (B): X union Y' fits within some form of Z

## To Recognize:
1. Counting chords that can be "added" to a given chord within some scale context

# Context & Application
ADJOIN answers: "How many forms of chord Y can be added to chord X (without overlap) so the combination fits in some scale of type Z?" This models harmonic possibilities for expanding a sonority within a tonal or modal context.

# Examples
**Example 1** (p. 121): X = {C, E}, Y = {D, G}, Z = major scale. ADJOIN = 4: four perfect fourths ({D,G}, {F,Bb}, {F#,B}, {A,D}) can be added to {C,E} while fitting within some major scale.

# Relationships
## Builds Upon
- **EMB Function** — Uses embedding concepts

## Related
- **COV Function** — COV counts containing contexts; ADJOIN counts compatible additions

## Contrasts With
- **SNDW Function** — SNDW requires containment (X subset Y'); ADJOIN requires disjointness (X disjoint from Y')

# Common Errors
- **Error**: Confusing ADJOIN with SNDW
  **Correction**: SNDW sandwiches (X inside Y' inside Z); ADJOIN adjoins (X alongside Y' inside some Z')

# Common Confusions
- **Confusion**: Thinking the "some form of Z" must be specified
  **Clarification**: ADJOIN checks all forms of Z; any one sufficing is enough

# Source Reference
Chapter 5: Generalized Set Theory (1), section 5.5 Notes, p. 121.

# Verification Notes
- Definition source: Direct from section 5.5
- Confidence rationale: Explicit definition with example
- Re-extraction notes: Re-extracted from v2 card; preserved: C-E plus fourth example. Added v3.1 structure.
