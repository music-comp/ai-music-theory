---
concept: Onto Function
slug: onto-function

category: mathematical-foundations
subcategory: functions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.6.1"

extraction_confidence: high

aliases:
  - surjection
  - surjective function

prerequisites:
  - function
extends:
  - function
related:
  - one-to-one-function
  - inverse-function
  - operation
contrasts_with:
  - one-to-one-function

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A function f from S into S' is onto S' if every member of S' is the value of some argument -- every element in the codomain is "hit" by at least one element of the domain.

# Core Definition

"The function f from S into S' is onto S' if every member of S' is the value of some argument. (Every member of S' appears at least once in the right-hand column of the function table.)" (Lewin, Definition 1.2.6.1, p. 33).

# Prerequisites

- **Function** — onto is an additional property of a function

# Key Properties

1. For every s' in S', there exists at least one s in S such that f(s) = s'
2. The image of f equals the codomain: f(S) = S'
3. A function can be onto without being one-to-one, or vice versa
4. A function that is both one-to-one and onto has an inverse

# Construction / Recognition

## To Construct:
1. Define f: S -> S'
2. Ensure that every element of S' appears as f(s) for some s in S

## To Recognize:
1. For each element s' in S', check if there exists some s with f(s) = s'
2. If any element of S' has no preimage, the function is not onto

# Context & Application

Onto functions ensure complete coverage of the codomain. In music theory, transposition on pitch classes is onto: every pitch class is the transposition of some other pitch class. The natural map from a set to its quotient is always onto. Together with the one-to-one property, "onto" characterizes operations (bijective transformations).

# Examples

**Example 1** (p. 33): Transposition T5 on the 12 pitch classes is onto: every pitch class y is T5(x) for x = y - 5 mod 12.

**Example 2** (p. 33): The natural map E from pitches to pitch classes is onto: every pitch class is represented by some pitch.

# Relationships

## Builds Upon
- **Function** — onto is a property of functions

## Enables
- **Operation** — an operation is a transformation that is both one-to-one and onto
- **Inverse Function** — requires both one-to-one and onto

## Contrasts With
- **One-to-One Function** — onto concerns coverage of the codomain; one-to-one concerns uniqueness of preimages

# Common Errors

- **Error**: Confusing "onto" with "one-to-one."
  **Correction**: "Onto" means every target is hit; "one-to-one" means each target is hit at most once.

# Common Confusions

- **Confusion**: Thinking "into S'" and "onto S'" mean the same thing.
  **Clarification**: "Into S'" simply specifies the codomain; "onto S'" additionally asserts that every element of S' is a value of the function.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.2.6.1, p. 33.

# Verification Notes

- Definition source: direct from Definition 1.2.6.1
- Confidence rationale: explicit definition in source
- Re-extracted from v2 card; preserved: transposition and natural map examples
