---
concept: Contour Segment (CSEG)
slug: contour-segment
category: analysis
subcategory: contour
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 143
section: "3.10.1 Contour segment (CSEG)"
extraction_confidence: high
aliases:
  - "CSEG"
  - "contour segment"
prerequisites:
  - contour-relations
extends:
  - contour-relations
related:
  - cseg-class
contrasts_with:
  - pitch-class-set
answers_questions:
  - "What is a contour segment?"
  - "How do I determine the CSEG of a melody?"
---

# Quick Definition
An ordered succession of relative pitch heights, notated as integers in angle brackets where 0 = lowest note, 1 = next lowest, up to (n-1) = highest; CSEGs capture melodic shape independently of specific pitches or intervals.

# Core Definition
A contour segment (CSEG) represents the relative registral positions of notes in an ordered succession. Each note receives an integer based on its position from lowest (0) to highest (n-1), where n is the number of distinct notes. The CSEG is these integers in temporal order, enclosed in angle brackets. Different melodies with the same CSEG share a common contour even if they contain different intervals and set classes. CSEGs are ordered (like series), not unordered (like pc sets) (Straus, pp. 143-144).

# Prerequisites
- **Contour relations** -- the theoretical framework

# Key Properties
1. Uses integers 0 through n-1 for n distinct pitch levels
2. Enclosed in angle brackets <...> indicating order matters
3. Independent of specific pitches, pitch classes, and intervals
4. Two melodies can share a CSEG while differing in every other parameter
5. The highest integer = n-1 (one less than the number of distinct notes)

# Construction / Recognition
To determine a CSEG:
1. Identify the number of distinct pitch levels
2. Assign 0 to the lowest, 1 to next lowest, etc.
3. Write integers in temporal (performance) order
4. Enclose in angle brackets

Example: pitches C5-E4-D4-G5 (in order):
- D4 = 0 (lowest), E4 = 1, C5 = 2, G5 = 3 (highest)
- CSEG = <2103>

# Context & Application
CSEGs reveal hidden motivic connections between intervallically different melodies. They are particularly useful for analyzing music with indeterminate pitch, graphic notation, or gestural composition. In fully determined music, contour analysis adds a perceptual dimension beyond pitch-class analysis.

# Examples
**Example 1** (p. 143, Ex. 3-36): Crawford Seeger, String Quartet, first movement -- three melodic fragments with CSEG <2013>. Each begins on its second-highest note, continues with lowest and second-lowest, concludes on highest. The fragments contain different intervals and set classes but share contour.

**Example 2** (p. 147, Ex. 3-40): Feldman, Projection No. 1 for Solo Cello -- first three pizzicato notes fall in middle, low, then high register = CSEG <102>. The same contour describes durational distances: medium-short-long.

# Relationships
## Builds Upon
- **Contour relations** -- the general framework

## Enables
- **CSEG-class** -- CSEGs are grouped into equivalence classes

## Contrasts With
- **Pitch-class set** -- unordered, uses square brackets; CSEGs are ordered, use angle brackets

# Common Errors
- Using actual pitch numbers instead of relative position numbers
- Confusing CSEGs (ordered, angle brackets) with pc sets (unordered, square brackets)

# Common Confusions
- CSEG captures order AND relative height, not just one or the other
- CSEG does not require specific pitches -- only relative relationships

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.1, pp. 143-144

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger and Feldman examples. Added explicit construction algorithm and v3 template fields.
