---
concept: Abstract Subset
slug: abstract-subset
category: set-theory
subcategory: inclusion
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 139
section: "3.8.4 Literal and abstract subsets and supersets"
extraction_confidence: high
aliases:
  - "abstract containment"
prerequisites:
  - subset-superset-relation
  - set-class
extends:
  - subset-superset-relation
related:
  - literal-subset
  - abstract-complement
  - inclusion-lattice
contrasts_with:
  - literal-subset
answers_questions:
  - "What is an abstract subset?"
  - "How does abstract subset differ from literal subset?"
---

# Quick Definition
A set-class relationship where some transposed or inverted form of Set X is contained within Set Y; abstract subsets preserve set-class inclusion even when specific pitch classes do not match.

# Core Definition
Set X is an abstract subset of Set Y if any Tn(X) or In(X) is a literal subset of Y -- that is, if any member of the set class containing X appears among Y's literal subsets. This generalizes the subset concept to the set-class level. We say sc(X) is an abstract subset of sc(Y) when any member of sc(Y) contains at least one literal subset belonging to sc(X). Abstract subset relations persist under transposition and inversion of the superset (Straus, pp. 139-141).

# Prerequisites
- **Subset and superset relation** -- the general framework
- **Set class** -- abstract subset operates at this level

# Key Properties
1. A set-class-level relationship, not pitch-class-level
2. If Tn(X) or In(X) is literally in Y, X is an abstract subset
3. The relationship persists under T and I of the superset
4. Inclusion lattices display abstract subset relationships
5. Abstract subset is a weaker condition than literal subset

# Construction / Recognition
To determine if X is an abstract subset of Y:
1. List all literal subsets of Y at the same cardinality as X
2. Put each in prime form
3. If the prime form of X matches any of them, X is an abstract subset

# Context & Application
Abstract subsets describe systematic relationships between set classes. If sc(X) is an abstract subset of sc(Y), forms of X can always be found within forms of Y, regardless of transposition level. This is the relationship that inclusion lattices display.

# Examples
**Example 1** (p. 140, Ex. 3-33): [E, F, G] (sc 013) is a literal subset of [E, F, G, Bb] (sc 0136). [A, Bb, C] (also sc 013) is NOT a literal subset, but IS an abstract subset -- it is T5 of [E, F, G], and sc(013) appears among the subset classes of sc(0136).

**Example 2** (p. 138, Ex. 3-30): In the inclusion lattice for (014589), sc(048) is an abstract subset of both (0148) and (014589). This holds for all members of these set classes.

# Relationships
## Builds Upon
- **Subset and superset relation** -- the concept being generalized

## Related
- **Inclusion lattice** -- displays abstract subset relationships
- **Abstract complement** -- analogous abstraction for complements

## Contrasts With
- **Literal subset** -- which requires specific pitch-class match

# Common Errors
- Confusing abstract with literal subset (abstract does not require pc match)
- Thinking [A, Bb, C] has no subset relation to [E, F, G, Bb] (it has an abstract relation)

# Common Confusions
- Abstract subset is a set-class concept -- it says "every member of sc(Y) contains some member of sc(X)"
- Not all members of a superset class contain the same number of instances of each subset type

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.8.4, pp. 139-141

# Verification Notes
Upgraded from old v2 card. Preserved [E, F, G] / [A, Bb, C] example and inclusion lattice reference. Added v3 template fields.
