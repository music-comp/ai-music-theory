---
# === CORE IDENTIFICATION ===
concept: Monoid of Integral Intervals
slug: monoid-of-integral-intervals

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: algebraic-structures
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "integral interval monoid"
  - "(Z+, *) as intervals"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - monoid
  - integral-intervals
  - multiplicative-composition-of-intervals
extends:
  - group-of-intervals
related:
  - prime-intervals
  - prime-interval-personality
  - powers-of-two-as-exact-keyboard-intervals
contrasts_with:
  - commutative-group

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What algebraic structure do the integral intervals form under composition?"
  - "Why do integral intervals form a monoid but not a group?"
  - "How does the Fundamental Theorem of Arithmetic relate to musical interval structure?"
---

# Quick Definition

The set of all integral intervals (intervals with positive integer frequency ratios) forms a monoid under composition of intervals, identifiable with the positive integers under multiplication (Z+, *), with unison (ratio 1) as the identity element.

# Core Definition

Wright states: "The set of integral intervals forms a monoid under composition of intervals; this monoid can be identified with (Z+, *)." The binary operation is composition of intervals, which corresponds to multiplication of frequency ratios. The identity element is the ratio 1 (unison). The monoid is commutative because integer multiplication is commutative. It is not a group because inverses -- ratios of the form 1/n for n > 1 -- are not positive integers (Wright, p. 110).

# Prerequisites

- **Monoid** -- Must understand the algebraic definition: a set with an associative binary operation and an identity element
- **Integral intervals** -- Must know that an integral interval is one whose frequency ratio is a positive integer
- **Multiplicative composition of intervals** -- Must understand that composing intervals multiplies their frequency ratios

# Key Properties

1. Closure: composing two integral intervals (multiplying two positive integers) yields another integral interval
2. Associativity: interval composition is associative because multiplication is associative
3. Identity: unison (ratio 1) is the identity element, since 1 * n = n for all n
4. Commutativity: the monoid is commutative (abelian) since integer multiplication is commutative
5. No inverses: downward intervals (ratios like 1/3) are not positive integers, so this is a monoid, not a group
6. Free generation by primes: by the Fundamental Theorem of Arithmetic, every integral interval factors uniquely into prime intervals

# Construction / Recognition

## To Identify the Monoid Structure
1. Consider the set of all positive integer frequency ratios: {1, 2, 3, 4, 5, ...}
2. Define the binary operation as multiplication (composition of intervals)
3. Verify closure: m * n is a positive integer whenever m and n are
4. Verify associativity: (m * n) * p = m * (n * p)
5. Identify the identity element: 1 (unison)
6. Observe that inverses do not exist in Z+ (e.g., 1/3 is not a positive integer)

# Context & Application

The monoid structure captures how integer intervals combine musically. Stacking intervals corresponds to multiplying their ratios, and the prime factorization of a composite integer determines the musical character of the corresponding interval. For example, 6 = 2 * 3 tells us that the interval of ratio 6 is an octave (2) composed with a twelfth (3), yielding two octaves plus a fifth.

The fact that this is a monoid rather than a group reflects a physical asymmetry: we can stack integer intervals upward indefinitely, but the set of integer ratios does not include downward intervals. The full group of all interval ratios is (R+, *), which contains this monoid as a substructure.

# Examples

**Example 1** (p. 110): 1 * n = n for any positive integer n -- unison composed with any interval yields that interval.

**Example 2** (p. 112): 2 * 3 = 6 -- octave composed with twelfth gives two octaves plus a fifth.

**Example 3** (p. 113): 3 * 3 = 9 -- twelfth composed with twelfth gives three octaves plus a major second.

**Example 4** (p. 113): 2 * 2 * 2 = 8 -- three octaves, rendered exactly on the keyboard since 8 = 2^3.

# Relationships

## Builds Upon
- **Monoid** -- The integral intervals instantiate the abstract monoid concept
- **Integral intervals** -- These are the elements of the monoid
- **Multiplicative composition of intervals** -- This is the monoid's binary operation

## Enables
- **Prime interval personality** -- The monoid's free generation by primes explains why each integer has a unique musical character determined by its prime factorization
- **Multiplicativity of interval errors** -- The monoid structure underlies the additivity of cent errors for composite intervals

## Related
- **Powers of two as exact keyboard intervals** -- The powers of 2 form a submonoid that is rendered exactly on the keyboard
- **Prime intervals** -- The generators of the monoid

## Contrasts With
- **Commutative group** -- A group requires inverses; this monoid lacks them
- **Group of intervals** -- The full group (R+, *) contains this monoid but also includes non-integer ratios

# Common Errors

- **Error**: Including ratios like 1/3 or 3/2 in the monoid of integral intervals
  **Correction**: Only positive integer ratios belong to this monoid; fractions and reciprocals are excluded

- **Error**: Assuming every algebraic structure with an identity and associativity is a group
  **Correction**: A group also requires inverses; since 1/n is not a positive integer for n > 1, this is only a monoid

# Common Confusions

- **Confusion**: Thinking the monoid of integral intervals includes all musical intervals
  **Clarification**: It includes only intervals with positive integer frequency ratios. The full set of musical interval ratios forms the group (R+, *), which is much larger

- **Confusion**: Believing that "integral interval" means an interval measured in whole semitones
  **Clarification**: "Integral" here means the frequency ratio is a positive integer, not that the interval spans a whole number of semitones

# Source Reference

Chapter 9: "The Integers as Intervals," p. 110 (PDF page 110). The monoid is identified in the introductory paragraph. Supporting examples of interval composition appear on pp. 111-115.

# Verification Notes

- Definition: Direct quotation from Wright, p. 110, paragraph 3
- Key Properties: Items 1-5 synthesized from the monoid definition and discussion; item 6 inferred from the Fundamental Theorem of Arithmetic discussion
- Examples: All drawn from Wright's worked examples of integers 1-13 (pp. 110-115)
- Confidence: HIGH -- explicit named identification of the monoid structure in the source
- Re-extraction notes: Re-extracted from v2 card to v3.1 format; preserved: examples of interval composition (2*3=6, 3*3=9, 2*2*2=8), clarification about group vs. monoid distinction
