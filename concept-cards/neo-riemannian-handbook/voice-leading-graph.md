---
concept: Voice-Leading Graph
slug: voice-leading-graph

category: transformations
subcategory: geometric-theory
tier: advanced

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Richard Cohn"
chapter: "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz"
chapter_number: 11
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "voice-leading space"
  - "parsimonious graph"
  - "chicken-wire torus"
  - "voice leadability"

prerequisites:
  - tonnetz
  - parsimonious-trichords
extends: []
related:
  - dvls-avls
  - plr-transformations
  - geometric-duals
contrasts_with: []

answers_questions:
  - "What are voice-leading graphs and how do they represent harmonic proximity?"
  - "How does the Tonnetz function as a voice-leading graph?"
  - "What is the chicken-wire torus and what does it reveal about triadic connections?"
---

# Quick Definition

Geometric representations of chords as points in multidimensional space where proximity reflects voice-leading efficiency, enabling the visualization and calculation of parsimonious connections between harmonies.

# Core Definition

**Voice-leading graphs** are spatial models where nodes represent chords, edges connect chords with efficient voice-leading relationships, and distance corresponds to total voice-leading displacement. Cohn (Ch. 11) introduces the concept of **voice leadability** -- the property of chord types that participate in smooth voice-leading networks. A chord class is highly voice-leadable when multiple other chords lie at minimal distance, these connections form regular patterns, and the chord can participate in parsimonious progressions.

Consonant triads [037] exhibit optimal voice leadability in 12-TET. The graph of all triadic P, L, R connections forms a torus with regular hexagonal tiling ("chicken-wire torus"): 24 vertices (triads), 36 edges (connections), vertex degree 3 (each triad has 3 parsimonious neighbors).

# Prerequisites

- **Tonnetz**: The underlying pitch-class lattice that voice-leading graphs relate to.
- **Parsimonious trichords**: The theory of minimal voice-leading connections.

# Key Properties

1. **Distance = voice-leading effort**: Proximity in the graph reflects efficient connections
2. **Dual structure**: Cohn shows the pitch-class Tonnetz and triadic Tonnetz are geometric duals
3. **Toroidal topology**: Under equal temperament, the graph wraps into a torus
4. **Regular connectivity**: Each triad connects to exactly 3 neighbors via P, L, R

# Construction / Recognition

The Tonnetz can be read as a voice-leading graph: each triangle (triad) is adjacent to three other triangles sharing two pitch classes. P, L, R operations connect adjacent triangles. Voice-leading distance on the Tonnetz: P = 1 semitone, L = 1 semitone, R = 2 semitones, LP = 2 semitones, PR = 3 semitones.

# Context & Application

Cohn (Ch. 11) demonstrates that the Tonnetz of pitch classes is equivalent to (geometric dual of) the Tonnetz of triads. He further argues that regions/keys can be mapped onto the same structure, conflating Lerdahl's three separate levels. The Tonnetz provides a "Babylonian" structure derivable from multiple independent assumptions (acoustic consonance, voice-leading parsimony, pc-intersection), making it robust against momentary coherence failures at any single level.

Tymoczko (Ch. 8) extends this to higher-dimensional chord spaces (orbifolds) where voice-leading appears as linear paths between points.

# Examples

**DOUTH2 relation** (Douthett/Steinbach): Two chords where two voices stay fixed and remaining voices move by semitone in parallel. P and L on triads are DOUTH2 relations. Extends to tetrachords and beyond.

**Chicken-wire torus**: The graph of 24 triads connected by P, L, R forms a regular hexagonal tiling on a torus -- connected (any triad reachable from any other), regular (all vertices equivalent), vertex degree 3.

**Chopin Prelude analysis** (Cohn, Ch. 11): The Tonnetz path reveals the central position of the tonic surrounded in all directions, with horizontal motion corresponding to diatonic step displacement and vertical motion to chromatic inflection.

# Relationships

## Builds Upon
- Tonnetz and parsimonious trichord theory

## Enables
- Systematic voice-leading analysis
- Geometric music theory (Tymoczko's orbifold approach)

## Related
- DVLS/AVLS: Quantitative measures of distances in these graphs
- Geometric duals: Pitch-class and triadic Tonnetze as dual graphs

## Contrasts With
- Transformation networks: Networks track operations; voice-leading graphs measure motion

# Common Errors

- **Error**: Assuming the Tonnetz preserves all information from full chord space.
  **Correction**: The Tonnetz is a 2D projection; some information is lost compared to Tymoczko's full n-dimensional spaces.

# Common Confusions

- **Confusion**: Conflating voice-leading distance with harmonic distance.
  **Clarification**: These are distinct measures. Dominant-to-tonic has high voice-leading distance but maximal harmonic proximity.

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." Ch. 11. See also Douthett & Steinbach, "Parsimonious Graphs" (1998); Tymoczko, Ch. 8. In *The Oxford Handbook of Neo-Riemannian Music Theories*.

# Verification Notes

Re-extracted from v2 card; preserved: voice leadability concept, Tonnetz as voice-leading graph, DOUTH2 relation, chicken-wire torus properties, Tymoczko's geometric extension. Enhanced with Cohn's geometric-dual argument and Babylonian robustness. Confidence high.
