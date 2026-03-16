---
# === CORE IDENTIFICATION ===
concept: Weak Condition B
slug: weak-condition-b

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: foundational-definitions
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Discussion following Definition 2.3.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "(weak B)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - gis-condition-b
  - equivalence-relation
  - quotient-set
extends: []
related:
  - simply-transitive-action
  - musical-space-s
contrasts_with:
  - gis-condition-b

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens if we weaken Condition (B) by replacing 'unique' with 'some'?"
  - "Why is the full Condition (B) 'no loss of generality'?"
  - "How does (weak B) lead to equivalence classes and a quotient GIS?"
---

# Quick Definition
Weak Condition B replaces "a unique t" with "some t" in Condition (B) of the GIS definition. Under this weakened condition, the space S is partitioned into equivalence classes, and the quotient space S/EQUIV forms a true GIS satisfying full Condition (B) -- showing the uniqueness requirement is "no loss of generality."

# Core Definition
"We might consider weakening the condition, replacing the words 'a unique,' where they appear in 2.3.1(B), with the word 'some.' Let us call the weakened condition '(weak B).' ... Under condition (weak B), the space S would be partitioned into equivalence classes: s and s' would be equivalent if and only if int(s, s') = e. Given s' equivalent to s and t' equivalent to t, it would be true that int(s', t') = int(s, t). We could thus think of the intervals as being from one equivalence class to another. We could replace S by the quotient family S/EQUIV, the family of equivalence classes, and obtain a GIS thereby" (Lewin, pp. 47-48).

# Prerequisites
- **Generalized Interval System** — (weak B) is a hypothetical weakening of its Condition (B)
- **GIS Condition B** — The full condition requiring unique t
- **Equivalence relation** — s ~ s' iff int(s, s') = e partitions S
- **Quotient set** — S/EQUIV gives the reduced space

# Key Properties
1. Condition (B): there exists a unique t with int(s, t) = i
2. (Weak B): there exists some t with int(s, t) = i (not necessarily unique)
3. Under (weak B): define s ~ s' iff int(s, s') = e (identity)
4. This is an equivalence relation (reflexive, symmetric, transitive)
5. Intervals are well-defined on equivalence classes: int(s', t') = int(s, t) when s ~ s' and t ~ t'
6. The quotient space S/EQUIV with induced int satisfies full Condition (B)

# Construction / Recognition
## To Construct:
1. Start with a space satisfying only (weak B)
2. Define equivalence: s ~ s' iff int(s, s') = e
3. Form the quotient S/EQUIV
4. Define int on equivalence classes; this gives a true GIS
## To Recognize:
1. A space where multiple elements have the same interval relationships (redundancy)
2. Elements that are "at the same position" with respect to all intervals

# Context & Application
Lewin argues that weakening Condition (B) does not gain genuine generality: "It is hard to see what we could possibly want to do with S that we could not do as well or better with the reduced space S/EQUIV of equivalence classes" (p. 48). The discussion justifies requiring uniqueness in Condition (B) -- it eliminates redundant elements and is mathematically cleaner without sacrificing generality.

# Examples
**Example 1** (hypothetical, following Lewin's logic): Suppose S contains both "C4 played on piano" and "C4 played on violin" as distinct elements, but intervals measure only pitch, so int(C4-piano, C4-violin) = e. Then S satisfies only (weak B). The quotient collapses these to a single pitch C4, recovering full Condition (B).

# Relationships
## Builds Upon
- **GIS Condition B** — (weak B) is its hypothetical weakening
- **Equivalence relation** — Used to partition redundant elements
## Enables
- **Quotient GIS** — The quotient S/EQUIV satisfies full Condition (B)
## Contrasts With
- **GIS Condition B** — Full (B) requires uniqueness; (weak B) does not

# Common Errors
- **Error**: Using (weak B) in place of full Condition (B) in the GIS definition
  **Correction**: (weak B) is not part of the GIS definition; it is a hypothetical weakening discussed to justify the uniqueness requirement

# Common Confusions
- **Confusion**: Thinking (weak B) provides greater generality than full (B)
  **Clarification**: The quotient construction shows full (B) is "no loss of generality" -- any (weak B) space reduces to a true GIS on equivalence classes

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, discussion following Definition 2.3.1, pages 47-48.

# Verification Notes
- Definition source: Direct quotation from Lewin's discussion
- Confidence rationale: Explicitly discussed by Lewin to justify the GIS definition
- Re-extraction notes: Re-extracted from v2 card; preserved: quotient construction, "no loss of generality" argument, Lewin quotation
