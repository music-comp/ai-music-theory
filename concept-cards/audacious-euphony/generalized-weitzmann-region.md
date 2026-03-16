---
# === CORE IDENTIFICATION ===
concept: "Generalized Weitzmann Region (GWR)"
slug: generalized-weitzmann-region

# === CLASSIFICATION ===
category: neo-riemannian-theory
subcategory: generalized regions
tier: advanced

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Dissonance"
chapter_number: 7
pdf_page: 183
section: "Scriabin's Mystic Species and Generalized Weitzmann Regions"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "GWR"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - weitzmann-region
  - boretz-region
  - nearly-even
  - perfectly-even-glossary
extends:
  - weitzmann-region
  - boretz-region
related:
  - mystic-chord
  - wozzeck-chord
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a generalized Weitzmann region?"
  - "How do Weitzmann and Boretz regions relate to a general principle?"
  - "Why do triads have special voice-leading properties compared to larger chords?"
---

# Quick Definition
A generalized Weitzmann region (GWR) is the collection of all nearly even chords of cardinality n related to a single perfectly even chord by single semitonal displacement, abstracting the common structure of Weitzmann regions (triads) and Boretz regions (seventh chords) into a general framework applicable to chords of any size.

# Core Definition
"Given some nonprime universe of *nq* tones (*n* and *q* are integers, *n* > 2, *q* > 1), there exist *q* perfectly equal divisions of chord size *n*. Each perfectly even division serves as the *core* for 2*n* nearly even chords: *n upshifters* that result from upward semitonal perturbation, and *n downshifters* that result from downward perturbation. Each such region plays a role corresponding to that of a Weitzmann region (for q = 4, n = 3) or a Boretz region (for q = 3, n = 4); I shall refer to it as a *generalized Weitzmann region* (GWR)" (p. 183). Any two chords sharing a GWR "are exactly two voice-leading units apart, whether related to each other by transposition or inversion."

# Prerequisites
- **Weitzmann region**: The triadic instance (n=3, q=4)
- **Boretz region**: The tetrachordal instance (n=3, q=3)
- **Nearly even chord**: The chord type populating GWRs
- **Perfectly even chord**: The core of each GWR

# Key Properties
1. Each GWR has 2n members (n upshifters + n downshifters)
2. Intra-regional progressions always involve exactly 2 voice-leading units regardless of chord size
3. Bridging between adjacent GWRs requires n-2 moving voices
4. For n=3 (triads): bridges involve 1 moving voice (minimal work -- hence triads' special status)
5. For n=4 (Tristan genus): bridges involve 2 moving voices
6. For n=6 (mystic/Wozzeck): bridges would involve 4 moving voices (impractical)

# Construction / Recognition
Instances:
- **n=3, q=4** (triads in 12-tone): 4 Weitzmann regions, each with 6 triads around 1 augmented triad
- **n=4, q=3** (Tristan genus in 12-tone): 3 Boretz regions, each with 8 seventh chords around 1 diminished seventh
- **n=6, q=2** (hexachords in 12-tone): 2 GWRs, each with 12 chords (6 mystic + 6 Wozzeck) around 1 whole-tone scale

Adjacent GWRs and bridging:
- Two GWRs are adjacent if their cores differ by semitone
- The n upshifters of the lower GWR and n downshifters of the higher GWR form a bridging region
- The collective exchange (generalized H) displaces n-1 voices one direction and 1 voice opposite

# Context & Application
The GWR framework reveals why triads have uniquely privileged voice-leading properties: "from the standpoint of minimal voice leading, the ideal situation occurs when n = 3. It is only then that n - 2 = 1, that is, that a bridging motion involves only a single unit of voice-leading work" (p. 184). As chord cardinality grows, bridging becomes increasingly less efficient. The framework also shows that nearly even chords of prime cardinality relative to the chromatic universe (e.g., diatonic and pentatonic scales in 12-tone) do not participate in GWRs.

# Examples
- **Weitzmann regions (n=3)**: 4 GWRs partitioning 24 consonant triads (pp. 183-184)
- **Boretz regions (n=4)**: 3 GWRs partitioning 24 Tristan-genus chords (p. 183)
- **Hexachordal GWRs (n=6)**: Scriabin's mystic chords and Berg's Wozzeck chords as nearly even hexachords around whole-tone cores (pp. 183-184)

# Relationships
## Builds Upon
- Weitzmann region (the triadic template)
- Boretz region (the tetrachordal template)
## Enables
- Understanding why triads have uniquely efficient voice leading
- Extension of voice-leading theory to any chord cardinality
## Related
- Mystic chord and Wozzeck chord (the hexachordal instantiation)
## Contrasts With
- (No direct contrast; GWR is a generalization, not an alternative)

# Common Errors
- **Error**: Thinking GWR is a new concept alongside Weitzmann and Boretz regions
  **Correction**: It names the common structure already present in both; it is a generalization, not an addition

# Common Confusions
- **Confusion**: Assuming larger cardinalities produce more useful voice-leading systems
  **Clarification**: Larger n means more moving voices in bridges (n-2), making bridging progressions increasingly unparsimonious

# Source Reference
Cohn, R. *Audacious Euphony*, Chapter 7: "Dissonance," pp. 183-185.

# Verification Notes
Re-extracted from v2 card; preserved: three cardinality examples, formal definition. Fresh extraction adds extensive direct quotations, bridging formula (n-2), collective exchange generalization, and prime cardinality exclusion.
