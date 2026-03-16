---
# === CORE IDENTIFICATION ===
concept: Seitenwechsel (Mode Exchange)
slug: seitenwechsel

# === CLASSIFICATION ===
category: transformations
subcategory: neo-riemannian-operations
tier: intermediate

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Nora Engebretsen"
chapter: "The 'Over-Determined' Triad as a Source of Discord"
chapter_number: 12
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "side exchange"
  - "lateral change"
  - "W0"
  - "mode change"
  - "Variante relationship"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang
  - harmonic-dualism
extends: []
related:
  - parallel-transformation
  - neo-riemannian-operations
  - plr-transformations
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is Seitenwechsel and how does it relate to the P transformation?"
  - "Why is Seitenwechsel the most fundamental Wechsel operation?"
  - "How are all other Wechsel derived from Seitenwechsel?"
---

# Quick Definition

The most fundamental Wechsel operation in Riemann's system, exchanging a triad's mode (major to minor or vice versa) while keeping the root stationary, corresponding to the Parallel (P) transformation in neo-Riemannian theory.

# Core Definition

**Seitenwechsel** (German: "side exchange") maps a Klang to its modal opposite while preserving the root and fifth: C major (c+) to C minor (-c). In the Schritt/Wechsel notation, Seitenwechsel = W0 -- the only Wechsel with zero root motion. Engebretsen (Ch. 12) shows it is one of three generators for the complete group, with all other Wechsel derivable from combining Seitenwechsel with Schritte: Quintwechsel = Q + W0; Terzwechsel = T + W0; Leittonwechsel = QT + W0.

Seitenwechsel is exactly the P (Parallel) transformation: both change mode without changing root, preserve root and fifth as common tones, and move the third by semitone.

# Prerequisites

- **Klang**: The major/minor triad that Seitenwechsel operates upon.
- **Harmonic dualism**: The dualist conception of major and minor as opposite modes.

# Key Properties

1. **Involution**: W0 applied twice returns to original (self-inverse)
2. **Zero root motion**: The only Wechsel preserving the root
3. **Generator**: Combined with Schritte, generates all other Wechsel
4. **Minimal voice leading**: One voice (third) moves by semitone
5. **Identity with P**: Seitenwechsel = P transformation

# Construction / Recognition

Combined with Schritte:
| Combination | Result |
|-------------|--------|
| W0 + W0 | Identity |
| Q + W0 | Quintwechsel |
| W0 + Q | Gegenquintwechsel |
| T + W0 | Terzwechsel (= R) |
| W0 + T | Gegenterzwechsel |

# Context & Application

Within Riemann's function theory, the Seitenwechselklang shares function with its modal partner: C minor in C major = variant tonic; F minor in C major = subdominant's Seitenwechselklang. Major keys include minor subdominant (Seitenwechsel of IV); minor keys include major dominant (Seitenwechsel of V). The repeated Seitenwechsel creates major/minor oscillation (period 2).

# Examples

**Basic mode exchange**: C major (C-E-G) --W0--> C minor (C-Eb-G). Only E->Eb moves.

**Mode mixture**: Passages alternating major and minor versions (C+ -> c- -> C+ -> c-...) each change = one Seitenwechsel.

**Minor subdominant in major**: iv in major keys (F minor in C major) is reached via Seitenwechsel from IV -- common in 19th-century practice.

# Relationships

## Builds Upon
- Klang and harmonic dualism

## Enables
- Complete Schritt/Wechsel system (as generator)
- Mode mixture analysis

## Related
- Parallel transformation (P): Identical operation, different terminology
- PLR transformations: P/Seitenwechsel is one of the three generators

## Contrasts With
- Riemann's Parallele: Seitenwechsel (= P) is NOT the same as Riemann's "Parallele" (= R)

# Common Errors

- **Error**: Confusing Seitenwechsel with Riemann's "Parallele."
  **Correction**: Riemann's Parallele = modern R (relative); Seitenwechsel = modern P (parallel).

# Common Confusions

- **Confusion**: Thinking Seitenwechsel changes the root.
  **Clarification**: The root stays the same; only the mode (major/minor) changes.

# Source Reference

Engebretsen, Nora. "The 'Over-Determined' Triad as a Source of Discord." Ch. 12. In *The Oxford Handbook of Neo-Riemannian Music Theories*.

# Verification Notes

Re-extracted from v2 card; preserved: definition, identity with P, generator properties, derivation of other Wechsel, functional implications, terminological caution. Confidence high.
