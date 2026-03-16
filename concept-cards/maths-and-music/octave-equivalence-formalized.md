---
concept: Octave Equivalence Formalized
slug: octave-equivalence-formalized

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Octave identification"

extraction_confidence: high

aliases:
  - octave identification
  - pitch class equivalence

prerequisites:
  - twelve-chromatic-scale
extends:
  - octave-equivalence
related:
  - modular-equivalence-on-the-integers
  - modular-chromatic-intervals
  - modular-integers
  - z-twelve-as-chromatic-interval-group
contrasts_with: []

answers_questions:
  - "How does modular arithmetic model octave equivalence?"
  - "How are the 12 note classes formally represented?"
  - "What happens to interval arithmetic under octave equivalence?"
---

# Quick Definition

The mathematical formalization of the musical practice of treating notes separated by octaves as equivalent, using modular arithmetic and the group Z_12.

# Core Definition

Octave identification equates notes whose frequencies differ by a factor of 2^m for integer m. Under additive measurement in semitones, two intervals k and l are octave-equivalent if k - l is a multiple of 12, i.e., k = l (mod 12). The 12 note classes correspond to the elements of Z_12, numbered from C: C = [0], C#/Db = [1], D = [2], ..., B = [11]. Going up an octave becomes the identity operation, and interval composition becomes addition in Z_12 (Wright, p. 82).

# Prerequisites

- **Twelve-chromatic scale** — Octave equivalence operates on the 12-chromatic framework

# Key Properties

1. Under octave equivalence, there are exactly 12 distinct note classes
2. The octave becomes the identity: [12] = [0]
3. Interval composition is addition in Z_12
4. A fourth + a fifth = unison: [5] + [7] = [0]
5. Two fifths = a whole step: [7] + [7] = [2]
6. The chromatic scale modulo octave is parameterized by Z_12

# Construction / Recognition

## To Reduce an Interval to Its Modular Equivalent
1. Express the interval in semitones as an integer k
2. Compute k mod 12 (the unique remainder r with 0 <= r < 12)
3. The modular chromatic interval is [r]
4. Identify the corresponding interval name

# Context & Application

Musical notation implicitly equates notes differing by octaves. A C in any register is "the same note" as any other C. This practice is nearly universal across musical traditions and stems from the 2:1 frequency ratio being perceived as a strong consonance. The formalization via Z_12 enables rigorous treatment of interval arithmetic, twelve-tone row charts, and pitch-class set theory.

# Examples

**Example 1** (p. 82): Numbering from C: C = 0, C#/Db = 1, D = 2, D#/Eb = 3, E = 4, F = 5, F#/Gb = 6, G = 7, G#/Ab = 8, A = 9, A#/Bb = 10, B = 11.

**Example 2** (p. 89): Minor third + octave + fourth = [3] + [12] + [5] = [3] + [0] + [5] = [8] (augmented fifth), since 20 = 8 (mod 12).

**Example 3** (p. 82): Fourth + fifth = unison (modulo octave): [5] + [7] = [0]. Two fifths = whole step: [7] + [7] = [2].

# Relationships

## Builds Upon
- **Twelve-chromatic scale** — Octave equivalence partitions the 12-chromatic scale into 12 classes

## Enables
- **Modular chromatic intervals** — The interval algebra under octave equivalence
- **Z_12 as chromatic interval group** — The algebraic formalization of octave-equivalent chromatic intervals

## Related
- **Modular equivalence on the integers** — Octave equivalence is the specific case m = 12
- **Modular integers** — Z_12 is the group of modular chromatic intervals

# Common Errors

- **Error**: Forgetting that the octave becomes the identity under octave equivalence
  **Correction**: [12] = [0]; adding an octave does not change the modular interval

# Common Confusions

- **Confusion**: Thinking octave equivalence is a physical necessity
  **Clarification**: It is a convention reflecting the perceptual similarity of octave-related pitches, not a physical law

- **Confusion**: Confusing octave equivalence with enharmonic equivalence
  **Clarification**: C# = Db (enharmonic) is a separate convention from treating all C's across registers as equivalent (octave)

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 82 (octave identification section). See also pp. 88-89 for interval composition examples.

# Verification Notes

- Definition source: Direct from Wright, p. 82
- Confidence rationale: High — explicit definition with numbered note classes
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete note class numbering, composition examples, distinction from enharmonic equivalence
