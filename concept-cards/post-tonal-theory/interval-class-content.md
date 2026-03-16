---
concept: Interval-Class Content
slug: interval-class-content

category: intervals
subcategory: interval class
tier: foundational

source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 29
section: "1.11 Interval-Class Content"

extraction_confidence: high

aliases:
  - ic content
  - interval content

prerequisites:
  - interval-class
  - pitch-class
extends: []
related:
  - interval-class-vector
contrasts_with: []

answers_questions:
  - "What is interval-class content?"
  - "How do I determine the interval-class content of a collection?"
  - "How do I calculate an interval-class vector?"
---

# Quick Definition
The interval-class content of a pitch-class collection is the count of each interval class (1-6) that can be formed between all pairs of pitch classes in the collection.

# Core Definition
The interval-class content of a collection of pitch classes is the number of times each non-zero interval class can be formed between pitch classes in the collection. The sound quality of a collection can be roughly suggested by its interval-class content. Crucially, all intervals in a collection contribute to the overall sound, not just those formed by adjacent notes. The total number of intervals in a collection of n pitch classes is n(n-1)/2.

# Prerequisites
- **Interval Class** — the interval types being counted
- **Pitch Class** — the elements forming the collection

# Key Properties
1. Counts all six non-zero interval classes (ic 1 through ic 6)
2. Counts all pairs, not just adjacent pairs
3. Number of intervals in a collection of n pitch classes = n(n-1)/2
4. Different collections may share the same interval-class content (Z-relation)
5. Interval-class content characterizes the sonic fingerprint of a collection

# Construction / Recognition
## To Construct:
1. List all pairs of pitch classes in the collection
2. Determine the interval class for each pair
3. Tally the occurrences of each ic (1 through 6)

## To Recognize:
1. A tabulation or listing of how many of each ic appear in a collection
2. Six counts, one per non-zero interval class

# Context & Application
Interval-class content provides a fingerprint for the sonic character of a collection. Comparing interval-class content reveals why certain harmonies sound similar or different. Different composers tend to favor collections with different intervallic profiles: Schoenberg, Stravinsky, and Varese each gravitate toward characteristic interval-class contents.

# Examples
**Example 1-19** (p. 30): In Schoenberg, Three Piano Pieces, op. 11, no. 1, the three-note collection {B, G#, G} contains one each of ic 1, ic 3, and ic 4 (no ic 2, ic 5, or ic 6).

**Example 1-20** (p. 31): In Stravinsky, The Rake's Progress, Act I, chords contain only ic 2 and ic 5 -- a very different sonic profile.

**Example 1-21** (p. 31): In Varese, Density 21.5, melodic cells contain only ic 1, ic 5, and ic 6 -- yet another distinctive sound.

**Example 1-18** (p. 30): Number of intervals by collection size:

| Pitch Classes | Intervals |
|---|---|
| 2 | 1 |
| 3 | 3 |
| 4 | 6 |
| 5 | 10 |
| 6 | 15 |
| 7 | 21 |

# Relationships
## Builds Upon
- **Interval Class** — the unit being counted

## Enables
- **Interval-Class Vector** — the standard format for representing interval-class content

## Related
- **Set Class** — all members of a set class share the same interval-class content (excepting Z-related pairs)

## Contrasts With
- (no direct contrast within this chapter)

# Common Errors
- **Error**: Counting only adjacent intervals
  **Correction**: Count all pairs of pitch classes, not just those adjacent in pitch or time. A three-note collection has three intervals (1-2, 1-3, 2-3), all contributing to its sound.

# Common Confusions
- **Confusion**: Thinking interval-class content uniquely identifies a set class
  **Clarification**: Z-related set classes share the same interval-class content but have different prime forms. However, most set classes are uniquely identified by their ic content.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.11, pp. 29-31.

# Verification Notes
- Definition source: direct (Straus Section 1.11 and "IN BRIEF" box)
- Confidence rationale: explicit definition with multiple composer examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg/Stravinsky/Varese contrast, n(n-1)/2 formula, collection-size table
