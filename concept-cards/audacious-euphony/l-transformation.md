---
# === CORE IDENTIFICATION ===
concept: "L Transformation (Leittonwechsel)"
slug: l-transformation

# === CLASSIFICATION ===
category: transformations
subcategory: null
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Hexatonic Cycles"
chapter_number: 2
pdf_page: 35
section: "Hexatonic Progressions, Tonnetz Representations, and Triadic Transformations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Leittonwechsel"
  - "leading-tone exchange"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - single-semitonal-displacement
  - minimal-work-relation
extends: []
related:
  - p-transformation
  - h-transformation
  - lp-transformation
  - hexatonic-cycle
  - chromatic-versus-diatonic-semitone
contrasts_with:
  - p-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the L transformation?"
  - "What is a Leittonwechsel?"
  - "How does L differ from P?"
---

# Quick Definition
The L (Leittonwechsel or leading-tone exchange) transformation connects opposite-mode triads that share a minor-third dyad, moving one voice by diatonic semitone, with roots a major third apart.

# Core Definition
Cohn uses "**L** (*Leittonwechsel*) to indicate triads that share two common tones and whose roots are a major third apart" (p. 47). L connects "opposite-mode triads that share a minor-third dyad, as exemplified by the diagonal arrows in figure 2.10" (p. 47). The operation was identified by Arthur von Oettingen (1866), developed by Riemann (1880), and "revived and formalized a century later by David Lewin (1982, 1987)" (p. 47). Like P, L is an involution. L produces a diatonic semitone (different letter names), appearing as diagonal motion (120 degrees) on the Tonnetz (Table 2.1, p. 48).

# Prerequisites
- **single-semitonal-displacement** — L involves one voice moving one semitone
- **minimal-work-relation** — L is one of two transformations realizing the minimal-work relation

# Key Properties
1. Voice-leading work: 1 semitone
2. Common tones: 2 (the minor third)
3. Root motion: major third (4 semitones)
4. Semitonal species: diatonic (different letter names)
5. Tonnetz angle: 120 degrees (diagonal)
6. Involution: L(L(X)) = X

# Construction / Recognition
- L(C major) = e minor: (**C**, E, G) -> (**B**, E, G); root descends by semitone
- L(e minor) = C major: (E, G, **B**) -> (E, G, **C**); fifth ascends by semitone
- L(Ab major) = c minor: (**Ab**, C, Eb) -> (**G**, C, Eb)

For major triads: root moves down by semitone
For minor triads: fifth moves up by semitone

# Context & Application
In hexatonic cycles, L alternates with P to generate smooth chromatic progressions. On the Tonnetz, L appears as diagonal motion. The term "Leittonwechsel" reflects the traditional leading-tone relationship: each triad contains the other's leading tone. The bidirectionality of the leading tone is important: "B leads up to C, but C also leads down to B" in nineteenth-century German theory.

# Examples
**Example 1** (Fig. 2.10, p. 47): L transformations appear as diagonal arrows on a Tonnetz strip, alternating with P's vertical arrows.

**Example 2** (p. 48): Brahms, Concerto for Violin and Cello, Op. 102, mm. 270-79: L transformations in a hexatonic progression.

**Example 3** (p. 47): Historical origin: "Oettingen's exchange operations were developed by Riemann (1880), and some of them were revived and formalized a century later by David Lewin (1982, 1987)."

# Relationships
## Builds Upon
- **single-semitonal-displacement** — L realizes a single semitonal displacement
- **minimal-work-relation** — L is one of two minimal-work transformations

## Enables
- **hexatonic-cycle** — L alternates with P to generate the cycle
- **h-transformation** — H = LP = PL
- **lp-transformation** — LP combines L and P

## Related
- **chromatic-versus-diatonic-semitone** — L produces the diatonic species

## Contrasts With
- **p-transformation** — L preserves the minor third, P preserves the perfect fifth; L produces diatonic semitone, P produces chromatic semitone

# Common Errors
- **Error**: Assuming L moves the root in the same direction for both major and minor triads
  **Correction**: For major triads, the root descends; for minor triads, the fifth ascends

# Common Confusions
- **Confusion**: Thinking L connects triads whose roots are a minor third apart
  **Clarification**: L connects triads whose roots are a major third (4 semitones) apart

# Source Reference
Chapter 2: Hexatonic Cycles, pp. 47-48. Table 2.1 on p. 48. Historical origins in Oettingen (1866), Riemann (1880), Lewin (1982, 1987).

# Verification Notes
- Re-extracted from v2 card; preserved: the Oettingen/Riemann/Lewin lineage, the Table 2.1 data, the bidirectional leading-tone note
- Confidence: HIGH — explicitly defined in Table 2.1 with clear notation and historical attribution
