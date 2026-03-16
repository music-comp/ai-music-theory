---
concept: Tritone Preservation in Substitution
slug: tritone-preservation

category: geometric-theory
subcategory: interval-geometry
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Jazz"
chapter_number: 10
pdf_page: 379
section: "10.3"

extraction_confidence: high

aliases:
  - "tritone invariance"
  - "tritone symmetry"

prerequisites:
  - tritone-substitution-geometry
extends: []
related:
  - near-symmetry
contrasts_with: []

answers_questions:
  - "Why are tritones preserved under tritone transposition?"
  - "Why are perfect fourths/fifths minimally affected by tritone transposition?"
  - "How does the geometry of two-note chord space explain tritone substitution?"
---

# Quick Definition
The geometric fact that tritones are exactly preserved by tritone transposition (they lie on the "mirror" line of the Mobius strip) while perfect fourths and fifths move by only one semitone, providing the foundation for tritone substitution's effectiveness.

# Core Definition
On the two-note Mobius strip, tritone transposition corresponds to reflection across the central horizontal line. Tritones sit exactly on this mirror line and are therefore mapped to themselves (zero voice-leading distance). Perfect fourths and fifths sit very close to the mirror and move by only one semitone under tritone transposition. Since dominant seventh chords decompose into a tritone (3rd and 7th) and a perfect fifth (root and 5th), tritone transposition preserves the crucial tendency tones exactly while perturbing the remaining notes minimally. This same geometric structure explains both descending-fifth progressions (which exploit efficient voice leading between fifth-related dominants) and tritone substitution (which exploits efficient voice leading between tritone-related dominants).

# Prerequisites
- Understanding of tritone substitution geometry

# Key Properties
1. Tritones are invariant under tritone transposition (on the mirror line)
2. Perfect fourths/fifths move by one semitone
3. Major and minor thirds move by two semitones
4. The two voice leadings (fifth-related and tritone-related dominants) are nearly the same size
5. Traditional tonal syntax exploits fifth-related connections; tritone substitution exploits tritone-related ones

# Construction / Recognition
## To Construct/Create:
1. Decompose a dominant seventh into tritone + fifth
2. Apply tritone transposition: tritone stays fixed, fifth moves by semitone
## To Identify/Recognize:
1. When two dominant chords share a tritone, they are related by tritone transposition
2. The shared tritone is the "tendency tones" (3rd and 7th)

# Context & Application
This principle makes the possibility of tritone substitution "latent in the basic routines of traditional tonality." Over the course of history, tonal harmony exploits this possibility with increasing frequency.

# Examples
**Example 1** (p. 380, Fig. 10.3.4): The Mobius strip showing tritones on the mirror line and perfect fourths/fifths near it.

**Example 2** (p. 382-383, Fig. 10.3.8): Voice leading between fifth-related and tritone-related dominant seventh chords are nearly the same size.

# Relationships
## Builds Upon
- **tritone-substitution-geometry** -- The broader context for this geometric fact
## Enables
- Understanding why tritone substitution works
## Related
- **near-symmetry** -- Tritone preservation is a consequence of the near-symmetry of the dominant seventh chord

# Source Reference
Chapter 10: Jazz, Section 10.3, pages 379-383.

# Verification Notes
- Definition source: Explicitly derived from Mobius strip geometry
- Confidence rationale: High -- geometric derivation clearly presented
- Cross-reference status: Connected to Chapter 3's Mobius strip discussion
