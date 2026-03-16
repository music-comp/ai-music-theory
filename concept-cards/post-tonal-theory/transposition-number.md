---
concept: Transposition Number
slug: transposition-number
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3 Transposition (Tn)"
extraction_confidence: high
aliases:
  - transposition interval
  - n in Tn
prerequisites:
  - transposition
  - pitch-class
extends:
  - transposition
related:
  - index-number
  - inverse-transposition
contrasts_with:
  - index-number
answers_questions:
  - "What is the transposition number?"
  - "How do I find the interval of transposition between two sets?"
---

# Quick Definition
The transposition number (n in Tn) is the pitch-class interval added to each element of a set or line under transposition, ranging from 0 to 11.

# Core Definition
In the operation Tn, n represents the transposition number -- the pitch-class interval added to each element. It ranges from 0 to 11, with T0 as the identity operation. The transposition number is found by subtracting corresponding elements of two transpositionally related sets: n = (y - x) mod 12.

# Prerequisites
- **Transposition (Tn)** -- the operation that uses the transposition number
- **Pitch class** -- mod 12 arithmetic applies

# Key Properties
1. n ranges from 0 to 11 (mod 12)
2. T0 = identity (no change)
3. T6 maps each pitch class to its tritone partner
4. Complementary pairs (summing to 12) are inverse operations
5. To find n: subtract corresponding elements of sets in normal form

# Construction / Recognition
**Finding the transposition number between two sets:**
1. Put both sets in normal form
2. Verify they have the same interval succession
3. Subtract any element of the first set from the corresponding element of the second: n = (y - x) mod 12
4. All pairs of corresponding elements should yield the same n

# Context & Application
The choice of transposition level often has compositional significance. Composers frequently choose transposition numbers that duplicate intervals found within the original set, creating unity between small-scale and large-scale intervallic relationships. The transposition number is conceptually a *difference* between pitch classes (contrast with the index number, which is a *sum*).

# Examples
**Example 2-11** (p. 67, Stravinsky, *Agon*): Two sets in normal form [7, 8, 10, 11] and [10, 11, 1, 2] have the same interval succession 1-2-1. Corresponding elements: 10-7=3, 11-8=3, 1-10=3 (mod 12), 2-11=3 (mod 12). Therefore Set 2 = T3(Set 1).

# Relationships
## Builds Upon
- **Transposition (Tn)** -- the transposition number is the n in Tn
## Enables
- **Inverse transposition** -- inverse pairs have numbers summing to 12
## Related
- **Normal form** -- facilitates finding n by aligning corresponding elements
## Contrasts With
- **Index number** -- the index number (n in In) is a *sum* of mapped pitch classes; the transposition number is a *difference*

# Common Errors
- **Error**: Using negative numbers. **Correction**: Always express as a positive integer 0--11 using mod 12.

# Common Confusions
- **Confusion**: Transposition number vs. index number. **Clarification**: For Tn, n = y - x (a difference). For In, n = x + y (a sum). These measure fundamentally different things.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3, pages 62--69.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: clearly defined through formula and examples
- Re-extraction notes: preserved old card's inverse pairs list; upgraded to v3 template with explicit contrast to index number
