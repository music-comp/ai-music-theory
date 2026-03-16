---
concept: Interval Group IVLS
slug: interval-group-ivls

category: generalized-interval-systems
subcategory: interval-mechanics
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Definition 2.3.1, Section 2.4"

extraction_confidence: high

aliases:
  - "group of intervals"
  - "interval group"

prerequisites:
  - group
  - generalized-interval-system
extends:
  - group
related:
  - musical-space-s
  - interval-function-int
  - direct-product
  - quotient-group
contrasts_with: []

answers_questions:
  - "What is IVLS in a GIS and what algebraic structure must it have?"
  - "What are examples of interval groups for different musical spaces?"
  - "Can the same musical space have different interval groups?"
---

# Quick Definition
IVLS is the group of intervals in a GIS -- a mathematical group whose elements represent all possible directed measurements, distances, or motions between elements of the musical space S.

# Core Definition
In a GIS (S, IVLS, int), "IVLS, the group of intervals for the GIS, is a mathematical group" (Lewin, Definition 2.3.1, p. 47). IVLS must satisfy the four group axioms (closure, associativity, identity, inverses). The interval function int maps pairs of elements from S into IVLS, and Condition (A) ensures that interval composition in IVLS corresponds to path concatenation in S.

# Prerequisites
- **Group** — IVLS must be a mathematical group with closure, associativity, identity, and inverses
- **Generalized Interval System** — IVLS is the second component of the GIS triple

# Key Properties
1. IVLS is always a group (not merely a set of intervals)
2. The group operation may be addition, multiplication, or any associative binary composition
3. IVLS may be commutative or non-commutative
4. The identity e in IVLS satisfies int(s, s) = e for all s
5. Inverses satisfy int(t, s) = int(s, t)^(-1)
6. |IVLS| = |S| when S is finite (simple transitivity)

# Construction / Recognition
## To Construct:
1. Identify the characteristic measurements, distances, or motions in the musical space
2. Verify they form a group under a natural composition operation
3. Verify that Conditions (A) and (B) hold for the function int
## To Recognize:
1. A group whose elements serve as directed intervals between musical objects
2. Paired with a space S and function int satisfying Conditions (A) and (B)

# Context & Application
Different musical dimensions use different interval groups. Lewin surveys this in Section 2.4: integers for pitch, integers mod 12 for pitch classes, positive rationals for frequency ratios, ordered pairs for harmonic space. The same musical space S can in principle have different IVLS for different analytical purposes.

# Examples
**Example 1** (Section 2.4, p. 48): Chromatic pitch space — IVLS = integers under addition. Intervals are "n semitones up."

**Example 2** (Section 2.4): Pitch-class space — IVLS = integers mod 12. Intervals wrap: 7 + 7 = 14 = 2 mod 12.

**Example 3** (Section 2.4): Just intonation — IVLS = multiplicative group of rationals of form 2^a * 3^b * 5^c.

**Example 4** (Section 2.4): Modular harmonic space (Figure 2.2) — IVLS = Z x Z (direct product of integers with itself), with intervals (b, c) measuring "b dominants, c mediants" under componentwise addition.

**Example 5** (Section 2.4): Duration proportion space — IVLS = multiplicative group of positive rationals, with int(s, t) = t/s.

# Relationships
## Builds Upon
- **Group** — IVLS instantiates the abstract group concept in a musical context
## Enables
- **Interval function int** — Maps into IVLS
- **Transposition** — Defined via IVLS elements
- **Interval-preserving operations** — Defined through IVLS
## Related
- **Musical space S** — The space whose elements IVLS measures intervals between
- **Direct product** — Used to construct multi-dimensional IVLS (e.g., harmonic space)
- **Quotient group** — Used to derive reduced IVLS (e.g., modular systems)

# Common Errors
- **Error**: Assuming IVLS must be commutative
  **Correction**: IVLS may be non-commutative, as explored in Chapters 3-4 and Appendix B

# Common Confusions
- **Confusion**: Thinking the group operation must be "addition"
  **Clarification**: IVLS may use multiplication (frequency ratios), componentwise addition (harmonic space), or other operations

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Section 2.4, pages 47-49.

# Verification Notes
- Definition source: Direct from Definition 2.3.1 and Section 2.4 survey
- Confidence rationale: Core component of GIS definition with extensive examples
- Re-extraction notes: Re-extracted from v2 card; preserved: all five example spaces, emphasis on group structure
