---
# === CORE IDENTIFICATION ===
concept: Voice-Leading Efficiency
slug: voice-leading-efficiency

# === CLASSIFICATION ===
category: voice-leading
subcategory: efficiency-metrics
tier: intermediate

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Dmitri Tymoczko"
chapter: "Dualism and the Beholder's Eye"
chapter_number: 8
pdf_page: null
section: "Sections 3-5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "parsimonious motion"
  - "efficient voice leading"
  - "voice-leading parsimony"
  - "minimal voice leading"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading
extends: []
related:
  - inversional-symmetry
  - plr-transformations
  - maximally-smooth-cycles
  - chromatic-second-practice
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is voice-leading parsimony?"
  - "How do PLR transformations relate to voice-leading efficiency?"
  - "Why do efficient voice leadings create inversional relationships?"
---

# Quick Definition

The principle of minimizing the total distance voices travel when moving between chords, with P and L transformations achieving maximal efficiency (one semitone total) and the observation that efficient voice leadings necessarily form inversionally related pairs, explaining the appearance of dualistic patterns in chromatic music.

# Core Definition

**Voice-leading efficiency** measures how much total motion occurs when one chord moves to another. Tymoczko formalizes this through DVLS (Displacement Voice-Leading Size, sum of semitones) and AVLS (Average Voice-Leading Size, DVLS divided by voice count).

The key theoretical insight: since transposition and inversion are the ONLY distance-preserving operations, "the efficient voice leadings between members of any two set classes can always be grouped into inversionally related pairs" (Ch. 8). This means that any musical style emphasizing efficient voice leading will necessarily exhibit inversional symmetry -- dualistic patterns emerge as by-products of contrapuntal parsimony.

# Prerequisites

- **Voice leading** -- The basic concept that efficiency applies to

# Key Properties

1. P and L transformations: DVLS = 1 (one voice, one semitone) -- maximal efficiency for triads
2. R transformation: DVLS = 2 (one voice, one whole tone) -- near-maximal efficiency
3. Major-third related triads: Most efficient same-quality connection (DVLS = 2)
4. Every efficient voice leading has an equally efficient inversional partner
5. Efficient voice leading is "centrifugal" -- it pulls music away from tonal centers

# Construction / Recognition

## Measuring Efficiency
1. Map each voice from source to target chord
2. Calculate semitone displacement for each voice
3. Sum = DVLS; average = AVLS
4. Lower values = more efficient

## PLR Transformations as Efficiency
| Transformation | Voices Moving | Semitones | DVLS |
|---------------|---------------|-----------|------|
| P (Parallel) | 1 | 1 | 1 |
| L (Leittonwechsel) | 1 | 1 | 1 |
| R (Relative) | 1 | 2 | 2 |

## Why Efficiency Creates Inversional Patterns
1. Start with an efficient voice leading (e.g., C major → E major)
2. Apply inversion to the entire voice leading
3. Result: An equally efficient voice leading (Ab major → E minor, or similar)
4. Both are equally likely in a style that values efficiency
5. Therefore: Inversional patterns are ubiquitous without being intentional

# Context & Application

Tymoczko's observation resolves a longstanding puzzle: why does 19th-century chromatic music exhibit inversional symmetry when composers were not thinking dualistically? The answer: they were thinking contrapuntally, seeking smooth connections between familiar sonorities. Inversion preserves smoothness, so inversionally related voice leadings are equally attractive.

This explains why neo-Riemannian theory (which emphasizes inversional operations) is analytically powerful for chromatic music even though composers were not consciously employing inversional thinking.

# Examples

**The 16 semitonal voice leadings** (Ch. 8): All voice leadings between consonant triads where no voice moves more than one semitone. These naturally group into inversionally related pairs.

**Wagner's Tarnhelm and Valhalla** (Ch. 8): The "Tarnhelm" voice leading (G# minor → E minor) and "Valhalla" voice leading (Gb major → F major) are inversionally equivalent. But the significance is that both use maximally efficient voice leading -- "the smoke that escapes from the locomotive's chimney, rather than the furnace that makes it go."

**Brahms Intermezzo Op. 76 No. 4** (Ch. 8): The piece "systematically explores the voice-leading possibilities of a few characteristic sonorities," with the Tristan chord resolving three ways via efficient voice leading. The piece demonstrates compositional control over centrifugal chromatic forces.

# Relationships

## Builds Upon
- **Voice leading** -- Efficiency is a metric applied to voice leading

## Enables
- **Maximally smooth cycles** -- Cycles that optimize voice-leading efficiency
- **Chromatic second practice** -- The compositional approach that prioritizes efficiency
- **PLR transformations** -- Defined by their voice-leading efficiency properties

## Related
- **Inversional symmetry** -- Efficient voice leadings form inversionally related pairs
- **Parsimonious trichords** -- Triads have optimal voice-leading properties in 12-TET

# Common Errors

- **Error**: Concluding that composers used inversional techniques because inversional patterns appear
  **Correction**: Inversional patterns are by-products of voice-leading efficiency; the composers sought smoothness, not inversion

# Common Confusions

- **Confusion**: Voice-leading efficiency is the same as common-tone preservation
  **Clarification**: Related but distinct -- efficiency measures total motion; common-tone preservation counts shared pitches. A voice leading with two common tones and one large leap is less efficient than one with zero common tones and three small moves.

- **Confusion**: Efficient voice leading implies functional connection
  **Clarification**: Efficiency is a contrapuntal property independent of harmonic function; C major can connect efficiently to E major despite having no functional relationship

# Source Reference

Chapter 8: Dmitri Tymoczko, "Dualism and the Beholder's Eye," in *The Oxford Handbook of Neo-Riemannian Music Theories*. Sections 3-5. Also: Cohn, "Maximally Smooth Cycles" (1996); Tymoczko, *A Geometry of Music* (2011).

# Verification Notes

- DVLS/AVLS: Referenced by Tymoczko, formally defined in his other publications
- PLR efficiency values: Derived from standard neo-Riemannian theory, confirmed in Ch. 8
- Inversional pairing: Directly from Tymoczko's central argument
- Confidence: HIGH -- voice-leading efficiency is the central topic of Ch. 8's sections 3-5
- Re-extracted from v2 card; preserved: PLR efficiency table, centrifugal metaphor, Tarnhelm/Valhalla comparison, Brahms analysis, 16 semitonal voice leadings
