---
concept: Inverse Transposition
slug: inverse-transposition
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3.8 Inverse"
extraction_confidence: high
aliases:
  - complementary transposition
  - inverse operation
prerequisites:
  - transposition
extends:
  - transposition
related:
  - transposition-number
contrasts_with:
  - inversion
answers_questions:
  - "What is the inverse of a transposition?"
  - "How do I undo a transposition?"
---

# Quick Definition
Inverse transpositions are pairs of Tn operations that undo each other (sum to T0), such as T3 and T9, or T4 and T8.

# Core Definition
Two transpositions Tn and T(12-n) are inverses: performing one after the other returns to the starting point (Tn followed by T(12-n) = T0). The numbers n and (12-n) are *complements mod 12* -- they add up to 12 (= 0 mod 12). If Set X and Set Y are related at Tn, then Set Y and Set X are related at T(12-n).

# Prerequisites
- **Transposition (Tn)** -- inverse transposition is defined in terms of Tn

# Key Properties
1. Tn and T(12-n) are inverses: they cancel each other out
2. T0 is its own inverse (0 + 0 = 0)
3. T6 is its own inverse (6 + 6 = 12 = 0 mod 12)
4. All other transpositions come in complementary pairs: T1/T11, T2/T10, T3/T9, T4/T8, T5/T7
5. Composing two transpositions: Tn(Tm(x)) = T(n+m) mod 12(x)

# Construction / Recognition
**Inverse pairs:**
- T0 and T0
- T1 and T11
- T2 and T10
- T3 and T9
- T4 and T8
- T5 and T7
- T6 and T6

To undo a transposition at Tn, apply T(12-n).

# Context & Application
Understanding inverse transpositions helps in analyzing passages where music moves away from and returns to a pitch level. The concept reinforces that transposition is a *reversible* operation: any Tn can be undone by its complement.

# Examples
**Example 2-13** (p. 69): T3[7, 8, 10, 11] = [10, 11, 1, 2] and T9[10, 11, 1, 2] = [7, 8, 10, 11]. T3 and T9 are inverses that undo each other.

**Example 2-14** (p. 69): Diagram showing Tn followed by T(12-n) results in T0 (return to starting point).

# Relationships
## Builds Upon
- **Transposition (Tn)** -- inverse transposition is defined relative to Tn
## Enables
- **Group structure** -- the existence of inverses is one of the group axioms
## Related
- **Transposition number** -- inverse pairs sum to 12
## Contrasts With
- **Inversion (In)** -- In is its own inverse (In(In(x)) = x); transposition requires a different operation (T(12-n)) to undo it

# Common Errors
- **Error**: Confusing inverse transposition with pitch-class inversion. **Correction**: Inverse transpositions are complementary Tn operations (T3/T9). Inversion (In) is a fundamentally different operation.

# Common Confusions
- **Confusion**: Thinking T6 needs a different operation to undo. **Clarification**: T6 is its own inverse because 6 + 6 = 12 = 0 mod 12.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3.8, pages 68--69.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: clearly defined with formula and diagram
- Re-extraction notes: preserved old card's complete list of inverse pairs; upgraded to v3 template
