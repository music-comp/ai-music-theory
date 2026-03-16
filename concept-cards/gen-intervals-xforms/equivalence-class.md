---
# === CORE IDENTIFICATION ===
concept: Equivalence Class
slug: equivalence-class

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: equivalence-relations
tier: foundational

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.9.3-1.9.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - equivalence-relation
extends: []
related:
  - quotient-set
  - natural-map
  - congruence
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An equivalence class is the collection of all elements equivalent to a given element under an equivalence relation -- all elements that "go together."

# Core Definition

"Given an equivalence relation EQUIV on a family S, for each s in S let E(s) be the subfamily of S comprising exactly those members of S which are in the EQUIV relation to s." Then for any s and t in S, either (A) s and t are equivalent and E(s) = E(t) are the same collection, or (B) s and t are not equivalent and E(s) and E(t) are disjoint (Lewin, Theorem 1.9.3, p. 37). The sets E(s) are the equivalence classes. An equivalence relation partitions S into mutually disjoint equivalence classes; each element of S belongs to exactly one class (Section 1.9.4).

# Prerequisites

- **Equivalence Relation** — equivalence classes are defined by an equivalence relation

# Key Properties

1. E(s) = {t in S : s ~ t} is the equivalence class containing s
2. If s ~ t, then E(s) = E(t) (same class)
3. If s is not equivalent to t, then E(s) and E(t) are disjoint
4. S is partitioned into disjoint equivalence classes
5. Every element belongs to exactly one equivalence class

# Construction / Recognition

## To Construct:
1. Fix an equivalence relation on S
2. For a given element s, collect all t such that s ~ t
3. This collection is the equivalence class E(s)

## To Recognize:
1. Verify that all elements in the collection are mutually equivalent
2. Verify that no element outside the collection is equivalent to any element inside

# Context & Application

Pitch classes are equivalence classes of pitches under octave equivalence. Set-types are equivalence classes of pitch-class sets under transposition/inversion. Beat classes are equivalence classes of time points under metric equivalence. Equivalence classes allow us to work with abstract categories rather than specific instances.

# Examples

**Example 1** (1.9.6.1, p. 37): The pitch class C is the equivalence class containing C4, C5, C3, C6, etc. -- all pitches with the letter name C.

**Example 2** (1.9.6.2, p. 37): In a waltz, beat-class 1 contains all first beats, beat-class 2 all second beats, beat-class 3 all third beats.

**Example 3** (1.9.6.3, p. 37): Set-type 3-11 is the equivalence class containing all major and minor triads (24 total). Set-type 3-12 contains the 4 augmented triads.

# Relationships

## Builds Upon
- **Equivalence Relation** — classes are defined by the relation

## Enables
- **Quotient Set** — the family of equivalence classes forms the quotient set
- **Natural Map** — maps each element to its class

# Common Errors

- **Error**: Thinking two different equivalence classes can overlap.
  **Correction**: Theorem 1.9.3 proves that equivalence classes are either identical or disjoint.

# Common Confusions

- **Confusion**: Confusing an element with its class.
  **Clarification**: The class is the entire collection, not just a representative. E(C4) is the whole pitch class C (containing C4, C5, C3, ...), not just the pitch C4.

# Source Reference

Chapter 1: Mathematical Preliminaries, Theorem 1.9.3, Section 1.9.4, pp. 37-38.

# Verification Notes

- Definition source: direct from Theorem 1.9.3 and Section 1.9.4
- Confidence rationale: explicit theorem with proof in source
- Re-extracted from v2 card; preserved: pitch class, beat class, and set-type examples
