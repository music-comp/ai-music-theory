---
# === CORE IDENTIFICATION ===
concept: Equivalence Classes
slug: equivalence-classes

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: sets-and-relations
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Equivalence relations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "equivalence class"
  - "quotient set"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - equivalence-relations
extends:
  - equivalence-relations
related:
  - octave-equivalence
  - note-classes
  - enharmonic-equivalence
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an equivalence class?"
  - "How do equivalence classes partition a set?"
  - "What musical concepts are examples of equivalence classes?"
---

# Quick Definition

The set of all elements equivalent to a given element under an equivalence relation, forming one block of the partition induced by that relation.

# Core Definition

Given an equivalence relation $\sim$ on a set $S$, the equivalence class of $s \in S$ is the set $\{t \in S \mid t \sim s\}$. The equivalence classes form a partition of $S$, meaning $S$ is the disjoint union of all equivalence classes and every element of $S$ belongs to exactly one class (Wright, p. 17).

# Prerequisites

- **Equivalence Relations** — Equivalence classes are defined by equivalence relations

# Key Properties

1. Each equivalence class can be represented by any of its members (a "representative")
2. The equivalence classes partition $S$ — no element belongs to more than one class
3. Two equivalence classes are either identical or disjoint
4. The set of all equivalence classes is called the quotient set, denoted $S/\!\sim$

# Construction / Recognition

## To determine equivalence classes:

1. Start with an element $s \in S$
2. Find all elements $t$ such that $t \sim s$
3. The resulting set is the equivalence class of $s$
4. Repeat for elements not yet classified until $S$ is fully partitioned

# Context & Application

Wright identifies several musical equivalence classes:
- **Note classes**: equivalence classes under octave equivalence (e.g., the class "B$\flat$" contains all $B^\flat_n$ for $n \in \mathbb{Z}$)
- **Durational notes**: equivalence classes of notes having the same duration (e.g., "half note" regardless of pitch)
- **Interval classes**: equivalence classes of intervals modulo octave (e.g., whole step and ninth are equivalent)
- **Enharmonic classes**: equivalence classes under enharmonic equivalence (e.g., $\{F^\sharp, G^\flat\}$)

# Examples

- Under octave equivalence, the equivalence class of $B^\flat$ is $\{\ldots, B^\flat_1, B^\flat_2, B^\flat_3, B^\flat_4, B^\flat_5, \ldots\}$ (p. 20)
- Under octave equivalence of intervals, each class has a unique representative that is positive and strictly less than an octave (p. 20)
- The equivalence class of $(2, 3)$ under the relation $(a,b) \sim (a',b')$ iff $ab' - a'b = 0$ corresponds to the rational number $2/3$ (Exercise 4)

# Relationships

## Builds Upon
- **Equivalence Relations** — Equivalence classes are the quotient structure of an equivalence relation

## Enables
- **Note Classes** — Note classes are equivalence classes under octave equivalence
- **Octave Equivalence** — Its equivalence classes are the 12 note classes

## Related
- **Enharmonic Equivalence** — Produces enharmonic equivalence classes

## Contrasts With
- None within scope

# Common Errors

- **Error**: Treating an equivalence class as a single element rather than a set
  **Correction**: A note class like "C" is an infinite set $\{C_0, C_1, C_2, \ldots\}$, not a single note

# Common Confusions

- **Confusion**: Confusing different equivalence relations on the same set
  **Clarification**: Durational equivalence classes (grouping by duration) and octave equivalence classes (grouping by pitch class) are defined by different relations and produce different partitions
- **Confusion**: Thinking all equivalence classes have the same size
  **Clarification**: Under octave equivalence, each note class is infinite; under enharmonic equivalence in equal temperament, each class has 1-3 members

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Equivalence relations" section, p. 17 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 17
- Confidence rationale: High — explicit definition with partition property stated
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all musical equivalence class examples, durational vs. octave distinction
