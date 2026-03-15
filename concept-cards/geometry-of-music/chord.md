---
# === CORE IDENTIFICATION ===
concept: Chord (OPC)
slug: chord

# === CLASSIFICATION ===
category: harmony
subcategory: classification
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "unordered set of pitch classes"
  - "OPC class"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-musical-object
  - octave-symmetry
  - permutation-symmetry
  - cardinality-change-symmetry
extends:
  - basic-musical-object
related:
  - optic-symmetries
  - transpositional-set-class
  - set-class
  - multiset
contrasts_with:
  - transpositional-set-class

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a chord vs. a chord type?"
  - "What are OPTIC symmetries?"
---

# Quick Definition
A chord, in Tymoczko's formal framework, is an equivalence class of basic musical objects related by octave shifts (O), permutations (P), and cardinality changes (C) — formally, an unordered set of pitch classes like {C, E, G}.

# Core Definition
A chord is defined by three OPTIC symmetry operations (OPC). Starting from basic musical objects, we apply octave shifts (so register does not matter), permutations (so voice assignment does not matter), and cardinality changes (so doublings do not matter). The result is an unordered set of pitch classes. For example, (C4, E4, G4), (E4, G4, C5), (G3, G4, C5, E4), and (E2, G3, C4, E4, E5) all represent the C major chord {C, E, G}. This formalization, implicit in Rameau's eighteenth-century theory, represents an unordered collection of points on the pitch-class circle.

# Prerequisites
- **basic-musical-object** — Chords are formed from basic objects
- **octave-symmetry** — O discards octave information
- **permutation-symmetry** — P discards ordering
- **cardinality-change-symmetry** — C discards duplication information

# Key Properties
1. Represented as unordered sets of pitch classes: {C, E, G}
2. Curly braces indicate unordered (vs. parentheses for ordered)
3. Geometrically: a collection of points on the pitch-class circle
4. Order, register, and doublings are irrelevant
5. Implicitly defined by Rameau in the early 18th century

# Construction / Recognition
## To Construct/Create:
1. Take any basic musical object
2. Reduce all pitches to pitch classes (apply O)
3. Discard ordering (apply P)
4. Discard duplicate pitch classes (apply C)
5. The result is an unordered set of pitch classes
## To Identify/Recognize:
1. Identify the distinct pitch classes present
2. Two musical passages represent the same chord if they contain the same pitch classes

# Context & Application
Chords are the central harmonic objects of Western music theory. The formal definition makes precise what musicians intuitively mean by "C major chord" — any collection of notes containing exactly the pitch classes C, E, and G, regardless of voicing, register, or doubling. Chord progressions are sequences of chords.

# Examples
**Example 1** (p. 55, Fig 2.4.2): All objects (C4, E4, G4), (E4, G4, C5), (G3, G4, C5, E4), etc., represent the chord {C, E, G}, shown as three points on the pitch-class circle.

# Relationships
## Builds Upon
- **basic-musical-object** — Formed by applying OPC
## Enables
- **transpositional-set-class** — Add T to OPC to get chord types
- **set-class** — Add T and I to OPC to get set classes
## Related
- **optic-symmetries** — Chord = OPC in the OPTIC framework
## Contrasts With
- **transpositional-set-class** — Chord type abstracts further by transposition
- **multiset** — Multiset (OP) preserves note multiplicity; chord (OPC) does not

# Common Errors
- **Error**: Thinking a chord specifies voicing or register
  **Correction**: A chord (in this formal sense) is an unordered set of pitch classes; voicing and register are discarded

# Common Confusions
- **Confusion**: Confusing "chord" with "chord type"
  **Clarification**: {C, E, G} and {D, F#, A} are different chords but the same chord type (major triad)

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 54-55.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figure 2.4.7
- Confidence rationale: High — precisely defined as OPC equivalence class
- Cross-reference status: Verified; used throughout the book
