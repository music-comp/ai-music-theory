---
# === CORE IDENTIFICATION ===
concept: Wedge Transformation
slug: wedge-transformation

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
section: "6.2.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "w^E"
  - wedging-to-E
  - wedge-to-E

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
  - transformation
extends: []
related:
  - angst-hoffen-analysis
  - die-kreuze-analysis
  - progressive-transformation
  - system-modulation
contrasts_with:
  - inversion-operation
  - transposition-operation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a wedge transformation?"
  - "Why does INJ handle wedge transformations but IFUNC cannot?"
---

# Quick Definition
A wedge transformation systematically moves pitch classes toward a focal point, advancing each non-fixed pitch class one semitone closer along the shorter path on the pitch-class clock. It is NOT an operation (neither 1-to-1 nor onto), making it a key example of INJ's power beyond IFUNC.

# Core Definition
Example 6.2.3 (Lewin, pp. 155-156): The wedge-to-E transformation w^E maps E to E and Bb to Bb (the two fixed points). Every other pitch class advances one semitone toward E along the shorter clockwise or counterclockwise route. w^E is not an operation: it is neither 1-to-1 (e.g., w^E(E) = w^E(F) = E) nor onto (no pitch class maps to F). For wedge-to-u: w^u = T_n * w^E * T_n^{-1} where n = u - E.

# Prerequisites
- **INJ Function** — Wedge transformations require INJ since they are not operations
- **Transformation** — w^E is a transformation (not an operation) on S

# Key Properties
1. Two fixed points: the focal point (E) and its tritone complement (Bb)
2. Not 1-to-1: adjacent pitch classes on either side of E converge
3. Not onto: the semitone neighbors of E (Eb and F) have no preimage
4. Conjugation relates wedges: w^u = T_n * w^E * T_n^{-1}
5. INJ handles wedges naturally; IFUNC and interval-based methods cannot

# Construction / Recognition
## To Construct w^u:
1. Fix focal point u on the pitch-class clock
2. u maps to u; the tritone of u maps to itself
3. All other pitch classes move one semitone toward u along the shorter route

## To Recognize:
1. A transformation where pitch classes converge toward a focal point

# Context & Application
Wedge transformations model voice-leading convergence in post-tonal music. In Schoenberg's "Angst und Hoffen" and "Die Kreuze," wedges structure harmonic progressions and give the focal point a quasi-tonic character. The wedge's non-operation status demonstrates why INJ is needed beyond IFUNC.

# Examples
**Example 1** (p. 156, Figure 6.1): w^E maps: D->Eb, C#->D, C->C#, B->Bb, A->Bb, Ab->A, G->Ab, F#->G; E->E, F->E, Bb->Bb.

**Example 2** (pp. 164-165, Figure 6.6): In "Die Kreuze," w^{C/C#} and w^{F#/G} are wedges converging to dyads rather than single points.

# Relationships
## Builds Upon
- **INJ Function** — Required because wedges are not operations

## Enables
- **Angst und Hoffen Analysis** — w^E is the progressive transformation
- **System Modulation** — w^{F#} = T_2 * w^E * T_2^{-1}

## Related
- **Progressive Transformation** — Wedges often function as progressive transformations
- **Die Kreuze Analysis** — Uses dyadic wedge transformations

## Contrasts With
- **Inversion Operation** — Inversions are operations; wedges are not
- **Transposition Operation** — Transpositions are bijective; wedges are not

# Common Errors
- **Error**: Treating wedge transformations as operations (trying to find an inverse)
  **Correction**: Wedges have no inverse; they are many-to-one and not onto

# Common Confusions
- **Confusion**: Thinking wedges are just "approximate inversions"
  **Clarification**: Wedges model convergent voice-leading, not pitch inversion. They interact with inversions but serve different analytical functions.

# Source Reference
Chapter 6: Generalized Set Theory (2), Example 6.2.3, Figures 6.1-6.3, 6.6, pp. 155-165.

# Verification Notes
- Definition source: Direct from Example 6.2.3
- Confidence rationale: Detailed definition with multiple analytical applications
- Re-extraction notes: Re-extracted from v2 card; preserved: explicit w^E mapping, "Angst und Hoffen" examples, non-operation emphasis. Added Die Kreuze reference, v3.1 structure.
