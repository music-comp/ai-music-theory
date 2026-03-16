---
# === CORE IDENTIFICATION ===
concept: Nascent Group Structures
slug: nascent-group-structures

# === CLASSIFICATION ===
category: transformations
subcategory: group theory
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Nora Engebretsen"
chapter: "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems"
chapter_number: 12
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: medium

# === VARIANTS (authority control) ===
aliases:
  - "nascent groups"
  - "implicit group-theoretic content"
  - "proto-group structures"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - harmonieschritte
  - schritt-wechsel-system
extends: []
related:
  - over-determined-triad
  - combinatorial-group-theory
  - quintschritt-terzschritt
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are nascent group structures in 19th-century harmony?"
  - "Did 19th-century theorists engage with group theory?"
  - "How does the Harmonieschritte system contain implicit group properties?"
---

# Quick Definition

The implicit mathematical group structures present in 19th-century harmonic theories (especially Riemann's Harmonieschritte) that were not formally recognized as groups by their authors but contain the essential properties of closure, associativity, identity, and inverses that neo-Riemannian theorists later made explicit.

# Core Definition

**Nascent group structures** is Engebretsen's characterization of the implicit group-theoretic content in 19th-century harmonic theories, particularly Riemann's Harmonieschritte system. Although 19th-century theorists "did not explicitly engage combinatorial group theory, which was emerging roughly contemporaneously" (Ch. 12, note 17), their systems exhibit the essential properties of mathematical groups: Riemann's Harmonieschritte system has closure (combining any two operations yields another operation in the system), an identity element (the null Schritt), inverses (every schlicht Schritt has a gegen counterpart), and associativity. Engebretsen traces how these nascent properties were shaped by each theorist's conception of key and tonal coherence, which acted as constraints on the composition of generators. The formalization of these implicit structures as explicit mathematical groups was achieved by Klumpenhouwer (1994) and refined by Gollin and Engebretsen herself, answering a question posed by Cohn about whether "nascent group-theoretic content" exists in 19th-century harmonic theory.

# Prerequisites

- **Harmonieschritte**: The primary 19th-century system containing nascent group properties
- **Schritt/Wechsel system**: The explicit group formalization of those nascent properties

# Key Properties

1. **Closure**: Composing any two Harmonieschritte yields another Harmonieschritt
2. **Identity**: The identity operation (no change) exists in the system
3. **Inverses**: Every schlicht operation has a gegen inverse
4. **Associativity**: The order of composition brackets does not matter
5. **Generators and relators**: The system has implicit generators (Q, T, Seitenwechsel) and relations among them
6. **Key-based constraints**: 19th-century theorists constrained generator composition through key-based intelligibility criteria

# Construction / Recognition

Engebretsen identifies nascent group properties in multiple 19th-century systems (Ch. 12):

- **Hauptmann**: Key relations constrain composition of generators; group properties are implicit
- **Oettingen**: Root-interval classification with mode distinction; refines Hauptmann's approach
- **Riemann**: Most explicit system with Q, T, and Seitenwechsel as generators; diatonic then chromatic extension
- **Hostinsky**: Treats Terzwechsel as a generator alongside Terzschritt, producing a different generator set

Each system exhibits the same group (D12), but the generators and constraints differ, producing different "individuation" of the group.

Riemann's diatonic system (Ch. 12) derives relationships between primary and secondary triads within a key using only a restricted set of generators. The chromatic extension (Systematik, section 38) expands to the full group by removing key-based constraints.

# Context & Application

The concept of nascent group structures addresses a historiographical question: to what extent did 19th-century theorists anticipate the mathematical structures that neo-Riemannian theory makes explicit? Engebretsen's answer is nuanced: the group properties are genuinely present (not projected anachronistically), but the 19th-century context shaped them differently from their neo-Riemannian formalization. Specifically, 19th-century theorists used key-based constraints to limit the "chaos of possibilities" that unconstrained group operations would produce, while neo-Riemannian theory removes those constraints and embraces the full group.

This concept is relevant for anyone studying the intellectual history of music theory, the relationship between implicit and explicit mathematical structure in music, or the proper historiography of neo-Riemannian theory's claims about its predecessors.

# Examples

Engebretsen's comparison of diatonic and chromatic Harmonieschritte (Ch. 12):

**Diatonic system**: Riemann restricts to relationships involving primary triads (I, IV, V), producing a subset of the full group. The Quintschritt connects T to D; the Terzwechsel connects T to Tp; etc. Key-based constraints limit which compositions are "intelligible."

**Chromatic extension** (Systematik): Riemann removes key constraints, allowing all combinations of Q, T, and Seitenwechsel. The resulting 25 relationships (augmented to 24 unique operations plus identity) form the complete group D12.

The transition from diatonic to chromatic system "individuates" the group by progressively removing constraints on generator composition.

Gollin deserves credit (per Engebretsen, note 17) for introducing the formal terminology and apparatus of combinatorial group theory into neo-Riemannian discourse.

# Relationships

## Builds Upon
- harmonieschritte (the primary system exhibiting nascent properties)
- schritt-wechsel-system (the explicit group formalization)

## Enables
- Understanding the historical continuity between 19th-century theory and neo-Riemannian formalization
- combinatorial-group-theory (the mathematical framework applied to make nascent structures explicit)

## Related
- over-determined-triad (explains why different theorists arrived at different nascent groups)
- quintschritt-terzschritt (the generators whose composition properties create the nascent group)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Claiming that 19th-century theorists "knew" group theory
  **Correction**: Engebretsen carefully distinguishes between the presence of group properties (genuine) and conscious engagement with group theory (absent)

# Common Confusions

- **Confusion**: Nascent group structures are just a retroactive projection by modern theorists
  **Clarification**: Engebretsen argues the group properties are genuinely present in the historical sources, not projected; what is modern is the explicit formalization, not the underlying structure

- **Confusion**: All 19th-century theorists found the same group
  **Clarification**: They found different "individuations" of the same abstract group (D12), reflecting different choices of generators and constraints shaped by different conceptions of key

# Source Reference

Engebretsen, Nora. "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 12.

Cohn, Richard. "Neo-Riemannian Operations, Parsimonious Trichords, and Their Tonnetz Representations." *Journal of Music Theory* 41.1 (1997) — posed the question about nascent group content.

# Verification Notes

New card (no previous version). Medium confidence: the concept is synthesized from Engebretsen's overall argument rather than a single explicit definition; the term "nascent" is used throughout the chapter but the concept requires reconstruction from her historiographical discussion.
