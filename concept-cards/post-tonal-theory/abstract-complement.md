---
concept: Abstract Complement
slug: abstract-complement
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
  - "abstract complementation"
prerequisites:
  - complement-relation
  - set-class
extends:
  - complement-relation
related:
  - literal-complement
contrasts_with:
  - literal-complement
answers_questions:
  - "What distinguishes literal from abstract complement?"
  - "What is an abstract complement?"
---

# Quick Definition
The set-class relationship between two sets whose set classes are complement-related; abstract complements share similar interval distributions even when they have pitch classes in common.

# Core Definition
Two sets are abstract complements if they are members of complement-related set classes, regardless of whether they share specific pitch classes. Unlike literal complements (which must have no pcs in common), abstract complements can overlap in content while maintaining the proportional intervallic similarity that characterizes the complement relation. Abstract complementation is a set-class-level relationship: if sc(X) complements sc(Y), then any member of sc(X) is abstractly complementary to any member of sc(Y) (Straus, pp. 133-135).

# Prerequisites
- **Complement relation** -- the general framework
- **Set class** -- abstract complement operates at the set-class level

# Key Properties
1. Operates at the set-class level, not the pitch-class level
2. Abstract complements may share pitch classes (unlike literal complements)
3. The intervallic similarity persists regardless of overlap
4. Forte-names: same number after dash, numbers before dash sum to 12
5. The relationship is preserved under transposition and inversion

# Construction / Recognition
Sets A and B are abstract complements if:
- The set class of A is the complement of the set class of B
- Forte-name check: numbers after the dash match, numbers before the dash sum to 12
- Their vectors have the proportional relationship of complements

# Context & Application
Abstract complementation describes the more general and often more analytically significant relationship. It explains intervallic similarities between sets that share pitch classes, and it persists under all transpositions and inversions of the component sets.

# Examples
**Example 1** (p. 134, Ex. 3-25): Original: [2, 3, 6, 9] = sc4-18 and [4, 5, 7, 8, 10, 11, 0, 1] = sc8-18 are literal AND abstract complements. After transposing the 4-note set at T1 to [3, 4, 7, 10], it shares 3 notes with the 8-note set. They are no longer literal complements but remain abstract complements (members of complement-related set classes) with similar interval distributions.

**Example 2** (p. 134, Ex. 3-26): Schoenberg, Little Piano Pieces, op. 19, no. 2 -- the final 4-note chord sc4-19 (0148) is part of the final 8-note collection sc8-19 (01245689). They are abstract complements (obviously not literal, since the chord is contained within the collection). Both emphasize ic4 -- no 4- or 8-note set has more ic4s.

# Relationships
## Builds Upon
- **Complement relation** -- abstract complement is the set-class-level version

## Contrasts With
- **Literal complement** -- which requires no shared pitch classes

# Common Errors
- Thinking abstract complements cannot share pitch classes (they can)
- Assuming literal complements are analytically more important than abstract ones

# Common Confusions
- Abstract complement is a set-class relationship; literal complement is a pitch-class relationship
- The interval similarity holds even when sets overlap in content

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7.2, pp. 133-135

# Verification Notes
Upgraded from old v2 card. Preserved Schoenberg op. 19 and T1 transposition examples. Added emphasis on ic4 prominence per source.
