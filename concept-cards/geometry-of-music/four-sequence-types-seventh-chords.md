---
concept: Four Sequence Types for Seventh Chords
slug: four-sequence-types-seventh-chords

category: harmony
subcategory: chromatic-techniques
tier: advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Chromaticism"
chapter_number: 8
pdf_page: 306
section: "8.5"

extraction_confidence: high

aliases:
  - "Chopin's four sequence types"
  - "dominant-seventh sequence varieties"

prerequisites:
  - chopin-open-form
  - tesseract-model-dominant-sevenths
extends: []
related:
  - chopin-e-minor-prelude
  - minor-third-substitution-seventh-chords
contrasts_with: []

answers_questions:
  - "What four fundamental sequence types arise from staggered descent through dominant seventh chords?"
  - "How are these four types related to each other?"
---

# Quick Definition
Four fundamentally related sequence types arise from the same "directions for improvisation" depending on which note of the diminished seventh pivot is lowered: descending by semitone (Mazurka), descending by major third, descending by fifth (Prelude), and ascending by major second.

# Core Definition
Since the diminished seventh chord is completely symmetrical, lowering any of its four notes produces a dominant seventh. The four possible choices correspond to voices that contained the root, third, fifth, or seventh of the original dominant seventh. Lowering the former root produces descending semitones (F minor Mazurka); lowering the former fifth produces descending fifths (E minor Prelude); lowering the former third produces ascending major seconds; lowering the former seventh produces descending major thirds. These four types form a complete system: any chromatic sequence of dominant sevenths connected by staggered semitonal descent will exhibit one of these root motions. They appear repeatedly in Chopin's music and throughout the nineteenth-century repertoire.

# Prerequisites
- Chopin's open form and the staggered-descent procedure
- The tesseract model showing why four types exist

# Key Properties
1. Lower root -> descending semitone
2. Lower third -> ascending major second
3. Lower fifth -> descending fifth
4. Lower seventh -> descending major third
5. All four preserve the overall descending stepwise voice leading
6. The formula: root motion = -1 (mod 12/4), giving -1, -4, -7, -10 semitones

# Construction / Recognition
## To Construct/Create:
1. Start with a dominant seventh (e.g., G7)
2. Lower third, fifth, seventh in any order to reach G diminished
3. Choose which diminished-seventh note to lower:
   - Lower G (root) -> Gb7 (down semitone)
   - Lower B (was third) -> A7 (up major second)
   - Lower D (was fifth) -> C7 (down fifth)
   - Lower F (was seventh) -> E7 (down major third)

## To Identify/Recognize:
1. A series of dominant seventh chords with chromatic passing chords between them
2. The root motion between successive dominant sevenths identifies the type
3. Intermediate chords (diminished, half-diminished, minor seventh, French sixth) between dominants

# Context & Application
Once sensitized to these four types, one finds them throughout Chopin's music and in many other nineteenth-century works. The A minor Mazurka Op. 7 No. 2 combines ascending second and descending semitone types; the F# minor Mazurka Op. 6 No. 1 descends by fifths but ends with a semitone; the Db major Nocturne Op. 27 No. 2 mixes fifths and semitones.

# Examples
**Example 1** (Fig. 8.5.7, p. 307): All four types shown musically in parallel notation.

**Example 2** (Fig. 8.5.8, p. 307): Geometric representation of the four possibilities showing paths through the lattice.

**Example 3** (Fig. 8.5.9, p. 308): Mixed types in Chopin's A minor Mazurka, F# minor Mazurka, and Db major Nocturne.

# Relationships
## Builds Upon
- **chopin-open-form** — The staggered-descent procedure
- **tesseract-model-dominant-sevenths** — The geometric constraint producing four types
## Related
- **chopin-e-minor-prelude** — The descending-fifth type
- **minor-third-substitution-seventh-chords** — The relationship between types

# Common Errors
- **Error**: Thinking these are four unrelated sequences
  **Correction**: They are four manifestations of a single procedure, differing only in one choice

# Common Confusions
- **Confusion**: Assuming each piece uses only one type
  **Clarification**: Chopin frequently mixes types within a single piece

# Source Reference
Chapter 8: Chromaticism, Section 8.5, pages 306-308, Figures 8.5.7-8.5.9.

# Verification Notes
- Definition source: Directly from Section 8.5 with systematic enumeration
- Confidence rationale: High — explicitly laid out with geometric and musical examples
- Cross-reference status: Verified against multiple Chopin pieces
