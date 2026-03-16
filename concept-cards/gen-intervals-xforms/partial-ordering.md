---
# === CORE IDENTIFICATION ===
concept: Partial Ordering in Serial Theory
slug: partial-ordering

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
section: "6.2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - strict partial ordering

# === TYPED RELATIONSHIPS ===
prerequisites:
  - protocol-pairs
extends:
  - protocol-pairs
related:
  - signature-motive
  - semi-simple-variations-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a partial ordering in serial theory?"
  - "How do partial orderings model incomplete serial structures?"
---

# Quick Definition
A partial ordering is a subset of PROT satisfying antisymmetry (PO1) and transitivity (PO2) but not necessarily totality (SIMP). It models "knowing some precedence relations while being uncertain about others" — useful for aggregates and motivic fragments.

# Core Definition
Example 6.2.4 (Lewin, p. 167): A subset X of PROT is a (strict) partial ordering if: (PO1) X never contains both (p, q) and (q, p); (PO2) if (p, q) and (q, r) are in X, then so is (p, r). Linear orderings (rows) additionally satisfy (SIMP): for any distinct p, q, either (p, q) or (q, p) is in X.

# Prerequisites
- **Protocol Pairs** — Partial orderings are subsets of PROT

# Key Properties
1. Partial orderings model incomplete serial structures
2. Rows (linear orderings) are the special case satisfying SIMP
3. SATB aggregates can be modeled as partial orderings (each voice ordered, cross-voice unspecified)
4. INJ(L, X)(f) measures compatibility of partial ordering X with row form f(L)
5. A motive E-A-Bb is a partial ordering containing 3 pairs

# Construction / Recognition
## To Construct:
1. Identify the known precedence relations among pitch classes
2. Include all pairs (p, q) where p is known to precede q
3. Verify PO1 (no contradictions) and PO2 (transitivity)

## To Recognize:
1. A subset of PROT with no contradictory orderings and transitive closure

# Context & Application
Partial orderings model aggregate structures in Babbitt's music, motivic fragments in Schoenberg, and any situation where serial ordering is partially specified. They enable INJ analysis without requiring a complete row.

# Examples
**Example 1** (p. 167): X_1 = E-A-Bb (motive) = {(E,A), (E,Bb), (A,Bb)}: 3 pairs, a linear ordering on 3 elements.

**Example 2** (pp. 167-168, Figure 6.7): X_2 = SATB aggregate from Semi-Simple Variations: 12 pairs (3 per voice), partial ordering (not linear — cross-voice ordering unspecified).

# Relationships
## Builds Upon
- **Protocol Pairs** — Partial orderings are subsets of PROT

## Enables
- **Signature Motive** — Small partial orderings can identify row forms
- **Semi-Simple Variations Analysis** — Aggregate partial orderings analyzed via INJ

# Common Errors
- **Error**: Thinking a partial ordering determines a unique row
  **Correction**: Many rows may be compatible with a given partial ordering

# Common Confusions
- **Confusion**: Conflating partial ordering with "incomplete row"
  **Clarification**: A partial ordering captures known precedence without implying any particular completion

# Source Reference
Chapter 6: Generalized Set Theory (2), Example 6.2.4, pp. 167-168.

# Verification Notes
- Definition source: Direct from Example 6.2.4
- Confidence rationale: Explicit definition with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: E-A-Bb example, SATB aggregate example. Added v3.1 structure.
