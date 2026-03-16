---
# === CORE IDENTIFICATION ===
concept: Signature Motive
slug: signature-motive

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
  - uniquely characteristic motive

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
  - protocol-pairs
  - partial-ordering
extends: []
related:
  - semi-simple-variations-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a signature motive in twelve-tone theory?"
---

# Quick Definition
A signature motive is a small partial ordering (melodic fragment) that uniquely identifies a specific row form among a family of forms, having high "signature value" measured by INJ. If you hear the motive embedded in some row form, it identifies which form is at hand.

# Core Definition
Lewin (pp. 168-170): X has high signature value for f(L) if INJ(L, X)(f) = card(X) while INJ(L, X)(g) < card(X) for other operations g. Babbitt describes such motives as "uniquely characteristic" of a row form within a subarray. INJ provides the formal measure.

# Prerequisites
- **INJ Function** — Measures signature value
- **Protocol Pairs** — Motives modeled as subsets of PROT
- **Partial Ordering** — Motives are small partial orderings

# Key Properties
1. INJ(L, X)(f) = card(X) means X is fully embedded in f(L)
2. Uniqueness among a family means only one form achieves the maximum
3. Can be a signature among transposed forms, inverted forms, or all 48 forms
4. Noted by Michael Cherlin for Moses und Aron and by Babbitt for his own music

# Construction / Recognition
## To Identify:
1. Compute INJ(L, X)(f) for all operations f in the family
2. If exactly one f achieves the maximum, X is a signature for that form

## To Recognize:
1. A short melodic fragment that uniquely identifies a row form

# Context & Application
Signature motives allow economical identification of row forms in performance and listening. Composers may use them for thematic clarity at structural moments. Cherlin identified Moses's E-A-Bb as a signature for J(L_1) in Moses und Aron; Babbitt discussed B-D-A as a signature for his Reflections row.

# Examples
**Example 1** (pp. 168-169): Moses und Aron — X_1 = E-A-Bb. INJ(L_1, X_1)(J) = 3 uniquely among inversions. Moses enters with E-A-Bb on loud trombone, identifying J(L_1).

**Example 2** (p. 170): Babbitt's Reflections — X = B-D-A is a signature for L among transposed forms and for J(L) among inverted forms. Y = B-D-A-Db is a signature for L among all 48 forms.

# Relationships
## Builds Upon
- **INJ Function** — Formal measure of signature value
- **Protocol Pairs** — Framework for the analysis

## Related
- **Semi-Simple Variations Analysis** — Uses INJ on PROT for aggregate analysis

# Common Errors
- **Error**: Confusing signature motives with pitch-class set classes
  **Correction**: Signature motives are ordered (partial orderings), not unordered sets

# Common Confusions
- **Confusion**: Thinking any short motive is a signature
  **Clarification**: Only motives with unique INJ maximum are signatures; many short motives embed in multiple row forms

# Source Reference
Chapter 6: Generalized Set Theory (2), Example 6.2.4 (Moses und Aron and Babbitt discussions), pp. 168-170.

# Verification Notes
- Definition source: Synthesized from Cherlin's and Babbitt's observations formalized by Lewin
- Confidence rationale: Detailed discussion with two specific analytical applications
- Re-extraction notes: Re-extracted from v2 card; preserved: Moses entrance, Reflections example. Added v3.1 structure.
