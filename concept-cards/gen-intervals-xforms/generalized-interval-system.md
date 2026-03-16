---
concept: Generalized Interval System
slug: generalized-interval-system

category: generalized-interval-systems
subcategory: core-definitions
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.3.1"

extraction_confidence: high

aliases:
  - GIS

prerequisites:
  - group
  - function
extends: []
related:
  - gis-condition-a
  - gis-condition-b
  - gis-theorem-2-3-2
  - directed-interval
contrasts_with: []

answers_questions:
  - "What is a Generalized Interval System (GIS)?"
  - "How do I construct a GIS from a musical space?"
  - "How does the interval function int relate to the group IVLS?"
---

# Quick Definition

A Generalized Interval System (GIS) is an ordered triple (S, IVLS, int) consisting of a musical space S, a mathematical group IVLS of intervals, and a function int mapping pairs of elements to intervals, subject to two conditions governing interval composition and space completeness.

# Core Definition

"A Generalized Interval System (GIS) is an ordered triple (S, IVLS, int), where S, the space of the GIS, is a family of elements, IVLS, the group of intervals for the GIS, is a mathematical group, and int is a function mapping S x S into IVLS, all subject to the two conditions (A) and (B) following. (A): For all r, s, and t in S, int(r, s)int(s, t) = int(r, t). (B): For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s, that is a unique t which satisfies the equation int(s, t) = i" (Lewin, Definition 2.3.1, p. 52).

# Prerequisites

- **Group** — IVLS must be a mathematical group
- **Function** — int: S x S -> IVLS is a function

# Key Properties

1. S: the musical space (family of elements)
2. IVLS: the interval group (a mathematical group)
3. int: S x S -> IVLS (the interval function)
4. Condition (A): int(r, s) * int(s, t) = int(r, t) (interval path composition)
5. Condition (B): for all s and i, exists unique t with int(s, t) = i (space completeness)
6. Derived theorems: int(s, s) = e and int(t, s) = int(s, t)^(-1) (Theorem 2.3.2)

# Construction / Recognition

## To Construct:
1. Identify the musical space S
2. Identify the interval group IVLS (must be a group)
3. Define the interval function int: S x S -> IVLS
4. Verify Condition (A): intervals compose along paths
5. Verify Condition (B): every interval is realized uniquely from every starting point

## To Recognize:
1. Three components: space, group, function
2. Condition (A) holds
3. Condition (B) holds

# Context & Application

The GIS framework unifies diverse musical concepts under a single mathematical structure. Traditional pitch intervals, pitch-class intervals, rhythmic intervals, harmonic distances, and many other musical measurements can be formalized as GIS structures. This allows theorems proved for abstract GIS to apply across all these domains. Lewin presents 12 examples in Chapter 2, of which 11 form valid GIS structures (Example 2.2.5 being the exception).

# Examples

**Example 1** (Section 2.4, p. 53): Twelve GIS examples:
- Diatonic pitch space: S = diatonic pitches, IVLS = (Z, +), int = scale steps
- Chromatic pitch space: S = chromatic pitches, IVLS = (Z, +), int = semitones
- Pitch-class space: S = 12 pitch classes, IVLS = (Z12, +), int = clockwise hours
- Just intonation: S = JI pitches, IVLS = {2^a * 3^b * 5^c} under multiplication, int = frequency ratio
- Modular harmonic space: S = game board, IVLS = Z x Z, int = (dominants, mediants)
- Time-point space: S = time points, IVLS = (Z, +), int = units later

**Example 2** (p. 56): Example 2.2.5 (durations under subtraction) does NOT satisfy Condition (B), demonstrating that not every intuitive space forms a GIS.

# Relationships

## Builds Upon
- **Group** — IVLS must be a group
- **Function** — int is a function

## Enables
- **GIS Condition A** — the path-composition law
- **GIS Condition B** — the completeness condition
- **GIS Theorem 2.3.2** — derived properties of int

## Related
- **Directed Interval** — the GIS formalizes directed intervals

# Common Errors

- **Error**: Defining a GIS without verifying Condition (B).
  **Correction**: Both conditions must be checked. Example 2.2.5 shows that Condition (A) alone is insufficient.

- **Error**: Using a semigroup instead of a group for IVLS.
  **Correction**: IVLS must be a group (not just a semigroup), providing identity and inverses.

# Common Confusions

- **Confusion**: Thinking the space S must be finite or practical.
  **Clarification**: S is a space of "theoretical potentialities, rather than a compendium of musical practicalities." It may extend beyond any practical context.

- **Confusion**: Thinking Condition (B) can be weakened to "some t" instead of "unique t."
  **Clarification**: With "weak B," the space reduces to equivalence classes, yielding a GIS on the quotient space.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, pp. 52-53.

# Verification Notes

- Definition source: direct quotation from Definition 2.3.1
- Confidence rationale: the central definition of the book, stated with complete precision
- Re-extracted from v2 card; preserved: all GIS examples summary, "weak B" discussion, theoretical potentialities quote
