---
# === CORE IDENTIFICATION ===
concept: Parsimonious Trichords
slug: parsimonious-trichords

# === CLASSIFICATION ===
category: transformations
subcategory: voice-leading-theory
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Suzannah Clark"
chapter: "Analyzing Schubert"
chapter_number: 10
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "parsimonious voice leading"
  - "generated trichords"
  - "maximally smooth trichords"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - plr-transformations
  - voice-leading-graph
extends: []
related:
  - hexatonic-systems
  - octatonic-systems
  - dvls-avls
  - tonnetz
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What makes consonant triads 'parsimonious' in voice-leading terms?"
  - "How do PLR operations generate cycles of triads?"
  - "What is the relationship between near-evenness and parsimonious voice leading?"
---

# Quick Definition

Three-note chords (trichords) that can be connected through maximally smooth voice leading, where only one voice moves by a single semitone, forming the basis for neo-Riemannian transformational systems.

# Core Definition

**Parsimonious trichords** are set classes that participate in efficient voice-leading networks. A connection is parsimonious when total voice-leading distance is minimal and the number of moving voices is minimal. Richard Cohn demonstrated that consonant triads [037] are optimal for parsimonious voice leading because of their near-evenness: the interval pattern (4-3-5) is close to even division of the octave, enabling semitone displacements. Clark (Ch. 10) explores these properties through Riemann's concept of Klangvertretung, showing how attention to changing triadic identities of tones enriches analysis of Schubert Lieder.

For triads: P and L operations achieve DVLS = 1 (most parsimonious); R achieves DVLS = 2 (next most parsimonious).

# Prerequisites

- **PLR transformations**: The operations that connect parsimonious trichords.
- **Voice-leading graphs**: The geometric framework for measuring parsimony.

# Key Properties

1. **Two common tones**: Each PLR operation preserves two of three tones
2. **Semitone displacement**: P and L move one voice by semitone
3. **Near-evenness**: Triads' interval pattern (4-3-5) nearly divides the octave equally
4. **Cycle generation**: Compound operations produce closed cycles of specific lengths

# Construction / Recognition

Repeated application of operations generates cycles:
| Operation | Cycle Length | Set Traversed |
|-----------|-------------|---------------|
| LP | 6 | Hexatonic cycle [014589] |
| PR | 8 | Octatonic cycle [0134679T] |
| LR | 24 | All 24 triads |

PLR operations are involutions (self-inverse). Their combinations produce all possible triadic relations, and the group has rich internal structure isomorphic to D12.

# Context & Application

Clark (Ch. 10) begins from Riemann's "Ideen" (1914-15), Figure 3: three pairs of triads showing P, R, and L relationships. Riemann observed that imagining any perfect fifth, major third, or minor third yields exactly one major and one minor triad sharing those tones. These common-tone relationships anchor parsimonious voice-leading connections. Clark applies Klangvertretung to show how tones change their triadic significance in Schubert's chromatic passages.

# Examples

**Common-tone anchoring** (Clark, Ch. 10): P holds root and fifth, third moves; L holds third and fifth, root moves; R holds root and third, fifth moves. Each preserves a different dyad from the original triad.

**Hexatonic cycle**: C+ --P--> c- --L--> Ab+ --P--> ab- --L--> E+ --P--> e- --L--> C+. Six triads, returning to start after six steps, all within hexatonic collection [014589].

**DOUTH2 relation** (Douthett): Two chords where two tones remain fixed and remaining tones move by semitone. P and L are DOUTH2 relations on triads. Extends to larger sets (e.g., [0148] tetrachords).

# Relationships

## Builds Upon
- PLR transformations and voice-leading theory

## Enables
- Hexatonic and octatonic systems as analytical frameworks
- Maximally smooth cycles in chromatic analysis

## Related
- Tonnetz: Parsimonious connections form the edges of the triadic Tonnetz
- DVLS/AVLS: Metrics that formalize parsimony

## Contrasts With
- Functional harmonic analysis: Parsimony measures voice-leading distance, not harmonic function

# Common Errors

- **Error**: Assuming parsimony applies only to triads.
  **Correction**: Parsimonious behavior is possible for any set class with near-even interval structure.

# Common Confusions

- **Confusion**: Equating "parsimonious" with "efficient" generally.
  **Clarification**: "Parsimonious" specifically means minimal voice-leading displacement; "efficient" can mean many things.

- **Confusion**: Thinking acoustic consonance and voice-leading parsimony are the same property.
  **Clarification**: These are independent properties that happen to coincide in consonant triads.

# Source Reference

Clark, Suzannah. "Analyzing Schubert." In *The Oxford Handbook of Neo-Riemannian Music Theories*, Chapter 10. See also Cohn, "Neo-Riemannian Operations, Parsimonious Trichords, and Their Tonnetz Representations" (1997).

# Verification Notes

Re-extracted from v2 card; preserved: definition of parsimony, PLR comparison, cycle generation table, DOUTH2 relation, near-evenness concept, extension to other set classes. Corrected chapter attribution to Clark (Ch. 10) as primary. Confidence high.
