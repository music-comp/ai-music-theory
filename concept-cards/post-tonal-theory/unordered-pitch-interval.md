---
# === CORE IDENTIFICATION ===
concept: Unordered Pitch Interval
slug: unordered-pitch-interval

# === CLASSIFICATION ===
category: intervals
subcategory: interval types
tier: foundational

# === PROVENANCE ===
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 25
section: "1.7 Pitch Intervals (Ordered and Unordered)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - upi

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch
  - pitch-interval
extends:
  - pitch-interval
related:
  - pitch-space
  - spacing-and-register
contrasts_with:
  - ordered-pitch-interval
  - unordered-pitch-class-interval
  - interval-class

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an unordered pitch interval?"
  - "What distinguishes ordered from unordered pitch intervals?"
---

# Quick Definition
An unordered pitch interval (upi) measures the absolute distance in semitones between two pitches, without indicating direction.

# Core Definition
An unordered pitch interval (abbreviated upi) expresses only the absolute space between two pitches, without regard to direction (ascending or descending). It is always expressed as a positive number representing the number of semitones separating the two pitches. The unordered pitch interval between C4 and E4 is 4, regardless of which note comes first.

# Prerequisites
- **Pitch** — unordered pitch intervals measure distance between specific pitches
- **Pitch Interval** — unordered pitch interval is a subtype of pitch interval

# Key Properties
1. Always a non-negative integer (absolute value of the directed distance)
2. Direction is ignored: upi(A, B) = upi(B, A)
3. Preserves compound intervals (upi = 13 is not reduced to 1)
4. Calculated as |pitch2 - pitch1| in semitone values

# Construction / Recognition
## To Construct:
1. Identify two pitches
2. Count semitones between them
3. Report the absolute value (no +/- sign)

## To Recognize:
1. A positive integer without a direction sign
2. Represents spacing between two pitches regardless of order

# Context & Application
Unordered pitch intervals are useful when analyzing the spacing or distribution of pitches without concern for direction. They describe the distances between notes in a chord or the registral span of a musical passage, focusing on "how far apart" rather than "which direction."

# Examples
**Example 1-11** (p. 25): In Schoenberg, String Quartet No. 3, first movement, the same melodic passage analyzed for unordered pitch intervals: (1, 11, 1, 3, 13, 1). These intervals ignore contour and focus on the distances between adjacent notes, revealing that the line is dominated by small intervals (1 and 3) with occasional large leaps (11 and 13).

**From the text** (p. 25): "When we say that there are four semitones between C4 and E4, we are talking about an unordered pitch interval (upi = 4)."

# Relationships
## Builds Upon
- **Pitch Interval** — unordered pitch interval removes direction from pitch interval

## Enables
- **Spacing analysis** — unordered pitch intervals describe chord voicings

## Related
- **Pitch Space** — unordered pitch intervals are measured in linear pitch space
- **Spacing and Register** — uses pitch intervals to describe chord arrangement

## Contrasts With
- **Ordered Pitch Interval** — includes direction (+/-)
- **Unordered Pitch-Class Interval (Interval Class)** — reduces to mod 12 and takes the smaller complement (max 6)

# Common Errors
- **Error**: Reducing an unordered pitch interval by mod 12
  **Correction**: Unordered pitch intervals preserve actual registral distance. A upi of 13 remains 13; it becomes 1 only when converted to interval class.

# Common Confusions
- **Confusion**: Thinking upi and ic are the same
  **Clarification**: upi 13 (minor ninth) = ic 1 (semitone). Unordered pitch intervals measure actual distance in pitch space; interval classes operate in pitch-class space.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.7, pp. 25-26.

# Verification Notes
- Definition source: direct (Straus Section 1.7)
- Confidence rationale: explicit definition with worked example
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg interval sequence, distinction from interval class
