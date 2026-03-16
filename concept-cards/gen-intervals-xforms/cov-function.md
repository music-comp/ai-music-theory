---
# === CORE IDENTIFICATION ===
concept: "COV (Covering Function)"
slug: cov-function

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
aliases:
  - covering number
  - "COV(X, Y)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - set-class
  - canonical-group
extends:
  - emb-function
related:
  - sndw-function
  - adjoin-function
contrasts_with:
  - emb-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the covering function COV?"
  - "How does COV differ from EMB?"
---

# Quick Definition
COV(X, Y) counts the number of forms of Y that include X — the covering number, measuring how many canonical transforms of Y serve as supersets of X.

# Core Definition
Section 5.5 Notes (Lewin, p. 120): "We may define COV(X, Y), the covering number of X in Y, as the number of forms of Y that include X. This is not necessarily the same number as EMB(X, Y)." If S is finite, then COV(X, Y) = EMB(complement of Y, complement of X), connecting covering to embedding via complementation.

# Prerequisites
- **EMB Function** — COV is a dual to EMB
- **Set Class** — COV counts forms of Y (members of /Y/)
- **Canonical Group** — COV depends on CANON

# Key Properties
1. COV(X, Y) = |{Y' in /Y/ : X subset of Y'}|
2. EMB(X, Y) != COV(X, Y) in general
3. If S is finite: COV(X, Y) = EMB(complement(Y), complement(X))
4. COV answers "how many key areas contain chord X?"

# Construction / Recognition
## To Compute:
1. Enumerate all forms Y' = A(Y) for A in CANON
2. Count how many contain X as a subset

## To Recognize:
1. Counting contexts (transpositions of a scale/aggregate) that contain a given chord

# Context & Application
COV complements EMB: while EMB asks "how many forms of X fit inside Y?", COV asks "how many forms of Y surround X?" This is useful for harmonic analysis where one asks which keys or scale contexts accommodate a given chord.

# Examples
**Example 1** (p. 120): X = {C, E}, Y = {C, E, G#} (augmented triad). EMB(X, Y) = 3 (three major-third dyads within the triad). COV(X, Y) = 1 (only one augmented triad contains the specific dyad {C, E}).

# Relationships
## Builds Upon
- **EMB Function** — COV is the dual operation

## Related
- **SNDW Function** — SNDW(X, Y, S) = COV(X, Y) when S is finite
- **ADJOIN Function** — Another set-relation function from section 5.5

## Contrasts With
- **EMB Function** — EMB counts small-set forms in a large set; COV counts large-set forms around a small set

# Common Errors
- **Error**: Assuming EMB(X, Y) = COV(X, Y)
  **Correction**: These are generally different; they measure different relationships

# Common Confusions
- **Confusion**: Thinking COV depends only on set classes
  **Clarification**: COV(X, Y) depends on the specific set X, not just /X/

# Source Reference
Chapter 5: Generalized Set Theory (1), section 5.5 Notes, p. 120.

# Verification Notes
- Definition source: Direct from section 5.5
- Confidence rationale: Explicit definition with example
- Re-extraction notes: Re-extracted from v2 card; preserved: augmented triad example, complement relationship. Added v3.1 structure.
