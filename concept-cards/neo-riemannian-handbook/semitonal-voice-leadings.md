---
concept: Semitonal Voice Leadings Between Triads
slug: semitonal-voice-leadings

category: voice-leading
subcategory: cataloging
tier: intermediate

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Dmitri Tymoczko"
chapter: "Dualism and the Beholder's Eye"
chapter_number: 8
pdf_page: null
section: "Section 3"

extraction_confidence: high

aliases:
  - "16 semitonal voice leadings"
  - "maximally efficient triadic connections"
  - "semitone-only voice leadings"

prerequisites:
  - voice-leading
  - voice-leading-efficiency
extends: []
related:
  - plr-transformations
  - maximally-smooth-cycles
  - inversional-symmetry
  - chromatic-second-practice
contrasts_with: []

answers_questions:
  - "How many semitonal voice leadings exist between consonant triads?"
  - "How can voice-leading possibilities be systematically cataloged?"
---

# Quick Definition

The complete catalog of 16 voice leadings between consonant triads where no individual voice moves more than one semitone, representing all maximally efficient connections between triads and naturally grouping into inversionally related, retrograde-related, and individually transpositionally equivalent pairs.

# Core Definition

Tymoczko catalogs all voice leadings between consonant triads (major and minor) where no voice moves more than one semitone (Ch. 8, Section 3, Example 8.6). There are exactly **16 such voice leadings** (beginning from C major or C minor), and they can be grouped by three equivalence relations:

1. **Inversional equivalence**: Pairs related by uniform inversion
2. **Retrograde equivalence**: Pairs where one is the reverse of the other
3. **Individual transpositional equivalence**: Pairs where the destination chord is individually transposed

These groupings reveal that the voice-leading possibilities of triads are highly structured and symmetric.

# Prerequisites

- **Voice leading** -- The basic concept of pitch-to-pitch mapping between chords
- **Voice-leading efficiency** -- The criterion for including a voice leading in the catalog

# Key Properties

1. Exactly 16 semitonal voice leadings exist between consonant triads
2. P and L transformations appear as single-semitone voice leadings (DVLS = 1)
3. Major-third relations involve two semitone moves (DVLS = 2)
4. Minor-third relations also appear with DVLS = 2
5. Perfect-fifth relations require more total motion and do not appear in the catalog
6. All 16 can be grouped into inversionally related pairs

# Construction / Recognition

## Generating the Catalog
1. Start from C major (or C minor) in close position
2. Allow each voice to move at most one semitone (up, down, or stay)
3. Require the destination to be a consonant triad (major or minor)
4. Enumerate all possibilities: 16 voice leadings result

## Notable Voice Leadings in the Catalog
- C major → C minor (P): one voice moves (E→Eb), DVLS = 1
- C major → E minor (L): one voice moves (C→B), DVLS = 1
- C major → E major: two voices move (C→B, G→G#), DVLS = 2
- C major → Ab major: two voices move (E→Eb, G→Ab), DVLS = 2
- C major → Db major: two voices move (C→Db, E→F), DVLS = 2

# Context & Application

The catalog systematizes what 19th-century chromatic composers discovered intuitively: the most efficient connections between triads. Major-third related triads (C→E, C→Ab) appear frequently in chromatic music precisely because they require only two semitone moves -- the most efficient connection between same-quality triads.

The catalog also demonstrates why dualistic patterns emerge: each voice leading has an inversional partner of equal efficiency, so composers seeking smooth connections inevitably produce inversionally related progressions.

# Examples

**P transformation** (Ch. 8): (C, E, G) → (C, Eb, G). DVLS = 1. The most efficient voice leading that changes chord quality.

**L transformation** (Ch. 8): (C, E, G) → (B, E, G). DVLS = 1. Equally efficient, changing both quality and root.

**Major-third relation** (Ch. 8): (C, E, G) → (B, E, G#). DVLS = 2. The most efficient connection between distinct major triads. This explains the frequency of Lisztian and Wagnerian major-third cycles.

# Relationships

## Builds Upon
- **Voice leading** -- The catalog systematizes voice-leading possibilities
- **Voice-leading efficiency** -- The selection criterion

## Enables
- **Chromatic second practice** -- The catalog provides the vocabulary for chromatic connections
- **Maximally smooth cycles** -- Chains of semitonal voice leadings

## Related
- **PLR transformations** -- P and L are the single-semitone members of the catalog
- **Inversional symmetry** -- The catalog's internal structure reveals inversional pairing

# Common Errors

- **Error**: Assuming the 16 voice leadings are all equally common in practice
  **Correction**: Musical context, style, and aesthetic preference constrain which efficient voice leadings composers actually use

# Common Confusions

- **Confusion**: "Semitonal voice leading" means all voices move by semitone
  **Clarification**: It means NO voice moves MORE than a semitone; voices may hold or not move at all

# Source Reference

Chapter 8: Dmitri Tymoczko, "Dualism and the Beholder's Eye," in *The Oxford Handbook of Neo-Riemannian Music Theories*. Section 3, Example 8.6.

# Verification Notes

- Count of 16: From Tymoczko's explicit catalog in Example 8.6
- Grouping by equivalence relations: From Tymoczko's discussion
- Confidence: HIGH -- explicitly enumerated and discussed
- New card (no previous v2 card)
