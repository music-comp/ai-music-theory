---
# === CORE IDENTIFICATION ===
concept: Commutative vs. Non-Commutative GIS
slug: commutative-vs-noncommutative-gis

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
section: "3.4-3.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "commutative GIS"
  - "non-commutative GIS"
  - "abelian vs non-abelian GIS"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - interval-group-ivls
  - transposition-operation
  - interval-preserving-operation
extends:
  - generalized-interval-system
related:
  - time-span-gis
  - central-interval
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do commutative and non-commutative GIS structures differ?"
  - "What distinguishes a commutative GIS from a non-commutative GIS?"
  - "What must I understand before non-commutative GIS structures?"
---

# Quick Definition
A GIS is commutative when its interval group IVLS is abelian (ij = ji for all intervals). The commutative/non-commutative distinction determines whether transpositions coincide with interval-preserving operations, whether inversions are self-inverse, and whether interval-reversing transformations exist.

# Core Definition
A GIS (S, IVLS, int) is commutative if IVLS is abelian. The key structural consequences are catalogued in Corollary 3.4.9 and throughout sections 3.4-3.6. In commutative GIS, T_i = P_i for all i, inversions are self-inverse (I^{-1} = I), and inversions are interval-reversing. In non-commutative GIS, transpositions and interval-preserving operations are distinct families, inversions are not generally self-inverse, and no interval-reversing transformations exist (Lewin, Corollary 3.4.9, Corollary 3.5.10, Theorems 3.6.3-3.6.4).

# Prerequisites
- **Generalized Interval System** — The framework in which commutativity is assessed
- **Interval Group (IVLS)** — Commutativity is a property of IVLS
- **Transposition Operation** — Key operation whose behavior differs between cases
- **Interval-Preserving Operation** — Coincides with transposition only in commutative case

# Key Properties

| Property | Commutative GIS | Non-Commutative GIS |
|----------|-----------------|---------------------|
| T_i = P_i | Always | Only when i is central |
| T_i preserves intervals | Always | Only when i is central |
| I_u^v = I_v^u | Always | Only when int(u,v) is central |
| Interval-reversing ops exist | Yes (= inversions) | No |
| I^{-1} = I | Always | Generally not |
| IT = T^{-1}I | Always | Generally not |

# Construction / Recognition
## To Recognize:
1. Check whether IVLS is abelian (ij = ji for all i, j)
2. Equivalently: check whether T_i = P_i for all i
3. Equivalently: check whether transpositions preserve intervals

# Context & Application
Most familiar music-theoretic GIS are commutative: pitch classes (Z/12Z), time-points (Z), just-intonation ratios, diatonic scale degrees. The theory developed for these structures often implicitly assumes commutativity. The time-span GIS of Chapter 4 provides the primary musical example of a non-commutative GIS, motivating the careful separation of commutative and non-commutative cases throughout Chapter 3.

# Examples
**Example 1**: Commutative GIS:
- Pitch classes mod 12: Z/12Z is abelian
- Time-points: Z under addition is abelian
- Just-intonation ratios: positive rationals under multiplication are abelian

**Example 2** (p. 92): Non-commutative time-span GIS:
- IVLS: pairs (i, p), composition (i, p)(j, q) = (i + pj, pq)
- (1, 2)(0, 3) = (1, 6) but (0, 3)(1, 2) = (3, 6) — not equal
- Only the identity (0, 1) is central (Note 4.1.7E)

# Relationships
## Builds Upon
- **Generalized Interval System** — commutativity is a GIS classification
- **Interval Group (IVLS)** — commutativity is a property of IVLS

## Enables
- Understanding when familiar pitch-class results generalize and when they fail

## Related
- **Time-Span GIS** — the principal non-commutative example
- **Central Interval** — the bridge concept: T_i = P_i exactly when i is central

# Common Errors
- **Error**: Applying commutative results (like IT = T^{-1}I) in non-commutative settings
  **Correction**: Always check whether IVLS is commutative before using commutative-specific formulas

# Common Confusions
- **Confusion**: Assuming non-commutativity of IVLS means GIS operations never commute
  **Clarification**: Even in non-commutative GIS, every transposition commutes with every interval-preserving operation (Theorem 3.4.10). It is within each family that non-commutativity manifests.

- **Confusion**: Thinking non-commutative GIS are exotic or purely theoretical
  **Clarification**: The time-span GIS models fundamental rhythmic intuitions about music with multiple local tempi (Carter, Stockhausen, Nancarrow), making it practically significant.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Corollary 3.4.9, sections 3.4-3.6, pp. 77-92.

# Verification Notes
- Definition source: synthesized from Corollary 3.4.9 and surrounding discussion
- Confidence rationale: high — explicit corollaries and theorems
- Re-extraction notes: Re-extracted from v2 card; preserved: comparison table, note about T commuting with P in all cases, non-commutativity example
