---
# === CORE IDENTIFICATION ===
concept: Inversion Equivalence Conditions
slug: inversion-equivalence-conditions

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: inversion-theory
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 84
section: "3.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inversion-operation
  - central-interval
extends: []
related:
  - inversion-transposition-combination
  - inversion-interval-preserving-combination
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When do two inversions I_u^v and I_x^w define the same operation?"
  - "Why are there more distinct inversions in non-commutative GIS?"
  - "When is I_u^v the same as I_v^u?"
---

# Quick Definition
Two inversions I_u^v and I_x^w are the same operation if and only if w = I_u^v(x) and int(x, u) is central. In commutative GIS, the centrality condition is automatic; in non-commutative GIS, it provides a significant additional constraint.

# Core Definition
**Theorem 3.5.3**: I_u^v = I_x^w as an operation on S if and only if: (1) w = I_u^v(x), and (2) the interval int(x, u) is central. The proof proceeds via LABEL computations: the condition I_u^v = I_x^w for all s requires that k^{-1}i = mj^{-1} = c where c is a central interval (with i, j, k, m being the LABELs of v, u, w, x respectively).

# Prerequisites
- **Inversion operation (I_u^v)** — The operations being compared
- **Central interval** — The algebraic condition governing equivalence

# Key Properties
1. In commutative GIS, condition (2) is automatic, so I_u^v = I_x^w iff w = I_u^v(x)
2. **Corollary 3.5.4**: I_u^v = I_v^u if and only if int(v, u) is central
3. **Corollary 3.5.5**: In commutative GIS, I_u^v always equals I_v^u; in non-commutative GIS, some I_u^v differs from I_v^u
4. In commutative GIS with n elements: typically n distinct inversions
5. In non-commutative GIS: potentially n^2 distinct inversions

# Construction / Recognition
## To Construct:
1. Given I_u^v and candidate parameters (x, w), compute w' = I_u^v(x)
2. Check whether w = w'
3. If so, check whether int(x, u) is central
4. Both conditions must hold for I_u^v = I_x^w
## To Recognize:
1. Two inversions that agree on all elements of S
2. Parameters satisfying both the image condition and the centrality condition

# Context & Application
In familiar pitch-class analysis, many different parameter pairs define the same inversion: e.g., I_0 = I_C^C = I_G^F = I_D^{Bb}. The theorem explains when such equivalences hold. In non-commutative GIS, inversions are more rigid: the centrality constraint means fewer parameter pairs define the same operation, potentially giving n^2 distinct inversions rather than n.

# Examples
**Example 1** (p. 84): In commutative pitch-class GIS: I_C^E = I_A^G since G = I_C^E(A) and int(A, C) = 3 is central (trivially, in commutative group).

**Example 2** (Notes 4.1.7(H), p. 113): In the non-commutative time-span GIS: I_s^t = I_{s'}^{t'} only when s' = s and t' = t, because only (0, 1) is central. Every ordered pair of time spans defines a distinct inversion.

**Example 3** (Corollary 3.5.4): I_u^v = I_v^u iff int(v, u) is central. This shows that "inversion about u and v" may depend on order in non-commutative GIS.

# Relationships
## Builds Upon
- **Inversion operation** — The operations whose equivalence is characterized
- **Central interval** — The algebraic condition controlling equivalence
## Enables
- **Inversion-transposition combination** — Understanding which inversions are the same clarifies composition formulas
## Related
- **Inversion-interval-preserving combination** — Both theorems use equivalence conditions

# Common Errors
- **Error**: Checking only w = I_u^v(x) without verifying centrality of int(x, u)
  **Correction**: Both conditions are necessary in non-commutative GIS

# Common Confusions
- **Confusion**: Rules for inversion equivalence from commutative GIS generalize directly
  **Clarification**: The centrality condition is an additional constraint absent in commutative settings
- **Confusion**: The theorem depends on the choice of ref
  **Clarification**: While the proof uses LABEL (which depends on ref), the statement of Theorem 3.5.3 is independent of ref

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.3 and Corollaries 3.5.4-3.5.5, pages 84-86.

# Verification Notes
- Definition source: Direct from Theorem 3.5.3
- Confidence rationale: High -- theorem and proof are explicit
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class and time-span examples, counting argument, Corollary 3.5.4 discussion
