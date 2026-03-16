---
concept: Self-Complementary Hexachords
slug: self-complementary-hexachords
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
  - "self-complementary hexachord"
prerequisites:
  - complement-relation
  - hexachordal-complement-theorem
extends:
  - hexachordal-complement-theorem
related:
  - z-relation
contrasts_with: []
answers_questions:
  - "What is a self-complementary hexachord?"
  - "Why must non-self-complementary hexachords be Z-related to their complement?"
---

# Quick Definition
A hexachord whose complement belongs to the same set class; such hexachords can map onto their complements under transposition or inversion.

# Core Definition
A self-complementary hexachord is one whose literal complement is a member of the same set class. This means some Tn or In maps the hexachord onto its complement. In the List of Set Classes, self-complementary hexachords have nothing listed across from them. If a hexachord is not self-complementary, it must be Z-related to its complement, because all hexachords share identical interval-class vectors with their complements (since the size difference is zero). Non-self-complementary hexachords appear across from their Z-correspondent (which is their complement's set class) (Straus, pp. 135-136).

# Prerequisites
- **Complement relation** -- the underlying framework
- **Hexachordal complement theorem** -- explains why hexachord and complement have identical vectors

# Key Properties
1. The complement is a member of the same set class
2. Some T or I maps the hexachord onto its complement
3. Listed alone (nothing across) in the List of Set Classes
4. Non-self-complementary hexachords are necessarily Z-related to their complement
5. This dichotomy (self-complementary or Z-related) is exhaustive for hexachords

# Construction / Recognition
Simple example: [2, 3, 4, 5, 6, 7] = sc6-1 (012345). Its literal complement = [8, 9, 10, 11, 0, 1], also sc6-1 (012345). Both belong to the same set class, so 6-1 is self-complementary.

In the List: if a hexachord has nothing across from it, it is self-complementary. If it has another hexachord across from it, those two are Z-related (non-self-complementary).

# Context & Application
The self-complementary/Z-related dichotomy is fundamental to twelve-tone music. When a twelve-tone row is divided into two hexachords, those hexachords always have the same interval content. If they are self-complementary, they belong to the same set class; if not, they are Z-related. This affects the harmonic unity of the row's hexachordal areas.

# Examples
**Example 1** (p. 135): sc6-1 (012345): [2, 3, 4, 5, 6, 7] and its complement [8, 9, 10, 11, 0, 1] are both members of sc6-1. Self-complementary.

**Example 2** (p. 136, Ex. 3-27): Crawford Seeger, String Quartet, third movement -- hexachords X and Y are literal complements but NOT self-complementary (not related by T or I). They are Z-related, sharing the same interval-class vector.

# Relationships
## Builds Upon
- **Hexachordal complement theorem** -- the property making this dichotomy possible

## Related
- **Z-relation** -- non-self-complementary hexachords are Z-related to their complement

# Common Errors
- Thinking all hexachords are self-complementary (only those mappable to their complement by T or I)
- Not recognizing the exhaustive dichotomy: every hexachord is either self-complementary or Z-related to its complement

# Common Confusions
- "Self-complementary" does not mean the hexachord contains its own complement (that is impossible) -- it means the complement belongs to the same set class
- The Z-relation between non-self-complementary hexachords is a consequence of the hexachordal complement theorem

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7.4, pp. 135-137

# Verification Notes
New card extracted from source. Content was briefly mentioned in hexachordal-complement-theorem card but deserves its own card for the self-complementary/Z-related dichotomy.
