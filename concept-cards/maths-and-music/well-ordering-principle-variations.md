---
concept: Well-Ordering Principle Variations
slug: well-ordering-principle-variations

category: algebra-in-music
subcategory: groups
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Variations On The Well-Ordering Principle"

extraction_confidence: high

aliases:
  - WOP variations
  - well-ordering axiom

prerequisites: []
extends:
  - well-ordering-principle
related:
  - generalized-division-algorithm
  - division-algorithm
contrasts_with: []

answers_questions:
  - "What are the equivalent formulations of the Well-Ordering Principle?"
  - "How is WOP.4 used in the proof of the Generalized Division Algorithm?"
---

# Quick Definition

Four equivalent formulations of the Well-Ordering Principle, extending the basic statement about positive integers to cover negative integers and bounded subsets of all integers.

# Core Definition

The four equivalent formulations (Wright, p. 83):
- **WOP.1**: Any non-empty subset of Z+ has a smallest element.
- **WOP.2**: Any non-empty subset of Z- has a largest element.
- **WOP.3**: Any non-empty subset of Z which has a lower bound has a smallest element.
- **WOP.4**: Any non-empty subset of Z which has an upper bound has a largest element.

A lower bound for a set T is a real number y with y <= t for all t in T. Upper bound is analogous. These are taken as axioms and are easily seen to be equivalent.

# Prerequisites

This is a foundational concept with no prerequisites within this source (it is taken as an axiom).

# Key Properties

1. All four formulations are logically equivalent
2. WOP.1 is the classical statement (restated from Chapter 1)
3. WOP applies only to subsets of Z, not R
4. The bound in WOP.3/WOP.4 need not be an integer
5. These are taken as axioms, not proved from more basic principles

# Construction / Recognition

## To Apply WOP in a Proof
1. Identify a non-empty subset S of Z
2. Determine which formulation applies (does S have a lower or upper bound?)
3. Conclude the existence of a smallest/largest element
4. Use this extremal element in the proof

# Context & Application

WOP.4 is used in the proof of the Generalized Division Algorithm: the set S = {l in Z | l*m <= x} has x/m as an upper bound, so by WOP.4 it has a largest element q, which becomes the quotient. These variations provide the logical foundation for proofs about modular arithmetic and group generators.

# Examples

**Example 1** (p. 83): WOP.1: {3, 7, 11, 15, ...} has smallest element 3.

**Example 2** (p. 83): WOP.2: {-2, -5, -8, ...} has largest element -2.

**Example 3** (p. 84): WOP.4 in the Division Algorithm proof: S = {l in Z | l*m <= x} has upper bound x/m, so it has a largest element q.

# Relationships

## Builds Upon
- **Well-ordering principle** — WOP.1 is the original statement; the variations extend it

## Enables
- **Generalized division algorithm** — WOP.4 is used in its proof

## Related
- **Division algorithm** — The original algorithm also uses WOP

# Common Errors

- **Error**: Applying WOP to subsets of R (the real numbers)
  **Correction**: WOP applies only to subsets of Z; the set (0, 1) in R has a lower bound but no smallest element

# Common Confusions

- **Confusion**: Thinking WOP.3 and WOP.4 are stronger than WOP.1
  **Clarification**: All four statements are logically equivalent; proving any one implies the others

- **Confusion**: Believing the bound must be an integer
  **Clarification**: The bound is a real number (y <= t for all t in T); it need not be an element of the set or even an integer

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 83 (Variations On The Well-Ordering Principle section).

# Verification Notes

- Definition source: Direct from Wright, p. 83, with all four formulations stated explicitly
- Confidence rationale: High — explicitly stated axioms
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all four WOP formulations, connection to Division Algorithm proof, R vs. Z distinction
