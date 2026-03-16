---
# === CORE IDENTIFICATION ===
concept: Pitch Class
slug: pitch-class

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
pdf_page: 20
section: "1.3 Pitch and Pitch Class"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - pc

# === TYPED RELATIONSHIPS ===
prerequisites:
  - octave-equivalence
  - enharmonic-equivalence
extends: []
related:
  - integer-notation
  - pitch-class-space
  - pitch-class-clockface
  - mod-12-arithmetic
contrasts_with:
  - pitch

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a pitch class?"
  - "How does pitch class differ from pitch?"
  - "What must I know before understanding pitch-class sets?"
---

# Quick Definition
A pitch class is a group of pitches related by octave and enharmonic equivalence, all sharing the same (or enharmonically equivalent) letter name.

# Core Definition
A pitch class is a collection of pitches related by octave and enharmonic equivalence. There are exactly twelve pitch classes in the equal-tempered system, represented by integers 0 through 11. Pitch-class A, for example, contains all the pitches named A in any octave, and any pitch named A is a member or representative of pitch-class A. A pitch class is an abstraction and cannot be adequately notated on musical staves; it is "not a single thing; it is a class of things: namely, pitches one or more octaves apart" (Straus).

# Prerequisites
- **Octave Equivalence** — pitches separated by octaves are treated as equivalent, forming a class
- **Enharmonic Equivalence** — differently-spelled notes at the same sounding pitch belong to the same class

# Key Properties
1. There are exactly 12 pitch classes (integers 0-11)
2. Each pitch class contains infinitely many pitches (one per octave, in each enharmonic spelling)
3. A pitch class is an abstraction, not a specific note
4. Pitch classes are represented by integers using the C=0 convention
5. Pitch classes reside in circular pitch-class space (mod 12), not linear pitch space

# Construction / Recognition
## To Construct:
1. Take any pitch (e.g., the Eb above middle C)
2. Apply octave equivalence: all Ebs in any octave belong to the same class
3. Apply enharmonic equivalence: D# belongs to the same class
4. Assign the corresponding integer: pitch class 3

## To Recognize:
1. Any note name (with any enharmonic spelling) in any octave belongs to one of twelve pitch classes
2. Map the note name to its integer (C=0, C#/Db=1, ..., B=11)

# Context & Application
Pitch class is the fundamental unit of post-tonal theory. Virtually all analytical concepts in set theory (pitch-class sets, set classes, transposition, inversion) operate on pitch classes rather than specific pitches. Understanding the distinction between pitch and pitch class is the essential first step in post-tonal analysis.

# Examples
**Example 1-5** (p. 20): In Feldman, Durations III, No. 3, each of three instruments plays a series of notes spanning many different pitches, but the passage as a whole uses only three pitch classes: F# (6), G (7), and Ab (8). The tuba alone plays five different pitches, all representatives of these three pitch classes.

# Relationships
## Builds Upon
- **Octave Equivalence** — collapses pitches across octaves into a single class
- **Enharmonic Equivalence** — collapses different spellings into a single class

## Enables
- **Integer Notation** — provides numerical labels for pitch classes
- **Pitch-Class Interval** — distance between two pitch classes
- **Pitch-Class Space** — the modular space pitch classes inhabit
- **Interval Class** — abstracted interval between pitch classes
- **Pitch-Class Set** — collections of pitch classes for set-theoretic analysis

## Related
- **Pitch-Class Clockface** — visual representation of the twelve pitch classes

## Contrasts With
- **Pitch** — a specific note at a specific register; pitch class is the abstract category

# Common Errors
- **Error**: Notating a pitch class on the staff and treating it as a specific pitch
  **Correction**: When we place a note on the staff to represent a pitch class, it is a convenience. The pitch class includes all octave and enharmonic equivalents of that note.

# Common Confusions
- **Confusion**: Thinking there are more than 12 pitch classes
  **Clarification**: All B#s, Cs, and Dbbs are members of a single pitch class (0). Enharmonic and octave equivalence collapse the infinite range of pitches into exactly 12 pitch classes.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.3, pp. 20-21.

# Verification Notes
- Definition source: direct (Straus Section 1.3 and "IN BRIEF" box)
- Confidence rationale: core definition clearly stated with Feldman example
- Re-extraction notes: Re-extracted from v2 card; preserved: integer table, Feldman example, confusion about abstraction
