---
concept: Congruence Relation in GIS
slug: congruence-relation-gis

category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.2 Quotient GIS"

extraction_confidence: high

aliases:
  - "CONG"
  - "congruence on IVLS"
  - "group congruence"

prerequisites:
  - interval-group-ivls
  - equivalence-relation
  - generalized-interval-system
extends:
  - congruence
related:
  - quotient-group
  - induced-equivalence
  - quotient-gis
contrasts_with:
  - equivalence-relation

answers_questions:
  - "How do I construct a quotient GIS from a congruence relation?"
  - "What is a congruence relation on IVLS?"
  - "How does pitch-class space relate to chromatic pitch space via quotient GIS?"
---

# Quick Definition
A congruence relation on the interval group IVLS is an equivalence relation compatible with the group operation, ensuring that the quotient group IVLS/CONG is well-defined and enabling construction of quotient GIS structures.

# Core Definition
A relation CONG on the group IVLS is a congruence if: (1) CONG is an equivalence relation (reflexive, symmetric, transitive), and (2) whenever i is congruent to i' and j is congruent to j', then ij is congruent to i'j'. Condition (2) ensures the quotient group IVLS/CONG has a well-defined binary operation: [i] * [j] = [ij] (Lewin, Section 3.2, pp. 63-65; referencing Definition 1.10.1).

# Prerequisites
- **Interval Group (IVLS)** — The congruence is defined on this group
- **Equivalence Relation** — A congruence is first an equivalence relation with additional compatibility
- **Generalized Interval System** — Congruences on IVLS induce quotient GIS structures

# Key Properties
1. A congruence is an equivalence relation compatible with the group operation
2. The congruence class of the identity e forms a normal subgroup (the kernel)
3. All congruence classes are cosets of the kernel
4. The quotient IVLS/CONG inherits a group structure: [i] * [j] = [ij]
5. The congruence is defined on IVLS (intervals), not on S (elements)
6. The congruence on IVLS induces an equivalence on S (see induced-equivalence)

# Construction / Recognition
## To Construct:
1. Identify a normal subgroup N of IVLS
2. Declare i congruent to i' whenever i^{-1}i' is in N (equivalently, i and i' are in the same coset of N)
3. Verify the compatibility condition: if i ~ i' and j ~ j', then ij ~ i'j'

## To Recognize:
1. Check that the relation is an equivalence relation on IVLS
2. Verify compatibility with the group operation
3. Identify the kernel (congruence class of e)

# Context & Application
Congruence relations formalize the concept of "modularization" — the process of identifying intervals that differ by some structural unit. This is ubiquitous in music theory whenever we treat intervals modulo some cycle (octave, measure length, etc.). The congruence determines which intervals are "equivalent to zero" and thus which elements of S will become identified in the quotient space.

# Examples
**Example 1** (p. 63): Chromatic to pitch-class intervals:
- IVLS = Z (integers, semitones), CONG: i ~ i' iff i' - i is divisible by 12
- Quotient: IVLS/CONG = Z/12Z (integers mod 12)
- Kernel: multiples of 12

**Example 2** (p. 67): Just-intonation modularization:
- IVLS = {2^a * 3^b * 5^c} under multiplication
- CONG: i ~ i' iff i'/i is a power of 2
- Quotient: pairs (b, c) representing dominants and mediants
- Kernel: powers of 2

**Example 3** (p. 68): Time-points to beat-classes:
- IVLS = Z (beats), CONG: i ~ i' iff i' - i is divisible by N
- Quotient: Z/NZ (beat-classes in N-beat measure)

# Relationships
## Builds Upon
- **Equivalence Relation** — a congruence is an equivalence relation with additional structure
- **Interval Group (IVLS)** — the congruence is defined on IVLS

## Enables
- **Quotient GIS** — congruence on IVLS is the prerequisite for constructing a quotient GIS
- **Induced Equivalence** — the congruence induces an equivalence relation on S

## Related
- **Quotient Group** — IVLS/CONG is the interval group of the quotient GIS
- **Normal Subgroup** — the kernel of the congruence

## Contrasts With
- **Equivalence Relation** — a congruence has the additional compatibility constraint with the group operation

# Common Errors
- **Error**: Defining a congruence on S (elements) instead of IVLS (intervals)
  **Correction**: The congruence is on IVLS; the equivalence on S is induced by the congruence

- **Error**: Using an equivalence that is not compatible with the group operation
  **Correction**: Always verify that the product of congruent elements is congruent; otherwise the quotient group operation is not well-defined

# Common Confusions
- **Confusion**: Believing any equivalence on S yields a quotient GIS
  **Clarification**: Not every equivalence on S gives a quotient GIS; the equivalence must be induced by a congruence on IVLS for int to be well-defined on quotient elements

- **Confusion**: Confusing congruence classes with equivalence classes on S
  **Clarification**: Congruence classes are subsets of IVLS (intervals); equivalence classes are subsets of S (elements). The congruence on IVLS induces the equivalence on S.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Sections 3.2.1-3.2.4, pp. 63-68.

# Verification Notes
- Definition source: direct from Section 3.2, referencing Definition 1.10.1
- Confidence rationale: high — explicit definitions and examples
- Re-extraction notes: Re-extracted from v2 card; preserved: three concrete examples, kernel interpretation, confusion about equivalence on S vs congruence on IVLS
