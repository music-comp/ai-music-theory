---
# === CORE IDENTIFICATION ===
concept: Tonnetz Key Template
slug: tonnetz-key-template

# === CLASSIFICATION ===
category: analysis
subcategory: chromatic-tonality
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "David Kopp"
chapter: "Key and Function in Chromatic/Relational Harmonic Systems"
chapter_number: 14
pdf_page: 413
section: "III. Postscript"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "key-flip on the Tonnetz"
  - "diatonic template on Tonnetz"
  - "Kopp's key template"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - tonnetz
  - chromatic-mediant
extends: []
related:
  - key-template-tonnetz-flips
  - common-tone-tonality
  - tonalitaet-vs-tonart
contrasts_with:
  - hexatonic-systems

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a Tonnetz key template and how does it generate chromatic mediants?"
  - "How do key-flips on the Tonnetz produce the four chromatic mediant keys?"
  - "Why does Kopp argue chromatic mediants are tonal rather than hexatonic?"
---

# Quick Definition

A geometric template representing the diatonic set of a key projected onto the Tonnetz, which can be flipped along its edges to generate the keys of the four chromatic mediants, demonstrating their intrinsic relationship to the tonic key.

# Core Definition

The **Tonnetz key template** is an analytical tool derived from Riemann's classic Tonnetz (from "Ideen," 1914-15). It represents the seven-note diatonic collection as a connected parallelogram on the pitch-class lattice. Kopp (Ch. 14, p. 413) shows that 180-degree flips of the template along its long edges produce exactly the keys of the four chromatic mediants:

| Flip Direction | D Position | Result |
|---------------|------------|--------|
| Upward flip | D at lower right | Upper Sharp Mediant |
| Downward flip | D at lower right | Upper Flat Mediant |
| Upward flip | D at upper left | Lower Sharp Mediant |
| Downward flip | D at upper left | Lower Flat Mediant |

D appears at symmetrically opposite positions as the axis of diatonic symmetry. The Tonnetz tends upward-to-the-right and sharpward-toward-the-top.

# Prerequisites

- **Tonnetz**: The pitch-class lattice on which templates are projected.
- **Chromatic mediant**: The four chromatic-third relations that templates generate.

# Key Properties

1. **Seven pitch classes**: Contains all diatonic scale members
2. **D as symmetry axis**: D appears at opposite corners reflecting diatonic set symmetry
3. **Common-tone distribution**: Flipped templates share 3-4 or 4-3 pitch classes with original
4. **Direct relation**: Chromatic mediants arise from key structure, not compound voice-leading operations

# Construction / Recognition

Project the diatonic collection onto the Tonnetz as a parallelogram. The template has two "long edges" serving as flip axes. Flip 180 degrees along either edge. The resulting collection is the key of one of the four chromatic mediants. The D at opposite corners reflects the axis of diatonic symmetry (not a special property of the pitch D itself).

# Context & Application

Kopp argues that this intrinsic property of the Tonnetz validates chromatic mediants as essential tonal relationships, not as evidence of atonality or hexatonicism. "This is in contrast to the neo-Riemannian hexatonic model, which generates chromatic mediants as the compound products of two different successive diatonic voice-leading operations at the chordal level" (p. 414). Kopp's broader argument (Ch. 14) is that neo-Riemannian theory's abstraction of Harmonieschritte has resulted in "a view of harmonic relations uncomfortably divorced from the tonal and functional contexts in which they were conceived."

# Examples

**C major to E major (USM)** (Kopp, Ch. 14, p. 413): Original C major: C-D-E-F-G-A-B. Flipped to E major: E-F#-G#-A-B-C#-D#. Common tones: E, A, B (three shared). The three-to-four common-tone ratio explains the characteristic "distance-yet-connectedness."

**C major to Ab major (LFM)**: Original: C-D-E-F-G-A-B. Flipped: Ab-Bb-C-Db-Eb-F-G. Common tones: C, F, G (three shared).

**Short-edge flips**: Produce different nondiatonic cyclic sets -- incomplete octatonic in one direction, hexatonic with redundant member in the other (p. 414, n. 21).

# Relationships

## Builds Upon
- Tonnetz structure and diatonic set theory

## Enables
- Understanding chromatic mediants as tonal (not extra-tonal) relationships
- Key-level analysis on the Tonnetz

## Related
- Common-tone tonality (Kopp): Broader framework for chromatic-mediant function

## Contrasts With
- Hexatonic systems: Neo-Riemannian model generates mediants via compound PLR operations; Kopp's template shows them arising directly from key structure

# Common Errors

- **Error**: Confusing key templates (7-note collections) with PLR chord transformations (3-note).
  **Correction**: Key-flip operates on keys, not individual chords, though both use the Tonnetz.

# Common Confusions

- **Confusion**: Thinking the D at corners reflects a special property of the pitch D.
  **Clarification**: It reflects diatonic set symmetry -- D is the axis of symmetry whether the diatonic set is arranged in scalar steps or fifths.

# Source Reference

Kopp, David. "Key and Function in Chromatic/Relational Harmonic Systems." In *The Oxford Handbook of Neo-Riemannian Music Theories*, Chapter 14, pp. 413-414.

# Verification Notes

Re-extracted from v2 card; preserved: template properties, flip-direction table, common-tone distribution, contrast with hexatonic model, short-edge flip note. Enhanced with Kopp's broader argument about tonal context and precise page citations. Confidence high.
