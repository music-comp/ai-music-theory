---
# === CORE IDENTIFICATION ===
concept: Ordered Pitch Interval
slug: ordered-pitch-interval

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
  - opi
  - directed pitch interval

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch
  - pitch-interval
extends:
  - pitch-interval
related:
  - pitch-space
contrasts_with:
  - unordered-pitch-interval
  - ordered-pitch-class-interval

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an ordered pitch interval?"
  - "What distinguishes ordered from unordered pitch intervals?"
---

# Quick Definition
An ordered pitch interval (opi) measures the directed distance between two pitches in semitones, with + for ascending and - for descending motion.

# Core Definition
An ordered pitch interval (abbreviated opi) specifies both the size and direction of the interval between two pitches. A plus sign indicates an ascending interval and a minus sign indicates a descending interval. Ordered pitch intervals are also called directed pitch intervals. They capture the contour of a melodic line by indicating whether each successive interval rises or falls.

# Prerequisites
- **Pitch** — ordered pitch intervals connect specific pitches
- **Pitch Interval** — ordered pitch interval is a subtype of pitch interval

# Key Properties
1. Includes both magnitude (number of semitones) and direction (+/-)
2. + indicates ascending; - indicates descending
3. Can be any integer (positive or negative), not limited to 0-11
4. Preserves compound intervals without reduction
5. opi from A to B = -(opi from B to A)

# Construction / Recognition
## To Construct:
1. Identify two pitches in order (first, second)
2. Count semitones from first to second
3. If second is higher: prepend +
4. If second is lower: prepend -

## To Recognize:
1. An interval with a + or - sign and a semitone count
2. Specifies both distance and direction

# Context & Application
Ordered pitch intervals are essential for analyzing melodic contour and the specific shape of a musical line. They distinguish between ascending and descending motion of the same size and preserve compound intervals without reduction. Use ordered pitch intervals when contour matters to the analysis.

# Examples
**Example 1-11** (p. 25): In Schoenberg, String Quartet No. 3, first movement, a melodic line analyzed as ordered pitch intervals: <+1, -11, -1, +3, -13, +1>. These intervals reveal the alternation between small ascending motions and larger descending leaps, capturing the contour of the line.

**From the text** (p. 25): "When we say that C4 ascends four semitones to E4, we are talking about an ordered pitch interval (opi = +4)."

# Relationships
## Builds Upon
- **Pitch Interval** — ordered pitch interval adds direction to pitch interval

## Enables
- **Contour analysis** — ordered pitch intervals describe the shape of melodic lines

## Related
- **Pitch Space** — ordered pitch intervals are measured in linear pitch space

## Contrasts With
- **Unordered Pitch Interval** — ignores direction, reports only absolute distance
- **Ordered Pitch-Class Interval** — directed but operates mod 12 in pitch-class space

# Common Errors
- **Error**: Omitting the direction sign
  **Correction**: An ordered pitch interval must include + or -. Without a sign, the interval is unordered.

# Common Confusions
- **Confusion**: Conflating opi with opci
  **Clarification**: An opi of +16 (ascending compound major third) preserves the compound interval; the corresponding opci reduces to 4 (mod 12). The two capture different levels of specificity.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.7, pp. 25-26.

# Verification Notes
- Definition source: direct (Straus Section 1.7)
- Confidence rationale: explicit definition with formula and musical example
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg interval sequence, C4-E4 example
