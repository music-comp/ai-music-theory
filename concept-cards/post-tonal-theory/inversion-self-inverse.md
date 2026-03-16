---
concept: Inversion as Self-Inverse
slug: inversion-self-inverse
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.4.3 Line (or series) of pitch classes"
extraction_confidence: high
aliases:
  - In is its own inverse
  - involutory operation
prerequisites:
  - inversion
  - inverse-transposition
extends:
  - inversion
related:
  - nodes-and-arrows
contrasts_with:
  - inverse-transposition
answers_questions:
  - "How do I undo an inversion?"
  - "Why are inversion arrows double-headed?"
---

# Quick Definition
Inversion is its own inverse: performing the same In twice returns to the original, unlike transposition where Tn requires T(12-n) to undo.

# Core Definition
"In is its own *inverse*: the operation that undoes the effect of an operation" (Straus, Ch. 2). If you invert something (a note, a line, or a set) by some In and want to return to the starting point, simply perform the same In again. Mathematically: In(In(x)) = In(n - x) = n - (n - x) = x. This is why inversional arrows in networks are always double-headed. By contrast, transposition requires the complementary operation T(12-n) to undo Tn.

# Prerequisites
- **Inversion (In)** -- the operation with this property
- **Inverse transposition** -- provides the contrast case

# Key Properties
1. In(In(x)) = x for all pitch classes x
2. All inversional arrows in networks are double-headed
3. In both sends x to y and y to x (simultaneously)
4. Contrast: Tn requires T(12-n) to undo
5. Formally, In is an *involution* (a function equal to its own inverse)

# Construction / Recognition
**Demonstration**: Take D (2) under I9:
- I9(2) = 9 - 2 = 7 (G)
- I9(7) = 9 - 7 = 2 (D)
- Applying I9 twice returns to the original

This is why Example 2-19 uses double-headed arrows for inversional relationships.

# Context & Application
The self-inverse property has practical consequences for analysis:
- In networks, inversional arrows are always double-headed (the relationship is symmetric)
- To undo an inversion, apply the same inversion (not a different one)
- Inversion of an inversion (at different index numbers) yields transposition: In followed by Im = T(m-n)

# Examples
**Example 2-19** (p. 71, Schoenberg, *String Quartet No. 4*): Two inversionally related melodies connected by I9. All arrows are double-headed because "if you invert something... by some In and want to get back to where you started, just perform the same In again."

**Section 2.4.3** (p. 72): I9(D) = G and I9(G) = D. "By contrast... if you transpose something at Tn, you will need to perform the complementary transposition, T(12-n), to get back where you started."

# Relationships
## Builds Upon
- **Inversion (In)** -- the operation with this property
## Enables
- **Network diagrams** -- determines arrow convention (double-headed for In)
## Related
- **Nodes and arrows** -- double-headed arrows represent self-inverse operations
## Contrasts With
- **Inverse transposition** -- Tn requires T(12-n) to undo; In requires only In itself

# Common Errors
- **Error**: Trying to undo In by applying a different Ik. **Correction**: To undo In, apply In itself. (Applying Ik instead produces transposition: Ik(In(x)) = T(k-n)(x).)

# Common Confusions
- **Confusion**: Inversion of an inversion. **Clarification**: In followed by In = identity. But In followed by Im (different index) = T(m-n), a transposition.

# Source Reference
Chapter 2: Pitch-Class Sets, Sections 2.4.3, page 72.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly stated as a key property distinguishing inversion from transposition
- Re-extraction notes: new card; extracted from section 2.4.3
