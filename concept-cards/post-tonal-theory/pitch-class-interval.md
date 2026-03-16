---
# === CORE IDENTIFICATION ===
concept: Pitch-Class Interval
slug: pitch-class-interval

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
pdf_page: 26
section: "1.8 Ordered Pitch-Class Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - pci

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class
  - mod-12-arithmetic
  - pitch-class-space
extends: []
related:
  - ordered-pitch-class-interval
  - unordered-pitch-class-interval
  - pitch-class-clockface
contrasts_with:
  - pitch-interval

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a pitch-class interval?"
  - "What distinguishes ordered from unordered pitch-class intervals?"
  - "How do I calculate an interval-class vector?"
---

# Quick Definition
A pitch-class interval (pci) is the distance between two pitch classes, measured in modular pitch-class space and never exceeding 11 semitones.

# Core Definition
A pitch-class interval (abbreviated pci) is the distance between two pitch classes, created when moving from pitch class to pitch class in modular pitch-class space. Unlike pitch intervals (which can be arbitrarily large), pitch-class intervals can never exceed eleven semitones because no two pitch classes are more than eleven semitones apart in pitch-class space. Pitch-class intervals may be ordered (directed) or unordered (shortest distance). The pitch-class clockface is the most useful tool for calculating them.

# Prerequisites
- **Pitch Class** — pitch-class intervals measure distance between pitch classes
- **Mod 12 Arithmetic** — pitch-class intervals operate mod 12
- **Pitch-Class Space** — the modular space in which pitch-class intervals are measured

# Key Properties
1. Maximum value: 11 (ordered) or 6 (unordered)
2. Operates in mod 12 pitch-class space
3. Compound intervals reduce to within-octave equivalents (e.g., pi 15 becomes pci 3)
4. Can be ordered (direction matters) or unordered (shortest path)

# Construction / Recognition
## To Construct:
1. Identify two pitch classes by their integers
2. For ordered: calculate (y - x) mod 12
3. For unordered: take the smaller of (y - x) mod 12 and (x - y) mod 12

## To Recognize:
1. An interval expressed as an integer 0-11 (ordered) or 0-6 (unordered) between pitch classes
2. Register-independent: the interval between any C and any E is the same

# Context & Application
Pitch-class intervals provide an abstract measure of distance that removes registral information. This abstraction enables analysts to identify relationships between musical events in different registers. The four interval types (opi, upi, opci, upci/ic) offer a spectrum from most concrete to most abstract; which one to use depends on the musical relationship being examined.

# Examples
**Example 1-17** (p. 29): A single interval (+19 semitones) described four ways:
- opi = +19 (most specific: size and direction)
- upi = 19 (size only)
- opci = 7 or -5 (mod 12, with direction)
- upci = 5 or ic 5 (most abstract)

"None of these labels is better or more right than the others--it's just that some are more concrete and specific while others are more general and abstract."

# Relationships
## Builds Upon
- **Pitch Class** — the elements between which pci is measured
- **Mod 12 Arithmetic** — the arithmetic framework

## Enables
- **Ordered Pitch-Class Interval** — directed version
- **Unordered Pitch-Class Interval** — non-directed version (= interval class)
- **Interval-Class Content** — tallying all pcis in a collection
- **Interval-Class Vector** — compact summary of interval content

## Related
- **Pitch-Class Clockface** — the visual tool for calculating pcis

## Contrasts With
- **Pitch Interval** — operates in linear pitch space, preserves register and compound intervals

# Common Errors
- **Error**: Reporting a pitch-class interval larger than 11
  **Correction**: Always reduce mod 12. A pitch interval of 15 becomes pci 3.

# Common Confusions
- **Confusion**: Not distinguishing ordered from unordered pitch-class intervals
  **Clarification**: The opci from C# to A is 8; the opci from A to C# is 4. The unordered pci (interval class) between them is 4 regardless of order.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Sections 1.8-1.9, pp. 26-28.

# Verification Notes
- Definition source: direct (Straus Sections 1.8-1.9)
- Confidence rationale: explicit definition with formula and four-interval-type comparison
- Re-extraction notes: Re-extracted from v2 card; preserved: four-type comparison from Example 1-17, Straus quote about specificity levels
