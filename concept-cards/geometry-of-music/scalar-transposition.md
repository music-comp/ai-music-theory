---
concept: Scalar Transposition
slug: scalar-transposition

category: scales-modes
subcategory: operations
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 137
section: "4.2"

extraction_confidence: high

aliases:
  - "diatonic transposition"
  - "transposition within a scale"
  - "step transposition"

prerequisites:
  - scale-as-ruler
  - scale-degree-arithmetic
extends: []
related:
  - scalar-inversion
  - interscalar-transposition
  - goldilocks-principle
contrasts_with:
  - chromatic-transposition

answers_questions:
  - "What is scalar transposition?"
  - "How does scalar transposition differ from chromatic transposition?"
  - "When does scalar transposition produce musically useful results?"
---

# Quick Definition
Scalar transposition shifts a musical pattern by a fixed number of scale steps, preserving scalar intervals while potentially altering chromatic intervals. It is the scale-based analogue of chromatic transposition.

# Core Definition
Scalar transposition is the operation of adding a constant to each scale degree number. Given a melody described by scale degree numbers (e.g., 1, 2, 3), scalar transposition by n steps produces (1+n, 2+n, 3+n), using "scale degree arithmetic" (modular arithmetic wrapping at the scale's size). Unlike chromatic transposition, which preserves exact semitone intervals, scalar transposition preserves scalar intervals (seconds, thirds, etc.) while allowing chromatic intervals to vary. For example, in C major, transposing (C, D, E) up by one step yields (D, E, F) — both are "do re mi" patterns, but the chromatic intervals change from (2, 2) to (2, 1) semitones. Scalar transposition can even act on notes outside the scale by assigning fractional scale degree numbers to chromatic pitches.

# Prerequisites
- Scale as ruler (the metric framework)
- Scale degree arithmetic

# Key Properties
1. Adds a constant to each scale degree number
2. Preserves scalar intervals (seconds, thirds, etc.)
3. May alter chromatic intervals (semitone sizes change)
4. Produces exact chromatic transposition only when the scale is perfectly even
5. Can act on notes outside the scale via fractional scale degree numbers
6. The induced chromatic variations are musically interesting when small (Goldilocks Principle)

# Construction / Recognition
## To Apply:
1. Assign scale degree numbers to the notes
2. Add the transposition constant to each
3. Use scale degree arithmetic (wrap around at the scale's size)
4. Convert back to pitch names
## For Notes Outside the Scale:
1. Assign fractional scale degree numbers (e.g., C# = degree 1.5 in C major)
2. Apply the same addition

# Context & Application
Scalar transposition is ubiquitous in tonal music — it is what happens when a musical pattern is repeated at a different scale level (a "real" sequence becomes a "tonal" sequence). It is one of the most basic compositional techniques, found in everything from Bach fugues to pop songs. The concept becomes especially powerful in twentieth-century music, where composers like Debussy, Shostakovich, and Steve Reich combine scalar and chromatic transposition to create complex transformations.

# Examples
**Example 1** (p. 137-138): In C major, transposing (C, D, E) up by one step yields (D, E, F) — the same scalar pattern, but with different chromatic intervals (Figure 4.2.2a).
**Example 2** (p. 138): In C harmonic minor, {B, D, F} and {C, Eb, G} are both "triads" (stacks of two scalar thirds), transpositionally related by one scale step, even though one is diminished and the other is minor (Figure 4.2.3).
**Example 3** (p. 139): Scalar transposition can act on out-of-scale notes: shifting (C, D, E) up by half a scale step produces (C#, D#, E-quarter-sharp) in C major.

# Relationships
## Builds Upon
- **scale-as-ruler** — Defines the units used for transposition
- **scale-degree-arithmetic** — The mathematical framework
## Enables
- **interscalar-transposition** — Generalization to transposition between different scales
- **goldilocks-principle** — Evenness determines the quality of scalar transposition
## Related
- **scalar-inversion** — The other distance-preserving scalar operation
## Contrasts With
- **chromatic-transposition** — Preserves chromatic intervals instead of scalar ones

# Common Errors
- **Error**: Assuming scalar transposition always preserves chord quality
  **Correction**: Scalar transposition preserves scalar chord type but typically changes chromatic chord quality (e.g., major triad -> minor triad)

# Common Confusions
- **Confusion**: Is scalar transposition the same as modulation?
  **Clarification**: No. Scalar transposition moves a pattern within a fixed scale; modulation changes the scale itself. However, combining scalar and chromatic transposition can produce something like modulation (Section 4.8).

# Source Reference
Chapter 4: Scales, Section 4.2, pages 137-140.

# Verification Notes
- Definition source: Directly from Section 4.2
- Confidence rationale: High — formally defined with algebraic notation
- Cross-reference status: Verified against examples in Sections 4.2, 4.7, and 4.8
