---
concept: Literal Subset
slug: literal-subset
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
  - "literal containment"
prerequisites:
  - subset-superset-relation
extends:
  - subset-superset-relation
related:
  - abstract-subset
  - literal-complement
contrasts_with:
  - abstract-subset
answers_questions:
  - "What is a literal subset?"
  - "How does literal subset differ from abstract subset?"
---

# Quick Definition
A set whose specific pitch classes are all contained within another set; literal subsets share actual pitch-class content with their supersets.

# Core Definition
Set X is a literal subset of Set Y if all pitch classes in X are also in Y. This is the most direct form of the subset relationship: the exact pitch classes of X appear among those of Y. Transposing or inverting X may remove it from being a literal subset of Y while preserving abstract subset status. Set Y is correspondingly the literal superset of X (Straus, pp. 139-141).

# Prerequisites
- **Subset and superset relation** -- the general framework

# Key Properties
1. Every pitch class of X must be in Y
2. The relationship depends on specific pitch-class content, not set class
3. Transposition or inversion may destroy literal subset status
4. A literal subset is always also an abstract subset (but not vice versa)

# Construction / Recognition
X is a literal subset of Y if and only if every element of X is in Y.

Example: Y = [E, F, G, Bb], X = [E, F, G]. All three pcs of X are in Y, so X is a literal subset.

# Context & Application
Literal subsets identify how smaller motives are literally present within larger harmonies. Registral projections (highest notes, lowest notes) form literal subsets. Voice-leading analysis tracks which literal subsets persist as the harmony changes.

# Examples
**Example 1** (p. 140, Ex. 3-33): [E, F, G] is a literal subset of [E, F, G, Bb] -- all three pcs appear in both. But T5([E, F, G]) = [A, Bb, C] is NOT a literal subset of [E, F, G, Bb] -- A and C are not in the superset.

**Example 2** (p. 138, Ex. 3-31): Schoenberg, op. 19, no. 2 -- the highest 4 notes of the (014589) chord form a literal subset belonging to (0148); the lowest 3 notes form a literal subset that is an augmented triad (048).

**Example 3** (p. 139, Ex. 3-32): Schoenberg, Ode to Napoleon -- [G, Bb, D] is a literal subset of [D, Eb, F#, G, Bb, B], projecting a G minor triad.

# Relationships
## Builds Upon
- **Subset and superset relation** -- the general concept

## Related
- **Literal complement** -- analogous "literal" specificity for complements

## Contrasts With
- **Abstract subset** -- which allows T/I transformations

# Common Errors
- Thinking transposition preserves literal subset status (it changes which pcs are present)
- Confusing literal subsets with abstract subsets

# Common Confusions
- A set has many literal subsets, most of which are not musically relevant
- Literal subset is about pitch-class identity, not set-class membership

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.8.4, pp. 139-141

# Verification Notes
Upgraded from old v2 card. Preserved [E, F, G] example and both Schoenberg examples. Added v3 template fields.
