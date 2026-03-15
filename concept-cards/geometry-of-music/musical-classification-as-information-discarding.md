---
# === CORE IDENTIFICATION ===
concept: Musical Classification as Information Discarding
slug: musical-classification-as-information-discarding

# === CLASSIFICATION ===
category: fundamentals
subcategory: formalism
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
  - "progressive abstraction"
  - "classification by symmetry"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-musical-object
  - optic-symmetries
extends:
  - optic-symmetries
related:
  - chord
  - transpositional-set-class
  - set-class
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does musical classification work?"
  - "What are OPTIC symmetries?"
---

# Quick Definition
Musical classification proceeds by the progressive discarding of information through symmetry operations: from specific pitches to pitch classes (discard octave), to chords (discard order and duplication), to chord types (discard absolute pitch level), to set classes (discard inversion distinction).

# Core Definition
Tymoczko frames all of music-theoretical classification as a process of progressive abstraction through symmetry. Starting from a basic musical object (a sequence of specific pitches), we discard information in stages: applying O discards octave/register information; applying P discards voice-ordering; applying C discards note multiplicity; applying T discards absolute pitch level; applying I discards the distinction between a chord and its inversion. At each stage, we obtain a more abstract musical category. Different combinations of discarded information yield different categories, and there is no single "optimal" level of abstraction — different musical purposes require different amounts of information.

# Prerequisites
- **basic-musical-object** — The starting point of classification
- **optic-symmetries** — The operations that discard information

# Key Properties
1. Classification = applying symmetry operations = discarding information
2. Progressive: each additional symmetry discards more
3. No single optimal level — different contexts need different amounts of information
4. 32 possible OPTIC combinations, each a valid classification scheme
5. The process reveals that standard music-theoretical concepts are not arbitrary but correspond to natural symmetry combinations

# Construction / Recognition
## To Construct/Create:
1. Start with a basic musical object
2. Apply OPTIC symmetries in any combination
3. Each combination yields a different level of abstraction
## To Identify/Recognize:
1. When a theorist says "this is a major chord," they have applied OPC (discarding octave, order, duplication)
2. When they say "this is a major chord type," they have also applied T

# Context & Application
This meta-theoretical insight unifies disparate music-theoretical concepts under a single framework. It explains why there are many possible levels of harmonic analysis and why debates about "the right level" are often misguided — different analytical questions require different levels. It also reveals that seemingly exotic classifications (like multisets, or tone rows) are just alternative OPTIC combinations, no less natural than familiar ones.

# Examples
**Example 1** (p. 58): "When we say that an object is a major chord, we are neglecting an enormous number of musical details, leaving behind something that is very abstract — an ordered sequence of clockwise distances around the pitch-class circle."

# Relationships
## Builds Upon
- **basic-musical-object** — The most specific starting point
- **optic-symmetries** — The mechanism of abstraction
## Enables
- Principled choice of analytical level for any musical context
## Related
- **chord** — OPC abstraction
- **transpositional-set-class** — OPTC abstraction
- **set-class** — OPTIC abstraction
## Contrasts With
- No direct contrast within this source

# Common Errors
- **Error**: Assuming one level of abstraction is always correct
  **Correction**: Different musical purposes require different levels; the OPTIC framework provides 32 options

# Common Confusions
- **Confusion**: Thinking more abstraction is better
  **Clarification**: More abstraction discards more information; sometimes that information is musically relevant and should be preserved

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 53-59.

# Verification Notes
- Definition source: Synthesized from Section 2.4's meta-theoretical discussion
- Confidence rationale: High — explicitly argued throughout Section 2.4
- Cross-reference status: Verified; this perspective underlies the entire book
