---
concept: Complement Relation
slug: complement-relation
category: set-theory
subcategory: complement
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 131
section: "3.7 Complement Relation"
extraction_confidence: high
aliases:
  - "complementation"
  - "set complement"
prerequisites:
  - pitch-class-set
  - interval-class-vector
extends:
  - pitch-class-set
related:
  - literal-complement
  - abstract-complement
  - complement-interval-content
  - hexachordal-complement-theorem
  - self-complementary-hexachords
contrasts_with:
  - z-relation
answers_questions:
  - "How does the complement relation work?"
  - "What is the complement of a pitch-class set?"
---

# Quick Definition
The relationship between a set and the collection of all pitch classes it excludes; a set and its complement together contain all twelve pitch classes, and they share a proportionally similar distribution of intervals.

# Core Definition
For any pitch-class set, its complement contains all pitch classes not in the original. If a set has n pitch classes, its complement has 12 - n. Together they constitute the aggregate (all twelve pitch classes). The complement relation has a crucial intervallic property: complementary sets have strikingly similar interval distributions. The difference in occurrences of each interval class equals the difference in set sizes (except for ic6, where it is half the size difference) (Straus, pp. 131-137).

# Prerequisites
- **Pitch-class set** -- the object being complemented
- **Interval-class vector** -- reveals the proportional similarity

# Key Properties
1. Complement of an n-note set has 12 - n notes
2. A set and its complement together = the aggregate
3. Interval-class vector relationship: if size difference = k, the complement has k more of each ic (except ic6: k/2 more)
4. Forte-names identify complements: same number after the dash, numbers before the dash sum to 12
5. Complement-related set classes share the same degree of symmetry
6. Complements of Z-related sets are also Z-related

# Construction / Recognition
For a set with vector [a, b, c, d, e, f] of size n:
- Complement size = 12 - n
- Size difference k = (12 - n) - n = 12 - 2n
- Complement vector = [a+k, b+k, c+k, d+k, e+k, f+k/2]

In the List of Set Classes: complementary set classes are listed in the same row, across from each other. Forte-names: 4-18 and 8-18 are complements; 3-6 and 9-6 are complements.

# Context & Application
The complement relation is crucial in twelve-tone music and any music where the aggregate is a structural unit. When melody uses certain pitch classes and accompaniment uses the rest, their similar interval distributions create a unified sound despite maximum pitch-class contrast.

# Examples
**Example 1** (p. 131, Ex. 3-22/3-23): [2, 3, 6, 9] = sc4-18 (0147), vector [102111]. Its complement [4, 5, 7, 8, 10, 11, 0, 1] = sc8-18 (01235689), vector [546553]. The 8-note set has 4 more of each ic (except ic6: 2 more).

**Example 2** (p. 132, Ex. 3-24): Schoenberg, String Quartet No. 3 -- a 7-note melody [5, 6, 8, 10, 11, 1, 2] = sc7-32, vector [335442], is accompanied by a 5-note ostinato using the complementary pitch classes. The melody has 2 more of each ic (1 more ic6) than the ostinato.

# Relationships
## Builds Upon
- **Pitch-class set** -- the objects in the relationship

## Enables
- **Hexachordal complement theorem** -- the special case for hexachords
- **Self-complementary hexachords** -- hexachords mapping onto their complements

## Related
- **Literal complement** and **Abstract complement** -- the two modes of the relationship
- **Z-relation** -- complements of Z-correspondents are Z-related

## Contrasts With
- **Z-relation** -- complement relation is about excluded pitch classes; Z-relation is about shared interval content without T/I relationship

# Common Errors
- Thinking complementary sets have opposite interval content (they actually have similar content)
- Forgetting the tritone exception (ic6 difference = half the size difference)

# Common Confusions
- Literal vs. abstract complement is a crucial distinction (see those cards)
- Equal complement sizes are not the same as equal cardinalities (except for hexachords)

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7, pp. 131-137

# Verification Notes
Upgraded from old v2 card. Preserved Schoenberg String Quartet No. 3 example, interval vector formula, and Forte-name identification system. Added shared symmetry and Z-relation properties per source.
