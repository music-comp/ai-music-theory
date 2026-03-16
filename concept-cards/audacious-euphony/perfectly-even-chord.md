---
concept: Perfectly Even Chord
slug: perfectly-even-chord

category: neo-riemannian-theory
subcategory: theoretical-framework
tier: intermediate

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Hexatonic Cycles"
chapter_number: 2
pdf_page: 35
section: "Near Evenness, Minimal Voice Leading, and the Central Role of Augmented Triads"

extraction_confidence: high

aliases:
  - "perfectly symmetric chord"
  - "equal division of the octave"

prerequisites: []
extends: []
related:
  - near-evenness
  - augmented-triad-as-axis
contrasts_with:
  - near-evenness
  - consonant-triad

answers_questions:
  - "What is a perfectly even chord?"
  - "Why can't perfectly even chords voice-lead smoothly to chords of the same type?"
---

# Quick Definition
A chord that divides the octave into exactly equal parts, such as the augmented triad (three parts of 4 semitones each) or the diminished seventh chord (four parts of 3 semitones each).

# Core Definition
A chord is perfectly even when all constituent intervals are identical. In 12-tone equal temperament, this requires the number of notes to evenly divide 12. Perfectly even chords serve as the structural axes from which nearly even chords are derived. The key theoretical point: "In the perfectly even case, the axis that reflects C and E into each other also reflects G# into itself. Consequently, the perfectly even augmented triad cannot be distinguished from its reflection, and the transformation is a phantom" (Cohn, p. 54). This means perfectly even chords cannot voice-lead to different chords of the same type by single semitone.

# Prerequisites
This concept has no strict prerequisites within this source.

# Key Properties
1. All constituent intervals are identical
2. Only possible when chord cardinality evenly divides 12
3. Invariant under inversion about any axis bisecting an interval
4. Cannot voice-lead to another chord of the same type by single semitone
5. Serve as the "parent" structures from which nearly even chords are derived

# Construction / Recognition
Perfectly even chords in 12-TET:
- 2 notes: tritone (C-F#), pattern [6-6], 6 instances
- 3 notes: augmented triad (C-E-G#), pattern [4-4-4], 4 instances
- 4 notes: diminished seventh (C-Eb-Gb-A), pattern [3-3-3-3], 3 instances
- 6 notes: whole-tone scale (C-D-E-F#-G#-A#), pattern [2-2-2-2-2-2], 2 instances

# Context & Application
Perfectly even chords are "the invisible axes about which pan-triadic progressions spin" (p. 52). The augmented triad plays this role for consonant triads; the diminished seventh plays it for dominant/half-diminished seventh chords; the whole-tone collection plays it for mystic chords.

# Examples
**Example 1** (p. 54): Figure 2.17(a) shows the augmented triad under reflection: "the axis that reflects C and E into each other also reflects G# into itself."

**Example 2** (p. 55): Other perfectly even structures and their nearly even derivatives: diminished seventh -> dominant/half-diminished sevenths; whole-tone -> mystic chords; maximally even heptachords -> diatonic collections.

# Relationships
## Builds Upon
This is a foundational structural concept.

## Enables
- **near-evenness** — Nearly even chords are defined as minimal perturbations of perfectly even chords
- **augmented-triad-as-axis** — The augmented triad is the perfectly even trichord

## Related
No additional related concepts within this scope.

## Contrasts With
- **near-evenness** — Perfectly even chords lack the "wobble" that enables smooth voice leading
- **consonant-triad** — Consonant triads are nearly (not perfectly) even

# Common Errors
- **Error**: Assuming perfectly even chords voice-lead smoothly to each other
  **Correction**: They cannot, because reflection of a perfectly even chord produces itself (the "phantom" transformation)

# Common Confusions
- **Confusion**: Thinking "perfectly even" means "consonant" or "stable"
  **Clarification**: "Even" refers to intervallic distribution; augmented triads and diminished sevenths are typically considered dissonant

# Source Reference
Chapter 2: Hexatonic Cycles, pp. 52-55. Figures 2.15, 2.17.

# Verification Notes
- Re-extracted from v2 card; preserved: the complete list of perfectly even chords in 12-TET, the "phantom transformation" concept, the Weitzmann historical reference
- Confidence: HIGH — explicitly described with geometric demonstration
