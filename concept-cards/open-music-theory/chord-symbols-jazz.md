---
# === CORE IDENTIFICATION ===
concept: Jazz Chord Symbols
slug: chord-symbols-jazz

# === CLASSIFICATION ===
category: harmony
subcategory: jazz-notation
tier: intermediate

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Chord Symbols"
chapter_number: 2
pdf_page: null
section: "VI.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "lead sheet symbols"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - triad-types
  - seventh-chord-types
extends: []
related:
  - roman-numerals
  - jazz-voicings
contrasts_with:
  - roman-numerals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do chord symbols work in jazz lead sheets?"
  - "What are the defaults and alterations in chord symbol notation?"
---

# Quick Definition
Chord symbols are absolute labels (not relative like Roman numerals) that tell performers the root, triad quality, extensions, and bass note. Defaults: triads are major, added sevenths are minor, extensions (9ths, 11ths, 13ths) are major/perfect. Alterations shown with sharp/flat symbols are relative (raise/lower from default).

# Core Definition
Four components: (1) root letter name, (2) triad quality (major by default; mi/m/- for minor, dim/o for diminished, aug/+ for augmented), (3) extensions beyond the triad (7 = minor seventh by default; ma7/maj7 = major seventh; higher extensions implied cumulatively), (4) bass note after slash if non-root. Key conventions: C7 = dominant seventh (major triad + minor 7th); Cma7 = major seventh; Cmi7 = minor seventh. Extensions 9/11/13 are assumed major/perfect unless altered. Altered extensions in parentheses: C7(#11), C7(b9). "add" indicates a note added without implying seventh: Cadd9. "sus" replaces third with fourth (or second): C7sus. Chord symbols are not analytical — they are practical shorthand for performers.

# Prerequisites
- Triad types and seventh chord types
- Interval quality

# Key Properties
1. Absolute labels (not relative to key, unlike Roman numerals)
2. Triads default to major; C = C major triad
3. Seventh defaults to minor; C7 = dominant seventh
4. Extensions default to major/perfect; C13 has all extensions major/perfect except m7
5. Higher extension implies all lower extensions: C13 implies 7th, 9th, 11th, 13th
6. Sharp/flat on extensions are relative: #11 = "raise the 11th," not necessarily a sharp note
7. Slash notation: C/E = C major with E in bass (bass need not be chord member)
8. "add" = add note without implying seventh; "sus" = replace third

# Context & Application
Jazz scores (lead sheets) typically notate only melody and chord symbols. The practical intent means simpler symbols are often preferred: C/Ab may be clearer than Ab7(#5) even if less analytically precise. Extensions are often not all played — performers choose which to include based on voicing conventions. Never completely standardized; multiple notation systems exist.

# Examples
**Example 1**: C = C major triad; Cmi7 = C-Eb-G-Bb; Cma7 = C-E-G-B; C7(#11) = C-E-G-Bb with F#.
**Example 2**: C/Ab is sometimes preferred over Ab7(#5) for practical clarity.

# Relationships
## Builds Upon
- **triad-types** — Foundation of chord symbols
- **seventh-chord-types** — Most common extension
## Related
- **jazz-voicings** — How symbols are realized in performance
## Contrasts With
- **roman-numerals** — Roman numerals are relative to key; chord symbols are absolute

# Common Confusions
- **Confusion**: C7 means major seventh.
  **Clarification**: C7 = dominant seventh (major triad + MINOR seventh). Cma7/Cmaj7 = major seventh.
- **Confusion**: Sharp/flat in symbols means literal sharps/flats.
  **Clarification**: They indicate raising/lowering from the default interval, not literal accidentals.

# Source Reference
Open Music Theory, Part VI: Jazz, Chapter 2: "Chord Symbols."

# Verification Notes
- Re-extracted from source chapter 06-02; merged with existing v2 card
- Preserved distinction between absolute (symbols) and relative (Roman numerals) systems
- Confidence rationale: High — comprehensive source chapter
