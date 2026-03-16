---
concept: Hexachordal Complement Theorem
slug: hexachordal-complement-theorem
category: set-theory
subcategory: complement
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 135
section: "3.7.4 Hexachords"
extraction_confidence: high
aliases:
  - "hexachord theorem"
  - "hexachordal complement property"
prerequisites:
  - complement-relation
  - complement-interval-content
extends:
  - complement-interval-content
related:
  - self-complementary-hexachords
  - z-relation
contrasts_with: []
answers_questions:
  - "Why do hexachords always share the same interval vector as their complement?"
  - "What is the hexachordal complement theorem?"
---

# Quick Definition
The theorem that any hexachord has exactly the same interval-class vector as its complement; this follows from the general complement interval formula because the size difference between complementary hexachords is zero.

# Core Definition
The Hexachordal Complement Theorem states that a hexachord always has an identical interval-class vector to its complement. This is a special case of the general complement interval property: since both sets have 6 notes, the size difference k = 12 - 2(6) = 0, so the complement has 0 more of each interval class. The consequence: if the hexachord maps to its complement via T or I, it is self-complementary; if not, it is Z-related to its complement. This relationship is fundamental to twelve-tone music (Straus, pp. 135-136).

# Prerequisites
- **Complement relation** -- the general framework
- **Complement interval content** -- the formula this theorem specializes

# Key Properties
1. For hexachords: k = 0, so vectors are identical
2. Every hexachord falls into one of two categories: self-complementary or Z-related to its complement
3. This is the only cardinality where complement vectors are exactly equal
4. The property is crucial for twelve-tone music (rows divide into complementary hexachords)

# Construction / Recognition
Derivation from the general formula:
- Set size n = 6, complement size = 12 - 6 = 6
- Size difference k = 12 - 2n = 12 - 12 = 0
- Vector difference = 0 for all interval classes
- Therefore: hexachord vector = complement vector

In the List: self-complementary hexachords appear alone; Z-related pairs face each other.

# Context & Application
The theorem explains why dividing twelve tones into two hexachords creates a balanced, unified sound: both halves have identical intervallic resources. It is the theoretical foundation for hexachordal combinatoriality and aggregate-based composition.

# Examples
**Example 1** (p. 136, Ex. 3-27): Crawford Seeger, String Quartet, third movement:
- Hexachord X and hexachord Y are literal complements
- They share the same interval-class vector (as the theorem guarantees)
- They are Z-related (not self-complementary, since no T or I maps one to the other)
- Hexachord Z = T7 of Y; X and Z are abstract complements sharing 4 notes

**Example 2**: sc6-1 (012345) is self-complementary. Its complement is also (012345). The identical vectors are guaranteed by the theorem.

**Example 3**: 6-Z3 (012356) and 6-Z36 (012347) are Z-related hexachords -- their complements belong to each other's set class, sharing vector [433221].

# Relationships
## Builds Upon
- **Complement interval content** -- this theorem is the k=0 special case

## Enables
- **Self-complementary hexachords** -- one outcome of the theorem
- **Hexachordal combinatoriality** -- built on the shared interval content

## Related
- **Z-relation** -- the other outcome when hexachords are not self-complementary

# Common Errors
- Thinking this property applies to other cardinalities with exact equality (it does not; only hexachords have k=0)
- Confusing identical vectors with identical set class (they are different things)

# Common Confusions
- The theorem is a special case of the general complement formula, not an independent fact
- Non-self-complementary hexachords are Z-related to their complement, not unrelated

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7.4, pp. 135-137

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger example and self-complementary/Z-related dichotomy. Added derivation from general formula and v3 template fields.
