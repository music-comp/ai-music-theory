---
# === CORE IDENTIFICATION ===
concept: Generic Interval
slug: generic-interval

# === CLASSIFICATION ===
category: intervals
subcategory: null
tier: foundational

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "The Keyboard and the Grand Staff"
chapter_number: 4
pdf_page: null
section: "Generic Intervals (Interval Size)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "interval size"
  - "interval number"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch
  - staff
extends: []
related:
  - interval
  - interval-quality
  - melodic-interval
  - harmonic-interval
contrasts_with:
  - interval-quality

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a generic interval?"
  - "How do you count the size of an interval?"
  - "Why must you count the first note as one?"
---

# Quick Definition

The distance between two notes measured by counting letter names, always starting with the first note as "one."

# Core Definition

A **generic interval** (also called interval size) measures the distance between two pitches by counting the number of letter names spanned, including both the starting and ending notes. Generic intervals are named with ordinal numbers: unison (1), second (2), third (3), fourth (4), fifth (5), sixth (6), seventh (7), octave (8). The critical rule: always count the first note as "one," not zero. Generic intervals are independent of accidentals -- C to E and C to Eb are both generic thirds.

# Prerequisites

- **Pitch**: the notes being measured
- **Staff**: where intervals are visually counted as lines and spaces

# Key Properties

- Always count the first note as "one"
- Accidentals do not affect generic size (C-E and C-Eb are both thirds)
- Special names: "unison" (not "first") and "octave" (not "eighth")
- Size alone does not tell you the exact number of half steps -- quality is also needed

# Construction / Recognition

Count every line and space from the bottom note to the top note, inclusive. On a staff, intervals of the same generic size always span the same number of lines and spaces regardless of accidentals.

# Context & Application

Generic intervals provide a quick way to gauge melodic leaps and harmonic distances before considering exact quality. They are essential for building triads ("stacked thirds") and for understanding figured bass.

# Examples

- F to C: F(1), G(2), A(3), B(4), C(5) = a generic fifth
- C to E: C(1), D(2), E(3) = a generic third
- D to D (same pitch): unison
- Common mistake: counting F-G as "one" gives 4 instead of the correct 5

# Relationships

- **Leads to**: interval-quality, interval, simple-compound-intervals
- **See also**: scale-degree, triad, figured-bass

# Common Errors

- Counting the first note as zero instead of one (the most common mistake)
- Confusing generic size with specific quality (both C-E and C-Eb are generic thirds)

# Common Confusions

- Generic interval alone does not tell you the number of half steps -- you need quality too
- The interval from C to E and C to Eb are both generic thirds but different specific intervals (major vs. minor)

# Source Reference

Open Music Theory, Part I, Chapter 4: "The Keyboard and the Grand Staff"

# Verification Notes

Re-extracted from v2 card; preserved: counting rule emphasis, F-to-C worked example, distinction from interval quality.
