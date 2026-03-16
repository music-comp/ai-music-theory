---
concept: Strongly Crossing-Free Voice Leading
slug: strongly-crossing-free-voice-leading

category: voice-leading
subcategory: properties
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 163
section: "4.9"

extraction_confidence: high

aliases:
  - "strongly crossing free"
  - "registrally robust crossing-free"

prerequisites:
  - interscalar-transposition
extends: []
related:
  - voice-leading-size
  - combining-scalar-chromatic-transposition
contrasts_with: []

answers_questions:
  - "What is a 'strongly crossing-free' voice leading?"
  - "How does it differ from ordinary crossing-free voice leading?"
  - "Why is it connected to interscalar transposition?"
---

# Quick Definition
A voice leading that remains crossing-free no matter how its voices are distributed in register. Every strongly crossing-free voice leading is an interscalar (or scalar) transposition, and vice versa. This equivalence connects the practical concern of avoiding voice crossings to the theoretical concept of interscalar transposition.

# Core Definition
A voice leading is "strongly crossing-free" if no octave transposition of any voice can create a crossing. This is a stronger condition than ordinary crossing-free voice leading, which can become crossed when voices change octave. The key theorem is: a voice leading is strongly crossing-free if and only if it is a scalar or interscalar transposition. The proof relies on the fact that a strongly crossing-free voice leading can always be arranged so that each chord spans less than an octave, with ascending steps in one chord mapping to ascending steps in the other — which is precisely the definition of interscalar transposition. Since removing crossings never increases voice-leading size, there is always a maximally efficient voice leading that is strongly crossing-free, hence an interscalar transposition.

# Prerequisites
- Interscalar transposition
- Voice-leading size

# Key Properties
1. Remains crossing-free regardless of octave placement of voices
2. Equivalent to being a scalar or interscalar transposition
3. Maximally efficient voice leadings are always strongly crossing-free
4. Dramatically reduces the search space for efficient voice leadings (by factor of n for n-note chords)
5. Corresponds to paths that do not bounce off mirror boundaries of chord space (Appendix A)

# Construction / Recognition
## To Test:
1. Try all possible octave placements of the voices
2. If no placement creates a crossing, the voice leading is strongly crossing-free
## Shortcut:
1. Arrange both chords in close position (spanning < octave)
2. Check if ascending steps map to ascending steps

# Context & Application
This concept explains why voice-crossing avoidance is so pedagogically useful: by teaching students to avoid crossings, harmony teachers are implicitly directing them toward interscalar transpositions, which are both maximally efficient and easy to find. The concept also explains how composers manage to find efficient voice leadings so quickly — they need only search the small number of interscalar transpositions.

# Examples
**Example 1** (p. 163): Figure 4.9.1a shows a strongly crossing-free voice leading (no octave rearrangement creates crossings). Figure 4.9.1b shows a voice leading that is crossing-free but not strongly so — transposing one voice up by octave creates a crossing.
**Example 2** (p. 164): Figure 4.9.3 shows the process of repeatedly removing crossings and octave-transposing until a strongly crossing-free voice leading is reached.

# Relationships
## Builds Upon
- **interscalar-transposition** — Equivalent characterization
## Enables
- **combining-scalar-chromatic-transposition** — Uses the strongly crossing-free property
## Related
- **voice-leading-size** — Strongly crossing-free voice leadings include the most efficient options
- **generalized-line-segments** — In chord space, these are paths that don't bounce off mirrors

# Common Errors
- **Error**: Thinking any crossing-free voice leading is strongly crossing-free
  **Correction**: Ordinary crossing-free voice leadings may become crossed when voices change octave

# Common Confusions
- **Confusion**: Does this mean voice crossings are always bad?
  **Clarification**: No — crossings are rare (~5% in Renaissance music) but sometimes useful for melodic interest. The point is that maximally efficient voice leadings never require crossings.

# Source Reference
Chapter 4: Scales, Section 4.9, pages 163-167.

# Verification Notes
- Definition source: From Section 4.9
- Confidence rationale: High — formally stated equivalence with proof sketch
- Cross-reference status: Verified against Appendix A and the statistical note on crossing frequency
