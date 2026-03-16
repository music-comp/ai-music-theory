---
# === CORE IDENTIFICATION ===
concept: Interval Class
slug: interval-class

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
pdf_page: 28
section: "1.10 Interval Class"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - ic
  - unordered pitch-class interval

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class
  - unordered-pitch-class-interval
  - complementary-intervals
  - octave-equivalence
extends:
  - unordered-pitch-class-interval
related:
  - interval-class-content
  - interval-class-vector
contrasts_with:
  - ordered-pitch-class-interval
  - pitch-interval

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an interval class?"
  - "How many interval classes are there?"
  - "How do I calculate an interval-class vector?"
---

# Quick Definition
An interval class (ic) is a category containing all pitch intervals related by octave equivalence and complementation mod 12, representing the most abstract way of describing intervals. There are seven interval classes (0-6).

# Core Definition
An interval class (abbreviated ic) is synonymous with unordered pitch-class interval. Just as each pitch class contains many individual pitches, each interval class contains many individual pitch intervals. Because of octave equivalence, compound intervals (larger than an octave) are equivalent to their within-octave counterparts. Furthermore, pitch-class intervals larger than 6 are equivalent to their complements mod 12 (the pairs 0/12, 1/11, 2/10, 3/9, 4/8, 5/7, 6/6). These pairs are what traditional tonal theory calls "inversions" (e.g., minor thirds and major sixths). All intervals within an interval class share a characteristic sound quality: "there is something distinctively 1-ish about all of the intervals belonging to interval-class 1" (Straus).

# Prerequisites
- **Pitch Class** — interval classes operate in pitch-class space
- **Unordered Pitch-Class Interval** — interval class is synonymous with upci
- **Complementary Intervals** — ic groups complementary intervals together
- **Octave Equivalence** — ic groups octave-equivalent intervals together

# Key Properties
1. There are exactly 7 interval classes: ic 0 through ic 6
2. ic 6 (tritone) is its own complement
3. Each ic contains infinitely many pitch intervals (related by octave equivalence and complementation)
4. All members of an ic share a characteristic sound quality
5. ic is the most abstract of the four interval types

# Construction / Recognition
## To Construct:
1. Take any pitch interval
2. Reduce mod 12 to get the pitch-class interval (0-11)
3. If the result is greater than 6, subtract from 12 to get the complement
4. The result (0-6) is the interval class

## To Recognize:
1. An interval labeled "ic" followed by a number 0-6
2. Any grouping of intervals related by octave equivalence and complementation

# Context & Application
Interval class provides the most abstract intervallic description, useful for characterizing the overall sonic quality of pitch-class collections. The four interval types offer a spectrum from most concrete (opi) to most abstract (ic). Which type to use depends on the analytical question being asked.

# Examples
**Example 1-16** (p. 29): The seven interval classes and some of their pitch-interval contents:

| ic | Pitch Intervals |
|----|-----------------|
| 0 | 0, 12, 24 |
| 1 | 1, 11, 13, 23 |
| 2 | 2, 10, 14, 22 |
| 3 | 3, 9, 15, 21 |
| 4 | 4, 8, 16, 20 |
| 5 | 5, 7, 17, 19 |
| 6 | 6, 18, 30 |

**Example 1-17** (p. 29): A single interval (+19 semitones) described four ways: opi = +19, upi = 19, opci = 7 (or -5), upci = 5 (= ic 5). "None of these labels is better or more right than the others."

# Relationships
## Builds Upon
- **Unordered Pitch-Class Interval** — ic is synonymous with upci
- **Complementary Intervals** — ic groups complements together
- **Octave Equivalence** — ic groups octave-equivalent intervals

## Enables
- **Interval-Class Content** — tallying ic occurrences in a collection
- **Interval-Class Vector** — compact representation of ic content

## Related
- **Interval-Class Content** — the count of each ic in a collection
- **Interval-Class Vector** — the standard format for reporting ic content

## Contrasts With
- **Ordered Pitch-Class Interval** — preserves direction (range 0-11)
- **Pitch Interval** — preserves register and compound intervals

# Common Errors
- **Error**: Thinking there are 12 interval classes
  **Correction**: There are only 7 (ic 0 through ic 6). Intervals larger than 6 reduce to their complement.

- **Error**: Forgetting that ic 5 contains both perfect fourths and perfect fifths
  **Correction**: pi 5 and pi 7 are both members of ic 5 because they are complements mod 12.

# Common Confusions
- **Confusion**: Confusing interval class with ordered pitch-class interval
  **Clarification**: Interval class always takes the shorter route around the clockface (max 6). Ordered pitch-class intervals can range from 0 to 11.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.10, pp. 28-29.

# Verification Notes
- Definition source: direct (Straus Section 1.10 and "IN BRIEF" box)
- Confidence rationale: explicit definition with complete ic content table and four-type comparison
- Re-extraction notes: Re-extracted from v2 card; preserved: ic content table, four-type comparison from Example 1-17, "1-ish" Straus quote
