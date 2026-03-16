---
concept: "Schritt/Wechsel System"
slug: schritt-wechsel-system

category: transformations
subcategory: group theory
tier: advanced

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Nora Engebretsen"
chapter: "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems"
chapter_number: 12
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "S/W system"
  - "S/W group"
  - "Schritt-Wechsel group"

prerequisites:
  - harmonieschritte
  - schritte-wechsel
extends:
  - harmonieschritte
related:
  - plr-transformations
  - quintschritt-terzschritt
  - nascent-group-structures
  - over-determined-triad
  - combinatorial-group-theory
contrasts_with:
  - plr-transformations

answers_questions:
  - "What is the Schritt/Wechsel system and how is it structured?"
  - "How does the S/W system relate to the PLR group?"
  - "What distinguishes the S/W system from PLR as an analytical framework?"
---

# Quick Definition

The complete group of 24 transformations on consonant triads, comprising 12 mode-preserving Schritte and 12 mode-reversing Wechsel, isomorphic to the PLR group but organized by root-interval relationships rather than voice-leading parsimony.

# Core Definition

The **Schritt/Wechsel system** formalizes Riemann's Harmonieschritte as a mathematical group of order 24 acting on the 24 consonant triads. It contains 12 **Schritte** (mode-preserving, analogous to transpositions) and 12 **Wechsel** (mode-reversing, analogous to inversions), forming a group isomorphic to the dihedral group D12 (equivalently Z12 semidirect product Z2). Engebretsen (Ch. 12) traces how this group structure, though implicit in Riemann's 1880 Skizze, was first explicitly recognized by Klumpenhouwer (1994), who proposed it as a corrective to Lewin/Hyer's PLR system. Klumpenhouwer argued that the PLR system inconsistently mixed dualist operations (P, L, R are all Wechsel) with monist ones (the Dominant transformation D transposes all triads up a fifth regardless of mode), producing counterintuitive results. The S/W system avoids this by being fully dualist: every operation respects the schlicht/gegen distinction based on the direction of chord generation.

# Prerequisites

- **Harmonieschritte**: The complete taxonomy that the system formalizes
- **Schritte and Wechsel**: The fundamental mode-preserving/reversing distinction

# Key Properties

1. **Order 24**: 12 Schritte (including identity) + 12 Wechsel = 24 total operations
2. **Isomorphic to D12**: The dihedral group of order 24 (equivalently Z12 semidirect product Z2)
3. **Three generators**: Q (Quintschritt), T (Terzschritt), Seitenwechsel; or equivalently S1 and W0
4. **Fully dualist**: All operations respect the inversional structure of major/minor duality
5. **Group presentation**: (Q, T, Seitenwechsel | Q^12 = T^4 = Seitenwechsel^2 = e, QT = TQ, (Q*Seitenwechsel)^2 = (T*Seitenwechsel)^2 = e)
6. **Isomorphic to PLR**: Every S/W operation has a PLR equivalent and vice versa

# Construction / Recognition

Key isomorphisms between S/W and PLR (Ch. 12):

| S/W | PLR | Root interval |
|-----|-----|--------------|
| W0 (Seitenwechsel) | P | 0 semitones, mode change |
| W3 (Leittonwechsel) | L | 3 semitones, mode change |
| W4 (Terzwechsel) | R | 4 semitones, mode change |
| S7 (Quintschritt) | LR | 7 semitones, same mode |
| S4 (Terzschritt) | LPL or RPR | 4 semitones, same mode |

Generator presentation: (S1, W0 | S1^12 = W0^2 = (S1*W0)^2 = e)

# Context & Application

Engebretsen situates the S/W system within a combinatorial tradition where the choice of generators and the constraints placed on their composition determine the character of the resulting transformational system. She identifies a crucial difference between Riemann's and neo-Riemannian usage: Riemann subordinated the Harmonieschritte to key-based constraints (only certain progressions are "intelligible" within a key), while neo-Riemannian theory extracts the unconstrained group, treating all 24 operations as equally valid.

The S/W system is preferable to PLR when:
- Analyzing root-motion patterns
- Working in a dualistic framework
- Reconstructing 19th-century hearing
- Comparing with function theory

The PLR system is preferable when:
- Analyzing voice-leading parsimony
- Working with hexatonic/octatonic cycles
- Emphasizing smooth voice-leading connections

# Examples

Engebretsen's derivation table (Ch. 12) shows how all relationships derive from Q, T, and Seitenwechsel:
- Kleinterzschritt = Q^-1 * T (fifth down, then third up)
- Ganztonschritt = Q^2 (two fifths up)
- Leittonschritt = Q * T (fifth up, then third up)
- Tritonusschritt = Q^3 * T (three fifths plus a third)

Kopp's critique (cited by Engebretsen): PLR makes the fifth relation (dominant) seem "indirect" (requiring LR compound), obscuring its acoustic and perceptual directness. The S/W system preserves this directness with the single symbol Q.

# Relationships

## Builds Upon
- harmonieschritte (the taxonomy that the system formalizes)
- schritte-wechsel (the mode-preserving/reversing distinction)

## Enables
- nascent-group-structures (the implicit group properties Engebretsen identifies)
- combinatorial-group-theory (the formal mathematical framework)

## Related
- quintschritt-terzschritt (the primary generators Q and T)
- over-determined-triad (explains why the generators produce a group of exactly this order)

## Contrasts With
- plr-transformations (voice-leading-based vs. root-interval-based; isomorphic but conceptually different)

# Common Errors

- **Error**: Treating the S/W system as "more correct" than PLR
  **Correction**: They are isomorphic groups with different analytical emphases; neither is inherently superior

# Common Confusions

- **Confusion**: Riemann explicitly formalized the S/W group
  **Clarification**: Riemann described the relationships; Klumpenhouwer (1994) extracted the explicit group structure. Engebretsen describes this as "nascent" group content.

- **Confusion**: The S/W system and PLR system give different analytical results
  **Clarification**: They are isomorphic and produce identical structural analyses; the difference is in how operations are named and which relationships are treated as "basic"

# Source Reference

Engebretsen, Nora. "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 12.

Klumpenhouwer, Henry. "Some Remarks on the Use of Riemann Transformations." *Music Theory Online* 0.9 (1994).

Hook, Julian. "Uniform Triadic Transformations." *Journal of Music Theory* 46.1/2 (2002): 57-126.

# Verification Notes

Re-extracted from v2 card; preserved: group presentation, S/W-PLR isomorphism table, D12 structure, Klumpenhouwer recovery narrative, Kopp critique. Enhanced with Engebretsen's specific framing of nascent group content and combinatorial tradition. High confidence: the central formal structure discussed in Ch. 12.
