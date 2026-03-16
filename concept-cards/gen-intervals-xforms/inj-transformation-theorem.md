---
concept: INJ Transformation Theorem
slug: inj-transformation-theorem

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.7.2"

extraction_confidence: high

aliases:
  - "Theorem 6.7.2"
  - INJ conjugation formula

prerequisites:
  - inj-function
  - operation
extends:
  - inj-function
related:
  - system-modulation
  - inj-generalizes-ifunc
  - ifunc-transposition-theorem
contrasts_with: []

answers_questions:
  - "How does INJ behave when sets are transformed by an operation?"
  - "What is system modulation in terms of INJ?"
---

# Quick Definition
When sets X and/or Y are transformed by an operation A, INJ values conjugate: INJ(A(X), A(Y))(f) = INJ(X, Y)(A^{-1}fA). This describes "system modulation."

# Core Definition
Theorem 6.7.2 (Lewin, p. 180): Given sets X, Y, transformation f, and operation A:
- (A): INJ(A(X), Y)(f) = INJ(X, Y)(fA)
- (B): INJ(X, A(Y))(f) = INJ(X, Y)(A^{-1}f)
- (C): INJ(A(X), A(Y))(f) = INJ(X, Y)(A^{-1}fA)

Formula (C) is key: when the system "modulates" by A, the transformation f' = AfA^{-1} plays the role in the modulated system that f played in the original.

# Prerequisites
- **INJ Function** — The function whose behavior under set transformation is described
- **Operation** — A must be an operation (1-to-1 and onto)

# Key Properties
1. Formula (C): INJ is conjugated, not simply preserved
2. The conjugate f' = AfA^{-1} is the "modulated" transformation
3. INJ(modulated X, modulated Y)(f') = INJ(X, Y)(f)
4. Generalizes IFUNC transposition theorems (5.1.6)
5. f need not be an operation; only A must be

# Construction / Recognition
## To Apply:
1. Identify the modulation operation A
2. Compute the conjugate transformation f' = AfA^{-1}
3. INJ(A(X), A(Y))(f') = INJ(X, Y)(f)

## To Recognize:
1. When harmonic relationships "transpose" from one key center to another

# Context & Application
This theorem formalizes system modulation: transposing an entire harmonic context preserves structural relationships up to conjugation. In "Angst und Hoffen," modulating from E-centered to F#-centered structure by T_2 transforms w^E to w^{F#} = T_2 w^E T_2^{-1} and I to J = T_2 I T_2^{-1}.

# Examples
**Example 1** (pp. 180-181): "Angst und Hoffen" modulation by T_2: w^{F#} = T_2 w^E T_2^{-1}. INJ(T_2(X), T_2(Y))(w^{F#}) = INJ(X, Y)(w^E).

**Example 2** (p. 181): Hexachord X inverts to complement via I. Modulate by T_n: T_n(X) inverts to complement via J = T_n I T_n^{-1}, not via I.

# Relationships
## Builds Upon
- **INJ Function** — Describes INJ under set transformations

## Enables
- **System Modulation** — The formal framework for modulating transformational systems

## Related
- **IFUNC Transposition Theorem** — Special case when f = T_i and A = T_n

# Common Errors
- **Error**: Assuming INJ(A(X), A(Y))(f) = INJ(X, Y)(f) (invariance)
  **Correction**: INJ conjugates: the transformation must also be conjugated

# Common Confusions
- **Confusion**: Confusing f with its conjugate AfA^{-1}
  **Clarification**: The order matters: A^{-1}fA (not fAA^{-1} = f)

# Source Reference
Chapter 6: Generalized Set Theory (2), Theorem 6.7.2, pp. 180-181.

# Verification Notes
- Definition source: Direct from Theorem 6.7.2 with proofs
- Confidence rationale: Explicit theorem with proofs and examples
- Re-extraction notes: Re-extracted from v2 card; preserved: conjugation formula, Angst/Hoffen modulation example. Added v3.1 structure.
