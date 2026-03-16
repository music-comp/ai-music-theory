---
concept: Maximally Smooth Cycles
slug: maximally-smooth-cycles

category: pitch-space
subcategory: cyclic triadic structures
tier: intermediate

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Richard Cohn"
chapter: "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz"
chapter_number: 11
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "MSC"
  - "LP/PL cycles"
  - "semitonal cycles"

prerequisites:
  - plr-transformations
  - common-tone-relationships
extends: []
related:
  - hexatonic-systems
  - tonnetz
  - voice-leading-efficiency
contrasts_with: []

answers_questions:
  - "What are maximally smooth cycles?"
  - "How do maximally smooth cycles relate to hexatonic systems?"
---

# Quick Definition

Closed sequences of chords where each successive pair differs by the smallest possible voice-leading motion (typically a single semitone in a single voice), with the LP/PL cycle on triads (length 6, generating hexatonic systems) being the principal example.

# Core Definition

A **maximally smooth cycle** is a sequence of chords (C1, C2, ... Cn, C1) where each adjacent pair shares all tones but one, and that non-shared tone moves by semitone — achieving the minimum possible voice-leading displacement (DVLS = 1) throughout. For consonant triads in 12-TET, the LP (or PL) cycle is the principal maximally smooth cycle: it has length 6, returns to the starting triad, and generates the four hexatonic systems that partition the 24 consonant triads. Cohn (Ch. 11) connects these cycles to the Tonnetz, where they trace small closed loops through adjacent triangles, and to the toroidal Tonnetz, where they appear as tubes. The concept originates in Cohn's 1996 article "Maximally Smooth Cycles, Hexatonic Systems, and the Analysis of Late-Romantic Triadic Progressions."

# Prerequisites

- **PLR transformations**: L and P are the operations that produce maximally smooth steps
- **Common-tone relationships**: Maximum common-tone retention (2 of 3) is required at each step

# Key Properties

1. **Minimal displacement**: Each step moves exactly one voice by exactly one semitone (DVLS = 1)
2. **Maximum common tones**: k-1 tones retained at each step (for k-note chords)
3. **Cyclic closure**: The sequence returns to the starting chord
4. **LP generation**: For triads, LP (or PL) is the only maximally smooth cycle generator
5. **Length 6**: The LP cycle has order 6, producing hexatonic systems
6. **Partition property**: Four disjoint LP cycles partition all 24 consonant triads

# Construction / Recognition

To construct a maximally smooth cycle from a triad:
1. Start from any consonant triad (e.g., C major: C-E-G)
2. Apply L: C major to E minor (C moves to B; E-G retained)
3. Apply P: E minor to E major (G moves to G#; E-B retained)
4. Continue LP alternation: E+ to Ab- to Ab+ to C- to C+ (return)
5. Total: 6 triads, each connected by single-semitone motion

On the Tonnetz, this cycle traces a compact closed path through 6 adjacent triangles.

# Context & Application

Maximally smooth cycles provide voice-leading logic for chromatic progressions that resist functional explanation. They are analytically central for:
- Wagner (Tristan, Parsifal)
- Liszt's late piano works
- Schubert's chromatic third-related passages
- Any passage where stepwise voice-leading appears to generate harmonic motion

Cohn's Parsifal analysis (Ch. 11) demonstrates how LP cycles create local coherence within hexatonic systems, while transitions between systems require non-smooth operations (R).

# Examples

Example LP cycle (Ch. 11): C+ -> C- -> Ab+ -> Ab- -> E+ -> E- -> C+ (each step: DVLS = 1, one voice moves by semitone). This traces through the "Northern" hexatonic system.

PR cycles (length 8, generating octatonic systems) are smooth but not maximally smooth, because R involves a whole-step displacement (DVLS = 2).

# Relationships

## Builds Upon
- plr-transformations (L and P produce the cycle)
- common-tone-relationships (maximum retention at each step)

## Enables
- hexatonic-systems (MSCs generate the four hexatonic partitions)
- Voice-leading analysis of chromatic music

## Related
- tonnetz (MSCs trace minimal paths on the Tonnetz)
- voice-leading-efficiency (MSCs are the most efficient triadic cycles)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Calling PR cycles "maximally smooth"
  **Correction**: R involves whole-step motion (DVLS = 2), so PR cycles are smooth but not maximally smooth; only LP cycles achieve DVLS = 1 at every step for triads

# Common Confusions

- **Confusion**: Maximally smooth cycles are the only analytically important cycles
  **Clarification**: PR cycles (octatonic), LR cycles (chromatic), and other compound cycles are also analytically important; "maximally smooth" is one specific property

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 11.

Cohn, Richard. "Maximally Smooth Cycles, Hexatonic Systems, and the Analysis of Late-Romantic Triadic Progressions." *Music Analysis* 15.1 (1996): 9-40.

# Verification Notes

Re-extracted from v2 card; preserved: LP cycle example, DVLS = 1 property, length 6, partition into 4 systems, PR distinction, Parsifal reference. High confidence: central to Cohn's analytical framework in Ch. 11 and his earlier published work.
