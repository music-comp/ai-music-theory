---
concept: IFUNC Symmetry Theorem
slug: ifunc-symmetry-theorem

category: generalized-set-theory
subcategory: interval-functions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.1.4"

extraction_confidence: high

aliases:
  - "Theorem 5.1.4"
  - IFUNC exchange theorem

prerequisites:
  - ifunc
  - generalized-interval-system
extends:
  - ifunc
related:
  - ifunc-transposition-theorem
  - ifunc-inversion-theorem
contrasts_with: []

answers_questions:
  - "What happens to IFUNC when the roles of sets X and Y are exchanged?"
  - "How does IFUNC(Y, X) relate to IFUNC(X, Y)?"
---

# Quick Definition
When the roles of sets X and Y are exchanged in IFUNC, the resulting function is "inverted": IFUNC(Y, X)(i) = IFUNC(X, Y)(i^{-1}).

# Core Definition
Theorem 5.1.4: "IFUNC(Y, X)(i) = IFUNC(X, Y)(i^{-1})" (Lewin, p. 130). The proof observes that IFUNC(X, Y)(i^{-1}) counts pairs (s, t) with s in X, t in Y, int(s, t) = i^{-1}, which equals the number of pairs (t, s) with t in Y, s in X, int(t, s) = i — precisely IFUNC(Y, X)(i).

# Prerequisites
- **IFUNC** — This theorem describes a structural property of the interval function
- **Generalized Interval System** — Uses the group inverse operation in IVLS

# Key Properties
1. Exchanging X and Y inverts the interval argument
2. In a commutative GIS with additive notation: IFUNC(Y, X)(i) = IFUNC(X, Y)(-i)
3. The total count is preserved: sum of IFUNC(X, Y)(i) over all i equals card(X) * card(Y) regardless of argument order
4. IFUNC(X, Y) = IFUNC(Y, X) as functions if and only if every interval equals its own inverse

# Construction / Recognition
## To Apply:
1. Given IFUNC(X, Y) for all intervals i
2. To find IFUNC(Y, X)(i), look up IFUNC(X, Y)(i^{-1})
3. In pitch-class space (mod 12), i^{-1} = 12 - i

## To Recognize:
1. Any situation where IFUNC arguments are reversed can be simplified using this theorem

# Context & Application
This theorem is fundamental to understanding the directional nature of IFUNC. It shows that the intervallic relationship from X to Y and from Y to X are systematically related through group inversion, not generally equal. In pitch-class terms, if there are N ways to span interval i from X to Y, there are N ways to span interval 12-i from Y to X.

# Examples
**Example 1** (derived from p. 120): In the standard pitch-class GIS with X = {C, E} and Y = {G, B}: IFUNC(X, Y)(7) = 2 (two perfect fifths: C->G, E->B). Therefore IFUNC(Y, X)(5) = 2 (two perfect fourths: G->C, B->E), since 5 = 12 - 7 is the mod-12 inverse of 7.

# Relationships
## Builds Upon
- **IFUNC** — Describes a structural property of the interval function

## Enables
- **IFUNC Transposition Theorem** — Used in proofs of transposition formulas (5.1.6B)
- **INJ Generalizes IFUNC** — Symmetry carries over to INJ via transpositions

## Related
- **IFUNC Inversion Theorem** — Another structural theorem about IFUNC under transformations

# Common Errors
- **Error**: Assuming IFUNC(Y, X) = IFUNC(X, Y)
  **Correction**: The functions are related by interval inversion, not equality

# Common Confusions
- **Confusion**: Thinking this theorem only applies in commutative groups
  **Clarification**: The theorem holds in any GIS; the inversion i^{-1} is the group inverse in IVLS, commutative or not

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.1.4, p. 130.

# Verification Notes
- Definition source: Direct from Theorem 5.1.4 with proof
- Confidence rationale: Explicit theorem with proof in source
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, core theorem statement, musical intuition about ascending/descending intervals. Added v3.1 structure.
