---
concept: Ties and Slurs
slug: ties-and-slurs

category: rhythm-and-form
subcategory: duration
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Ties and Slurs"

extraction_confidence: high

aliases:
  - "tie"
  - "slur"

prerequisites:
  - note-durational-values
extends: []
related:
  - dotted-note-duration-formula
  - rhythm
  - accidentals
  - meter-and-time-signatures
contrasts_with: []

answers_questions:
  - "What is the difference between a tie and a slur?"
  - "How do ties affect note duration?"
  - "How do ties interact with accidentals across bar lines?"
---

# Quick Definition

A tie is a curved line connecting two notes of the same pitch, combining their durations into one sustained note; a slur looks similar but connects notes of different pitches, indicating smooth, connected performance.

# Core Definition

A *tie* connects two notes of the same pitch with a curved line, indicating they are to be treated as one note whose duration is the sum of the two tied notes' durations. A *slur* connects notes of different pitches, indicating they should be performed with minimal rearticulation (e.g., in one bow stroke for a string instrument) (Wright, pp. 35-36).

# Prerequisites

- **Note Durational Values** — Ties combine note durations additively

# Key Properties

1. Ties combine durations: if durations are $d_1$ and $d_2$, the result is $d_1 + d_2$
2. Ties connect notes of the SAME pitch; slurs connect DIFFERENT pitches
3. Ties and slurs look nearly identical visually
4. Combined with dots, ties allow representation of any sum of dyadic rationals
5. When a tied note crosses a bar line, its accidental applies only to that note, not to others of the same class in the new measure

# Construction / Recognition

## To distinguish ties from slurs:

1. Check if the two connected notes have the same pitch
2. Same pitch = tie (durations add)
3. Different pitches = slur (performance instruction for smooth connection)

# Context & Application

Ties are essential when a note's duration must span across a bar line or when the desired duration cannot be expressed by a single note value (with or without dots). Slurs are performance instructions affecting articulation. Exercise 8 asks which fractions of a whole note can be achieved using $\frac{1}{2^n}$-notes with dots and ties.

# Examples

- A quarter note ($d_1 = 1$ beat) tied to a dotted sixteenth ($d_2 = \frac{3}{8}$ beat) gives duration $1 + \frac{3}{8} = \frac{11}{8}$ beats (p. 35)
- A half note tied across a bar line to a quarter note gives 3 beats of sustained sound
- A slur over C-D-E indicates all three notes played smoothly in one bow stroke (p. 35)
- Tied accidental rule: alteration carries across bar line but does not affect other notes of the same class (p. 37)

# Relationships

## Builds Upon
- **Note Durational Values** — Ties add durations from the power-of-2 system

## Enables
- **Rhythm** — Ties create durations not possible with single notes
- Complex durational patterns spanning bar lines

## Related
- **Dotted Note Duration Formula** — Dots and ties complement each other for duration expression
- **Accidentals** — Tied notes have special accidental rules across bar lines

# Common Errors

- **Error**: Confusing ties and slurs based on visual appearance
  **Correction**: Check the pitches — ties connect same pitch, slurs connect different pitches

# Common Confusions

- **Confusion**: Thinking tied accidentals carry to all notes of the same class in the new measure
  **Clarification**: A tied accidental applies only to the tied note itself, not to other notes of the same class in the new measure (p. 37)
- **Confusion**: Thinking dots can always replace ties
  **Clarification**: Some durations (like $\frac{11}{8}$ beats) can only be expressed with ties

# Source Reference

Chapter 2: "Horizontal Structure", "Ties and Slurs" section, pp. 35-36 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 35-36
- Confidence rationale: High — explicit definitions with worked duration example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: $\frac{11}{8}$ example, tied accidental rule from p. 37, dyadic rational observation
