---
concept: Z-Correspondent
slug: z-correspondent
category: set-theory
subcategory: Z-relation
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 129
section: "3.6.1 Z-correspondents"
extraction_confidence: high
aliases:
  - "Z-related partner"
prerequisites:
  - z-relation
  - forte-names
extends:
  - z-relation
related:
  - all-interval-tetrachords
  - list-of-set-classes
contrasts_with: []
answers_questions:
  - "What is a Z-correspondent?"
  - "How do I find a set class's Z-correspondent?"
---

# Quick Definition
The partner set class in a Z-related pair; any set class with a Z in its Forte-name has exactly one Z-correspondent with a different prime form but identical interval-class vector.

# Core Definition
A Z-correspondent is the other member of a Z-related pair. If set class X is Z-related to set class Y, then Y is the Z-correspondent of X (and vice versa). Z-correspondents have different prime forms, share the same interval-class vector, and cannot be related by T or I. In the List of Set Classes, Z-related hexachords are listed directly across from each other with their shared vector between them. For other cardinalities, Z-correspondents must be found by searching for matching vectors (Straus, p. 129).

# Prerequisites
- **Z-relation** -- the relationship between correspondents
- **Forte-names** -- the naming system that marks Z-relations with "Z"

# Key Properties
1. Z-correspondents come in pairs
2. Different prime forms, same interval-class vector
3. Indicated by "Z" in Forte-names (e.g., 4-Z15 and 4-Z29)
4. In the List, Z-related hexachords face each other
5. Complements of Z-correspondents are also Z-related: if 4-Z15 ~ 4-Z29, then 8-Z15 ~ 8-Z29

# Construction / Recognition
Z-correspondent pairs by cardinality:
- Tetrachords: 1 pair (4-Z15 and 4-Z29)
- Pentachords: 3 pairs
- Hexachords: 15 pairs
- (Septachords and octachords have corresponding pairs via the complement relation)

To find a Z-correspondent:
1. Identify the "Z" in the Forte-name
2. For hexachords: look across the row in the List
3. For others: search for matching interval-class vector

# Context & Application
Z-correspondents are compositionally useful for creating harmonic variety while maintaining intervallic consistency. Carter deliberately juxtaposes Z-correspondent tetrachords to create relationships that are neither transposition nor inversion.

# Examples
**Example 1** (p. 129): The all-interval tetrachord pair:
- 4-Z15 (0146): vector [111111]
- 4-Z29 (0137): vector [111111]

**Example 2**: Some hexachord Z-correspondent pairs:
- 6-Z3 (012356) and 6-Z36 (012347): vector [433221]
- 6-Z6 (012567) and 6-Z38 (012378): vector [421242]
- 6-Z11 (012457) and 6-Z40 (012358): vector [333231]

# Relationships
## Builds Upon
- **Z-relation** -- the underlying relationship

## Related
- **All-interval tetrachords** -- the most famous Z-correspondent pair
- **Complement relation** -- complements of Z-correspondents are Z-related

# Common Errors
- Expecting Z-correspondents to have the same number after the dash in Forte-names (they often differ)
- Looking for Z-correspondents among trichords (none exist)

# Common Confusions
- Z-correspondents share interval content but do not systematically share subset structure
- The Forte-name numbers after the dash may or may not match between Z-correspondents

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.6.1, p. 129

# Verification Notes
Upgraded from old v2 card. Preserved all Z-correspondent pair listings, complement-relation property, and search instructions. Added v3 template fields.
