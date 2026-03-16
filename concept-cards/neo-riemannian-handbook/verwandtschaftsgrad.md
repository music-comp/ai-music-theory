---
# === CORE IDENTIFICATION ===
concept: Verwandtschaftsgrad
slug: verwandtschaftsgrad

# === CLASSIFICATION ===
category: pitch-space
subcategory: distance metrics
tier: intermediate

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
  - "degree of relatedness"
  - "grade of kinship"
  - "Verwandtschaftsgrad (degree of relationship)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - tonnetz
extends: []
related:
  - traversable-landscape
  - harmonieschritte
  - tonal-pitch-space
  - quintschritt-terzschritt
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is harmonic distance measured on the Tonnetz?"
  - "What is Verwandtschaftsgrad and how does it work?"
---

# Quick Definition

Riemann's concept of measuring harmonic distance between tones, chords, or keys by counting generator steps along paths on the Tonnetz, marking the conceptual transition from viewing the Tonnetz as a static frequency table to a navigable space with a path-based distance metric.

# Core Definition

**Verwandtschaftsgrad** (German: "degree of relationship" or "grade of kinship") quantifies the closeness of harmonic relationship by measuring the number of generator steps required to connect two points on the Tonnetz. Introduced by Riemann in the *Musik-Lexikon* (1894 edition), it formalized the idea that the Tonnetz is a space with meaningful distances. Given generators Q (fifth) and T (third), the Verwandtschaftsgrad between two tones or chords equals the minimum number of generator steps required to traverse the path connecting them. Gollin identifies this introduction as a pivotal moment in the Tonnetz's evolution: the table ceased being a static reference of frequency calculations and became a "traversable landscape" where distance, direction, and path carry meaning (Ch. 9).

# Prerequisites

- **Tonnetz**: The spatial framework within which distances are measured

# Key Properties

1. **Path-based measurement**: Distance equals the minimum number of generator steps (Q and T) along any path
2. **Generator-dependent**: The distance metric depends on which intervals are chosen as generators
3. **Shortest-path principle**: Verwandtschaftsgrad takes the shortest available path (e.g., C to E = 1 step via T, not 4 steps via Q^4)
4. **Applies at multiple levels**: Works for tone-to-tone, chord-to-chord, and key-to-key relationships
5. **Includes mode change**: Adding Seitenwechsel counts as an additional step

# Construction / Recognition

To calculate Verwandtschaftsgrad between two points:
1. Locate both points on the Tonnetz
2. Count horizontal moves (Q or Q^-1, fifths)
3. Count vertical moves (T or T^-1, thirds)
4. Sum for total Verwandtschaftsgrad
5. If comparing across mode (major to minor), add 1 for Seitenwechsel

Example: C to E = 1 (one T step). C major to A minor = 2 (Q^-1 + T, or T then Seitenwechsel, depending on path).

# Context & Application

Verwandtschaftsgrad represents an intermediate stage between pure acoustics and full transformational theory. It presupposes that the Tonnetz is a navigable space (contra Oettingen's static table) but does not yet treat the operations as formal group transformations (as Lewin and neo-Riemannians later would). The concept anticipates Lewin's "characteristic gesture" between points and Cohn's emphasis on cycles and paths.

# Examples

From Gollin's discussion (Ch. 9):
- C to G: Verwandtschaftsgrad = 1 (one Q step)
- C to E: Verwandtschaftsgrad = 1 (one T step)
- C to D: Verwandtschaftsgrad = 2 (Q^2, two fifths)
- C to A: Verwandtschaftsgrad = 2 (Q^-1 then T, or TQ^-1)

Riemann's introduction in the 1894 Musik-Lexikon accompanied the use of Drobisch's symbolic notation (Q for fifth, T for third), which had shifted thinking from ratios to step-counting.

# Relationships

## Builds Upon
- tonnetz (the space in which distances are measured)

## Enables
- traversable-landscape (Verwandtschaftsgrad operationalizes the notion of the Tonnetz as a navigable space)
- harmonieschritte (the classification of chord-to-chord relationships by distance and type)

## Related
- tonal-pitch-space (Lerdahl's modern hierarchical distance metric addresses similar questions)
- quintschritt-terzschritt (Q and T are the generators used to measure Verwandtschaftsgrad)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Assuming Verwandtschaftsgrad is the same as voice-leading distance
  **Correction**: Verwandtschaftsgrad measures path length in generator steps on the Tonnetz; voice-leading distance (DVLS/AVLS) measures total semitone displacement between chords. These are related but distinct metrics.

# Common Confusions

- **Confusion**: There is always a unique shortest path between two points
  **Clarification**: Multiple paths of the same minimal length may exist (e.g., C to A can go Q^-1 then T, or T then Q^-1); Verwandtschaftsgrad counts the minimal length regardless of which path is taken

- **Confusion**: Verwandtschaftsgrad is a fully formalized mathematical distance
  **Clarification**: Riemann introduced it as a conceptual measure of harmonic proximity; the full group-theoretic formalization came later through Klumpenhouwer and others

# Source Reference

Gollin, Edward. "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 9.

Riemann, Hugo. *Musik-Lexikon*, 1894 and later editions.

# Verification Notes

Re-extracted from v2 card; preserved: calculation examples, comparison with modern distance metrics, historical positioning between acoustics and transformation theory. High confidence: explicitly discussed by Gollin as a pivotal development in Ch. 9.
