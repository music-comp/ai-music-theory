---
concept: Two Musical Geometries
slug: two-musical-geometries

category: geometric-theory
subcategory: representations
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 130
section: "3.12"

extraction_confidence: high

aliases:
  - "circular vs higher-dimensional representation"
  - "complementary geometries"

prerequisites:
  - two-note-chord-space
  - higher-dimensional-chord-spaces
extends: []
related:
  - voice-leading-lattices
contrasts_with: []

answers_questions:
  - "What are the two geometric models of musical space?"
  - "What are the strengths and weaknesses of each?"
---

# Quick Definition
The circular pitch-class space of Chapter 2 and the higher-dimensional chord spaces of Chapter 3 are complementary representations of the same musical facts, each with distinct strengths. The circle is simpler and handles chords of any size; chord spaces reveal deeper structural relationships.

# Core Definition
Tymoczko identifies two geometric representations that recur throughout the book. The circular model represents any chord as a collection of points on a circle, with voice leadings as collections of paths. It handles chords of any size uniformly, accommodates doublings naturally, and is easy to use. However, it can obscure important relationships (such as the decomposition into parallel and contrary motion). The higher-dimensional chord spaces provide separate spaces for chords of each size (2D for dyads, 3D for triads, etc.) and can reveal structural relationships invisible in the circular model. However, they are hard to visualize beyond three dimensions, cannot easily handle doublings, and do not provide a unified space for chords of different sizes. For practical purposes, the discrete lattices of Section 3.11 combine the advantages of both approaches.

# Prerequisites
- Both the circular pitch-class model and the chord space model

# Key Properties
1. Circular model: simple, uniform, handles any chord size, accommodates doublings
2. Chord space model: reveals structural relationships, decomposition into components, lattices
3. Neither model is strictly superior; deep understanding requires fluency with both
4. The discrete lattices combine advantages of both approaches

# Context & Application
Tymoczko compares the two approaches, quoting Feynman's observation that "every theoretical physicist who is any good knows six or seven different theoretical representations for exactly the same physics." The comparison serves as a summary of Chapter 3's contribution and a transition to the analytical applications in later chapters.

# Examples
**Example 1** (p. 131): Voice leadings between {C,E,G} and {D,F,A} can be represented either as line segments in 3D chord space or as collections of paths on the pitch-class circle (Figure 3.12.1).
**Example 2** (p. 132): The question "which transposition is chord X closest to?" is hard to answer on the circle but easy using the lattice in Figure 3.11.8.

# Relationships
## Builds Upon
- **two-note-chord-space** — The simplest chord space
- **higher-dimensional-chord-spaces** — The full system of chord spaces
## Related
- **voice-leading-lattices** — Practical tools that bridge both representations

# Common Errors
- **Error**: Treating one model as the "correct" one and abandoning the other
  **Correction**: Both are valid representations of the same musical facts; each reveals different aspects

# Common Confusions
- **Confusion**: Thinking the lattices are a third, separate model
  **Clarification**: The lattices are extracted from the chord spaces — they are a practical simplification, not a separate theory

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.12, pages 130-133.

# Verification Notes
- Definition source: From Section 3.12 comparison
- Confidence rationale: High — explicitly stated as a summary comparison
- Cross-reference status: Verified against both Chapter 2 and Chapter 3 models
