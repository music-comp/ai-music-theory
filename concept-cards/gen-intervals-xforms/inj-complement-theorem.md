---
concept: INJ Complement Theorem
slug: inj-complement-theorem

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.6.1"

extraction_confidence: high

aliases:
  - "Theorem 6.6.1"
  - Generalized Babbitt Hexachord Theorem

prerequisites:
  - inj-function
  - inj-operation-theorem
extends:
  - inj-function
related:
  - inj-generalizes-ifunc
contrasts_with: []

answers_questions:
  - "How does INJ relate to set complementation?"
  - "What is the Generalized Babbitt Hexachord Theorem?"
---

# Quick Definition
When S is finite, INJ values for a set and its complement are related by specific formulas. Formula (E), the Generalized Babbitt Hexachord Theorem, states that if card(X) = 1/2 card(S), then INJ(X, X)(OP) = INJ(complement(X), complement(X))(OP) for any operation OP.

# Core Definition
Theorem 6.6.1 (Lewin, pp. 176-177): For finite S, sets X, Y with complements, and any operation OP:
- (A): INJ(X, complement(Y))(OP) = card(X) - INJ(X, Y)(OP)
- (B): INJ(X, complement(Y))(OP) = card(complement(Y)) - INJ(X, Y)(OP)
- (C): INJ(complement(X), complement(Y))(OP) = card(complement(Y)) - card(X) + INJ(X, Y)(OP)
- (D): If card(Y) = card(X), then INJ(complement(X), complement(Y))(OP) = INJ(X, Y)(OP)
- (E): If card(X) = 1/2 card(S), then INJ(X, X)(OP) = INJ(complement(X), complement(X))(OP)

# Prerequisites
- **INJ Function** — The function being analyzed under complementation
- **INJ Operation Theorem** — Required for proofs of (B) and beyond

# Key Properties
1. Formula (A) works for any transformation f, not just operations
2. Formulas (B)-(E) require OP to be an operation
3. S must be finite for complements to be "sets"
4. Formula (E) generalizes Babbitt's hexachord theorem to any operation, any finite S
5. Setting OP = T_i and using Theorem 6.7.1: IFUNC(X, X)(i) = IFUNC(complement(X), complement(X))(i)

# Construction / Recognition
## To Apply:
1. Verify S is finite and OP is an operation
2. Choose the appropriate formula (A)-(E) based on cardinality conditions
3. Compute INJ for the complement using the formula

## To Recognize:
1. When complementary sets share structural properties (e.g., identical interval vectors)

# Context & Application
The Generalized Hexachord Theorem (E) explains why complementary hexachords have the same interval vector — but goes far beyond: it works for any operation (not just transpositions), any set that is half the size of S, and any finite S. In PROT (protocol pairs), a row and its retrograde are complements, so the theorem applies to row/retrograde symmetries.

# Examples
**Example 1** (p. 177): Babbitt's Hexachord Theorem as special case: X = hexachord, OP = T_i. INJ(X, X)(T_i) = INJ(complement(X), complement(X))(T_i), which via Theorem 6.7.1 gives IFUNC(X, X)(i) = IFUNC(complement(X), complement(X))(i).

**Example 2** (pp. 177-178): In PROT, a row L has 66 pairs, complement(L) (the retrograde) also has 66 pairs. Theorem 6.6.1(E) tells us INJ(L, L)(OP) = INJ(complement(L), complement(L))(OP) for any operation on PROT.

# Relationships
## Builds Upon
- **INJ Function** and **INJ Operation Theorem** — Required for proofs

## Enables
- **Hexachord Theorem** — Special case of formula (E)

# Common Errors
- **Error**: Applying formulas (B)-(E) to non-operations
  **Correction**: Only formula (A) works for general transformations

# Common Confusions
- **Confusion**: Thinking this only applies to hexachords
  **Clarification**: It applies to any set that is half the size of S, in any finite space

# Source Reference
Chapter 6: Generalized Set Theory (2), Theorem 6.6.1 and Example 6.6.2, pp. 176-178.

# Verification Notes
- Definition source: Direct from Theorem 6.6.1
- Confidence rationale: Explicit theorem with proofs
- Re-extraction notes: Re-extracted from v2 card; preserved: Babbitt connection, PROT example. Added v3.1 structure.
