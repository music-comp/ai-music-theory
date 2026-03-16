---
# === CORE IDENTIFICATION ===
concept: Tonnetz
slug: tonnetz

# === CLASSIFICATION ===
category: pitch-space
subcategory: spatial representations of pitch
tier: foundational

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Edward Gollin"
chapter: "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann"
chapter_number: 9
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Tone Network"
  - "Table of Tonal Relations"
  - "Tonnetze (plural)"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - toroidal-tonnetz
  - enharmonic-equivalence
  - syntonic-comma-striche
  - verwandtschaftsgrad
  - traversable-landscape
  - oettingens-acoustical-matrix
  - regional-space
  - tonal-pitch-space
  - geometric-duals
contrasts_with:
  - tonal-pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Tonnetz and how is it structured?"
  - "How do I construct a Tonnetz representation?"
  - "How does the Tonnetz connect pitch space to key space?"
  - "What concepts are needed to understand the Tonnetz?"
---

# Quick Definition

A two-dimensional lattice representing pitch-class relationships organized by perfect fifths along one axis and major thirds along another, used to visualize triadic relationships, voice-leading connections, and key relations.

# Core Definition

The **Tonnetz** (German: "tone network") is a spatial representation of pitch classes arranged according to consonant interval relationships. In its standard IC4 x IC5 form, the horizontal axis represents perfect fifths and the vertical axis represents major thirds, with minor thirds emerging along the diagonal. Triads appear as triangular regions: major triads as triangles pointing one direction, minor triads pointing the other. Gollin traces its evolution from Oettingen's "literal-acoustical" multiplication table for calculating frequencies in just intonation (1866) to Riemann's "spatial-metaphorical" traversable landscape of tonal relations (Ch. 9). Martin Vogel observed the Tonnetz was "discovered twice" (Euler 1773, Oettingen 1866), and Cohn catalogs further rediscoveries organized by generating interval classes (Ch. 11).

# Prerequisites

Foundational concept with minimal prerequisites:
- **Interval**: The Tonnetz is organized by consonant intervals (fifths and thirds)
- **Triad**: Triads appear as triangular regions on the Tonnetz

# Key Properties

1. **Two generating axes**: Perfect fifths (horizontal) and major thirds (vertical), with minor thirds as diagonals
2. **Triadic representation**: Major and minor triads appear as adjacent triangles sharing an edge
3. **Two geometric interpretations**: Nonconforming (infinite plane, just intonation) and conforming (torus, equal temperament)
4. **Multi-level representation**: Simultaneously represents pitch-class space, chord space, and key space (regional encapsulation)
5. **Multiple rediscoveries**: IC4 x IC5 (Euler, Oettingen, Riemann), IC3 x IC4 (Weitzmann, Balzano), IC3 x IC5 (Weber, Schoenberg, Lerdahl)

# Construction / Recognition

To construct a standard Tonnetz:
1. Place pitch classes along the horizontal axis in fifths: ...F-C-G-D-A-E-B...
2. Place pitch classes along the vertical axis in major thirds: ...C-E-G#...
3. Minor thirds emerge as the diagonal connecting fifths and thirds axes
4. Draw triangles connecting adjacent pitch classes to form triads
5. Major triads point one direction (e.g., upward); minor triads point the opposite
6. PLR transformations correspond to moving between adjacent triangles sharing an edge

# Context & Application

The Tonnetz serves as the primary spatial model in neo-Riemannian theory for analyzing triadic relationships. It is used for:
- Visualizing voice-leading parsimony (adjacent triangles share 2 common tones)
- Tracking chromatic progressions independent of key
- Mapping hexatonic and octatonic cycles as specific pathways
- Representing diatonic regions as parallelograms (regional encapsulation)
- Modeling modulation as movement through tonal space

# Examples

Gollin traces five historical stages of the Tonnetz (Ch. 9):

1. **Oettingen's table (1866)**: A literal multiplication table where the expression 5^m x 3^n generates tone frequencies; entries are calculation results with Striche marking comma differences.

2. **Riemann's early adoption (1873)**: Used in his dissertation "Ueber das musikalische Hören" to demonstrate intonational differences, listing 133 distinct intervals within an octave.

3. **Drobisch notation adoption (1880s)**: Riemann adopted Q (fifth) and T (third) as symbolic path labels, shifting from ratio-based to step-based thinking.

4. **Verwandtschaftsgrad introduction (1894)**: In Musik-Lexikon, Riemann introduced path-distance measurement, transforming the table into a navigable space.

5. **Psychological foundation (1914-15)**: In "Ideen zu einer 'Lehre von den Tonvorstellungen,'" Riemann fully embraced the Tonnetz as mediating between "the phenomenal world of musical practice and the unbounded noumenal realm of musical meanings."

Cohn catalogs further rediscoveries (Ch. 11): Euler (1773), Vial, Weber, Schoenberg, Lerdahl (IC3 x IC5 for key relationships), Weitzmann (1853), Balzano (1980) (IC3 x IC4), and Hostinsky (1879), Krumhansl, Hyer, Cohn (IC3 x IC4 x IC5).

# Relationships

## Builds Upon
- Interval relationships (fifths and thirds as generating axes)

## Enables
- plr-transformations (movements between adjacent triangles)
- hexatonic-systems (LP cycles as pathways on the Tonnetz)
- octatonic-systems (PR cycles as pathways)
- regional-space (diatonic regions as parallelograms)
- voice-leading-graph (dual graph representation)

## Related
- toroidal-tonnetz (conforming geometry under equal temperament)
- geometric-duals (pitch-class and triadic Tonnetze as duals)
- tonal-pitch-space (Lerdahl's alternative hierarchical model)
- oettingens-acoustical-matrix (historical origin)

## Contrasts With
- tonal-pitch-space (Lerdahl separates pitch, chord, and key levels; Cohn argues the Tonnetz unifies them)

# Common Errors

- **Error**: Treating the Tonnetz as exclusively an equal-tempered (conforming) structure
  **Correction**: The Tonnetz exists in two forms: nonconforming (infinite plane, just intonation) and conforming (torus, equal temperament), each valid for different analytical purposes

- **Error**: Assuming the Tonnetz was always understood as a spatial network
  **Correction**: Oettingen's original was a static multiplication table for calculating frequencies; the spatial-navigational interpretation developed over decades through Riemann's work

# Common Confusions

- **Confusion**: The Tonnetz is a neo-Riemannian invention
  **Clarification**: The Tonnetz predates neo-Riemannian theory by over a century (Euler 1773, Oettingen 1866); neo-Riemannians revived and reinterpreted it under equal temperament

- **Confusion**: There is only one Tonnetz
  **Clarification**: Different generating interval classes produce different Tonnetze; IC4 x IC5 is the "standard" form, but IC3 x IC5 (Weber, Lerdahl) and IC3 x IC4 (Weitzmann, Balzano) serve different analytical purposes

- **Confusion**: Pitch-class, triadic, and key representations require separate spaces
  **Clarification**: Cohn argues (Ch. 11) that these are mutually implied levels of a single Tonnetz: nodes are pitch classes, triangles are triads, parallelograms are diatonic regions

# Source Reference

Gollin, Edward. "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 9.

Additional treatment in Cohn, Richard. Chapter 11 (multiple rediscoveries and analytical applications) and Clark, Suzannah. Chapter 10 (Tonnetz in context of Klangvertretung).

# Verification Notes

Re-extracted from v2 card; preserved: historical development stages, multiple rediscoveries table content, geometric duals discussion. High confidence: the Tonnetz is the central concept of Part 3 and is explicitly defined and extensively discussed across Chapters 9, 10, and 11.
