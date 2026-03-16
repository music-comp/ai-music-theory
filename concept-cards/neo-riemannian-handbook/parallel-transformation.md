---
# === CORE IDENTIFICATION ===
concept: Parallel Transformation (P)
slug: parallel-transformation

# === CLASSIFICATION ===
category: transformations
subcategory: neo-riemannian-operations
tier: intermediate

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Henry Klumpenhouwer"
chapter: "Dualist Tonal Space and Transformation in Nineteenth-Century Musical Thought"
chapter_number: 6
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "P transformation"
  - "Quintwechsel"
  - "Seitenwechsel"
  - "Variante"
  - "parallel mode change"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang
extends:
  - plr-transformations
related:
  - relative-transformation
  - leittonwechsel
  - seitenwechsel
  - hexatonic-systems
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Parallel (P) transformation and how does it work?"
  - "How does P relate to Riemann's Seitenwechsel/Variante terminology?"
  - "Why is P called 'parallel' in English but corresponds to Riemann's 'Variante'?"
---

# Quick Definition

A neo-Riemannian transformation that exchanges a major triad for its parallel minor (or vice versa) by moving the third by one semitone while holding the root and fifth fixed -- one of the three fundamental PLR operations and the simplest mode-changing transformation.

# Core Definition

The **Parallel transformation (P)** maps C major (C-E-G) to C minor (C-Eb-G) by moving the third by semitone while the root and fifth remain stationary. In Riemann's terminology, P corresponds to *Seitenwechsel* or *Quintwechsel* -- the transformation preserving root and fifth while exchanging modal quality. Klumpenhouwer (Ch. 6) formalizes P as W0 in the Schritt/Wechsel system.

P is an involution (P(P(X)) = X), preserves two common tones, and involves minimal voice leading (one semitone).

# Prerequisites

- **Klang**: Understanding major and minor triads as the fundamental harmonic units.

# Key Properties

1. **Common tones**: Root and fifth preserved (the perfect-fifth frame)
2. **Voice leading**: Third moves by semitone (1 semitone total displacement)
3. **Mode change**: Switches between major and minor on the same root
4. **Involution**: Applying P twice returns the original triad
5. **Maximally smooth**: Alongside L, achieves minimum voice-leading distance for distinct triads

# Construction / Recognition

P is identified whenever a triad changes quality (major to minor or vice versa) while maintaining the same root. In functional analysis, Riemann called P-related chords *Variante*: the parallel minor is the Variante of the major. In the Schritt/Wechsel notation, P = W0 (Seitenwechsel with zero root motion).

# Context & Application

P appears in modal mixture passages, Picardy thirds (minor to parallel major at cadences), and structural major/minor alternation. Rehding (Ch. 7) analyzes the first subject of Brahms's Clarinet Trio Op. 114 closing on the tonic *Variante* -- moving from A minor to A major, where P marks structural points in the form (p. 224). The "characteristic dissonances" of P-related chords are inversionally related in Riemann's system: major dominant adds minor seventh; minor subdominant adds major sixth (p. 224).

# Examples

**Basic P transformation**: C major (C-E-G) --P--> C minor (C-Eb-G). Root C and fifth G stay; third E moves to Eb.

**Brahms Op. 114** (Rehding, Ch. 7): Opening phrase moves from A minor (tonic) through D minor (subdominant) to A major (tonic Variante). The final A major is the P-transform of tonic A minor (p. 224).

**Compound transformations**: LP: C major -> C minor -> Ab major (major third down). PL: C major -> E minor -> E major (major third up). PR: C major -> C minor -> Eb major.

# Relationships

## Builds Upon
- PLR transformations as a system

## Enables
- Hexatonic systems: LP/PL cycles generate hexatonic spaces
- Mode mixture analysis

## Related
- Seitenwechsel: Riemann's own term for the same operation
- Relative transformation (R): Completes the PLR trio

## Contrasts With
- R transformation: P changes mode on same root; R changes to the relative key

# Common Errors

- **Error**: Confusing "parallel" (English neo-Riemannian) with "Parallele" (German Riemannian).
  **Correction**: In German, "Parallele" means relative (C major's Parallele is A minor). P is called "parallel" in English but Variante/Quintwechsel/Seitenwechsel in German.

# Common Confusions

- **Confusion**: Thinking P and R are similar operations.
  **Clarification**: P exchanges C major with C minor (same root); R exchanges C major with A minor (relative key). These are fundamentally different.

# Source Reference

Klumpenhouwer, Henry. "Dualist Tonal Space and Transformation." In *The Oxford Handbook of Neo-Riemannian Music Theories*, Chapter 6. See also Ch. 7 (Rehding, Brahms analysis) and Ch. 8 (Tymoczko, voice-leading perspective).

# Verification Notes

Re-extracted from v2 card; preserved: formal definition, PLR comparison, Brahms Op. 114 analysis, compound transformations, terminological confusion between English "parallel" and German "Parallele." Confidence high.
