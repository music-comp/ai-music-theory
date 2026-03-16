---
# === CORE IDENTIFICATION ===
concept: "Protocol Pairs (PROT)"
slug: protocol-pairs

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
  - PROT

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
extends: []
related:
  - partial-ordering
  - signature-motive
  - semi-simple-variations-analysis
  - inj-complement-theorem
contrasts_with:
  - ordinal-pitch-pairs

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are protocol pairs and how do they model twelve-tone rows?"
---

# Quick Definition
PROT is the space of ordered pairs (p, q) of distinct pitch classes (132 total). A twelve-tone row is modeled as a subset of PROT containing all precedence relations: (p, q) is in the set iff p precedes q in the row. This treats all rows as conceptually equal.

# Core Definition
Example 6.2.4 (Lewin, pp. 166-167): "A protocol pair is an ordered pair (p, q) of distinct chromatic pitch classes. There are thus 132 = 12 times 11 protocol pairs." A row L contains 66 pairs (11+10+...+1). Operations: T_i(p,q) = (T_i(p), T_i(q)); I(p,q) = (I(p), I(q)); R(p,q) = (q,p). The retrograde R(L) = complement of L in PROT, connecting row/retrograde to set/complement duality.

# Prerequisites
- **INJ Function** — PROT is a space on which INJ operates

# Key Properties
1. |PROT| = 132; each row has 66 pairs; its retrograde (complement) also has 66
2. Row and retrograde are complementary subsets of PROT
3. Operations T_i, I, R, and combinations form a group isomorphic to the twelve-tone group
4. No a priori ordering assumed — all rows are conceptually equal
5. Partial orderings (subsets satisfying PO1, PO2 but not SIMP) model incomplete serial structures

# Construction / Recognition
## To Construct a Row as Subset of PROT:
1. Given row p_1, p_2, ..., p_12
2. Include pair (p_i, p_j) whenever i < j
3. Result is a 66-element subset of PROT

## To Recognize:
1. A 66-element subset of PROT satisfying PO1, PO2, and SIMP

# Context & Application
This model enables applying INJ set-theoretic techniques to serial analysis. INJ(L, X)(f) measures how well partial ordering X fits within row form f(L). The complement relationship (row = complement of retrograde) parallels hexachord/complement in pitch-class theory, and the Generalized Hexachord Theorem (6.6.1E) applies.

# Examples
**Example 1** (p. 167): Schoenberg's Fourth Quartet row D-C#-A-Bb-...: contains pairs (D, C#), (D, A), (C#, A), (A, Bb), etc.

**Example 2** (pp. 168-169): Moses und Aron row L_1 with motive X_1 = E-A-Bb: INJ(L_1, X_1)(J) = 3 uniquely identifies J(L_1) among inverted forms.

# Relationships
## Builds Upon
- **INJ Function** — INJ on PROT space

## Enables
- **Partial Ordering** — Generalizes rows to incomplete orderings
- **Signature Motive** — INJ on PROT identifies row forms
- **INJ Complement Theorem** — Row/retrograde as complement sets

# Common Errors
- **Error**: Confusing protocol pairs with ordered pitch-class pairs in other contexts
  **Correction**: PROT specifically models precedence relations for serial analysis

# Common Confusions
- **Confusion**: Thinking this model privileges one row over others
  **Clarification**: No a priori ordering is assumed; "any row orders [the pitch classes] as well as any other row"

# Source Reference
Chapter 6: Generalized Set Theory (2), Example 6.2.4, pp. 166-170.

# Verification Notes
- Definition source: Direct from Example 6.2.4
- Confidence rationale: Explicit definition with multiple analytical applications
- Re-extraction notes: Re-extracted from v2 card; preserved: cardinality 132/66, retrograde as complement, Moses und Aron reference. Added v3.1 structure.
