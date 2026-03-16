---
concept: Inversion
slug: inversion

category: geometric-theory
subcategory: transformation
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.3"

extraction_confidence: high

aliases:
  - "pitch-space inversion"
  - "reflection"
  - "I symmetry"

prerequisites:
  - pitch-space
  - distance-in-music
extends: []
related:
  - transposition
  - optic-symmetries
  - set-class
  - pitch-class-space
contrasts_with:
  - transposition

answers_questions:
  - "What does inversion mean in pitch and pitch-class spaces?"
  - "What are OPTIC symmetries?"
---

# Quick Definition
Inversion turns musical space "upside down," reversing the direction of all intervals while preserving their sizes, corresponding to reflection in pitch space or pitch-class space.

# Core Definition
Inversion is the second of only two distance-preserving transformations of musical space. The inversion that maps pitch x to pitch y, written I_x^y(p), is computed as (x + y) - p. Geometrically, inversion is reflection: if a mirror is placed at the fixed point (x + y)/2, each pitch maps to the position of its reflection. In pitch space, each inversion has exactly one fixed point; in pitch-class space, each inversion fixes two antipodal points. Inversion changes the character of musical passages more dramatically than transposition (it is direction-reversing), but inversionally related chords often sound reasonably similar.

# Prerequisites
- **pitch-space** — Inversion is defined in pitch space
- **distance-in-music** — Inversion preserves distances

# Key Properties
1. I_x^y(p) = (x + y) - p in both pitch and pitch-class space
2. Distance-preserving but direction-reversing
3. Geometrically: reflection in pitch space; reflection in pitch-class space
4. Each pitch-space inversion has one fixed point at (x + y)/2
5. Each pitch-class inversion fixes two antipodal points
6. Inversionally related chords share the same interval content (same arc lengths on the circle)

# Construction / Recognition
## To Construct/Create:
1. Choose a fixed point (or pair of pitches x, y that define it)
2. Reflect each pitch around the fixed point: p becomes (x + y) - p
3. In pitch-class space, reduce modulo 12
## To Identify/Recognize:
1. Check whether intervals are preserved but directions reversed
2. Ascending motion becomes descending motion of the same size

# Context & Application
Inversion is the I in OPTIC symmetries. It is the basis for defining set classes (which group transpositionally and inversionally equivalent chords). Many twentieth-century composers treat inversionally related chords as equivalent. The relationship between major and minor triads is an inversion: C major {C, E, G} inverts to C minor {C, Eb, G}. Inversional near-symmetry is key to understanding efficient voice leading between certain chord pairs.

# Examples
**Example 1** (p. 52, Fig 2.3.2): The theme of Bach's A minor prelude, WTC II, and its inversion — ascending motion becomes descending motion of the same magnitude.

**Example 2** (p. 52, Fig 2.3.3): Inversionally related chords sound more similar to each other than to non-related chords — chords containing a minor third/seventh/fifth vs. chords containing a fifth/tritone/minor second.

# Relationships
## Builds Upon
- **pitch-space** — The space in which inversion is defined
- **distance-in-music** — Inversion is distance-preserving
## Enables
- **set-class** — Defined using both transposition and inversion equivalence
- **optic-symmetries** — I is one of the five OPTIC symmetries
## Related
- **pitch-class-space** — In this space, inversion is reflection
## Contrasts With
- **transposition** — Preserves both distance and direction; inversion preserves distance but reverses direction

# Common Errors
- **Error**: Confusing pitch-space inversion with registral inversion (changing which octave notes are in)
  **Correction**: Registral inversion is the O symmetry (octave shift); pitch-space inversion is the I symmetry (reflection)

# Common Confusions
- **Confusion**: Thinking inversion always relates major to minor
  **Clarification**: Major/minor is one example; inversion is a general transformation applicable to any musical object

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.3, pages 51-53.

# Verification Notes
- Definition source: Direct from Section 2.3 with mathematical formula
- Confidence rationale: High — precisely defined with multiple examples
- Cross-reference status: Verified; used throughout the book
