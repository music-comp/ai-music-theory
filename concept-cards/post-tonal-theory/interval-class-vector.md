---
# === CORE IDENTIFICATION ===
concept: Interval-Class Vector
slug: interval-class-vector

# === CLASSIFICATION ===
category: intervals
subcategory: interval class
tier: foundational

# === PROVENANCE ===
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 32
section: "1.12 Interval-Class Vector"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - ic vector
  - interval vector

# === TYPED RELATIONSHIPS ===
prerequisites:
  - interval-class
  - interval-class-content
extends:
  - interval-class-content
related:
  - pitch-class
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an interval-class vector?"
  - "How do I calculate an interval-class vector?"
  - "How do I read an interval-class vector?"
---

# Quick Definition
An interval-class vector is a six-digit string summarizing the interval-class content of a pitch-class collection, with each position giving the count of interval classes 1 through 6.

# Core Definition
An interval-class vector is a summary of the interval-class content of a collection of pitch classes, written as a six-element string of numbers with no spaces. The first number gives the count of ic 1, the second gives ic 2, and so on through ic 6. The vector is typically enclosed in square brackets: [ic1 ic2 ic3 ic4 ic5 ic6]. It can be constructed for collections of any size by methodically extracting all intervals between all pairs of pitch classes.

# Prerequisites
- **Interval Class** — the six non-zero interval classes whose counts form the vector
- **Interval-Class Content** — the ic vector is the standard format for representing ic content

# Key Properties
1. Exactly six positions, one per non-zero interval class (ic 1 through ic 6)
2. ic 0 (unisons) is excluded
3. Can be computed for any collection of any size
4. All members of a set class share the same vector (except Z-related pairs)
5. The sum of all entries equals n(n-1)/2 for a collection of n pitch classes

# Construction / Recognition
## To Construct:
1. List all pitch classes in the collection
2. Extract intervals formed with the first pitch class and all others
3. Then intervals formed with the second and all remaining, and so on
4. Classify each interval by interval class (1-6)
5. Tally counts into a six-position string

## To Recognize:
1. A string of six numbers in brackets, e.g., [254361]
2. Position i gives the count of interval class i

# Context & Application
The interval-class vector provides a compact fingerprint for identifying and comparing sonorities. It reveals characteristic features such as intervallic saturation or absence. The vector of the major scale ([254361]) has the rare property that each entry is a different number -- only three other collections share this property. The interval-class vector is central to set-class identification and to comparing sonorities across different musical contexts.

# Examples
**Example 1-19** (p. 30): Schoenberg, Three Piano Pieces, op. 11, no. 1: {B, G#, G} has vector [101100] (one ic 1, one ic 3, one ic 4).

**Example 1-20** (p. 31): Stravinsky, The Rake's Progress: chord vector [010020] (one ic 2, two ic 5s).

**Example 1-21** (p. 31): Varese, Density 21.5: melodic cell vector [100011] (one ic 1, one ic 5, one ic 6).

**Example 1-22** (p. 32): The major scale vector [254361]: 2 semitones, 5 whole tones, 4 minor thirds, 3 major thirds, 6 perfect fourths/fifths, 1 tritone. This vector has all different values -- an extremely rare property.

# Relationships
## Builds Upon
- **Interval-Class Content** — the vector is the standard notation for ic content
- **Interval Class** — the units being counted

## Enables
- **Set-Class Identification** — the vector helps characterize set classes
- **Z-Relation** — identified when two different set classes share the same vector

## Related
- **Pitch-Class Set** — the collection whose vector is calculated

## Contrasts With
- (no direct contrast within this chapter)

# Common Errors
- **Error**: Including ic 0 in the vector
  **Correction**: The vector has exactly six positions for ic 1-6. Unisons (ic 0) are excluded.

- **Error**: Counting only some pairs of pitch classes
  **Correction**: Every pair must be counted. Systematically work through all combinations.

# Common Confusions
- **Confusion**: Thinking the vector positions represent ordered pitch-class intervals
  **Clarification**: The vector counts unordered pitch-class intervals (interval classes), not ordered intervals. Position 1 counts all ic 1 occurrences, whether the ordered interval is 1 or 11.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.12, pp. 32-33.

# Verification Notes
- Definition source: direct (Straus Section 1.12 and "IN BRIEF" box)
- Confidence rationale: explicit definition with complete worked example (major scale)
- Re-extraction notes: Re-extracted from v2 card; preserved: all four vector examples (Schoenberg, Stravinsky, Varese, major scale), unique-multiplicity property
