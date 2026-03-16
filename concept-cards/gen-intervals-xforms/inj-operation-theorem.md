---
# === CORE IDENTIFICATION ===
concept: INJ Operation Theorem
slug: inj-operation-theorem

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
section: "6.5.1-6.5.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 6.5.1"
  - "Corollary 6.5.2"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
  - operation
extends:
  - inj-function
related:
  - inj-complement-theorem
  - inj-generalizes-ifunc
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens to INJ when f is an operation?"
---

# Quick Definition
When f is an operation OP (1-to-1 and onto), INJ(X, Y)(OP) equals the cardinality of OP(X) intersect Y — the number of common members shared by the transformed set and the target set. This generalizes Regener's Common-Note Function.

# Core Definition
Theorem 6.5.1 (Lewin, p. 175): "If f is an operation OP, then INJ(X, Y)(OP) is the cardinality of OP(X) intersect Y." Corollary 6.5.2: "If f is an operation OP, then INJ(Y, X)(OP) = INJ(X, Y)(OP^{-1})."

# Prerequisites
- **INJ Function** — The function whose behavior for operations is described
- **Operation** — OP must be 1-to-1 and onto S

# Key Properties
1. INJ(X, Y)(OP) = card(OP(X) intersect Y)
2. INJ(Y, X)(OP) = INJ(X, Y)(OP^{-1})
3. Generalizes Regener's Common-Note Function (for transpositions on pitch classes)
4. Only valid for operations, not general transformations

# Construction / Recognition
## To Apply:
1. Verify f is an operation (1-to-1 and onto)
2. Compute f(X)
3. Count elements common to f(X) and Y

## To Recognize:
1. Common-tone counting between a transformed set and a target set

# Context & Application
This theorem connects INJ to the familiar notion of common tones. Regener's Common-Note Function counts how many common tones T_i(X) shares with Y; INJ generalizes this to all operations. The restriction to operations is essential for the equality; for non-operations, INJ can exceed the intersection cardinality.

# Examples
**Example 1** (derived): T_5 on pitch classes. X = {C, E, G}, Y = {C, F, A}. T_5(X) = {F, A, C}. T_5(X) intersect Y = {C, F, A} = 3 elements. INJ(X, Y)(T_5) = 3.

# Relationships
## Builds Upon
- **INJ Function** — Special case when f is an operation

## Enables
- **INJ Complement Theorem** — Requires this theorem for its proof
- **INJ Generalizes IFUNC** — Combined with 6.7.1

# Common Errors
- **Error**: Applying this theorem to non-operations like wedge transformations
  **Correction**: For non-operations, INJ may exceed the intersection cardinality

# Common Confusions
- **Confusion**: Thinking INJ always equals set intersection size
  **Clarification**: Only for operations. For wedges and other non-1-to-1 maps, the relationship breaks.

# Source Reference
Chapter 6: Generalized Set Theory (2), Theorem 6.5.1, Corollary 6.5.2, pp. 175-176.

# Verification Notes
- Definition source: Direct from Theorem 6.5.1
- Confidence rationale: Explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: Regener connection, operations-only caveat. Added v3.1 structure.
