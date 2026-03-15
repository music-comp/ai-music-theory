---
# === CORE IDENTIFICATION ===
concept: Pitch vs Pitch Class
slug: pitch-vs-pitch-class

# === CLASSIFICATION ===
category: fundamentals
subcategory: pitch-systems
tier: advanced

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Pitch and Pitch Class"
chapter_number: 8
pdf_page: null
section: "VIII.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "pitch versus pitch class"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - enharmonic-equivalence
  - chromatic-scale
extends: []
related:
  - integer-notation
  - pitch-class-set
  - interval-class
contrasts_with:
  - pitch-class

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the difference between a pitch and a pitch class?"
  - "Why does post-tonal theory use pitch classes instead of pitches?"
  - "How are the twelve pitch classes numbered?"
---

# Quick Definition
A pitch is a discrete tone at a specific frequency and octave (e.g., C4), while a pitch class is the group of all pitches related by octave equivalence and enharmonic equivalence (e.g., all Cs, B-sharps, and D-double-flats across all octaves), represented in set theory by integers 0-11 where C=0.

# Core Definition
In set theory, "class" means "group." A pitch class is a group of pitches related by octave equivalence (C4 = C3 = C9) and enharmonic equivalence (A-flat = G-sharp). Pitch refers to a discrete tone with a specific frequency and octave placement: C4 is not the same pitch as C3. Pitch class abstracts away octave and spelling information. In post-tonal music, without a tonal center constraining enharmonic spellings, pitch class becomes the primary unit of analysis. The twelve pitch classes are represented with integer notation (0-11, where C=0), visualized on a clock face. This distinction parallels the interval/interval-class distinction: each moves from concrete to abstract.

# Prerequisites
- Enharmonic equivalence (understanding that G-sharp and A-flat are "the same key" on the piano)
- Chromatic scale (the twelve equally spaced pitches)

# Key Properties
1. Pitch = specific frequency + specific octave; pitch class = abstract category, octave-independent
2. Pitch class assumes both octave equivalence and enharmonic equivalence
3. There are exactly 12 pitch classes, numbered 0-11 (C=0 through B=11)
4. The clock face diagram represents pitch classes, not pitches (no octave dimension)
5. Tonal music distinguishes enharmonic spellings; post-tonal music often does not
6. The distinction is fundamental to all set-theory operations (Tn, In, etc.)

# Context & Application
In tonal music, enharmonic spelling matters: A-flat as le (flat-6) leads to G, while G-sharp as si (sharp-5) leads to A. In post-tonal music, freed from tonal syntax, composers treat A-flat and G-sharp as functionally identical. The pitch/pitch class distinction is the foundation for all post-tonal set theory, enabling integer notation, mod-12 arithmetic, and the classification of sonorities independent of register and spelling. Some quasi-tonal 20th-century music (e.g., Debussy) may require flexibility, sometimes benefiting from pitch-class analysis and sometimes from retaining enharmonic distinctions.

# Examples
**Example 1**: C4 (middle C, ~261.63 Hz) and C5 (~523.25 Hz) are different pitches but the same pitch class (pc 0).

**Example 2**: The twelve pitch classes in integer notation: 0=C(B-sharp), 1=C-sharp/D-flat, 2=D, 3=D-sharp/E-flat, 4=E(F-flat), 5=F(E-sharp), 6=F-sharp/G-flat, 7=G, 8=G-sharp/A-flat, 9=A, 10=A-sharp/B-flat, 11=B(C-flat).

**Example 3** (Webern, Symphony Op. 21): The twelve-tone structure is understood through pitch classes; the rows create symmetries at the pitch-class level regardless of register.

# Relationships
## Builds Upon
- **enharmonic-equivalence** -- Pitch class assumes enharmonic equivalence
- **chromatic-scale** -- The twelve pitch classes correspond to the twelve chromatic pitches
## Related
- **integer-notation** -- The system for numbering pitch classes 0-11
- **pitch-class-set** -- Collections of pitch classes, the next analytical unit
- **interval-class** -- The analogous concrete-to-abstract step for intervals

# Common Confusions
- **Confusion**: C4 and "C" are the same thing
  **Clarification**: C4 is a pitch; "C" (all octaves, all enharmonic spellings) is a pitch class
- **Confusion**: Integer 0 represents A
  **Clarification**: In set theory, 0 = C (not A as in some MIDI conventions)
- **Confusion**: Enharmonic equivalence always holds
  **Clarification**: It is assumed in pitch-class theory but may not apply in quasi-tonal 20th-century music like Debussy

# Source Reference
Open Music Theory, Part VIII, Chapter 1: "Pitch and Pitch Class." Concepts also referenced in Chapter 6: "American Standard Pitch Notation (ASPN)."

# Verification Notes
- Definition source: Directly from 08-01 source chapter
- Confidence rationale: High -- foundational concept with clear definition in source
- Preserved from v2: repertoire examples (Schoenberg, Webern, Debussy), integer notation table format
- Cross-reference status: Verified against multiple OMT chapters
