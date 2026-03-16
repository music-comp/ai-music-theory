---
concept: Ordered Pitch-Class Interval
slug: ordered-pitch-class-interval

category: intervals
subcategory: interval types
tier: foundational

source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 26
section: "1.8 Ordered Pitch-Class Intervals"

extraction_confidence: high

aliases:
  - opci
  - directed pitch-class interval

prerequisites:
  - pitch-class
  - pitch-class-interval
  - pitch-class-clockface
  - mod-12-arithmetic
extends:
  - pitch-class-interval
related:
  - complementary-intervals
contrasts_with:
  - unordered-pitch-class-interval
  - ordered-pitch-interval

answers_questions:
  - "What is an ordered pitch-class interval?"
  - "What distinguishes ordered from unordered pitch-class intervals?"
  - "How do I calculate an ordered pitch-class interval?"
---

# Quick Definition
An ordered pitch-class interval (opci) is the directed distance between two pitch classes, calculated by counting semitones clockwise on the pitch-class clockface from the first note to the second.

# Core Definition
An ordered pitch-class interval (abbreviated opci) is the interval between two pitch classes, calculated by counting the number of semitones on the pitch-class clockface from the first note to the second. Clockwise movement is equivalent to ascending; counterclockwise to descending. By convention, opci is usually denoted by a positive integer from 0 to 11, though intervals larger than 6 may be expressed as negative equivalents (7 = -5, 8 = -4, 9 = -3, 10 = -2, 11 = -1). The order of the two pitch classes matters: reversing the order produces the complement mod 12.

# Prerequisites
- **Pitch Class** — the elements between which opci is measured
- **Pitch-Class Interval** — opci is the ordered subtype
- **Pitch-Class Clockface** — the visual tool for calculation
- **Mod 12 Arithmetic** — the arithmetic framework

# Key Properties
1. Formula: opci from x to y = (y - x) mod 12
2. Range: 0 to 11 (positive convention) or equivalently -6 to 6
3. Reversing the order gives the complement: opci(x,y) + opci(y,x) = 12 (mod 12)
4. Negative equivalents: 7 = -5, 8 = -4, 9 = -3, 10 = -2, 11 = -1

# Construction / Recognition
## To Construct:
1. Identify the two pitch classes as integers (x and y)
2. Calculate (y - x) mod 12
3. Alternatively: on the clockface, count clockwise from x to y

## To Recognize:
1. An interval expressed as a positive integer 0-11 (or negative -1 to -6) between two pitch classes where order matters
2. Changing the order of the pitch classes changes the interval

# Context & Application
Ordered pitch-class intervals preserve directional information within mod 12 space. They are used for analyzing melodic motion and transformational relationships. They are essential for calculating transposition levels (T_n) and for describing the intervallic succession of twelve-tone rows.

# Examples
**Example 1-12** (p. 26): Calculating ordered pitch-class intervals:

| From | To | Calculation | opci |
|------|-----|-------------|------|
| C# (1) | Eb (3) | 3 - 1 = 2 | 2 |
| Eb (3) | C# (1) | 1 - 3 = -2 = 10 (mod 12) | 10 (or -2) |
| B (11) | F (5) | 5 - 11 = -6 = 6 (mod 12) | 6 |
| D (2) | Bb (10) | 10 - 2 = 8 | 8 (or -4) |
| Bb (10) | C (0) | 0 - 10 = -10 = 2 (mod 12) | 2 |

**Example 1-15a** (p. 28): In Schoenberg, String Quartet No. 3, first movement, the first melodic interval (B to Bb) is opci 11 (ascending major seventh or descending semitone). Subsequent intervals C#-D and F-F# are opci 1. As ordered intervals they differ; as unordered intervals all three are ic 1.

# Relationships
## Builds Upon
- **Pitch-Class Interval** — opci adds direction to pitch-class intervals

## Enables
- **Complementary Intervals** — opci pairs that sum to 12
- **Twelve-Tone Interval Succession** — ordered pcis define a row's intervallic identity
- **Spacing and Register** — spacing intervals use ordered pitch-class intervals

## Related
- **Complementary Intervals** — reversing pitch-class order yields the complement

## Contrasts With
- **Unordered Pitch-Class Interval** — takes the shorter path, ignoring direction (max 6)
- **Ordered Pitch Interval** — preserves register and compound intervals (not mod 12)

# Common Errors
- **Error**: Forgetting that order matters
  **Correction**: The opci from C# to Eb (= 2) differs from Eb to C# (= 10). Always specify which pitch class comes first.

# Common Confusions
- **Confusion**: Confusing opci 11 with a large ascending interval
  **Clarification**: opci 11 can represent either an ascending major seventh or a descending semitone. Both are the same in pitch-class space. The negative equivalent (-1) may make the descending semitone more intuitive.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.8, pp. 26-28.

# Verification Notes
- Definition source: direct (Straus Section 1.8 and "IN BRIEF" box)
- Confidence rationale: explicit formula, complete worked examples, and musical illustration
- Re-extraction notes: Re-extracted from v2 card; preserved: calculation table, Schoenberg opci 11 vs opci 1 example, negative equivalents
