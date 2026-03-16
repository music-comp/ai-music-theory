---
concept: Time-Span Interval Group
slug: time-span-interval-group

category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

extraction_confidence: high

aliases:
  - "IVLS for time spans"
  - "time-span IVLS"

prerequisites:
  - interval-group-ivls
  - group
extends:
  - interval-group-ivls
related:
  - time-span-gis
  - central-interval
contrasts_with: []

answers_questions:
  - "What is the interval group for the time-span GIS?"
  - "Why is the time-span interval group non-commutative?"
---

# Quick Definition
The time-span interval group consists of pairs (i, p) (i real, p positive real) with the non-commutative composition (i, p)(j, q) = (i + pj, pq), identity (0, 1), and inverse (i, p)^{-1} = (-i/p, 1/p).

# Core Definition
Lemma 4.1.3.1: Let IVLS be the family of pairs (i, p) where i is real and p is positive real. IVLS forms a group under (i, p)(j, q) = (i + pj, pq). The identity is (0, 1) and (i, p)^{-1} = (-i/p, 1/p). This group is non-commutative: the only central element is (0, 1) (Lewin, Lemma 4.1.3.1, p. 106; Note 4.1.7E, p. 113).

# Prerequisites
- **Interval Group (IVLS)** — This is a specific instance of the general IVLS concept
- **Group** — Must understand group axioms (closure, associativity, identity, inverse)

# Key Properties
1. Composition: (i, p)(j, q) = (i + pj, pq)
2. Identity: (0, 1)
3. Inverse: (i, p)^{-1} = (-i/p, 1/p)
4. Non-commutative: (i, p)(j, q) differs from (j, q)(i, p) in general
5. Only (0, 1) is central — no other element commutes with all others
6. Associative: ((i,p)(j,q))(k,r) = (i,p)((j,q)(k,r)) = (i + pj + pqk, pqr)
7. The formula (i, p)^{-1}(j, q) = ((j-i)/p, q/p) (Lemma 4.1.6.2)

# Construction / Recognition
## To Construct:
1. Take all pairs (i, p) with i real, p positive real
2. Define composition (i, p)(j, q) = (i + pj, pq)
3. Verify group axioms

## To Recognize:
1. The first component combines additively with scaling: i + pj
2. The second component combines multiplicatively: pq
3. The scaling factor p in i + pj is what makes the group non-commutative

# Context & Application
The composition law reflects how rhythmic relationships compound: the first component (temporal offset) is affected by the second (duration scaling). "Scaling then shifting" differs from "shifting then scaling," which is why the group is non-commutative. The i component measures relative temporal position in span-lengths; the p component measures duration ratio.

# Examples
**Example 1** (p. 106): Non-commutativity:
- (1, 2)(0, 3) = (1 + 2*0, 2*3) = (1, 6)
- (0, 3)(1, 2) = (0 + 3*1, 3*2) = (3, 6) — different!

**Example 2**: Inverse calculation:
- (2, 3)^{-1} = (-2/3, 1/3)
- Check: (2, 3)(-2/3, 1/3) = (2 + 3*(-2/3), 3*(1/3)) = (0, 1)

**Example 3**: Associativity:
- ((1,2)(3,4))(5,6) = (7, 8)(5, 6) = (7 + 8*5, 48) = (47, 48)
- (1,2)((3,4)(5,6)) = (1,2)(23, 24) = (1 + 2*23, 48) = (47, 48)

# Relationships
## Builds Upon
- **Interval Group (IVLS)** — this is the interval group for the time-span GIS
- **Group** — satisfies all group axioms

## Enables
- **Time-Span GIS** — serves as the IVLS for the non-commutative time-span GIS
- **Time-Span Transposition** — transpositions by elements of this group

## Related
- **Central Interval** — only (0, 1) is central, with far-reaching consequences

# Common Errors
- **Error**: Computing (i, p)(j, q) as (i + j, pq) (forgetting the scaling factor)
  **Correction**: The correct formula is (i + pj, pq) — the p scales j before adding

# Common Confusions
- **Confusion**: Thinking the non-commutativity is incidental
  **Clarification**: Non-commutativity is forced by the reference-independence properties. The scaling of j by p (measuring j in the "units" established by p) is what makes the group non-commutative.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Lemma 4.1.3.1, p. 106. Central element: Note 4.1.7(E), p. 113.

# Verification Notes
- Definition source: direct from Lemma 4.1.3.1
- Confidence rationale: high — explicit lemma with verification
- Re-extraction notes: Re-extracted from v2 card; preserved: non-commutativity proof, inverse calculation, associativity verification, scaling interpretation
