---
# === CORE IDENTIFICATION ===
concept: Semigroup of Transformations
slug: semigroup-of-transformations

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.3.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - closed collection of transformations

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation
  - composition-of-functions
extends: []
related:
  - semigroup
  - group-of-operations
contrasts_with:
  - group-of-operations

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a semigroup?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A semigroup of transformations on S is a collection F of transformations on S that is closed under composition: composing any two members of F always produces another member of F.

# Core Definition

"Given a family S, a collection F of transformations on S is called closed if, given any members f and g of F, the composition fg is a member of F. A closed collection of transformations on S will also be called a semigroup of transformations on S" (Lewin, Definition 1.3.2, p. 34).

# Prerequisites

- **Transformation** — the elements of the semigroup are transformations
- **Composition of Functions** — closure is defined in terms of composition

# Key Properties

1. Closure: if f and g are in F, then fg is in F
2. Associativity is automatic (composition of functions is always associative)
3. Need not contain the identity transformation
4. Need not contain inverse transformations
5. This is the concrete (transformation-based) version of the abstract semigroup concept

# Construction / Recognition

## To Construct:
1. Start with a set S
2. Collect transformations on S
3. Verify the collection is closed under composition

## To Recognize:
1. Check that all elements are transformations on the same set S
2. Check that composing any two elements yields another element in the collection

# Context & Application

Semigroups of transformations provide a concrete algebraic framework for collections of musical transformations that can be freely composed. The abstract notion of semigroup (Definition 1.4.3) generalizes this concept. Lewin uses the concrete version first to motivate the abstract definitions that follow.

# Examples

**Example 1** (p. 34): The collection of all transpositions {T0, T1, ..., T11} on pitch classes is a semigroup of transformations (in fact, a group of operations) since composing any two transpositions yields another transposition.

**Example 2**: Any collection containing a single transformation f and all its powers {f, ff, fff, ...} is a semigroup of transformations.

# Relationships

## Builds Upon
- **Transformation** — elements of the semigroup
- **Composition of Functions** — the operation that must be closed

## Enables
- **Group of Operations** — a group of operations is a semigroup with additional properties
- **Semigroup** — the abstract concept generalizes this concrete one

## Contrasts With
- **Group of Operations** — a group additionally requires inverses for all members

# Common Errors

- **Error**: Assuming a semigroup of transformations must contain the identity.
  **Correction**: The identity is not required; only closure under composition is needed.

# Common Confusions

- **Confusion**: Equating "semigroup of transformations" with the abstract "semigroup."
  **Clarification**: A semigroup of transformations is a concrete instance (elements are actual transformations on S); an abstract semigroup is a set with any associative binary composition.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.3.2, p. 34.

# Verification Notes

- Definition source: direct from Definition 1.3.2
- Confidence rationale: explicit definition in source
- New card (no prior version existed)
