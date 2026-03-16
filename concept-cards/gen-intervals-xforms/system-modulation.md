---
# === CORE IDENTIFICATION ===
concept: System Modulation
slug: system-modulation

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
section: "6.7.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - transformational modulation

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-transformation-theorem
  - inj-function
extends:
  - inj-transformation-theorem
related:
  - angst-hoffen-analysis
  - progressive-transformation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is system modulation in the context of INJ?"
---

# Quick Definition
System modulation occurs when an entire musical context transforms by an operation A, changing sets X, Y to A(X), A(Y) and transformations f to f' = AfA^{-1}. The conjugate f' plays the same structural role in the modulated system that f played in the original.

# Core Definition
Following Theorem 6.7.2(C) (Lewin, pp. 180-181): "We can imagine that the shift from X-and-Y to A(X)-and-A(Y) reflects a 'modulation' of the system by the operation A." The key insight: "INJ(modulated X, modulated Y)(f') = INJ(X, Y)(f)" where f' = AfA^{-1}. Lewin notes "the INJ function is itself 'modulated' according to the formula."

# Prerequisites
- **INJ Transformation Theorem** — Provides the conjugation formula
- **INJ Function** — The function whose behavior under modulation is described

# Key Properties
1. Modulation map: (X, Y, f) -> (A(X), A(Y), AfA^{-1})
2. INJ(A(X), A(Y))(AfA^{-1}) = INJ(X, Y)(f) — structural preservation
3. In commutative GIS: T_i remains T_i under transposition (transpositions commute)
4. Inversions shift: I_u becomes T_n I_u T_n^{-1} = I_{n+u}
5. Wedges shift: w^u becomes T_n w^u T_n^{-1} = w^{T_n(u)}

# Construction / Recognition
## To Apply:
1. Identify the modulation operation A (typically a transposition)
2. Transform all sets by A
3. Conjugate all transformations: f -> AfA^{-1}
4. The new system has the same INJ structure as the old

## To Recognize:
1. When the same types of relationships appear at a different pitch level or in a different context

# Context & Application
System modulation formalizes how harmonic structures transpose while maintaining internal relationships. The bass motion E -> F# in "Angst und Hoffen" corresponds to a T_2 modulation of the entire wedge/inversion system. This extends to hexachord modulations: when T_n(X) replaces X, the inversion that maps X to its complement is replaced by its conjugate.

# Examples
**Example 1** (pp. 128-129, 180-181): In "Angst und Hoffen," E-centered system modulates to F#-centered by T_2. w^E -> w^{F#} = T_2 w^E T_2^{-1}; I -> J = T_2 I T_2^{-1}. The T_2 relation between bass notes Gb and E expands into a full system modulation.

**Example 2** (p. 181): Hexachord X inverts to complement via I = I_0^E. After T_3 modulation, T_3(X) inverts to complement via J = T_3 I T_3^{-1} = I_3^{Ab}.

# Relationships
## Builds Upon
- **INJ Transformation Theorem** — Provides the formal basis

## Enables
- **Angst und Hoffen Analysis** — E-to-F# modulation is central

# Common Errors
- **Error**: Assuming the same transformation f works in the modulated system
  **Correction**: The transformation must be conjugated to AfA^{-1}

# Common Confusions
- **Confusion**: Thinking system modulation is just transposition
  **Clarification**: It is transposition of sets plus conjugation of transformations — a coordinated change of the entire system

# Source Reference
Chapter 6: Generalized Set Theory (2), discussion following Theorem 6.7.2, pp. 180-182.

# Verification Notes
- Definition source: Synthesized from discussion of Theorem 6.7.2
- Confidence rationale: Detailed discussion with multiple examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Angst/Hoffen modulation, hexachord example. Added v3.1 structure.
