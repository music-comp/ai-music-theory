---
concept: Near Identity (Musical Similarity as Voice-Leading Distance)
slug: near-identity

category: geometric-theory
subcategory: distance
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.8"

extraction_confidence: high

aliases:
  - "near-transpositional equivalence"
  - "approximate harmonic identity"
  - "voice-leading similarity"

prerequisites:
  - chord
  - voice-leading-size
extends:
  - harmonic-consistency
related:
  - near-symmetry
  - efficient-voice-leading
contrasts_with: []

answers_questions:
  - "How can musical similarity be measured by voice-leading distance?"
  - "What is harmonic consistency?"
---

# Quick Definition
Near identity uses voice-leading distance to model the intuition that two chords can be "similar" without being exactly transpositionally or inversionally related — the smaller the minimal voice leading between them, the more similar they are.

# Core Definition
Tymoczko proposes that exact transpositional or inversional identity is too restrictive. In practice, musicians treat chords as "similar" along a continuum: the equal-tempered {C, E, G, Bb} is nearly identical to its just-intonation counterpart, somewhat related to the minor seventh {C, Eb, G, Bb}, and very dissimilar to {F#, G, Ab, A}. These degrees of relatedness mirror voice-leading distances: there is a tiny voice leading between tuning variants, a small one to the minor seventh (move E to Eb), and a large one to the chromatic cluster. Near identity extends to chord types: two chord types are similar if there is a small voice leading between their transpositions. Thus the diminished triad is closer to the minor triad than to the chromatic cluster. Tymoczko calls this "near transpositional relatedness" and "near inversional relatedness."

# Prerequisites
- **chord** — The objects being compared
- **voice-leading-size** — The measure of similarity

# Key Properties
1. Musical similarity forms a continuum, not a binary
2. Voice-leading distance models this continuum
3. Extends to chord types (near-transpositional relatedness) and set classes (near-inversional relatedness)
4. Resolves issues of tuning/temperament (different tunings of "the same chord" are near-identical)
5. Chapter 3 will show this corresponds to actual geometric distance in chord space

# Construction / Recognition
## To Construct/Create:
1. Choose two chords
2. Find the minimal voice leading between them
3. The smaller the voice leading, the more "similar" the chords
## To Identify/Recognize:
1. Two chords connected by a small voice leading are "near" each other
2. Two chords connected only by a large voice leading are "far" apart

# Context & Application
Near identity allows the harmonic consistency constraint to be relaxed from "exactly the same chord type" to "approximately the same chord type." This is more realistic: composers frequently move between related but not identical chord types (e.g., dominant seventh to minor seventh). The concept also handles tuning issues elegantly — different temperaments of "the same chord" are genuinely near-identical.

# Examples
**Example 1** (p. 69-70): The equal-tempered {C, E, G, Bb} is very close to just-intonation {C, E, G, Bb}, somewhat close to {C, Eb, G, Bb}, and far from {F#, G, Ab, A}.

**Example 2** (p. 70): The diminished triad is "closer" to the minor triad than to the chromatic cluster, since (C, Eb, Gb) -> (C, Eb, G) is a one-semitone voice leading, but it takes at least four semitones to connect any diminished triad to any chromatic cluster.

# Relationships
## Builds Upon
- **chord** — The objects being compared
- **voice-leading-size** — The metric of similarity
## Enables
- Flexible harmonic analysis that doesn't require exact chord-type matching
- Chapter 3's geometric chord spaces
## Related
- **near-symmetry** — Expanding harmonic consistency to near-consistency doesn't change the near-symmetry requirement
- **harmonic-consistency** — Near identity relaxes the consistency requirement
## Contrasts With
- No direct contrast within this source

# Common Errors
- **Error**: Thinking voice-leading distance is the only notion of musical similarity
  **Correction**: Tymoczko acknowledges other possibilities (common tones, shared interval content, diatonic membership) but focuses on voice-leading distance for its versatility

# Common Confusions
- **Confusion**: Thinking near identity eliminates the need for exact equivalence classes
  **Clarification**: Exact equivalences (transposition, inversion) remain important; near identity supplements them with a continuous notion of similarity

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.8, pages 69-70.

# Verification Notes
- Definition source: Direct from Section 2.8
- Confidence rationale: High — clearly defined with examples
- Cross-reference status: Verified; formalized as distance in chord space in Chapter 3
