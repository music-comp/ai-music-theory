---
# === CORE IDENTIFICATION ===
concept: Transformation vs Operation
slug: transformation-vs-operation

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: foundational-definitions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "Convention 6.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - function
  - group
extends: []
related:
  - inj-function
  - ifunc
  - wedge-transformation
  - canonical-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the formal difference between a transformation and an operation?"
  - "Why can a transformation that is not an operation not belong to a group?"
  - "Why does INJ handle transformations that are not operations while IFUNC requires operations?"
---

# Quick Definition
A transformation is any mapping from S into itself; an operation is a transformation that is both 1-to-1 (injective) and onto (surjective), hence invertible. Operations can form groups; non-operational transformations cannot.

# Core Definition
"We shall be concerned with a family S of objects, and with various transformations f that map S into itself. We do not assume that the transformations are necessarily operations (1-to-1 and onto S). Operations are capable of entering into groups, e.g. canonical groups, groups of interval-preserving operations and/or transpositions in a GIS, and the like. A transformation that is not an operation can have no inverse transformation on S, and so cannot belong to any group of operations on S" (Lewin, Convention 6.1, p. 154).

# Prerequisites
- **Function** — Both transformations and operations are functions from S to S
- **Group** — Operations form groups; non-operational transformations cannot

# Key Properties
1. Transformation f: S -> S maps each element to some element (not necessarily 1-to-1 or onto)
2. Operation OP: S -> S is a transformation that is 1-to-1 (injective) AND onto (surjective)
3. Operations are invertible: OP^(-1) exists and is also an operation
4. Operations form groups; transformations in general form only semigroups
5. When f is an operation: INJ(X, Y)(f) = card(f(X) intersect Y)
6. When f is not an operation: INJ(X, Y)(f) may exceed card(f(X) intersect Y)

# Construction / Recognition
## To Construct:
1. Define a mapping f: S -> S
2. Check if f is 1-to-1: does f(s) = f(t) imply s = t?
3. Check if f is onto: is every element of S the image of something?
4. If both: f is an operation. Otherwise: f is a (non-operational) transformation.
## To Recognize:
1. Operations: transpositions T_i, inversions I, any bijection on S
2. Non-operations: wedges (w^E), projections, contractions

# Context & Application
The distinction is crucial for INJ theory. IFUNC only involves transpositions (which are operations). INJ generalizes IFUNC by handling all transformations, including wedges, projections, and contractions that are musically significant but not 1-to-1 or onto. This generality is why Lewin develops INJ as the "master function" for set theory. "Nowhere have we needed to suppose that the transformations we were inspecting were 1-to-1 or onto; many in fact were not. From all this we get some idea of how generally the INJ construct can be applied."

# Examples
**Example 1** (Convention 6.1, Example 6.2.2): Transformation f maps all white keys to C, all black keys to F#.
- Not 1-to-1: all seven white keys map to the same pitch class C
- Not onto: ten pitch classes are never images
- INJ(X, Y)(f) can be large even when f(X) intersect Y is small

**Example 2** (Example 6.2.3): Wedge w^E is not an operation:
- Not 1-to-1: w^E(E) = w^E(F) = E (two pitch classes map to E)
- Not onto: no pitch class maps to F under w^E

**Example 3**: Transposition T_5 is an operation:
- 1-to-1: T_5(s) = T_5(t) implies s = t
- Onto: every pitch class is T_5 of something
- Inverse: T_5^(-1) = T_7

**Example 4**: Inversion I_0 is an operation:
- 1-to-1 and onto
- Self-inverse: I_0^(-1) = I_0

# Relationships
## Builds Upon
- **Function** — Both are special cases of functions from S to S
- **Group** — Operations form groups; non-operations cannot
## Enables
- **INJ function** — Handles both operations and non-operational transformations
## Related
- **IFUNC** — Only involves transpositions (operations)
- **Wedge transformation** — A paradigmatic non-operational transformation
- **Canonical group** — A group of operations used in set-class theory

# Common Errors
- **Error**: Applying theorems that require bijectivity (like Theorem 6.5.1) to non-operational transformations
  **Correction**: When f is not an operation, INJ(X, Y)(f) may not equal card(f(X) intersect Y); special care is needed

# Common Confusions
- **Confusion**: Thinking all musically significant transformations must be operations
  **Clarification**: Many musically significant transformations -- wedges, contractions, registral projections -- are not operations. The greater generality of INJ over IFUNC lies precisely in handling such non-operational transformations.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Convention 6.1 and throughout, pages 154-155.

# Verification Notes
- Definition source: Direct quotation from Convention 6.1
- Confidence rationale: Fundamental convention stated at the opening of Chapter 6
- Re-extraction notes: Re-extracted from v2 card; preserved: wedge and white/black key examples, INJ vs IFUNC distinction, group membership criterion
