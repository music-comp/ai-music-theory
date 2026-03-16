---
concept: Order of an Element
slug: order-of-an-element

category: algebra-in-music
subcategory: groups
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Generators and Cyclic Groups"

extraction_confidence: high

aliases:
  - element order
  - order of a group element

prerequisites:
  - cyclic-group-and-generator
extends:
  - cyclic-group-and-generator
related:
  - gcd-condition-for-generators
  - generating-interval
  - group
contrasts_with: []

answers_questions:
  - "What is the order of a group element?"
  - "How does the order relate to the number of distinct powers?"
---

# Quick Definition

The smallest positive integer m such that t^m = e (the identity), determining how many distinct powers the element produces before cycling.

# Core Definition

Let G be a group and t in G. Consider S = {n in Z+ | t^n = e}. If S is non-empty, its smallest element m (existing by the Well-Ordering Principle) is the order of t. The element t generates a cyclic subgroup {e, t, t^2, ..., t^(m-1)} with exactly m distinct elements. If S is empty, t has infinite order (Wright, pp. 94-95).

# Prerequisites

- **Cyclic group and generator** — Order is defined in the context of cyclic subgroups

# Key Properties

1. The order m is the smallest positive integer with t^m = e
2. t generates a subgroup with exactly m elements
3. t^n = t^r where r = n mod m (by the Division Algorithm)
4. In Z_n, the order of [k] is n / gcd(k, n)
5. [k] is a generator of Z_n iff its order equals n, iff gcd(k, n) = 1
6. The order of an element divides the order of the group (Lagrange's theorem)

# Construction / Recognition

## To Find the Order of t in a Finite Group
1. Compute t, t^2, t^3, ... until t^m = e
2. The first m for which this holds is the order
3. In Z_n: order of [k] = n / gcd(k, n)

# Context & Application

The order of a modular chromatic interval [k] in Z_n tells how many iterations are needed before returning to the starting note class. For example, in Z_12, the minor third [3] has order 4, meaning iterating minor thirds cycles through exactly 4 note classes (C, Eb, Gb, A) before repeating. This is directly related to symmetric chord structures.

# Examples

**Example 1** (p. 95): In Z_12: order of [1] is 12 (generator); order of [3] is 4; order of [4] is 3; order of [6] is 2.

**Example 2** (p. 95): In Z_8: [1] has order 8 (generator); [2] has order 4; [4] has order 2.

**Example 3** (p. 95): The order of [k] in Z_n is n / gcd(k, n).

**Example 4**: In (Z, +), the element 1 has infinite order: n * 1 = n != 0 for all n > 0, so no positive multiple of 1 equals the identity 0.

# Relationships

## Builds Upon
- **Cyclic group and generator** — Order determines whether an element is a generator

## Enables
- **GCD condition for generators** — An element is a generator iff its order equals the group order

## Related
- **Generating interval** — A generating interval has order n in Z_n

# Common Errors

- **Error**: Confusing the order of an element with the order of the group
  **Correction**: The order of an element is the size of the subgroup it generates; the order of the group is the total number of elements

# Common Confusions

- **Confusion**: Thinking an element of order m generates a subgroup with m-1 elements
  **Clarification**: The subgroup has exactly m elements: {e, t, t^2, ..., t^(m-1)}

- **Confusion**: Believing any positive n with t^n = e is the order
  **Clarification**: The order is the SMALLEST such n; e.g., [3] in Z_12 satisfies 4*[3] = [0] and 8*[3] = [0] and 12*[3] = [0], but the order is 4

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 94-95 (Generators and Cyclic Groups section).

# Verification Notes

- Definition source: Direct from Wright, pp. 94-95
- Confidence rationale: High — explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_12 and Z_8 order tables, gcd formula, Lagrange's theorem reference
