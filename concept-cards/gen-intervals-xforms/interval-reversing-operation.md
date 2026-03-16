---
# === CORE IDENTIFICATION ===
concept: Interval-Reversing Operation
slug: interval-reversing-operation

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
section: "3.6 Interval-Reversing Transformations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "interval-reversing transformation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - inversion-operation
  - interval-preserving-operation
extends: []
related:
  - commutative-vs-noncommutative-gis
contrasts_with:
  - interval-preserving-operation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an interval-reversing transformation?"
  - "How do commutative and non-commutative GIS structures differ?"
---

# Quick Definition
An interval-reversing transformation maps elements so that the interval between images is the reverse of the original interval: int(Y(s), Y(t)) = int(t, s). In commutative GIS, inversions are exactly the interval-reversing operations; in non-commutative GIS, no interval-reversing transformations exist.

# Core Definition
A transformation Y on S is interval-reversing if int(Y(s), Y(t)) = int(t, s) for all s, t (Definition 3.6.1). Since int(t, s) = int(s, t)^{-1}, this means the image interval is the inverse of the original. Theorem 3.6.3 proves that in commutative GIS, inversions are interval-reversing and every interval-reversing transformation is an inversion. Theorem 3.6.4 proves that in non-commutative GIS, no interval-reversing transformation exists (Lewin, pp. 90-92).

# Prerequisites
- **Generalized Interval System** — The definition uses the int function
- **Inversion Operation** — In commutative GIS, inversions are the interval-reversing operations
- **Interval-Preserving Operation** — Contrasts with: preserving vs. reversing intervals

# Key Properties
1. int(Y(s), Y(t)) = int(t, s) = int(s, t)^{-1} for all s, t
2. If Y is interval-reversing, LABEL(Y(t)) = i * LABEL(t)^{-1} for some i (Lemma 3.6.2)
3. In commutative GIS: inversions = interval-reversing operations (Theorem 3.6.3)
4. In non-commutative GIS: no interval-reversing transformations exist (Theorem 3.6.4)

# Construction / Recognition
## To Construct:
1. Only possible in commutative GIS
2. Choose any inversion operation I_u^v
3. Verify: int(I_u^v(s), I_u^v(t)) = int(t, s)

## To Recognize:
1. Check whether int(Y(s), Y(t)) = int(t, s) for sample s, t
2. In commutative GIS, this identifies inversions
3. In non-commutative GIS, no transformation passes this test

# Context & Application
The intuition that inversion "reverses intervals" — ascending intervals become descending — is formalized here. This property crucially depends on commutativity. The proof of Theorem 3.6.4 shows that the existence of an interval-reversing transformation on S would force IVLS to be commutative, contradicting the premise. This is one of the sharpest distinctions between commutative and non-commutative GIS.

# Examples
**Example 1** (p. 91): Pitch-class inversion is interval-reversing:
- int(C, E) = 4; int(I_C^C(C), I_C^C(E)) = int(C, Ab) = 8 = -4 mod 12
- int(E, C) = -4 mod 12 = 8. So int(I(C), I(E)) = int(E, C).

**Example 2** (p. 92): In the non-commutative time-span GIS, no transformation reverses intervals. This is Note 4.1.7(I).

# Relationships
## Builds Upon
- **Inversion Operation** — inversions are interval-reversing in commutative GIS

## Enables
- Understanding the fundamental asymmetry between commutative and non-commutative GIS

## Related
- **Commutative vs. Non-Commutative GIS** — existence of interval-reversing ops is a key difference

## Contrasts With
- **Interval-Preserving Operation** — preserves int(s,t); interval-reversing reverses it to int(t,s)

# Common Errors
- **Error**: Assuming inversions always reverse intervals in any GIS
  **Correction**: Inversions reverse intervals only in commutative GIS. In non-commutative GIS, inversions exist but are not interval-reversing.

# Common Confusions
- **Confusion**: Thinking "no interval-reversing ops" means "no inversions" in non-commutative GIS
  **Clarification**: Non-commutative GIS still have inversion operations I_u^v (defined by int(v, I(s)) = int(s, u)). They simply cannot be characterized as "interval-reversing."

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definition 3.6.1, Theorems 3.6.3-3.6.4, pp. 90-92.

# Verification Notes
- Definition source: direct from Definition 3.6.1
- Confidence rationale: high — explicit definitions and proofs
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, proof outline, non-commutative failure result
