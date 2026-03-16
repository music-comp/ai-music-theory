---
# === CORE IDENTIFICATION ===
concept: Interval Composition
slug: interval-composition

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: interval-mechanics
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Definition 2.3.1(A)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "GIS Condition A"
  - "Condition A"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - interval-group-ivls
  - musical-space-s
  - interval-function-int
extends: []
related:
  - generalized-interval-system
  - associativity
  - group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do intervals combine along a path in a GIS?"
  - "What is Condition (A) of the GIS definition?"
  - "Why does GIS numbering avoid the '3rd + 3rd = 5th' problem?"
---

# Quick Definition
Interval composition is the combining of intervals along a path: the interval from r to s composed with the interval from s to t yields the interval from r to t, formalized as Condition (A) of the GIS definition.

# Core Definition
"For all r, s, and t in S, int(r, s)int(s, t) = int(r, t)" (Lewin, Definition 2.3.1(A), p. 47). This condition formalizes the intuition that "the interval-from-r-to-s composes with the interval-from-s-to-t to yield the interval-from-r-to-t" using the group operation of IVLS.

# Prerequisites
- **Interval group IVLS** — Provides the group operation under which intervals compose
- **Musical space S** — The space containing elements r, s, t between which intervals are measured
- **Interval function int** — The function that assigns intervals to ordered pairs

# Key Properties
1. int(r, s) * int(s, t) = int(r, t) — intervals compose along paths
2. The operation * is the binary operation of the group IVLS (addition, multiplication, etc.)
3. Composition is associative: int(r, s) * (int(s, t) * int(t, u)) = (int(r, s) * int(s, t)) * int(t, u) = int(r, u)
4. From Condition (A) alone, one can derive int(s, s) = e and int(t, s) = int(s, t)^(-1)

# Construction / Recognition
## To Construct:
1. Identify the group operation in IVLS
2. Compute int(r, s) and int(s, t) separately
3. Combine using the group operation to obtain int(r, t)
## To Recognize:
1. Intervals along a path through intermediate points combine via the group operation
2. The result equals the direct interval between endpoints

# Context & Application
Interval composition captures the fundamental musical intuition that intervals "add up" along paths. Lewin emphasizes this "obviates a defect in the traditional measurements which tell us, for example, that a '3rd' and another '3rd' compose to form a '5th.' (3 + 3 = 5???)" The GIS numbering counts steps (0, 1, 2, ...) rather than ordinal names (1st, 2nd, 3rd, ...), making arithmetic consistent.

# Examples
**Example 1** (p. 47, Example 2.1.1): Diatonic pitch space — "If we take 2 steps up (e.g. from C4 to E4) and then take 2 more steps up (in this case, from E4 to G4), we have taken 4 steps up in all (in this case, from C4 to G4)." int(C4, E4) = 2, int(E4, G4) = 2, int(C4, G4) = 4, and 2 + 2 = 4.

**Example 2** (p. 47, Example 2.1.5): Just intonation — int(C, E) = 5/4, int(E, G#) = 5/4, int(C, G#) = 25/16, and (5/4)(5/4) = 25/16. Here the group operation is multiplication of frequency ratios.

# Relationships
## Builds Upon
- **Interval group IVLS** — Provides the algebraic structure for composition
## Enables
- **Generalized Interval System** — Condition (A) is one of the two defining conditions
- **Transposition** — Transpositions are defined using interval composition
## Related
- **Associativity** — Interval composition inherits associativity from the group

# Common Errors
- **Error**: Using traditional ordinal interval names and expecting arithmetic consistency
  **Correction**: GIS intervals count steps from zero, so composition obeys standard arithmetic

# Common Confusions
- **Confusion**: Assuming composition is always addition
  **Clarification**: The group operation depends on IVLS — it is addition for integers, multiplication for ratio groups, componentwise addition for direct products, etc.

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(A), pages 47-48.

# Verification Notes
- Definition source: Direct quotation from Definition 2.3.1(A)
- Confidence rationale: Core axiom of GIS, explicitly stated and extensively illustrated
- Re-extraction notes: Re-extracted from v2 card; preserved: diatonic and just-intonation examples, "3+3=5" quotation
