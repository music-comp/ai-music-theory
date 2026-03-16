---
concept: Function
slug: function

category: mathematical-foundations
subcategory: functions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.1"

extraction_confidence: high

aliases:
  - mapping

prerequisites: []
extends: []
related:
  - functional-equality
  - composition-of-functions
  - one-to-one-function
  - onto-function
  - inverse-function
contrasts_with:
  - transformation

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
  - "What is a function in the context of Lewin's mathematical framework?"
---

# Quick Definition

A function (or mapping) from S into S' is a rule that assigns to each element of S exactly one corresponding element in S'.

# Core Definition

"A function or mapping from S into S' is a subfamily f of S x S' which has this property: Given any s in S, there is exactly one pair (s, s') within the family f which has the given s as the first entry of the pair." We write f(s) = s' for the value of f at argument s. Visualized as a table, each member of S appears once and only once in the left-hand column; members of S' may appear multiple times or not at all in the right-hand column (Lewin, Definition 1.2.1, p. 32).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Every element of S has exactly one value in S'
2. Different elements of S may share the same value in S'
3. Some elements of S' may not be values of any argument
4. Closure of argument coverage: the entire domain S is covered
5. Functions are formally subfamilies of the Cartesian product S x S'

# Construction / Recognition

## To Construct:
1. Specify the domain S and codomain S'
2. For each element s in S, assign exactly one element s' in S'
3. Verify that every element of S has been assigned a value

## To Recognize:
1. Check that every element in the domain has a value
2. Check that each element in the domain maps to exactly one value
3. A relation that assigns two different values to the same argument is not a function

# Context & Application

Functions are the most fundamental mathematical concept in Lewin's framework. All transformations, operations, homomorphisms, and interval functions are built on the concept of a function. Lewin defines functions set-theoretically as subfamilies of Cartesian products to establish maximum rigor for the algebraic developments that follow.

# Examples

**Example 1** (p. 32): Consider S, S', and S" all to be the family of positive integers. Let f1(s) = s + 3 and f2(s) = 2s. These are functions from the positive integers into the positive integers.

**Example 2** (p. 32): Let S and S' both be the family of twelve pitch classes. Let f(s) = s transposed by 2 and f'(s) = s inverted with respect to pitch class C. These are functions from pitch classes to pitch classes.

# Relationships

## Builds Upon
- No prerequisites; this is the most foundational concept in the text.

## Enables
- **Composition of Functions** — functions must exist before they can be composed
- **Transformation** — a transformation is a special case of a function (from S into S itself)
- **One-to-One Function** — an additional property a function may have
- **Onto Function** — an additional property a function may have

## Related
- **Functional Equality** — defines when two functions are the same

## Contrasts With
- **Transformation** — a transformation is a function from S into S itself, whereas a general function maps S into a potentially different S'

# Common Errors

- **Error**: Allowing an element of S to have no assigned value.
  **Correction**: Every element in the domain must be assigned exactly one value.

- **Error**: Allowing an element of S to map to multiple values.
  **Correction**: Each argument must have exactly one value; a rule assigning multiple outputs is not a function.

# Common Confusions

- **Confusion**: Thinking "function" and "mapping" are different concepts.
  **Clarification**: Lewin uses "function" and "mapping" synonymously (Definition 1.2.1).

- **Confusion**: Assuming every element of S' must be a value of some argument.
  **Clarification**: Some members of S' may not appear as values at all; that additional property is called "onto."

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.2.1, p. 32.

# Verification Notes

- Definition source: direct quotation from Definition 1.2.1
- Confidence rationale: explicit definition with clear terminology in the source
- Re-extracted from v2 card; preserved: musical examples of transposition/inversion as functions, table visualization metaphor
