---
concept: Commutativity
slug: commutativity

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.8.1"

extraction_confidence: high

aliases:
  - abelian property
  - commutative property

prerequisites:
  - binary-composition
extends: []
related:
  - group
  - center-of-a-group
  - commutative-vs-noncommutative-gis
contrasts_with:
  - associativity

answers_questions:
  - "What distinguishes a commutative GIS from a non-commutative GIS?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

Elements x and y commute if xy = yx; a group or binary composition is commutative (abelian) if all pairs of elements commute.

# Core Definition

"Given a binary composition BIN on a family X, elements x and y commute if BIN(y, x) = BIN(x, y), that is, if yx = xy in multiplicative notation. The composition BIN is commutative if every pair of elements commutes. A semigroup or group is commutative if its binary composition is commutative" (Lewin, Definition 1.8.1, p. 36).

# Prerequisites

- **Binary Composition** — commutativity is a property of a binary composition

# Key Properties

1. x and y commute if xy = yx
2. A commutative group is also called an abelian group
3. A non-commutative group has at least one pair that does not commute
4. Commutativity and associativity are independent properties
5. In a commutative group, the distinction between homomorphisms and anti-homomorphisms vanishes

# Construction / Recognition

## To Construct:
1. Not directly constructed; commutativity is a property of an existing composition

## To Recognize:
1. Check whether xy = yx for all pairs x, y
2. Finding any pair with xy different from yx proves non-commutativity

# Context & Application

Whether a group is commutative has profound consequences for GIS theory. In a commutative GIS, transpositions equal interval-preserving operations, and many simplifications apply. In a non-commutative GIS (such as the time-span GIS), these simplifications fail and more care is needed.

# Examples

**Example 1** (p. 36): The group of transposition and inversion operations on the twelve pitch classes is non-commutative. Let T2 be transposing-by-2, I be inverting-about-C, J be inverting-about-B, and K be inverting-about-C#. Then IT2 = J but T2I = K, so T2 and I do not commute.

**Example 2**: The group of transpositions alone (T0 through T11) IS commutative: TmTn = TnTm = Tm+n for all m, n.

**Example 3**: The integers mod 12 under addition are commutative: m + n = n + m.

# Relationships

## Builds Upon
- **Binary Composition** — commutativity is a property of binary compositions

## Enables
- **Center of a Group** — the center collects all elements that commute with everything
- **Commutative vs. Non-Commutative GIS** — determines which GIS theorems simplify

## Contrasts With
- **Associativity** — associativity concerns grouping; commutativity concerns order

# Common Errors

- **Error**: Assuming that because transpositions commute, all operations commute.
  **Correction**: The T/I group is non-commutative even though the transposition subgroup is commutative.

# Common Confusions

- **Confusion**: Conflating "abelian" and "commutative."
  **Clarification**: For groups, "abelian" and "commutative" mean exactly the same thing.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.8.1, p. 36.

# Verification Notes

- Definition source: direct from Definition 1.8.1
- Confidence rationale: explicit definition with detailed non-commutativity example
- Re-extracted from v2 card; preserved: T/I non-commutativity example with IT2=J and T2I=K
