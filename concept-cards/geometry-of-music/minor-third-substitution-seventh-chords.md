---
# === CORE IDENTIFICATION ===
concept: Minor-Third Substitution for Seventh Chords
slug: minor-third-substitution-seventh-chords

# === CLASSIFICATION ===
category: harmony
subcategory: chromatic-techniques
tier: advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Chromaticism"
chapter_number: 8
pdf_page: 307
section: "8.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "seventh-chord minor-third relations"
  - "diminished-seventh substitution"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - tesseract-model-dominant-sevenths
extends: []
related:
  - major-third-substitution
  - tritone-substitution-historical
contrasts_with:
  - major-third-substitution

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why do seventh chords substitute by minor third while triads substitute by major third?"
  - "How does the diminished seventh chord enable minor-third substitution?"
---

# Quick Definition
Minor-third substitution for seventh chords replaces one dominant seventh with another whose root is a minor third (or tritone) away — the seventh-chord analogue of major-third substitution for triads, enabled by the diminished seventh's four-fold symmetry and the tesseract's geometry.

# Core Definition
Chopin's descending semitonal motion produces harmonic progressions that can be obtained by applying minor-third substitution to a descending semitonal sequence: the resulting root motions are descending semitone, descending major third, descending fifth, and ascending major second. This follows from the formula -1 (mod 12/n) where n=4: the possible root-motion intervals are -1, -4, -7, and -10 semitones. The diminished seventh chord, which is completely symmetrical (four-fold), serves as the pivot: since lowering any of its four notes produces a dominant seventh, there are four possible continuations from each cycle's diminished-seventh midpoint. By contrast, triadic substitution uses major thirds (from three-fold augmented-triad symmetry), giving root motions of -1, -5, and -9 semitones.

# Prerequisites
- The tesseract model for dominant seventh chords

# Key Properties
1. Root-motion possibilities: -1, -4, -7, -10 semitones (for seventh chords)
2. Compared to -1, -5, -9 for triads (major-third substitution)
3. The diminished seventh's 4-fold symmetry enables the substitution
4. Tritone substitution is a special case of minor-third substitution
5. Minor-third-related seventh chords share more common tones than tritone-related ones

# Construction / Recognition
## To Construct/Create:
1. Take a dominant seventh (e.g., G7)
2. Its minor-third substitutes are Bb7, Db7, and E7
3. Each can replace G7 in a sequential context while preserving overall voice-leading descent
4. The diminished seventh (G, Bb, Db, E) is the pivot common to all four

## To Identify/Recognize:
1. Dominant seventh chords related by minor third or tritone in close succession
2. Voice leading that predominantly descends between the chords
3. A diminished seventh (or other symmetrical chord) serving as the pivot point

# Context & Application
This explains the distinctive root-motion vocabulary of seventh-chord chromaticism. Where triadic chromaticism produces major-third relations (Schubert), seventh-chord chromaticism produces minor-third and tritone relations (Chopin, Wagner, jazz). The asymmetry between triadic and seventh-chord chromaticism is one of Tymoczko's key explanatory achievements.

# Examples
**Example 1** (Fig. 8.5.7, p. 307): Four sequence types from the same "directions for improvisation" — descending by semitone, major third, fifth, and ascending by major second.

**Example 2** (Fig. 8.5.9, p. 308): Chopin mixing different substitution types within single pieces.

# Relationships
## Builds Upon
- **tesseract-model-dominant-sevenths** — The geometric basis
## Related
- **major-third-substitution** — The triadic analogue
- **tritone-substitution-historical** — A special case
## Contrasts With
- **major-third-substitution** — Triads use major thirds; seventh chords use minor thirds

# Common Errors
- **Error**: Assuming seventh chords should substitute by major third like triads
  **Correction**: The geometry is different — seventh chords are near diminished sevenths (4-fold symmetry) not augmented triads (3-fold)

# Common Confusions
- **Confusion**: Thinking minor-third substitution and tritone substitution are different mechanisms
  **Clarification**: Tritone substitution is a special case of the same geometric principle

# Source Reference
Chapter 8: Chromaticism, Section 8.5, pages 307-308.

# Verification Notes
- Definition source: From Section 8.5 discussion and the -1 (mod 12/n) formula
- Confidence rationale: High — fundamental geometric result with clear examples
- Cross-reference status: Verified against Chapter 3 discussion of near symmetries
