---
concept: Literal Complement
slug: literal-complement
category: set-theory
subcategory: complement
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 133
section: "3.7.2 Literal and abstract complements"
extraction_confidence: high
aliases:
  - "literal complementation"
prerequisites:
  - complement-relation
extends:
  - complement-relation
related:
  - abstract-complement
  - aggregate
contrasts_with:
  - abstract-complement
answers_questions:
  - "What distinguishes literal from abstract complement?"
  - "What is a literal complement?"
---

# Quick Definition
The specific collection of pitch classes excluded by a given set; two sets are literal complements if they share no pitch classes and together contain all twelve pitch classes.

# Core Definition
Two sets are literal complements if: (1) they have no pitch classes in common (empty intersection), and (2) together they contain all twelve pitch classes (union = aggregate). The literal complement of a set is unique. Literal complementation is stricter than abstract complementation, which considers set-class membership rather than specific pitch-class content (Straus, pp. 133-134).

# Prerequisites
- **Complement relation** -- the general framework

# Key Properties
1. No shared pitch classes between set and literal complement
2. Union = complete aggregate
3. Unique: each set has exactly one literal complement
4. Transposing a set changes its literal complement
5. Literal complements are always also abstract complements

# Construction / Recognition
To find the literal complement of set S:
- List all 12 pitch classes: {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11}
- Remove all members of S
- The remaining pitch classes = literal complement

# Context & Application
Literal complements appear when melody uses certain pitch classes while accompaniment uses all others, or when a twelve-tone row is divided into complementary hexachords. The literal complement guarantees maximum pitch-class contrast while maintaining intervallic similarity.

# Examples
**Example 1** (p. 131, Ex. 3-22): [2, 3, 6, 9] and [4, 5, 7, 8, 10, 11, 0, 1] are literal complements -- each contains the notes excluded by the other, and together they yield all 12 pitch classes.

**Example 2** (p. 136, Ex. 3-27): Crawford Seeger, String Quartet, third movement -- hexachords X and Y are literal complements, together creating the aggregate.

**Example 3** (p. 132, Ex. 3-24): Schoenberg, String Quartet No. 3 -- the 7-note melody and the 5-note ostinato are literal complements.

# Relationships
## Builds Upon
- **Complement relation** -- literal complement is the specific-pitch-class version

## Related
- **Aggregate** -- union of set and literal complement

## Contrasts With
- **Abstract complement** -- which allows shared pitch classes

# Common Errors
- Confusing literal complement with abstract complement
- Thinking transposition preserves literal complement status (it changes which specific pcs are excluded)

# Common Confusions
- Literal complements are always also abstract complements, but not vice versa
- By definition, literal complements share no common tones

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7.2, pp. 133-134

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger and Schoenberg examples. Tightened definition and added v3 template fields.
