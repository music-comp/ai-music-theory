---
# === CORE IDENTIFICATION ===
concept: GIS Theorem 2.3.2
slug: gis-theorem-2-3-2

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: core-definitions
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.3.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - self-interval and inverse-interval theorem

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - gis-condition-a
  - group
extends: []
related:
  - identity-element
  - inverse-element
  - directed-interval
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does the interval function int relate to the group IVLS?"
  - "What is a Generalized Interval System (GIS)?"
---

# Quick Definition

Theorem 2.3.2 proves that in any GIS, int(s, s) = e (the identity interval) and int(t, s) = int(s, t)^(-1) (reversing direction inverts the interval) -- these follow from Condition (A) and group structure alone.

# Core Definition

"In any GIS, int(s, s) = e and int(t, s) = int(s, t)^(-1) for every s and t in S" (Lewin, Theorem 2.3.2, p. 52).

**Proof of int(s, s) = e:** int(s, s)int(s, s) = int(s, s), via Condition (A). Multiply both sides by int(s, s)^(-1); we obtain int(s, s) = e.

**Proof of int(t, s) = int(s, t)^(-1):** int(s, t)int(t, s) = int(s, s) = e, via Condition (A). Multiply both sides on the left by int(s, t)^(-1); we obtain int(t, s) = int(s, t)^(-1).

# Prerequisites

- **Generalized Interval System** — the theorem applies to any GIS
- **GIS Condition A** — the proof uses Condition (A)
- **Group** — the proof uses group inverses

# Key Properties

1. int(s, s) = e: the interval from any element to itself is the identity
2. int(t, s) = int(s, t)^(-1): reversing direction inverts the interval
3. These are theorems, not axioms: they need not be stated in the GIS definition
4. The proofs use only Condition (A) and the group structure of IVLS
5. These properties confirm our musical intuitions: unison is the identity; reversing direction inverts the interval

# Construction / Recognition

## To Verify:
1. These hold automatically in any GIS; no separate verification needed

# Context & Application

This theorem confirms two basic musical intuitions: (1) the interval from any note to itself is trivially "zero" (the identity), and (2) the interval from t back to s is the reverse (inverse) of the interval from s to t. Going up a fifth inverts to going down a fifth. The elegance of the theorem is that these properties need not be stipulated separately -- they follow from the GIS axioms.

# Examples

**Example 1**: Chromatic pitch: int(C4, C4) = 0 (identity). int(C4, G4) = 7 and int(G4, C4) = -7. Check: 7 + (-7) = 0.

**Example 2**: Pitch-class mod 12: int(C, C) = 0. int(C, G) = 7 and int(G, C) = 5. Check: 7 + 5 = 12 = 0 mod 12.

**Example 3**: Just intonation: int(C4, C4) = 1 (multiplicative identity). int(C4, G4) = 3/2 and int(G4, C4) = 2/3. Check: (3/2)(2/3) = 1.

# Relationships

## Builds Upon
- **GIS Condition A** — the proof derives from Condition (A)
- **Group** — uses group inverse operation

## Related
- **Identity Element** — int(s, s) = e connects to the abstract identity
- **Inverse Element** — int(t, s) = int(s, t)^(-1) connects to the abstract inverse
- **Directed Interval** — formalizes the direction-reversal property

# Common Errors

- **Error**: Stating int(s, s) = e as a separate axiom of the GIS.
  **Correction**: It is a theorem, not an axiom. It follows from Condition (A) and group structure.

# Common Confusions

- **Confusion**: Confusing this theorem with Condition (B).
  **Clarification**: Condition (B) is about existence/uniqueness of elements at given intervals. Theorem 2.3.2 is about the algebraic behavior of the int function.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Theorem 2.3.2 with proof, pp. 52-53.

# Verification Notes

- Definition source: direct theorem statement and proof from the source
- Confidence rationale: explicit theorem with complete proof provided
- Re-extracted from v2 card; preserved: all three domain examples (chromatic, mod 12, just intonation), complete proof steps
