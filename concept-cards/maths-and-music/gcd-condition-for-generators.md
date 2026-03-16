---
concept: GCD Condition for Generators
slug: gcd-condition-for-generators

category: algebra-in-music
subcategory: groups
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Generators and Cyclic Groups"

extraction_confidence: high

aliases:
  - coprimality condition for generators
  - relatively prime generator criterion

prerequisites:
  - cyclic-group-and-generator
  - order-of-an-element
extends:
  - cyclic-group-and-generator
related:
  - generating-interval
  - greatest-common-divisor
  - relatively-prime-integers
  - euler-phi-function
contrasts_with: []

answers_questions:
  - "When does an element generate a cyclic group?"
  - "What is the relationship between gcd and generators of Z_n?"
---

# Quick Definition

The criterion that [n] generates the cyclic group Z_m if and only if gcd(m, n) = 1, connecting number theory to the structure of chromatic scales.

# Core Definition

If t is a generator of a cyclic group G having order m, then a power t^n is also a generator precisely when gcd(n, m) = 1, i.e., the only positive integer dividing both m and n is 1. Equivalently, [n] is a generator of (Z_m, +) iff gcd(m, n) = 1. The number of generators is phi(m), where phi is the Euler phi function (Wright, pp. 95-96).

# Prerequisites

- **Cyclic group and generator** — The GCD condition characterizes when t^n is a generator
- **Order of an element** — The order of t^n determines whether it generates the full group

# Key Properties

1. [n] generates Z_m iff gcd(m, n) = 1
2. The number of generators of Z_m is phi(m)
3. The condition is symmetric: gcd(m, n) = gcd(n, m)
4. For prime m, every non-zero element is a generator
5. The result also characterizes which [n] are units in the ring Z_m

# Construction / Recognition

## To Check if [n] Generates Z_m
1. Compute gcd(m, n) using the Euclidean algorithm
2. If gcd(m, n) = 1, then [n] is a generator
3. If gcd(m, n) > 1, then [n] generates only a proper subgroup of order m/gcd(m, n)

# Context & Application

This theorem precisely explains why certain intervals generate all note classes in a chromatic scale. In the 12-chromatic scale, only [1], [5], [7], [11] are generators because gcd(1,12) = gcd(5,12) = gcd(7,12) = gcd(11,12) = 1. The major third [4] is not a generator because gcd(4,12) = 4, so it generates only {[0], [4], [8]} (the augmented triad).

# Examples

**Example 1** (p. 95): gcd(7, 12) = 1: the fifth generates all 12 note classes (circle of fifths).

**Example 2** (p. 95): gcd(4, 12) = 4: the major third does NOT generate Z_12; iterating gives {[0], [4], [8]}.

**Example 3** (p. 95): gcd(3, 7) = 1: in the 7-chromatic scale, [3] generates all 7 intervals.

**Example 4** (p. 95): The number of generators of Z_m is phi(m): phi(12) = 4, phi(7) = 6, phi(14) = 6.

# Relationships

## Builds Upon
- **Cyclic group and generator** — The GCD condition characterizes generators
- **Order of an element** — Elements of order m are generators; order = m/gcd(n,m)

## Enables
- **Generating interval** — Musical generating intervals are characterized by this condition

## Related
- **Greatest common divisor** — The central number-theoretic concept in the criterion
- **Relatively prime integers** — gcd(m, n) = 1 means m and n are relatively prime
- **Euler phi function** — Counts generators

# Common Errors

- **Error**: Checking gcd(m, n) = m instead of gcd(m, n) = 1
  **Correction**: The generator condition requires gcd(m, n) = 1; gcd(m, n) = m would mean m divides n

# Common Confusions

- **Confusion**: Thinking the condition requires n < m
  **Clarification**: The condition applies to any integer n; [13] generates Z_12 because gcd(13, 12) = 1 (and [13] = [1] in Z_12)

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 95-96 (Generators and Cyclic Groups section). Proof completed in Chapter 8.

# Verification Notes

- Definition source: Direct from Wright, pp. 95-96
- Confidence rationale: High — explicit criterion with examples
- Uncertainties: Full proof deferred to Chapter 8
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: major third non-generator example, phi values for 12/7/14
