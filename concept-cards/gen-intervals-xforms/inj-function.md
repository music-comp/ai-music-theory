---
# === CORE IDENTIFICATION ===
concept: "INJ (Injection Function)"
slug: inj-function

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: injection-function
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - injection number
  - injection function
  - "INJ(X, Y)(f)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - set-in-gis
  - transformation
extends:
  - ifunc
related:
  - inj-generalizes-ifunc
  - inj-operation-theorem
  - inj-transformation-theorem
  - inj-complement-theorem
  - progressive-transformation
  - internal-transformation
contrasts_with:
  - ifunc
  - emb-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the INJ (injection number)?"
  - "How does IFUNC relate to INJ?"
  - "How do I compute INJ for a transformation?"
  - "What must I know before understanding INJ?"
---

# Quick Definition
INJ(X, Y)(f) counts the number of elements in set X whose images under transformation f are members of set Y. Unlike IFUNC, INJ works with any transformation, not just transpositions, and does not require a GIS.

# Core Definition
Definition 6.2.1: "Given sets X and Y, given a transformation f on S, then the injection number of X into Y for f, denoted INJ(X, Y)(f), is the number of elements s in X such that f(s) is a member of Y" (Lewin, p. 155). INJ answers: "If I apply transformation f to set X, how many members of X will I thereby map into members of Y?" The construction requires only a family S of objects and transformations on S — no GIS or group structure is needed.

# Prerequisites
- **Set in a GIS** — X and Y must be finite subsets of S (though S need not be a GIS)
- **Transformation** — f maps S into itself; need not be 1-to-1 or onto

# Key Properties
1. INJ(X, Y)(f) = |{s in X : f(s) in Y}|
2. 0 <= INJ(X, Y)(f) <= card(X)
3. If f is 1-to-1: INJ(X, Y)(f) = card(f(X) intersect Y) (Theorem 6.5.1)
4. If f is not 1-to-1: INJ may exceed card(f(X) intersect Y)
5. Does not require f to be an operation, a GIS, or any algebraic structure
6. Generalizes both IFUNC and EMB (Theorems 6.7.1, 6.8)
7. INJ(X, Y)(T_i) = IFUNC(X, Y)(i) when a GIS is present

# Construction / Recognition
## To Compute INJ(X, Y)(f):
1. For each element s in X, compute f(s)
2. Check whether f(s) is a member of Y
3. Count the elements s for which f(s) is in Y

## To Recognize:
1. Any count of how many elements of one set map into another under a transformation

# Context & Application
INJ is more general than IFUNC: it handles any transformation, including wedges, non-invertible mappings, and other musically significant transformations that are not operations. It enables analysis of inversional relationships, voice-leading processes, and compositional structures that IFUNC cannot capture. INJ is introduced at a "high level of abstraction" (p. 154) and then connected back to IFUNC and EMB, showing it generalizes both.

# Examples
**Example 1** (p. 155): S = 12 pitch classes. f maps white notes to C, black notes to F#. X = {C, C#, D, Eb, E}, Y = {B, C#, D, E, F, F#}. INJ(X, Y)(f) = 2: only the 2 black notes of X (C#, Eb) map to F# which is in Y.

**Example 2** (pp. 155-156): w^E = wedge-to-E. X = {Gb, Bb, D} (Angst), Y = {Fb, Bb, Eb} (Hoffen). INJ(X, Y)(w^E) = 2: D wedges to Eb (in Y), Bb stays at Bb (in Y), Gb wedges to G (not in Y). "Two-thirds of X" maps into Y.

# Relationships
## Builds Upon
- **IFUNC** — INJ generalizes IFUNC: INJ(X, Y)(T_i) = IFUNC(X, Y)(i)
- **Set in a GIS** — Arguments X and Y are sets

## Enables
- **Progressive Transformation** — Defined via high INJ(X, Y)(f)
- **Internal Transformation** — Defined via high INJ(X, X)(f)
- **K-Relation Generalized** — K/Kh relations reformulated via INJ
- **System Modulation** — INJ behavior under conjugation (6.7.2)

## Related
- **INJ Generalizes IFUNC** — Theorem 6.7.1
- **INJ Operation Theorem** — When f is an operation
- **INJ Complement Theorem** — Complement relations

## Contrasts With
- **IFUNC** — IFUNC requires a GIS and transpositions; INJ works with any transformation
- **EMB Function** — EMB counts forms; INJ counts mapped elements

# Common Errors
- **Error**: Assuming INJ(X, Y)(f) = card(f(X) intersect Y) for all f
  **Correction**: This holds only when f is 1-to-1. For non-1-to-1 f, multiple elements of X can map to the same element of Y.

# Common Confusions
- **Confusion**: Thinking INJ requires a GIS
  **Clarification**: INJ requires only a family S and transformations on S. No intervals, no group structure needed. This is its key advantage.

- **Confusion**: Thinking "injection" means f must be injective (1-to-1)
  **Clarification**: The name refers to "injecting" elements of X into Y, not to the mathematical sense of "injective function"

# Source Reference
Chapter 6: Generalized Set Theory (2), Definition 6.2.1, Examples 6.2.2-6.2.3, pp. 155-156.

# Verification Notes
- Definition source: Direct from Definition 6.2.1
- Confidence rationale: Explicit definition with multiple examples
- Re-extraction notes: Re-extracted from v2 card; preserved: white/black note example, Angst/Hoffen example, emphasis on generality beyond GIS. Added v3.1 structure, competency questions, typed relationships.
