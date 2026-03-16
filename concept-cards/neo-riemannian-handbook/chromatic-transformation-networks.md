---
# === CORE IDENTIFICATION ===
concept: Chromatic Transformation Networks
slug: chromatic-transformation-networks

# === CLASSIFICATION ===
category: analysis
subcategory: transformational-analysis
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
  - "transformation networks"
  - "triadic transformation networks"
  - "neo-Riemannian networks"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - plr-transformations
  - neo-riemannian-operations
  - tonnetz
extends: []
related:
  - hexatonic-systems
  - voice-leading-graph
  - combinatorial-group-theory
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are chromatic transformation networks and how are they used analytically?"
  - "How do transformation networks relate to the Tonnetz?"
  - "What is the difference between sequential, spatial, and motivic networks?"
---

# Quick Definition

Graphical representations of harmonic progressions showing the transformational operations connecting successive chords, used to analyze chromatic music independent of functional tonal syntax.

# Core Definition

**Chromatic transformation networks** are analytical diagrams where nodes represent chords (typically consonant triads), arrows/edges represent transformations (PLR, Schritte/Wechsel), and labels identify specific operations. Cohn (Ch. 11) systematizes these within his broader project of relating the Tonnetz to Lerdahl's Tonal Pitch Space, showing how the Tonnetz itself functions as a meta-network containing all possible triads as nodes and all PLR connections as edges.

The approach derives from Lewin's "transformational attitude" (GMIT, 1987): the shift from "What is this chord?" to "How does this chord relate to that chord?" Networks can be sequential (linear chains), spatial (two-dimensional Tonnetz-based), or motivic (highlighting recurring transformational patterns).

# Prerequisites

- **PLR transformations**: The operations that label network edges.
- **Neo-Riemannian operations**: The complete Schritt/Wechsel system.
- **Tonnetz**: The underlying spatial model.

# Key Properties

1. **Abstraction from pitch**: Networks capture structural similarity regardless of specific pitches
2. **Closed loops**: Cyclic progressions form closed paths on the network
3. **Multiple representations**: The same progression admits Tonnetz path, linear network, or set-class representations
4. **Non-unique**: A progression may have multiple valid network representations

# Construction / Recognition

Network components: **Nodes** represent major/minor triads (24 total in 12-TET). **Arrows** represent operations (PLR or Schritte/Wechsel). All PLR operations are self-inverse (reversible). Specific progressions trace paths on the Tonnetz surface -- bounded regions for diatonic passages, extended trajectories for chromatic ones.

# Context & Application

Cohn (Ch. 11) analyzes Schumann's "Im wunderschonen Monat Mai," the Faith Proclamation from Wagner's Parsifal, and Chopin's E-major Prelude op. 28 no. 9 using both Tonnetz and TPS combined-space representations. He shows that the Tonnetz provides "Babylonian" robustness -- derivable from several independent assumptions, overcoming momentary coherence failures at any single level (p. 325).

Engebretsen (Ch. 12) observes that networks provide coherence without tonic through structural relationships, embodying what Riemann called his "horror vision" of unconstrained harmonic possibility -- but order emerges through transformational logic.

# Examples

**Hexatonic cycle**: C+ --L--> e- --P--> E+ --L--> g#- --P--> G#+ --L--> c-. Complete cycle via LP compounds.

**Wagner Parsifal** (Cohn, Ch. 11): Faith Proclamation shows LP cycling through hexatonic space, departure through octatonic connection (via PR), and return path visible in network structure.

**Chopin Prelude analysis** (Cohn, Ch. 11): Tonnetz path reveals palindromic column progression, central position of tonic surrounded in all directions, and functional directionality (rightward = plagal, leftward = authentic).

# Relationships

## Builds Upon
- PLR transformations and the Tonnetz

## Enables
- Formal analysis of chromatic progressions resisting functional explanation
- Detection of motivic harmonic patterns and symmetrical structures

## Related
- Hexatonic systems: Specific cyclic network structures
- Voice-leading graphs: Complementary perspective (geometry vs. algebra)

## Contrasts With
- Functional analysis: Networks track operations, not functions; arrows are not voice-leadings

# Common Errors

- **Error**: Reading network arrows as representing voice leading.
  **Correction**: Arrows represent abstract operations, not specific pitch motion between voices.

# Common Confusions

- **Confusion**: Thinking network representations are unique for a given passage.
  **Clarification**: The same progression admits multiple valid network representations.

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." Ch. 11. Engebretsen, Nora. Ch. 12. In *The Oxford Handbook of Neo-Riemannian Music Theories*.

# Verification Notes

Re-extracted from v2 card; preserved: network types, Lewin's transformational attitude, Tonnetz as meta-network, Wagner and Chopin examples, Engebretsen's "horror vision" observation. Enhanced with Cohn's Babylonian robustness concept. Confidence high.
