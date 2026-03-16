---
concept: Internal Transformation
slug: internal-transformation

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.4"

extraction_confidence: high

aliases:
  - X-internal transformation
  - Y-internal transformation

prerequisites:
  - inj-function
extends: []
related:
  - progressive-transformation
  - external-transformation
  - dispersive-transformation
  - transitivity-sets
contrasts_with:
  - progressive-transformation
  - external-transformation

answers_questions:
  - "What distinguishes internal from progressive transformations?"
  - "What is an internal transformation?"
---

# Quick Definition
An internal transformation f is one for which INJ(X, X)(f) or INJ(Y, Y)(f) is high — it maps a set largely into itself, modeling self-reference, symmetry, or prolongation.

# Core Definition
Section 6.4 (Lewin, pp. 172-174): "For certain transformations f within the family INSPECT, the value of INJ(X, X)(f) or INJ(Y, Y)(f) will be high. We shall call these transformations X-internal or Y-internal accordingly." A transformation both X-internal and Y-internal is simply "internal" for the progression X-Y. "An X-internal transformation tends to extend/elaborate/develop/prolong X in the music."

# Prerequisites
- **INJ Function** — Internal is defined via INJ(X, X)(f) values

# Key Properties
1. f is X-internal if INJ(X, X)(f) is high
2. f is Y-internal if INJ(Y, Y)(f) is high
3. Composition of two X-internal transformations tends to be X-internal
4. Inverse of an X-internal operation tends to be X-internal
5. The identity is maximally internal for every set

# Construction / Recognition
## To Identify:
1. Compute INJ(X, X)(f) for each f in INSPECT
2. Those with highest values are X-internal

## To Recognize:
1. A transformation that preserves a chord's identity (maps it to something like itself)

# Context & Application
Internal transformations model inversional symmetry, prolongation, and self-reference. In "Angst und Hoffen," I = I_E^{Bb} is internal: INJ(X, X)(I) = 3 (all of Angst maps to itself) and INJ(Y, Y)(I) = 2. The "missing F" breaks Y's full inversional self-symmetry.

# Examples
**Example 1** (pp. 157-158): I is internal for Angst/Hoffen: INJ(X, X)(I) = 3, INJ(Y, Y)(I) = 2. The Fb of Y prevents full I-symmetry.

**Example 2** (p. 163): In the melodic analysis, (1, I) and (2, w) are internal for tetrad X_1^4; (2, I) and (3, w) are internal for tetrad X_5^8. No I or w arrows lead between tetrads.

# Relationships
## Builds Upon
- **INJ Function** — Defined through INJ values

## Enables
- **Transitivity Sets** — I-partnerships within chords are transitivity sets of internal operations

## Contrasts With
- **Progressive Transformation** — Progressive maps X toward Y; internal keeps X like itself
- **External Transformation** — External maps X outside itself; internal keeps X within itself

# Common Errors
- **Error**: Thinking internal means "trivial" or "identity"
  **Correction**: Non-identity transformations can be highly internal for symmetric sets

# Common Confusions
- **Confusion**: Assuming a transformation is either internal or progressive
  **Clarification**: A transformation can be Y-internal and X-Y-progressive simultaneously

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.4, pp. 172-174.

# Verification Notes
- Definition source: Direct from section 6.4
- Confidence rationale: Explicit definition with musical examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Angst/Hoffen examples, algebraic tendencies. Added v3.1 structure.
