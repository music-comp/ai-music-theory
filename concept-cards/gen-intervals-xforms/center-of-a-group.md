---
concept: Center of a Group
slug: center-of-a-group

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.8.2"

extraction_confidence: high

aliases:
  - central elements

prerequisites:
  - group
  - commutativity
extends: []
related:
  - commutative-vs-noncommutative-gis
contrasts_with: []

answers_questions:
  - "What distinguishes a commutative GIS from a non-commutative GIS?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

The center of a group consists of all elements that commute with every other element: c is central if cx = xc for all x.

# Core Definition

"Given a binary composition BIN on a family X, an element c of X is central if c commutes with every x in X. The family of all central c is the center of the system (X, BIN)" (Lewin, Definition 1.8.2, p. 36).

# Prerequisites

- **Group** — the center is defined for a group (or semigroup)
- **Commutativity** — centrality is defined in terms of commuting

# Key Properties

1. c is central if cx = xc for all x in X
2. The center is always a subgroup of the group
3. In a commutative group, the center equals the entire group
4. The center always contains at least the identity element
5. Non-commutative groups have proper centers (smaller than the whole group)

# Construction / Recognition

## To Construct:
1. For each element c of the group, check if cx = xc for all x
2. Collect all such c; they form the center

## To Recognize:
1. An element c is central if it commutes with every element of the group
2. If any x exists with cx different from xc, then c is not central

# Context & Application

In the T/I group on pitch classes, the center consists of T0 and T6 (the tritone transposition). These are the only operations that commute with all transpositions and inversions. In GIS theory, central intervals have special properties: transposition by a central interval preserves all intervals.

# Examples

**Example 1**: In the T/I group, T0 (identity) is central: T0X = XT0 = X for all X.

**Example 2**: In the T/I group, T6 is central: T6 commutes with all transpositions and all inversions.

**Example 3**: In the T/I group, T3 is NOT central: T3I0 = I3 but I0T3 = I9, so T3I0 differs from I0T3.

**Example 4**: In a commutative group like (Z12, +), every element is central.

# Relationships

## Builds Upon
- **Group** — center is defined within a group
- **Commutativity** — the center collects commuting elements

## Enables
- **Commutative vs. Non-Commutative GIS** — central intervals determine which GIS simplifications apply

# Common Errors

- **Error**: Thinking the center can be empty.
  **Correction**: Every group has at least the identity in its center.

# Common Confusions

- **Confusion**: Confusing "center" (algebraic) with "center" (spatial/musical).
  **Clarification**: The center of a group is the set of elements commuting with everything, not a "middle" element.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.8.2, p. 36.

# Verification Notes

- Definition source: direct from Definition 1.8.2
- Confidence rationale: explicit definition in source
- Re-extracted from v2 card; preserved: T/I center examples (T0, T6), T3 non-centrality example
