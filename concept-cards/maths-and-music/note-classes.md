---
# === CORE IDENTIFICATION ===
concept: Note Classes
slug: note-classes

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: notation
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Octave Equivalence"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "pitch classes"
  - "note class"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - octave-equivalence
  - equivalence-classes
extends:
  - equivalence-classes
related:
  - enharmonic-equivalence
  - diatonic-and-chromatic-scales
  - accidentals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a note class?"
  - "How many chromatic note classes exist?"
  - "How do note classes relate to octave equivalence?"
---

# Quick Definition

The equivalence classes of notes under octave equivalence, each identified by a letter name (with possible accidental) and containing all notes of that name across all octaves.

# Core Definition

A note class is an equivalence class of the octave equivalence relation on the set of keyboard notes. A note identified by a letter with no subscript (e.g., $B^\flat$) is viewed as an equivalence class: $B^\flat = \{B^\flat_n \mid n \in \mathbb{Z}\}$. Wright uses the term "note classes" for these equivalence classes (p. 20).

# Prerequisites

- **Octave Equivalence** — Note classes are the equivalence classes of this relation
- **Equivalence Classes** — Note classes are a specific instance of equivalence classes

# Key Properties

1. There are exactly 12 chromatic note classes (modulo enharmonic equivalence)
2. There are 7 diatonic note classes within any given major key
3. Each note class contains infinitely many notes (one per octave)
4. Note classes can be identified with $\mathbb{Z}/12\mathbb{Z}$ (integers modulo 12)
5. Key signatures, chord names, and scale patterns operate on note classes

# Construction / Recognition

## To identify a note class:

1. Take any specific note (e.g., $C_4$)
2. Remove the octave subscript
3. The resulting letter (with any accidental) names the note class
4. This class contains all notes of that name in every octave

# Context & Application

In practice, musicians routinely work with note classes: a chord labeled "C major" contains note classes C, E, G regardless of specific octave. Key signatures operate on note classes. When a composer writes in the key of G, $F^\sharp$ means every F in the piece is sharped, treating F as a note class. This is the concept Wright uses throughout the discussion of scales, modes, and keys.

# Examples

- The note class C contains $\ldots, C_2, C_3, C_4, C_5, \ldots$ (p. 20)
- There are 12 chromatic note classes: C, C$^\sharp$/D$^\flat$, D, D$^\sharp$/E$^\flat$, E, F, F$^\sharp$/G$^\flat$, G, G$^\sharp$/A$^\flat$, A, A$^\sharp$/B$^\flat$, B (p. 20)
- There are 7 diatonic note classes in C major: C, D, E, F, G, A, B (p. 21)

# Relationships

## Builds Upon
- **Octave Equivalence** — Note classes are the equivalence classes of octave equivalence
- **Equivalence Classes** — Note classes are a specific instance

## Enables
- **Diatonic and Chromatic Scales** — Scales are sequences of note classes
- **Key Signatures and the Circle of Fifths** — Key signatures alter note classes

## Related
- **Enharmonic Equivalence** — A note class may have multiple names ($F^\sharp = G^\flat$)

# Common Errors

- **Error**: Treating a note class as a single specific note
  **Correction**: A note class like "C" is an infinite set $\{C_0, C_1, C_2, \ldots\}$, not one note

# Common Confusions

- **Confusion**: Confusing note classes (octave equivalence) with durational notes (duration equivalence)
  **Clarification**: Both are equivalence classes but on different sets with different relations — note classes group by pitch, durational notes group by duration
- **Confusion**: Thinking a note class has a unique name
  **Clarification**: Due to enharmonic equivalence, note classes may have multiple names (e.g., $F^\sharp$ and $G^\flat$)

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Octave Equivalence" section, pp. 20-21 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 20
- Confidence rationale: High — explicitly defined as equivalence classes with term "note classes"
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: $\mathbb{Z}/12\mathbb{Z}$ identification, distinction from durational equivalence, multiple-name issue
