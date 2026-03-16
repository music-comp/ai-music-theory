---
concept: Unordered Pitch-Class Interval
slug: unordered-pitch-class-interval

category: intervals
subcategory: interval types
tier: foundational

source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 27
section: "1.9 Unordered Pitch-Class Intervals"

extraction_confidence: high

aliases:
  - upci
  - interval class

prerequisites:
  - pitch-class
  - pitch-class-interval
  - pitch-class-clockface
  - mod-12-arithmetic
extends:
  - pitch-class-interval
related:
  - interval-class
  - complementary-intervals
contrasts_with:
  - ordered-pitch-class-interval
  - unordered-pitch-interval

answers_questions:
  - "What is an unordered pitch-class interval?"
  - "What distinguishes ordered from unordered pitch-class intervals?"
  - "How does interval class relate to unordered pitch-class interval?"
---

# Quick Definition
An unordered pitch-class interval (upci) is the shortest distance between two pitch classes on the clockface, ranging from 0 to 6 semitones, regardless of order.

# Core Definition
An unordered pitch-class interval (abbreviated upci) measures the shortest distance between two pitch classes, regardless of the order in which they occur. Count from one pitch class to the other by the shortest available route, either clockwise or counterclockwise on the pitch-class clockface. Since one never has to travel farther than six semitones to reach any pitch class, 6 is the largest unordered pitch-class interval. An unordered pitch-class interval is synonymous with interval class (ic).

# Prerequisites
- **Pitch Class** — the elements between which upci is measured
- **Pitch-Class Interval** — upci is the unordered subtype
- **Pitch-Class Clockface** — the visual tool for shortest-path calculation
- **Mod 12 Arithmetic** — the arithmetic framework

# Key Properties
1. Formula: upci = (x - y) mod 12 or (y - x) mod 12, whichever is smaller
2. Range: 0 to 6 (the maximum shortest distance)
3. Order does not matter: upci(A, B) = upci(B, A)
4. Synonymous with interval class (ic)
5. There are only seven possible values (0 through 6)

# Construction / Recognition
## To Construct:
1. Calculate both (y - x) mod 12 and (x - y) mod 12
2. Take the smaller result
3. Alternatively: on the clockface, count the shorter way around from one pitch class to the other

## To Recognize:
1. An interval expressed as an integer 0-6 between two pitch classes
2. Order of the pitch classes does not affect the result

# Context & Application
Unordered pitch-class intervals (= interval classes) are the most abstract way to describe intervallic relationships. They identify a "type" of interval without regard to direction or registral placement. Interval class 1 encompasses all semitones, major sevenths, minor ninths, and their compounds. This abstraction is central to set-class theory and interval-class vector calculations.

# Examples
**Example 1-14** (p. 27): Calculating unordered pitch-class intervals:

| Between | Calculation | upci |
|---------|-------------|------|
| C# and Eb | 3-1=2 or 1-3=10; smaller is 2 | 2 |
| Eb and C# | same calculation | 2 |
| B and F | 5-11=-6=6 or 11-5=6 | 6 |
| D and Bb | 10-2=8 or 2-10=4; smaller is 4 | 4 |

**Example 1-15b** (p. 28): In Schoenberg, String Quartet No. 3, two statements of opci 4 are balanced by a concluding opci 8; all three represent the same unordered pitch-class interval (upci 4 = ic 4).

# Relationships
## Builds Upon
- **Pitch-Class Interval** — upci removes direction from pitch-class intervals

## Enables
- **Interval Class** — upci is synonymous with ic
- **Interval-Class Content** — counting upcis in a collection
- **Interval-Class Vector** — compact summary of upci content

## Related
- **Complementary Intervals** — opci pairs that share the same upci

## Contrasts With
- **Ordered Pitch-Class Interval** — preserves direction (range 0-11)
- **Unordered Pitch Interval** — preserves actual registral distance (not mod 12)

# Common Errors
- **Error**: Reporting an unordered pitch-class interval larger than 6
  **Correction**: Always take the smaller of the two complementary values. If you get 8, the upci is 4.

# Common Confusions
- **Confusion**: Thinking upci and ic are different concepts
  **Clarification**: They are the same thing. "An unordered pitch-class interval is also called an interval class" (Straus).

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.9, pp. 27-28.

# Verification Notes
- Definition source: direct (Straus Section 1.9 and "IN BRIEF" box)
- Confidence rationale: explicit formula, worked examples, and synonymy with ic clearly stated
- Re-extraction notes: Re-extracted from v2 card; preserved: calculation table, Schoenberg example, synonymy with interval class
