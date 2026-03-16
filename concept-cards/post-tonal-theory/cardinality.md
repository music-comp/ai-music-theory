---
concept: Cardinality
slug: cardinality
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.8 List of Set Classes"
extraction_confidence: high
aliases:
  - set size
prerequisites:
  - pitch-class-set
extends:
  - pitch-class-set
related:
  - forte-names
  - list-of-set-classes
  - complement-relation
contrasts_with: []
answers_questions:
  - "What is the cardinality of a pitch-class set?"
  - "What are the standard names for sets of different sizes?"
---

# Quick Definition
Cardinality is the number of distinct pitch classes in a set, indicated by the first number in a Forte name and determining the set's basic size classification.

# Core Definition
The cardinality of a pitch-class set is the number of distinct pitch classes it contains. Sets are classified by cardinality using standard names: monad (1), dyad (2), trichord (3), tetrachord (4), pentachord (5), hexachord (6), septachord (7), octachord (8), nonachord (9), decachord (10), undecachord (11), aggregate (12). The first number in a Forte name indicates cardinality.

# Prerequisites
- **Pitch-class set** -- cardinality is a property of sets

# Key Properties
1. Ranges from 0 to 12
2. Determines the number of intervals: n(n-1)/2
3. First number in Forte name = cardinality
4. Cardinalities n and (12-n) have the same number of set classes (complement symmetry)
5. Most analytical work focuses on cardinalities 3--6

# Construction / Recognition
Count the distinct pitch classes in the set, ignoring doublings.

| Cardinality | Name | # of Intervals | # of Set Classes |
|:-----------:|:------------|:--------------:|:----------------:|
| 3 | trichord | 3 | 12 |
| 4 | tetrachord | 6 | 29 |
| 5 | pentachord | 10 | 38 |
| 6 | hexachord | 15 | 50 |
| 7 | septachord | 21 | 38 |
| 8 | octachord | 28 | 29 |
| 9 | nonachord | 36 | 12 |

# Context & Application
Cardinality is a basic classifier for sets. Most analytical work focuses on sets of cardinality 3--6, which provide rich intervallic content without approaching the aggregate. Larger sets (7+) are often analyzed in relation to their complements. The number of set classes at each cardinality shows a symmetrical pattern (complement relation).

# Examples
From the List of Set Classes:
- 220 distinct trichords form only 12 trichordal set classes
- 29 tetrachordal set classes
- 50 hexachordal set classes (the maximum)
- Nonachords (9) also have 12 set classes, matching trichords (complement symmetry)

# Relationships
## Builds Upon
- **Pitch-class set** -- cardinality is a property of a set
## Enables
- **Forte names** -- first number indicates cardinality
- **List of Set Classes** -- organized by cardinality
## Related
- **Complement relation** -- cardinalities n and (12-n) have the same number of set classes

# Common Errors
- **Error**: Counting notes instead of pitch classes. **Correction**: A chord with doubled notes may have fewer pitch classes than actual notes. Count distinct pitch classes only.

# Common Confusions
- **Confusion**: Cardinality determines set class. **Clarification**: Many different set classes share the same cardinality. Cardinality is necessary but not sufficient for identification.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.8, pages 85--86.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: cardinality counts and set-class counts explicitly stated
- Re-extraction notes: preserved old card's cardinality table with interval counts; upgraded to v3 template
