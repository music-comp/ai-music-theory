---
concept: Ecclesiastical Modes
slug: ecclesiastical-modes

category: pitch-and-intervals
subcategory: scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Modality and Key"

extraction_confidence: high

aliases:
  - "church modes"
  - "medieval modes"
  - "modal scales"

prerequisites:
  - cyclic-permutations
  - diatonic-and-chromatic-scales
extends:
  - diatonic-and-chromatic-scales
related:
  - modality-and-key
  - major-and-minor-modes
  - key-signatures-and-the-circle-of-fifths
contrasts_with: []

answers_questions:
  - "What are the seven ecclesiastical modes?"
  - "How are modes related to cyclic permutations of the diatonic scale?"
  - "Why are all seven modes distinct?"
---

# Quick Definition

The seven scales obtained by starting the diatonic scale on each of its seven notes, each producing a distinct interval pattern, named (incorrectly) by the Swiss theorist Heinrich Glarean after ancient Greek modes.

# Core Definition

The seven ecclesiastical modes are the cyclic permutations of the standard diatonic scale, each yielding a distinct interval sequence. The names were assigned by Heinrich Glarean (1488-1563), who incorrectly identified them with ancient Greek modes, yet these names became accepted (Wright, pp. 26-27). Starting from the white keys:

| White keys | Mode | Interval pattern |
|-----------|------|-----------------|
| C-C | Ionian | $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ |
| D-D | Dorian | $1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}, 1$ |
| E-E | Phrygian | $\frac{1}{2}, 1, 1, 1, \frac{1}{2}, 1, 1$ |
| F-F | Lydian | $1, 1, 1, \frac{1}{2}, 1, 1, \frac{1}{2}$ |
| G-G | Mixolydian | $1, 1, \frac{1}{2}, 1, 1, \frac{1}{2}, 1$ |
| A-A | Aeolian | $1, \frac{1}{2}, 1, 1, \frac{1}{2}, 1, 1$ |
| B-B | Locrian | $\frac{1}{2}, 1, 1, \frac{1}{2}, 1, 1, 1$ |

# Prerequisites

- **Cyclic Permutations** — Modes are cyclic permutations of the diatonic scale
- **Diatonic and Chromatic Scales** — The standard scale whose permutations generate modes

# Key Properties

1. The interval sequence $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ has no non-trivial cyclic permutation equal to itself
2. Therefore all seven cyclic permutations produce distinct modes
3. Each mode can be transposed to any key by applying appropriate accidentals
4. A key signature determines a unique scale in each mode
5. The key signature alone does not determine the mode

# Construction / Recognition

## To build a mode on C:

1. Choose the mode (e.g., Dorian)
2. Start with the mode's interval pattern
3. Apply accidentals to C to match that pattern
4. Example: Dorian on C = C, D, $E^\flat$, F, G, A, $B^\flat$, C

# Context & Application

Each mode built on C is shown in Wright's chart with the appropriate accidentals. The modes demonstrate how a single set of notes (the white keys) can generate seven distinct musical "flavors" through cyclic permutation. Since the key signature determines a unique scale in each mode, the key signature does not determine the mode — the mode must be inferred from the music's tonal center.

# Examples

- Dorian on C: C, D, $E^\flat$, F, G, A, $B^\flat$, C (p. 27)
- Lydian on C: C, D, E, $F^\sharp$, G, A, B, C (p. 27)
- Phrygian on C: C, $D^\flat$, $E^\flat$, F, G, $A^\flat$, $B^\flat$, C (p. 27)
- C Ionian and F Lydian share the same key signature (no sharps or flats) but have different tonal centers (p. 27)

# Relationships

## Builds Upon
- **Cyclic Permutations** — Each mode is a cyclic permutation of the Ionian scale
- **Diatonic and Chromatic Scales** — All modes use the same 7 diatonic notes (in a given key signature)

## Enables
- **Major and Minor Modes** — Ionian = major, Aeolian = minor
- **Modality and Key** — Modes are one component of key determination

## Related
- **Key Signatures and the Circle of Fifths** — Key signatures interact with mode to determine key

# Common Errors

- **Error**: Assuming Glarean's mode names match the original Greek modes
  **Correction**: Glarean's names are historically incorrect attributions, but they are universally adopted

# Common Confusions

- **Confusion**: Thinking the key signature uniquely determines the mode
  **Clarification**: C Ionian and A Aeolian share the same key signature — the tonic must be identified from musical context
- **Confusion**: Thinking Locrian is as usable as other modes
  **Clarification**: Locrian is the only mode with a diminished fifth above the tonic, making it rarely used as a primary key

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Modality and Key" section, pp. 26-27 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 26-27 with complete mode table
- Confidence rationale: High — explicit table with all seven modes and their interval patterns
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete mode table, Glarean attribution, shared key signature example
