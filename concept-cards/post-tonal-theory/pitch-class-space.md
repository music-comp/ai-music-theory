---
# === CORE IDENTIFICATION ===
concept: Pitch-Class Space
slug: pitch-class-space

# === CLASSIFICATION ===
category: pitch
subcategory: null
tier: foundational

# === PROVENANCE ===
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 23
section: "1.5 Arithmetic modulo 12 (mod 12)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - modular pitch-class space

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class
  - mod-12-arithmetic
extends: []
related:
  - pitch-class-clockface
  - pitch-class-interval
  - interval-class
contrasts_with:
  - pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is pitch-class space?"
  - "How does pitch-class space differ from pitch space?"
---

# Quick Definition
Pitch-class space is a modular, circular space containing only the twelve pitch classes, where movement by twelve semitones returns to the starting point.

# Core Definition
Pitch-class space is a modular space, represented by the pitch-class clockface, that circles back on itself and contains only the twelve pitch classes. In this space, going up or down by twelve semitones leads to another member of the same pitch class. Pitch-class space can be understood as linear pitch space "wrapped around" onto a circular representation. It operates under mod 12 arithmetic, where all values reduce to integers 0-11.

# Prerequisites
- **Pitch Class** — the twelve elements that constitute pitch-class space
- **Mod 12 Arithmetic** — the mathematical framework governing pitch-class space

# Key Properties
1. Contains exactly 12 elements (pitch classes 0-11)
2. Cyclical: pc + 12 = pc (mod 12)
3. No inherent direction preference: moving 7 clockwise = moving 5 counterclockwise
4. Maximum distance between any two pitch classes is 6 semitones (interval class)
5. Pitch-class intervals are measured in this space

# Construction / Recognition
## To Construct:
1. Take linear pitch space
2. Identify all pitches that are octave-equivalent
3. Collapse them to a single point
4. The result is a circle of 12 pitch classes

## To Recognize:
1. A circular representation with exactly 12 positions
2. All octave-equivalent pitches map to the same point

# Context & Application
Pitch-class space provides the framework for most operations in post-tonal theory: transposition, inversion, set-class identification, and interval-class calculation all operate in this space. The abstraction removes registral information, enabling analysts to identify motivic and harmonic connections that might be obscured by registral distance in the actual music. Analysis in pitch-class space complements, rather than replaces, analysis in pitch space.

# Examples
**From the text** (p. 23): Straus draws an analogy with time: "If it's eleven o'clock now, it will be eleven o'clock in twelve hours (that's a mod 12 system), and if it's Friday today, it will be Friday again in seven days (that's a mod 7 system). Just as our lives unfold simultaneously in linear and modular time, music unfolds simultaneously in pitch and pitch-class space."

# Relationships
## Builds Upon
- **Pitch Class** — the elements of the space
- **Mod 12 Arithmetic** — the arithmetic governing the space

## Enables
- **Pitch-Class Interval** — distances measured in pitch-class space
- **Interval Class** — the shortest distance between two points in pitch-class space
- **Pitch-Class Set** — subsets of pitch-class space

## Related
- **Pitch-Class Clockface** — the standard visual representation of pitch-class space

## Contrasts With
- **Pitch Space** — linear and unbounded vs. circular and modular

# Common Errors
- **Error**: Assuming pitch-class space preserves register
  **Correction**: Pitch-class space removes all registral information. C2 and C5 occupy the same point.

# Common Confusions
- **Confusion**: Thinking pitch-class space is a "simplified" version of pitch space
  **Clarification**: It is a different kind of space altogether: circular rather than linear, finite rather than infinite. It captures different information (pitch-class relationships) while discarding other information (register, contour).

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.5, pp. 22-23.

# Verification Notes
- Definition source: direct (Straus Section 1.5 and "IN BRIEF" box)
- Confidence rationale: explicitly defined with time analogy and clockface illustration
- Re-extraction notes: Re-extracted from v2 card; preserved: time analogy, contrast with pitch space, maximum distance property
