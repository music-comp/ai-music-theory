---
concept: Macroharmonic Consistency Measurement
slug: macroharmonic-consistency

category: analysis
subcategory: macroharmony
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 182
section: "5.5"

extraction_confidence: high

aliases:
  - "macroharmonic consistency"
  - "macroharmonic emphasis"

prerequisites:
  - global-macroharmonic-profile
  - macroharmony
extends: []
related:
  - chromaticism-quantification
  - pitch-class-circulation
contrasts_with: []

answers_questions:
  - "How do we measure macroharmonic consistency?"
  - "What does it mean for music to be macroharmonically consistent?"
  - "How does macroharmonic consistency relate to tonality?"
---

# Quick Definition
Macroharmonic consistency is the degree to which a piece emphasizes one or a few structurally similar collections over time, measurable by the peakedness of its global macroharmonic profile.

# Core Definition
A piece is macroharmonically consistent when its pitch-class content, over moderate time spans, consistently reflects the same type of collection (or a small set of transpositionally related collections). This is measured by the global macroharmonic profile: a sharply peaked profile indicates high consistency (the piece clearly articulates specific scales), while a flat profile indicates low consistency. Macroharmonic consistency is logically independent of pitch-class circulation rate — music can circulate through pitch classes quickly while maintaining clear macroharmonic identity (Coltrane), or slowly while lacking it (Satie). The concept extends harmonic consistency to a larger time scale: just as chords within a phrase can be structurally similar, the collections articulated over successive passages can be transpositionally related.

# Prerequisites
- **global-macroharmonic-profile** — The measurement tool for consistency
- **macroharmony** — The concept being evaluated

# Key Properties
1. Measured by the peakedness of the global macroharmonic profile
2. Independent of pitch-class circulation rate
3. Trivially present in fully chromatic music (the chromatic scale is always the macroharmony)
4. Non-trivially present when the music articulates identifiable subcollections
5. Parallels harmonic consistency but operates on a larger time scale

# Construction / Recognition
## To Construct/Create:
1. Choose a target collection type (diatonic, acoustic, octatonic, etc.)
2. Ensure that moderate-length passages clearly articulate transpositions of this collection
3. Allow modulation between transpositions while maintaining the collection type
## To Identify/Recognize:
1. Compute the global macroharmonic profile
2. Check for strong peaks at particular set classes
3. Compare peak height to overall distribution

# Context & Application
Macroharmonic consistency explains the difference between fast-moving tonal music and atonal music. Coltrane's "Giant Steps" modulates so rapidly that its pitch-class circulation matches Schoenberg's Op. 11, yet Coltrane is clearly tonal because his macroharmonic profile is sharply peaked at the diatonic scale. Schoenberg's profile is flat — no collection dominates. Similarly, Debussy's music is consistently oriented toward particular scales (high macroharmonic consistency) while Satie's sporadic accidentals lack systematic macroharmonic organization (low consistency), even though both have low circulation rates.

# Examples
**Example 1** (p. 182-184): Coltrane's "Giant Steps" — high macroharmonic consistency (peaked at diatonic scale) despite fast circulation; Schoenberg's Op. 11 — low consistency (flat profile) with equally fast circulation.

**Example 2** (p. 183-184): Debussy's "La fille aux cheveux de lin" — high consistency (clearly articulates Eb minor and Gb acoustic); Satie's "Theme of the Order" — low consistency (accidentals appear sporadically without system).

# Relationships
## Builds Upon
- **global-macroharmonic-profile** — The tool that measures consistency
- **macroharmony** — The concept being evaluated
## Enables
- **chromaticism-quantification** — One axis of the chromaticism framework
## Related
- **pitch-class-circulation** — Independent but complementary measure
## Contrasts With
- None specifically

# Common Errors
- **Error**: Equating macroharmonic consistency with slow pitch-class circulation
  **Correction**: They are independent: Coltrane has fast circulation but high consistency; Satie has slow circulation but low consistency

# Common Confusions
- **Confusion**: Noting that atonal music is "consistent" by always using the chromatic scale
  **Clarification**: Tymoczko acknowledges this is trivially true but uninteresting; non-trivial macroharmonic consistency requires the music to articulate identifiable subcollections of the chromatic scale

# Source Reference
Chapter 5: Macroharmony and Centricity, Section 5.5, pages 182-187.

# Verification Notes
- Definition source: Section 5.5, discussion of macroharmonic profiles and their interpretation
- Confidence rationale: High — central concept with multiple worked examples
- Cross-reference status: Applied throughout Part II analyses
