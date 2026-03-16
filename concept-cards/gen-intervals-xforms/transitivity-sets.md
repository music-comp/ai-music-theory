---
concept: Transitivity Sets
slug: transitivity-sets

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: null

extraction_confidence: high

aliases:
  - orbits under an operation
  - I-partnerships

prerequisites:
  - inj-function
  - inversion-operation
extends: []
related:
  - angst-hoffen-analysis
  - internal-transformation
contrasts_with: []

answers_questions:
  - "What are transitivity sets and how do they reveal operation structure?"
---

# Quick Definition
Transitivity sets partition S into orbits under an operation, grouping elements that transform among themselves. For inversions, they reveal "partnerships" — pairs of pitch classes that map to each other.

# Core Definition
Given an operation OP on S, a transitivity set is a minimal non-empty subset T such that OP maps T into itself (Lewin, pp. 160-161). For inversion I = I_E^{Bb}: singletons {E} and {Bb} (fixed points); pairs {A, B}, {Ab, C}, {G, C#}, {Gb, D}, {F, Eb} (partnerships). Chords that embed entire transitivity sets have special structural significance. When I and w^E commute, w^E maps I-partnerships to I-partnerships as units.

# Prerequisites
- **INJ Function** — Transitivity sets are used within INJ analysis
- **Inversion Operation** — Most common application is to inversion partnerships

# Key Properties
1. Transitivity sets of OP partition S completely
2. OP(T) = T for each transitivity set T
3. T is minimal: no proper subset is also OP-invariant
4. For inversions: fixed points are singletons; all others are 2-element partnerships
5. When two operations commute, one maps transitivity sets of the other to themselves

# Construction / Recognition
## To Construct:
1. Given operation OP, compute OP(s) for each s in S
2. Group elements into orbits: {s, OP(s), OP^2(s), ...}
3. Each orbit is a transitivity set

## To Recognize:
1. Pairs (or singletons) of elements that are exchanged (or fixed) by an operation

# Context & Application
In "Angst und Hoffen," tracking I-partnerships reveals that entire partnerships wedge together (e.g., (Ab, C) -> (G, C#) under w^E). The "missing F" breaks the (F, Eb) partnership, leaving Eb "bereft of its I-partner." This connects the formal structure to the text about the absent lover.

# Examples
**Example 1** (pp. 160-161): I = I_E^{Bb} partitions pitch classes into: {Bb}, {A,B}, {Ab,C}, {G,C#}, {Gb,D}, {F,Eb}, {E}. The Angst chord {Gb, Bb, D} embeds the partnership {Gb, D} and the singleton {Bb}. The Seufzer chord embeds two full partnerships.

# Relationships
## Builds Upon
- **Inversion Operation** — Most common source of transitivity sets

## Enables
- **Angst und Hoffen Analysis** — I-partnerships track through the harmonic progression

## Related
- **Internal Transformation** — Internal transformations preserve transitivity-set structure

# Common Errors
- **Error**: Thinking transitivity sets are fixed for all operations
  **Correction**: Each operation has its own transitivity sets

# Common Confusions
- **Confusion**: Thinking transitivity sets are the same as set classes
  **Clarification**: Transitivity sets partition the space S (elements), not sets; set classes partition the collection of subsets

# Source Reference
Chapter 6: Generalized Set Theory (2), discussion following Figure 6.3, pp. 160-161.

# Verification Notes
- Definition source: Synthesized from discussion in context of Angst und Hoffen analysis
- Confidence rationale: Clear definition with detailed analytical application
- Re-extraction notes: Re-extracted from v2 card; preserved: I_E^Bb partition, partnership tracking, "bereft" language. Added v3.1 structure.
