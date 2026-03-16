---
concept: Stacks of Perfect Fifths
slug: stacks-of-fifths

category: scales-modes
subcategory: construction
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 142
section: "4.4"

extraction_confidence: high

aliases:
  - "Pythagorean construction"
  - "fifth-generated scales"

prerequisites:
  - goldilocks-principle
extends: []
related:
  - eight-important-scales
  - diatonic-scale
  - pentatonic-scale
contrasts_with:
  - stacks-of-thirds

answers_questions:
  - "How can scales be constructed from perfect fifths?"
  - "Why do stacks of fifths produce nearly even scales?"
  - "What three scales result from stacking fifths?"
---

# Quick Definition
Scales maximally saturated with perfect fifths can be constructed by stacking fifths. Three salient results emerge: 5 fifths produce the pentatonic scale, 7 fifths produce the diatonic, and 12 fifths produce the chromatic — each with one "imperfect fifth" that closes the octave.

# Core Definition
To construct octave-repeating scales maximally saturated with acoustically pure perfect fifths, one stacks fifths and looks for points where the stack nearly closes back on itself (approaches an octave multiple). A finite stack can never have pure fifths above and below every note; the best possible is a stack where all but one note has this property, leaving one "near fifth" that closes the circle. Three notable closure points occur: 5 fifths fall 0.9 semitones short of 3 octaves (producing pentatonic with a "near fifth" close to a minor sixth); 7 fifths overshoot 4 octaves by about 1.14 semitones (producing diatonic with a "near fifth" close to a tritone); 12 fifths overshoot 7 octaves by about 0.25 semitones (producing chromatic with a slightly impure fifth). Different tuning systems assign different sizes to these intervals, balancing evenness against acoustic purity.

# Prerequisites
- Goldilocks Principle (near-evenness as desirable)

# Key Properties
1. A stack of n pure fifths cannot exactly equal any number of octaves (for finite n)
2. Three salient near-closures: 5, 7, and 12 fifths
3. Each produces a nearly even scale with one "imperfect" fifth
4. Pentatonic: 5 notes, one "near fifth" ≈ minor sixth
5. Diatonic: 7 notes, one "near fifth" ≈ tritone
6. Chromatic: 12 notes, one "near fifth" slightly flat
7. Beyond 12 notes, microtonal steps become very small and difficult to sing

# Construction / Recognition
## To Construct:
1. Start from any note, stack pure fifths upward
2. After 5, 7, or 12 fifths, the result is close to an octave multiple
3. Remove the top note of the stack; fold remaining notes into a single octave
4. The result is a nearly even scale with one imperfect fifth

# Context & Application
This is the traditional Pythagorean approach to scale construction, given a modern geometric interpretation. The diatonic scale's emergence from this process is well known, but the parallel construction of pentatonic and chromatic scales as members of the same family is illuminating. The construction demonstrates that these scales are not arbitrary but arise from the mathematics of stacking the most consonant non-octave interval.

# Examples
**Example 1** (p. 142): Figure 4.4.1 shows the three stacks: (a) 5 fifths falling 0.9 semitones short of 3 octaves, (b) 7 fifths overshooting 4 octaves by 1.14 semitones, (c) 12 fifths overshooting 7 octaves by 0.25 semitones.

# Relationships
## Builds Upon
- **goldilocks-principle** — The scales are nearly even
## Enables
- **diatonic-scale** — Constructed from 7 fifths
- **pentatonic-scale** — Constructed from 5 fifths
## Related
- **eight-important-scales** — Three of the eight come from this construction
## Contrasts With
- **stacks-of-thirds** — An alternative construction principle producing different scales

# Common Errors
- **Error**: Thinking 12 equal-tempered fifths exactly close the circle
  **Correction**: They do in equal temperament (by definition), but pure acoustic fifths overshoot by the "Pythagorean comma" of about 0.25 semitones

# Common Confusions
- **Confusion**: Why not continue stacking beyond 12 fifths?
  **Clarification**: Beyond 12, the resulting scale steps are about a quarter of a semitone — too small to sing easily in most contexts

# Source Reference
Chapter 4: Scales, Section 4.4, pages 142-143.

# Verification Notes
- Definition source: From Section 4.4
- Confidence rationale: High — well-established construction with clear figures
- Cross-reference status: Verified against Figure 4.4.1 and footnotes 13-14
