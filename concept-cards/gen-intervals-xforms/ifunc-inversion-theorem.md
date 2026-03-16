---
# === CORE IDENTIFICATION ===
concept: IFUNC Inversion Theorem
slug: ifunc-inversion-theorem

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: interval-functions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.1.7"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 5.1.7"
  - IFUNC under inversion

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - inversion-operation
  - ifunc-symmetry-theorem
extends:
  - ifunc
related:
  - ifunc-interval-preserving
  - interval-reversing-operation
contrasts_with:
  - inj-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens to IFUNC when both sets are inverted in a commutative GIS?"
  - "Why can't IFUNC fully engage inversional relationships?"
---

# Quick Definition
In a commutative GIS, when both sets are inverted by the same inversion operation I, IFUNC exchanges the roles of the two sets: IFUNC(I(X), I(Y)) = IFUNC(Y, X).

# Core Definition
Theorem 5.1.7: "If I is any inversion operation in a commutative GIS, then IFUNC(I(X), I(Y)) = IFUNC(Y, X)" (Lewin, p. 132). The proof sketch notes that in a commutative GIS, inversions are interval-reversing operations (Theorem 3.6.3), and the theorem follows by techniques similar to Theorem 5.1.5. Lewin emphasizes that "there is not much to be said in general about the effect of applying an inversion operation to X, Y, or both, so far as IFUNC is concerned" in non-commutative settings.

# Prerequisites
- **IFUNC** — The function whose behavior under inversion is described
- **Inversion Operation** — Must be in a commutative GIS for this theorem
- **IFUNC Symmetry Theorem** — IFUNC(Y, X)(i) = IFUNC(X, Y)(i^{-1}), used implicitly

# Key Properties
1. Requires a commutative GIS — does not hold in general
2. Inversion exchanges the roles of X and Y in IFUNC
3. Combined with Theorem 5.1.4: IFUNC(I(X), I(Y))(i) = IFUNC(X, Y)(i^{-1})
4. In non-commutative GIS, the relationship is more complex and better handled by INJ

# Construction / Recognition
## To Apply:
1. Verify the GIS is commutative
2. Apply inversion I to both sets
3. IFUNC(I(X), I(Y)) = IFUNC(Y, X) — simply reverse the set arguments

## To Recognize:
1. When inversionally related sets have IFUNC values that mirror each other

# Context & Application
This theorem reveals a fundamental limitation of IFUNC: it cannot fully engage inversional relationships in non-commutative settings. Lewin uses this limitation to motivate the Injection Function (INJ) in Chapter 6, which can handle inversions and other non-operation transformations. Even within the passage from Webern's op. 7 no. 3, Lewin notes that IFUNC can identify T_3 and T_8 embeddings but "cannot suggest" inversional relations; the injection function is needed for that.

# Examples
**Example 1** (derived from p. 132): In pitch-class space with X = {C, E} and Y = {G, B}: Apply I_0 (inversion about C). IFUNC(I_0(X), I_0(Y)) = IFUNC(Y, X). The inversion swaps which set serves as "source" and "target."

# Relationships
## Builds Upon
- **IFUNC** — Property of the interval function under inversion
- **Interval-Reversing Operation** — Inversions reverse intervals in commutative GIS

## Enables
- **INJ Function** — Lewin's motivation: IFUNC's limitations with inversion lead to INJ

## Related
- **IFUNC Interval-Preserving** — Analogous invariance theorem for P-operations

## Contrasts With
- **INJ Function** — INJ handles inversional relationships that IFUNC cannot

# Common Errors
- **Error**: Applying this theorem in a non-commutative GIS
  **Correction**: The theorem requires commutativity; in non-commutative settings, use INJ

# Common Confusions
- **Confusion**: Thinking IFUNC can capture all inversional set relationships
  **Clarification**: IFUNC is fundamentally limited with respect to inversion, especially in non-commutative settings. This is a key motivation for developing INJ in Chapter 6.

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.1.7 and surrounding discussion, pp. 132-133.

# Verification Notes
- Definition source: Direct from Theorem 5.1.7
- Confidence rationale: Explicit theorem with proof sketch in source
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, emphasis on commutative restriction, connection to INJ motivation. Added v3.1 structure.
