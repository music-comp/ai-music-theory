---
# === CORE IDENTIFICATION ===
concept: Tonal Pitch Space
slug: tonal-pitch-space

# === CLASSIFICATION ===
category: pitch-space
subcategory: hierarchical tonal models
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Richard Cohn"
chapter: "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz"
chapter_number: 11
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "TPS"
  - "Lerdahl's Tonal Pitch Space"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - tonnetz
  - regional-space
contrasts_with:
  - tonnetz

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is Tonal Pitch Space and how does it differ from the Tonnetz?"
  - "How does the Tonnetz connect pitch space to key space?"
---

# Quick Definition

Fred Lerdahl's hierarchical model of tonal organization comprising three distinct levels (pitch-class space, chord space, and regional space), each with its own distance metric, which Cohn engages as a critical counterpoint to the Tonnetz's unified spatial representation.

# Core Definition

**Tonal Pitch Space** (TPS), developed by Fred Lerdahl in his 2001 book of the same name, proposes three hierarchically distinct spatial levels for modeling tonal relationships: (1) **pitch-class space** (chromatic proximity), (2) **chordal space** (triadic relationships measured by voice-leading and common-tone counts), and (3) **regional space** (key relationships measured by shared pitch content). Each level has its own structure and distance metric. Cohn (Ch. 11) engages Lerdahl's model as a critical foil for the Tonnetz, arguing against Lerdahl's claim that "the three levels of tonal space do not correspond to one another" and that their "homology" in the Riemannian Tonnetz "is misleading." Cohn's counter-argument is that a single Tonnetz naturally represents all three levels through what he calls **mutual implication**: pitch classes are nodes, chords are triangles grouping nodes, and regions are parallelograms grouping triangles.

# Prerequisites

Foundational concept in this context (Lerdahl's system provides the critical interlocutor for Cohn's argument).

# Key Properties

1. **Three separate levels**: Pitch-class, chord, and regional spaces, each with distinct structures
2. **Hierarchical embedding**: Stability measured by level of embedding in the hierarchy
3. **Distance formula**: Chord distance delta = i + j + k (non-common tones + fifth-distance + mode change)
4. **Regional determination required**: Every event must be assigned to a key for the model to work
5. **Cognitive grounding**: Designed to model tonal cognition and perception, not transformation

# Construction / Recognition

Lerdahl's **basic space** is a hierarchical reduction showing:
- Current pitch-class collection (chromatic level)
- Current chord as a subset (chordal level)
- Current key as reference point (regional level)

Distance between chords combines: circle-of-fifths distance between roots (j), mode change penalty (k), and non-common-tone count (i).

# Context & Application

Cohn presents TPS as the principal challenge to the Tonnetz's unified representation. The debate matters because it concerns whether a single spatial model can represent pitch, chord, and key relations, or whether these require separate formalisms. Cohn argues for the Tonnetz's greater parsimony and its ability to handle regionally indeterminate passages (where TPS requires a key assignment that may be analytically premature). However, Cohn also proposes a "pragmatic solution" and "hybrid spatial model" that draws on both traditions for analytical applications.

# Examples

Cohn illustrates the debate with analyses of Schumann, Wagner, and Chopin (Ch. 11). For regionally determinate passages, both TPS and the Tonnetz can produce valid analyses. But for passages like the Faith Proclamation in Wagner's Parsifal, where triadic progressions resist key assignment, Cohn argues the Tonnetz provides coherent analysis while TPS demands premature regional determination.

Cohn's comparison: In TPS, the fifth relation (I to V) is "close" by the distance formula; in the Tonnetz, it requires a compound transformation (LR). Conversely, the LP relation (maximally smooth voice leading) is "close" on the Tonnetz but receives no special status in TPS.

# Relationships

## Builds Upon
(external model introduced as interlocutor)

## Enables
- Hybrid analytical approaches combining TPS and Tonnetz insights

## Related
- tonnetz (the model Cohn advocates as an alternative to TPS)
- regional-space (the level of TPS that Cohn shows can be derived from the Tonnetz)

## Contrasts With
- tonnetz (Lerdahl argues they are fundamentally different; Cohn argues the Tonnetz subsumes TPS)

# Common Errors

- **Error**: Assuming TPS and the Tonnetz are incompatible
  **Correction**: Cohn proposes a hybrid model; TPS and Tonnetz can be complementary, each illuminating different aspects of the music

# Common Confusions

- **Confusion**: TPS is "wrong" because Cohn critiques it
  **Clarification**: Cohn's critique is that the Tonnetz does the same work more parsimoniously, not that TPS is incorrect; TPS is especially valuable for hierarchically structured tonal music

- **Confusion**: The Tonnetz replaces TPS entirely
  **Clarification**: TPS handles regionally determinate music well; the Tonnetz excels at regionally indeterminate chromatic music; a hybrid approach is often best

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 11.

Lerdahl, Fred. *Tonal Pitch Space*. Oxford University Press, 2001.

# Verification Notes

Re-extracted from v2 card; preserved: three-level structure, distance formula, Cohn's conflation argument, comparison table content (restructured), hybrid approach discussion. High confidence: TPS is explicitly discussed and critiqued by Cohn as the central interlocutor in Ch. 11.
