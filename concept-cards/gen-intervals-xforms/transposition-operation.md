---
# === CORE IDENTIFICATION ===
concept: Transposition Operation
slug: transposition-operation

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.4 Transpositions and Interval-Preserving Operations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Ti"
  - "T_i"
  - "transposition by i"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - interval-group-ivls
  - label-function
extends:
  - generalized-interval-system
related:
  - interval-preserving-operation
  - group-of-transpositions
  - inversion-operation
contrasts_with:
  - interval-preserving-operation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a transposition operation in a GIS?"
  - "How do commutative and non-commutative GIS structures differ?"
  - "What distinguishes a commutative GIS from a non-commutative GIS?"
---

# Quick Definition
The transposition operation T_i in a GIS maps each element s to the unique element that lies interval i from s, formalizing the intuition of moving every element by the same interval.

# Core Definition
Given a GIS (S, IVLS, int) and an interval i, transposition by i, denoted T_i, is defined by int(s, T_i(s)) = i. That is, T_i(s) is the unique member of S lying the interval i from s. Theorem 3.4.2 establishes that each T_i is an operation (bijection), the transpositions form a group anti-isomorphic to IVLS (T_i T_j = T_{ji}), and T_e is the identity (Lewin, Definition 3.4.1 and Theorem 3.4.2, pp. 77-79).

# Prerequisites
- **Generalized Interval System** — Transpositions are defined within a GIS
- **Interval Group (IVLS)** — The interval i comes from IVLS; the group structure determines composition
- **LABEL Function** — LABEL(T_i(s)) = LABEL(s) * i (Theorem 3.4.3)

# Key Properties
1. int(s, T_i(s)) = i for all s in S
2. Each T_i is 1-to-1 and onto (an operation)
3. Anti-isomorphism: T_i T_j = T_{ji} (composition reverses interval order)
4. T_e = identity operation
5. T_i^{-1} = T_{i^{-1}}
6. LABEL(T_i(s)) = LABEL(s) * i (right-multiplication of labels)
7. In commutative GIS: T_i = P_i for all i (transpositions are interval-preserving)
8. In non-commutative GIS: T_i preserves intervals iff i is central in IVLS (Theorem 3.4.8)

# Construction / Recognition
## To Construct:
1. Choose an interval i in IVLS
2. For each s in S, find the unique t with int(s, t) = i
3. Set T_i(s) = t

## To Recognize:
1. The transformation maps every element by the same interval
2. The interval from each element to its image is constant (= i)
3. Labels are uniformly right-multiplied by i

# Context & Application
Transposition generalizes the familiar pitch-class operation of "moving by a fixed interval." In the 12-tone GIS, T_5 shifts each pitch class up by 5 semitones. The definition extends to any GIS: temporal transposition shifts time-points, durational transposition scales durations, and so on. The anti-isomorphism (reversed composition order) is algebraically necessary and can be counterintuitive.

# Examples
**Example 1** (p. 78): In the 12-tone pitch-class GIS:
- T_5(C) = F, since int(C, F) = 5
- T_5 T_3 = T_8 (not T_{15}; in additive notation T_i T_j = T_{j+i} = T_{i+j} by commutativity)

**Example 2**: Webern Piano Variations op. 27 (Figure 3.1):
- In the direct-product GIS_3 (pitch-class x time-point), transposition by (i, j) shifts pitch-class by i and time-point by j

# Relationships
## Builds Upon
- **Generalized Interval System** — transposition is a fundamental GIS operation
- **LABEL Function** — LABEL behavior under transposition is key (Theorem 3.4.3)

## Enables
- **Group of Transpositions** — transpositions form a group anti-isomorphic to IVLS
- **PETEY Group** — transpositions are one of the two generating families
- **Time-Span Transposition** — transposition in the time-span GIS

## Related
- **Inversion Operation** — inversions combine with transpositions via Theorem 3.5.6

## Contrasts With
- **Interval-Preserving Operation** — P_i left-multiplies labels; T_i right-multiplies. In commutative GIS they coincide; in non-commutative GIS they differ.

# Common Errors
- **Error**: Assuming T_i T_j = T_{ij} (homomorphism instead of anti-homomorphism)
  **Correction**: The map i -> T_i is an anti-isomorphism: T_i T_j = T_{ji}

- **Error**: Assuming transposition always preserves intervals
  **Correction**: T_i preserves intervals only when i is central in IVLS (Theorem 3.4.8). In non-commutative GIS, most transpositions do not preserve intervals.

# Common Confusions
- **Confusion**: Conflating transposition with interval-preserving operation
  **Clarification**: In commutative GIS these are identical (Corollary 3.4.9A). In non-commutative GIS they are distinct families that only overlap at the identity (when the only central element is e).

- **Confusion**: Thinking int(T_i(s), s) = i
  **Clarification**: The defining equation is int(s, T_i(s)) = i. The reverse interval int(T_i(s), s) = i^{-1}.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definition 3.4.1, Theorems 3.4.2-3.4.3, pp. 77-79.

# Verification Notes
- Definition source: direct from Definition 3.4.1
- Confidence rationale: high — explicit definition with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: anti-isomorphism discussion, Webern example, comparison with interval-preserving operations
