---
concept: Group of Operations
slug: group-of-operations

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.3.4"

extraction_confidence: high

aliases: []

prerequisites:
  - operation
  - semigroup-of-transformations
  - identity-transformation
extends:
  - semigroup-of-transformations
related:
  - group
contrasts_with:
  - semigroup-of-transformations

answers_questions:
  - "What is a mathematical group?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A group of operations on S is a collection G of transformations on S that is closed under composition and in which every member has an inverse that is also in G.

# Core Definition

"By a group of operations on S we shall mean a family G of transformations on S which satisfies conditions (A) and (B) following. (A): G is a closed family, a semigroup of transformations in the sense of 1.3.2. (B): Given any member f of G, there exists a member f' of G satisfying f'f = ff' = 1" (Lewin, Definition 1.3.4, p. 34). Condition (B) guarantees that members of G are operations (via Theorem 1.3.3.3) and that G contains both the identity and the inverse of each member.

# Prerequisites

- **Operation** — the members of the group are operations
- **Semigroup of Transformations** — a group is a semigroup with additional structure
- **Identity Transformation** — implicitly contained via conditions (A) and (B)

# Key Properties

1. Closure: composing any two members yields a member (Condition A)
2. Inverses: every member has an inverse in G (Condition B)
3. The identity 1 is in G (follows from A and B together, provided G is non-empty)
4. Associativity is automatic (composition of functions is always associative)
5. All members are operations (1-to-1, onto transformations)

# Construction / Recognition

## To Construct:
1. Collect operations on a set S
2. Verify closure under composition
3. Verify that the inverse of each operation is in the collection

## To Recognize:
1. All elements are transformations on the same set S
2. Composing any two elements yields another element in the collection
3. Every element has an inverse in the collection

# Context & Application

This is the concrete (transformation-based) definition of a group, preceding the abstract definition in 1.7. The T/I group of 24 transposition and inversion operations on pitch classes is the prototypical example. Lewin later proves that the abstract definition (1.7) is consistent with this concrete one.

# Examples

**Example 1** (p. 34): The group of transposition and inversion operations on the twelve pitch classes. It contains 24 operations: T0 through T11 and I0 through I11. It is closed under composition, and every element has an inverse.

**Example 2**: The transpositions alone {T0, T1, ..., T11} form a group of operations on the 12 pitch classes.

# Relationships

## Builds Upon
- **Semigroup of Transformations** — a group is a semigroup satisfying Condition (B)

## Enables
- **Group** — the abstract definition generalizes this concrete one

## Contrasts With
- **Semigroup of Transformations** — a semigroup may lack inverses

# Common Errors

- **Error**: Checking only closure without checking for inverses.
  **Correction**: Both conditions (A) and (B) are required. A semigroup without inverses is not a group.

# Common Confusions

- **Confusion**: Thinking the identity must be separately required.
  **Clarification**: If G is non-empty, conditions (A) and (B) together imply G contains the identity.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.3.4, p. 34.

# Verification Notes

- Definition source: direct from Definition 1.3.4
- Confidence rationale: explicit definition in source with discussion of implied properties
- New card (no prior version existed)
