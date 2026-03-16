---
# === CORE IDENTIFICATION ===
concept: PLR Transformations
slug: plr-transformations

# === CLASSIFICATION ===
category: transformations
subcategory: neo-riemannian-operations
tier: intermediate

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Suzannah Clark"
chapter: "Analyzing Schubert"
chapter_number: 10
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "P, L, R operations"
  - "Parallel, Leittonwechsel, Relative"
  - "neo-Riemannian PLR group"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang
extends: []
related:
  - parallel-transformation
  - leittonwechsel
  - relative-transformation
  - neo-riemannian-operations
  - hexatonic-systems
  - tonnetz
contrasts_with:
  - common-tone-tonality

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the three fundamental PLR transformations?"
  - "How does the PLR system originate in Riemann's late writings?"
  - "What is the mathematical group structure of the PLR operations?"
---

# Quick Definition

The three fundamental neo-Riemannian operations (Parallel, Leittonwechsel, Relative) that transform major and minor triads by moving a single voice by semitone or whole tone while preserving two common tones.

# Core Definition

The **PLR transformations** are contextual, mode-reversing operations on consonant triads. **P** (Parallel) holds the perfect fifth, displaces the third by semitone (C major to C minor). **L** (Leittonwechsel) holds the minor third, displaces the root by semitone (C major to E minor). **R** (Relative) holds the major third, displaces the fifth by whole tone (C major to A minor). Clark (Ch. 10) traces these to Riemann's "Ideen" (1914-15), Figure 3: three pairs of triads showing that imagining any perfect fifth, major third, or minor third yields exactly one major and one minor triad sharing those tones.

The group presentation: L, P, R | L^2, P^2, R^2, (LP)^3, (PR)^4, (LR)^12. Order 24, isomorphic to D12.

# Prerequisites

- **Klang**: Understanding major and minor triads as the objects acted upon.

# Key Properties

1. **All three are involutions**: P^2 = L^2 = R^2 = identity
2. **Mode-reversing**: Each maps major to minor or vice versa
3. **Common-tone preserving**: Each holds two of three tones fixed
4. **Contextual**: Effect depends on the quality of the triad acted upon
5. **Generators**: P, L, R generate all 24 triadic transformations

# Construction / Recognition

| Compound | Effect | Cycle Length |
|----------|--------|--------------|
| LP (or PL) | Hexatonic poles | 3 (hexatonic cycle) |
| PR (or RP) | Minor-third related | 4 (octatonic cycle) |
| LR (or RL) | Dominant/subdominant | 12 (chromatic circle) |

LR from C+ yields G+ (dominant direction); RL yields F+ (subdominant direction).

# Context & Application

Clark (Ch. 10) analyzes three Schubert Lieder to demonstrate the analytical utility of PLR through Riemann's concept of Klangvertretung -- tracking how individual tones change their triadic-functional identities across chromatic third relations. The terminological note is critical: Riemann's "Parallele" = modern "Relative" (R); Riemann's "Variante" = modern "Parallel" (P); "Leittonwechsel" = same in both systems (L).

# Examples

**Riemann's Figure 3** (Clark, Ch. 10): C major / C minor (P -- perfect fifth common tones); C major / A minor (R -- major third common tones); C major / E minor (L -- minor third common tones).

**Kopp's critique** (Ch. 14): PLR compounds for fifth relations (LR for dominant) misrepresent their directness. Kopp proposes direct transformations (D, S, M, m, F) derivable from single common-tone groups.

**Tonnetz navigation**: P = vertical move; L = diagonal move; R = horizontal move. Compounds trace specific geometric paths.

# Relationships

## Builds Upon
- Triadic structure and common-tone relationships

## Enables
- Hexatonic and octatonic systems
- Tonnetz as analytical apparatus
- Chromatic transformation networks

## Related
- Harmonieschritte (Schritt/Wechsel): PLR corresponds to specific Wechsel operations
- Neo-Riemannian operations (complete system)

## Contrasts With
- Kopp's common-tone tonality: argues PLR obscures direct fifth relations

# Common Errors

- **Error**: Treating PLR operations as transpositions.
  **Correction**: PLR operations change mode, not just pitch level; they are contextual inversions.

# Common Confusions

- **Confusion**: Thinking LP and PL are "two steps" rather than unified transformations.
  **Clarification**: Compounds like LP can be conceived as single transformations with independent identity.

# Source Reference

Clark, Suzannah. "Analyzing Schubert." Ch. 10. See also Cohn, Ch. 11; Engebretsen, Ch. 12. In *The Oxford Handbook of Neo-Riemannian Music Theories*.

# Verification Notes

Re-extracted from v2 card; preserved: definitions of all three operations, group presentation, compound cycle table, Riemann's Figure 3 origin, terminological translations, Kopp's critique, Tonnetz navigation. Confidence high.
