---
# === CORE IDENTIFICATION ===
concept: "SNDW (Sandwich Function)"
slug: sndw-function

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
  - sandwich number

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - cov-function
  - set-class
extends:
  - emb-function
  - cov-function
related:
  - adjoin-function
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the sandwich function SNDW?"
---

# Quick Definition
SNDW(X, Y, Z) counts forms of Y that can be "sandwiched" between X and Z — forms that both include X and are included in Z.

# Core Definition
Section 5.5 Notes (Lewin, p. 120): "SNDW(X, Y, Z), the sandwich number of Y between X and Z, is the number of forms of Y that both include X and are included in Z." Special cases: SNDW(empty, Y, Z) = EMB(Y, Z); SNDW(X, Y, S) = COV(X, Y). SNDW(X, /Y/, Z) is well-defined (independent of Y's representative), but SNDW depends on the specific X and Z, not just their classes.

# Prerequisites
- **EMB Function** — SNDW generalizes EMB
- **COV Function** — SNDW generalizes COV
- **Set Class** — SNDW counts forms (members of /Y/)

# Key Properties
1. SNDW(X, Y, Z) = |{Y' in /Y/ : X subset Y' subset Z}|
2. Can write SNDW(X, /Y/, Z) without ambiguity
3. Cannot substitute /X/ or /Z/ — depends on specific X and Z
4. SNDW(empty, Y, Z) = EMB(Y, Z); SNDW(X, Y, S) = COV(X, Y) (when S finite)

# Construction / Recognition
## To Compute:
1. Enumerate forms Y' of Y
2. Count those satisfying both X subset Y' and Y' subset Z

## To Recognize:
1. Counting intermediate harmonies fitting between a given chord and scale

# Context & Application
SNDW answers: "Given chord X and scale Z, how many forms of trichord Y fit between them?" This is useful for analyzing intermediate harmonies that connect a specific chord to a specific context.

# Examples
**Example 1** (p. 120): Z = C-major scale, /Y/ = Forte-class 3-4. X1 = {C, E}: SNDW = 2 ({B,C,E} and {C,E,F}). X2 = {F, A} (same set class as X1): SNDW = 1 ({E,F,A} only). Different specific sets X yield different SNDW values.

# Relationships
## Builds Upon
- **EMB Function** and **COV Function** — SNDW generalizes both

## Related
- **ADJOIN Function** — Complementary: ADJOIN requires disjointness, SNDW requires inclusion

# Common Errors
- **Error**: Substituting set classes for X or Z in SNDW
  **Correction**: SNDW depends on specific X and Z, not their classes

# Common Confusions
- **Confusion**: Thinking SNDW is symmetric in X and Z
  **Clarification**: X must be contained in Y', and Y' must be contained in Z — the roles are asymmetric

# Source Reference
Chapter 5: Generalized Set Theory (1), section 5.5 Notes, p. 120.

# Verification Notes
- Definition source: Direct from section 5.5
- Confidence rationale: Explicit definition with worked example
- Re-extraction notes: Re-extracted from v2 card; preserved: C-major scale example showing set-class dependence. Added v3.1 structure.
