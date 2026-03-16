---
concept: Interval Vector
slug: interval-vector
category: analysis
subcategory: post-tonal-analysis
tier: advanced
source: "Music Theory for the 21st-Century Classroom"
source_slug: 21st-century-classroom
authors: "Robert Hutchinson"
chapter: "Set Theory"
chapter_number: 33
pdf_page: 483
section: "33.4 Interval Vector"
extraction_confidence: high
aliases:
  - "interval class content"
  - "IC vector"
prerequisites:
  - interval-class
  - normal-form
extends: []
related:
  - z-relations
  - forte-numbers
contrasts_with: []
answers_questions:
  - "What is an interval vector?"
  - "How do you calculate an interval vector?"
---

# Quick Definition
An interval vector is a six-digit representation showing how many of each interval class (1-6) occurs between all pairs of notes in a pitch-class set.

# Core Definition
An interval vector (also known as Interval Class Content) lists every possible interval occurring in a pitch-class set. It is always six digits, representing ic 1 through ic 6. To calculate: measure from each note to all subsequent notes, tally each interval class. A zero means that interval class does not occur (Hutchinson, Ch. 33, pp. 483-484).

# Prerequisites
- **Interval class** -- Vector tallies interval classes
- **Normal form** -- Start from normal form arrangement

# Key Properties
1. Always exactly six digits
2. Positions: [ic1, ic2, ic3, ic4, ic5, ic6]
3. Calculated by measuring all pairs of notes
4. Zero = interval class not present
5. Z-related sets share identical vectors despite different prime forms

# Construction / Recognition
**Steps:**
1. Arrange set in normal form
2. Measure from first note to all others (as interval classes)
3. Measure from second note to all subsequent notes
4. Continue for each note
5. Tally occurrences of each ic

**Example:** Half-diminished 7th (G-Bb-Db-F)
- G to Bb: ic 3; G to Db: ic 6; G to F: ic 2
- Bb to Db: ic 3; Bb to F: ic 5
- Db to F: ic 4
- Vector: 012111

# Context & Application
Interval vectors reveal the intervallic content of sets. Sets with similar vectors have similar sonic qualities. The half-diminished 7th chord (012111) has zero half steps, one M2, two m3s, no M3s, one P4/P5, one tritone.

# Examples
- Half-diminished 7th: 012111 (p. 484)
- Major triad (037): 001110
- Chromatic trichord (012): 210000

# Relationships
## Related
- **Z-relations** -- Z-related sets share identical interval vectors
- **Forte numbers** -- Tables include interval vectors alongside prime forms

# Common Errors
- **Error**: Reporting a vector with fewer or more than 6 digits
  **Correction**: Interval vectors always have exactly 6 digits

# Common Confusions
- **Confusion**: Thinking P5 goes in the ic 7 position
  **Clarification**: P5 = ic 5 (use shortest distance); there is no ic 7

# Source Reference
Chapter 33, Section 33.4, PDF pages 483-484. Half-diminished worked example.

# Verification Notes
- Calculation procedure from source, pp. 483-484
- Half-diminished example worked step by step in source
- Re-extracted from v2 card; preserved: worked half-diminished example, six-digit rule
- Confidence: HIGH -- source provides complete step-by-step procedure
