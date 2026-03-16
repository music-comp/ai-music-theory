---
concept: Index Number (Sum)
slug: index-number-sum
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.4.1 Index number (sum)"
extraction_confidence: high
aliases:
  - index number
  - sum
  - n in In
prerequisites:
  - inversion
  - pitch-class
extends:
  - inversion
related:
  - transposition-number
  - inversion-ixy
contrasts_with:
  - transposition-number
answers_questions:
  - "What is the index number in inversion?"
  - "Why is addition used to describe inversion?"
  - "How do I find the index number for two inversionally related notes?"
---

# Quick Definition
The index number (n in In) is the sum of any two pitch classes that map onto each other under a given inversion.

# Core Definition
When we invert a pitch class x at In, we subtract x from the index number n to produce a new pitch class y: In(x) = n - x (mod 12). The index number n represents the sum of any pair of inversionally related pitch classes: x + y = n. To find which In maps pitch class x onto pitch class y, add them: n = x + y (mod 12). This is fundamentally different from transposition, where n is a *difference* (n = y - x).

# Prerequisites
- **Inversion (In)** -- the index number is the n in In
- **Pitch class** -- mod 12 arithmetic applies

# Key Properties
1. n = x + y (mod 12) for any mapped pair
2. In(x) = (n - x) mod 12
3. n ranges from 0 to 11
4. There are exactly 12 possible index numbers (one per inversion)
5. The index number is a *sum*, not a difference

# Construction / Recognition
**Finding the index number:**
- If In(x) = y, then n = x + y (mod 12)
- Example: What In maps A (9) onto G (7)? n = 9 + 7 = 16 = 4 (mod 12). Answer: I4.

**Why addition?** Consider E (4) and F (5) on the clockface (Examples 2-15 through 2-17). Inverting E gives -4 = 8 (mod 12). To get from 8 to F (5), transpose by 4 + 5 = 9. So I9 maps E onto F. The index number equals the distance from -x to y, which is x + y.

# Context & Application
The index number contrasts fundamentally with the transposition number. For transposition, n = y - x (a difference). For inversion, n = x + y (a sum). This distinction is a key conceptual point in Chapter 2. The index number also connects to the Ixy notation: for Ixy, the index number is x + y.

# Examples
**Section 2.4.1** (p. 69): To find what In maps A (9) onto G (7):
- n = 9 + 7 = 16 = 4 (mod 12)
- Therefore I4(9) = 7

**Examples 2-15 through 2-17** (pp. 70--71): E (4) and F (5) on the clockface:
- E inverted = -4 = 8 (mod 12)
- To get from inverted E to F: 4 + 5 = 9 semitones
- Therefore I9 maps E onto F (and F onto E)

# Relationships
## Builds Upon
- **Inversion (In)** -- the index number is the defining parameter
## Enables
- **Inversional analysis** -- calculating which In relates two sets
- **Inversion (Ixy)** -- x + y = n connects the notations
## Related
- **Twelve inversions** -- each index number defines one of twelve inversions
## Contrasts With
- **Transposition number** -- Tn: n = y - x (difference); In: n = x + y (sum)

# Common Errors
- **Error**: Subtracting pitch classes to find the index number. **Correction**: *Add* the two pitch classes (mod 12). Subtraction finds the transposition number.

# Common Confusions
- **Confusion**: What does it mean to "add" two pitch classes? **Clarification**: The sum represents the total distance from -x to y on the clockface. It defines the axis of inversional symmetry and determines all exchange pairs for that inversion.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.4.1, pages 69--71.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly defined with conceptual explanation of why addition is used
- Re-extraction notes: new card; extracted from section 2.4.1
