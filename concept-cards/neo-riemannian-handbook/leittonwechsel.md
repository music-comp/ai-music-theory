---
# === CORE IDENTIFICATION ===
concept: Leittonwechsel (Leading-Tone Exchange, L)
slug: leittonwechsel

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
  - "L transformation"
  - "leading-tone exchange"
  - "Leittonwechselklang"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang
  - leading-tone
extends:
  - plr-transformations
related:
  - parallel-transformation
  - relative-transformation
  - hexatonic-systems
  - chromatic-mediant
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Leittonwechsel (L) transformation and how does it work?"
  - "How does L relate to chromatic third relations in Romantic harmony?"
  - "What are the compound transformations LP and PL?"
---

# Quick Definition

A neo-Riemannian transformation connecting a major triad to a minor triad whose root is a major third higher (or vice versa), achieved by moving one note by semitone while holding two notes fixed -- one of the three fundamental PLR operations.

# Core Definition

The **Leittonwechsel (L)** is one of the three fundamental neo-Riemannian operations (alongside P and R). It maps C major (C-E-G) to E minor (E-G-B) by moving the root C up to B by semitone, preserving the third and fifth (E and G) as common tones. In Riemann's terminology, *Leittonwechsel* means "leading-tone exchange" -- the root steps to become the leading tone of the new chord. Klumpenhouwer (Ch. 6) situates L within Riemann's broader system of Schritte and Wechsel, where it corresponds to the *Leittonwechsel* (W3) operation.

L is an involution (applying twice returns the original), preserves two common tones, and involves semitonal motion -- making it "maximally smooth" alongside P.

# Prerequisites

- **Klang**: Understanding major and minor triads as fundamental units.
- **Leading tone**: The semitone relationship that gives L its name.

# Key Properties

1. **Common tones**: Third and fifth are preserved (the minor-third dyad)
2. **Voice leading**: Root moves by semitone (1 semitone total displacement)
3. **Root relation**: Connects triads whose roots are a major third apart
4. **Involution**: L(L(X)) = X
5. **Inherently chromatic**: Unlike R, L creates chromatic third-relations in most contexts

# Construction / Recognition

| Operation | Common Tones | Moving Voice | Distance |
|-----------|-------------|--------------|----------|
| P | 2 (root, fifth) | Third | 1 semitone |
| L | 2 (third, fifth) | Root | 1 semitone |
| R | 2 (root, third) | Fifth | 2 semitones |

Compound transformations: LP (C major -> C minor -> Ab major) produces major-third descent between same-quality triads. PL (C major -> E minor -> E major) produces major-third ascent.

# Context & Application

L is essential for understanding hexatonic cycles (PLPLPL or LPLPLP), chromatic third relations characteristic of Romantic harmony, "maximally smooth" voice leading, and the harmonic language of Wagner and Liszt. Riemann labeled L-related chords as Leittonwechselklange (e.g., Tl = Tonic Leittonwechsel).

# Examples

**Basic L transformation**: C major (C-E-G) --L--> E minor (E-G-B). Two common tones (E, G), one semitone motion (C->B).

**In Brahms's Clarinet Trio** (Rehding, Ch. 7): The relationship between C major (second subject) and E minor (closing theme) is L. The second subject functions both as Tp to the tonic and as Dl (Dominantleittonwechsel) to the closing theme (p. 228).

**Compound transformations**: LP produces C major -> C minor -> Ab major (major third down, same quality). PL produces C major -> E minor -> E major (major third up, same quality). LPL produces tritone relations.

# Relationships

## Builds Upon
- PLR transformations as a system

## Enables
- Hexatonic systems: LP/PL cycles generate hexatonic spaces
- Chromatic third relations: L enables major-third-related triadic connections

## Related
- Parallel transformation (P): Fellow "maximally smooth" operation
- Relative transformation (R): Completes the PLR trio

## Contrasts With
- R transformation: L is chromatic where R is diatonic; L involves semitone where R involves whole tone

# Common Errors

- **Error**: Confusing L with R (L connects C major to E minor, not A minor).
  **Correction**: L involves major-third root motion; R involves minor-third root motion.

# Common Confusions

- **Confusion**: Thinking LP and PL are the same.
  **Clarification**: LP moves a major third DOWN between same-quality triads; PL moves a major third UP.

# Source Reference

Klumpenhouwer, Henry. "Dualist Tonal Space and Transformation in Nineteenth-Century Musical Thought." In *The Oxford Handbook of Neo-Riemannian Music Theories*, Chapter 6. See also Ch. 7 (Rehding) and Ch. 8 (Tymoczko).

# Verification Notes

Re-extracted from v2 card; preserved: formal definition, PLR comparison table, compound transformations, Brahms Clarinet Trio example. REMOVED LLM artifact "Wait - let me recalculate" and associated incorrect Wagner/Tarnhelm working. Confidence high due to well-established formal properties.
