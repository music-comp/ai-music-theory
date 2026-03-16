---
concept: Equivalence Relations
slug: equivalence-relations

category: mathematical-foundations
subcategory: sets-and-relations
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Equivalence relations"

extraction_confidence: high

aliases:
  - "equivalence relation"

prerequisites:
  - sets-and-number-systems
extends: []
related:
  - equivalence-classes
  - octave-equivalence
  - enharmonic-equivalence
  - note-classes
contrasts_with: []

answers_questions:
  - "What is an equivalence relation?"
  - "What are the three properties that define an equivalence relation?"
  - "How do equivalence relations formalize musical concepts like octave equivalence?"
---

# Quick Definition

A relation on a set that satisfies reflexivity, symmetry, and transitivity, providing the mathematical framework for identifying objects that are "the same" in some specified sense.

# Core Definition

Let $S$ be a set and $\sim$ a relation on $S$. The relation $\sim$ is an equivalence relation if for all $s, t, u \in S$ (Wright, p. 17):
1. $s \sim s$ (reflexivity)
2. If $s \sim t$, then $t \sim s$ (symmetry)
3. If $s \sim t$ and $t \sim u$, then $s \sim u$ (transitivity)

When these hold, the equivalence class of $s \in S$ is the set $\{t \in S \mid t \sim s\}$. The equivalence classes form a partition of $S$, meaning $S$ is the disjoint union of the equivalence classes.

# Prerequisites

- **Sets and Number Systems** — Equivalence relations are defined on sets

# Key Properties

1. All three properties (reflexivity, symmetry, transitivity) must hold simultaneously
2. The equivalence classes partition $S$ into disjoint subsets
3. Every element belongs to exactly one equivalence class
4. Any element of an equivalence class can serve as its representative

# Construction / Recognition

## To verify an equivalence relation:

1. Check reflexivity: Is $s \sim s$ for every $s \in S$?
2. Check symmetry: Does $s \sim t$ imply $t \sim s$?
3. Check transitivity: Do $s \sim t$ and $t \sim u$ imply $s \sim u$?
4. If all three hold, identify the equivalence classes

# Context & Application

Several fundamental musical concepts are formalized as equivalence relations:
- **Octave equivalence**: two notes are equivalent if the interval between them is $n$ octaves for some $n \in \mathbb{Z}$
- **Enharmonic equivalence**: two notes are equivalent if they produce the same pitch (e.g., $F^\sharp$ and $G^\flat$)
- **Durational equivalence**: notes are equivalent if they have the same duration, giving rise to "durational notes" (e.g., "half note" as an equivalence class)

# Examples

- "Same color" on a set of solid-colored objects satisfies all three properties (p. 17)
- On $\mathbb{Z}$, the relation $k \equiv \ell$ iff $n \mid (k - \ell)$ for a fixed positive integer $n$ is an equivalence relation (Exercise 3c)
- On piano notes, "interval is a major third" is NOT an equivalence relation — fails transitivity, since three major thirds span 12 semitones = octave, but the starting and ending notes are not a major third apart (Exercise 3d)
- The relation $(a,b) \sim (a',b')$ iff $ab' - a'b = 0$ on $\{(a,b) \in \mathbb{Z}^2 \mid b \neq 0\}$ gives equivalence classes corresponding to $\mathbb{Q}$ (Exercise 4)

# Relationships

## Builds Upon
- **Sets and Number Systems** — Defined on sets

## Enables
- **Equivalence Classes** — Equivalence relations produce equivalence classes
- **Octave Equivalence** — Formalized as an equivalence relation on notes
- **Enharmonic Equivalence** — Formalized as an equivalence relation on note names
- **Note Classes** — Equivalence classes under octave equivalence

## Related
- **Equivalence Classes** — The quotient structure produced by an equivalence relation

# Common Errors

- **Error**: Checking only one or two of the three properties
  **Correction**: All three (reflexivity, symmetry, transitivity) must be verified

# Common Confusions

- **Confusion**: Thinking every relation is an equivalence relation
  **Clarification**: $\leq$ on $\mathbb{R}$ fails symmetry — it is not an equivalence relation (p. 17)
- **Confusion**: Thinking "interval is a major third" is an equivalence relation on piano notes
  **Clarification**: It fails transitivity — three major thirds span 12 semitones (an octave), but the starting and ending notes are not a major third apart
- **Confusion**: Confusing an equivalence class with a single element
  **Clarification**: An equivalence class is a set of all elements equivalent to a given one

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Equivalence relations" section, p. 17 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 17
- Confidence rationale: High — explicit definition with three named properties and examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all examples including major-third counterexample and rational-number construction, musical applications list
