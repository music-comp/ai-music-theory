---
concept: X/Y Transformation Group
slug: xy-transformation-group

category: transformations
subcategory: neo-Riemannian-operations
tier: advanced

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Robert C. Cook"
chapter: "Transformational Approaches to Romantic Harmony and the Late Works of Cesar Franck"
chapter_number: 19
pdf_page: 529
section: null

extraction_confidence: high

aliases:
  - X/Y group
  - mode-exchange/mode-preservation group

prerequisites:
  - plr-transformations
  - voice-leading
  - dvls-avls
extends:
  - contextual-harmony-analysis
related:
  - major-third-cycle
  - chromatic-transformation-networks
contrasts_with: []

answers_questions:
  - "How do objects and transformations interact in neo-Riemannian theory?"
  - "How do I analyze chromatic progressions using neo-Riemannian tools?"
---

# Quick Definition

An algebraic group of transformations devised by Cook for analyzing Franck's chromatic music, dividing operations into X (mode-exchanging) and Y (mode-preserving) categories, distinguished by directed voice-leading sums.

# Core Definition

The X/Y group distinguishes between transformations that exchange mode (X) and those that preserve mode (Y), using directed voice-leading sums to differentiate specific operations within each category. "X will stand for mode-switching transformations: X as in 'eXchanging major for minor or vice versa.' Y will then stand for mode-preserving transformations" (Cook, p. 528). For example, X1 exchanges a triad for one of opposite mode with directed voice-leading sum 1 (e.g., G minor to Eb major); X10 does the same with sum 10 (e.g., G minor to F# major). Mode-preserving transformations include Y3, Y6, and Y9 (pp. 528-529). The group is "equivalent to one devised by Lewin to study operations among pitch classes in octatonic collections" (p. 529).

# Prerequisites

- **PLR transformations** — X and Y operations subsume and refine PLR operations
- **Voice-leading** — Directed voice-leading sums provide the metric for distinguishing operations
- **DVLS/AVLS** — Voice-leading sum classes are the technical basis for the group

# Key Properties

1. X transformations switch mode (major to minor or vice versa)
2. Y transformations preserve mode (major to major or minor to minor)
3. Each transformation is specified by a directed voice-leading sum (mod 12)
4. X1 is its own inverse; the inverse of X1 is X1 (also X10 is its own inverse)
5. Y3 = (X1, X10) and Y9 = (X10, X1) — mode-preserving operations are compounds of mode-exchanging ones
6. The group includes: X1, X4, X7, X10, Y0, Y3, Y6, Y9

# Construction / Recognition

## To Apply the X/Y Group

1. Identify triads in the passage and organize them into major-third pairs (nodes)
2. Determine the directed voice-leading sum between successive triads
3. Classify each transformation as X (mode-exchanging) or Y (mode-preserving)
4. Label with the appropriate subscript (the voice-leading sum)
5. Trace the path through the network of major-third pair nodes
6. Interpret: Y transformations keep spinning in one direction; X transformations can reverse direction

# Context & Application

Cook applies the X/Y group to Franck's *Le chasseur maudit*, where major-third pairs (e.g., {G minor, B minor, Eb minor}) form the nodes of a network. The music's path through this network models the narrative: Y transformations drive circumnavigation, while X transformations arrest and reverse motion. "Under our model, Y transformations will just keep spinning around the network in the same direction unless stopped by an X transformation" (p. 531).

# Examples

**Franck, *Le chasseur*, mm. 129-140** (pp. 529-531): After moving from G minor to Eb+ via X1, the music begins a counterclockwise Y3 tour. At mm. 132-133, X4 (D+ to A-) arrests the tour, approaching F+ "from the 'wrong' direction." The shift to minor through X4 "is all that prevents the music from circling back and beginning again."

**The complete circumnavigation, mm. 273-346** (pp. 534-542): Three ascents by Y9 (minor-third transposition) divide the passage. The final steps through Eb+ to G- complete the circumnavigation. "The hunter is damned."

# Relationships

## Builds Upon
- **Contextual harmony analysis** — The X/Y group is the algebraic formalization of the contextual model
- **PLR transformations** — X and Y generalize PLR concepts

## Enables
- **Major-third cycle analysis** — The group's nodes are organized by major-third relationships

## Related
- **Chromatic transformation networks** — An alternative formalization of chromatic relations

# Common Errors

- **Error**: Confusing directed voice-leading sums with root-motion intervals
  **Correction**: DVLS measures the sum of semitone displacements across all voices, not the interval between roots

# Common Confusions

- **Confusion**: Thinking X and Y are simply P and non-P
  **Clarification**: X operations include all mode-exchanging transformations (not just P), distinguished by their voice-leading sums

# Source Reference

Cook, Robert C. "Transformational Approaches to Romantic Harmony." Chapter 19 in *The Oxford Handbook of Neo-Riemannian Music Theories*, pp. 528-531. See also Lewin, *Generalized Musical Intervals and Transformations*, 251-253; Cohn, "Square Dances with Cubes," *JMT* 42.2 (1998): 283-296.

# Verification Notes

Fresh extraction — no prior card existed. Confidence is high: Cook explicitly defines and names the X/Y group with formal precision. Page references verified.
