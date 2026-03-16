---
# === CORE IDENTIFICATION ===
concept: Quintschritt and Terzschritt
slug: quintschritt-terzschritt

# === CLASSIFICATION ===
category: harmony
subcategory: fundamental generators
tier: intermediate

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Nora Engebretsen"
chapter: "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems"
chapter_number: 12
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Q and T generators"
  - "fifth step and third step"
  - "Quintschritt (Q)"
  - "Terzschritt (T)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - harmonieschritte
  - schritte-wechsel
  - harmonic-dualism
extends: []
related:
  - schritt-wechsel-system
  - tonnetz
  - verwandtschaftsgrad
  - over-determined-triad
contrasts_with:
  - plr-transformations

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the fundamental generators of Riemann's Harmonieschritte system?"
  - "How do Quintschritt and Terzschritt generate all triadic relationships?"
  - "How do Q and T relate to the Tonnetz axes?"
---

# Quick Definition

The two fundamental mode-preserving harmonic steps in Riemann's system: Quintschritt (Q) moves by perfect fifth and Terzschritt (T) moves by major third, together with Seitenwechsel generating all 24 operations in the Schritt/Wechsel group.

# Core Definition

**Quintschritt** (Q, "fifth step") and **Terzschritt** (T, "third step") are the two mode-preserving generators of Riemann's Harmonieschritte system. Q relates triads whose roots are a perfect fifth apart in the direction of chord generation (up for major, down for minor); T relates triads whose roots are a major third apart in that same direction. Together with the Seitenwechsel (mode exchange), Q and T generate the complete group of 24 triadic operations. Engebretsen (Ch. 12) emphasizes that Riemann treated Q and T as the fundamental relationships from which all others derive: the Ganztonschritt is Q^2, the Kleinterzschritt is Q^-1*T, the Leittonschritt is Q*T, and so on. The "gegen" (contrary) forms are simply the inverses: Gegenquintschritt = Q^-1, Gegenterzschritt = T^-1. Crucially, Q and T commute (QT = TQ), meaning the order of application does not matter — a property reflected in the Tonnetz's two independent axes.

# Prerequisites

- **Harmonieschritte**: The complete taxonomy within which Q and T serve as generators
- **Schritte and Wechsel**: Q and T are both Schritte (mode-preserving)
- **Harmonic dualism**: Determines the direction of "schlicht" vs. "gegen"

# Key Properties

1. **Q has order 12**: Twelve fifths cycle through all pitch classes (circle of fifths)
2. **T has order 3 or 4**: Three major thirds equal one octave in equal temperament (diminished seventh roots)
3. **Commutativity**: QT = TQ (order of application is irrelevant)
4. **Tonnetz axes**: Q generates the horizontal axis (fifths), T generates the vertical axis (thirds)
5. **Acoustic justification**: Q = 3:2 ratio (most consonant after octave), T = 5:4 ratio (next most consonant)
6. **Complete generation**: With Seitenwechsel, Q and T generate all 24 operations

# Construction / Recognition

Derivation of all Schritte from Q and T (Engebretsen Ch. 12):

| Schritt | Derivation | Word Length |
|---------|------------|-------------|
| Quintschritt | Q | 1 |
| Gegenquintschritt | Q^-1 | 1 |
| Terzschritt | T | 1 |
| Gegenterzschritt | T^-1 | 1 |
| Kleinterzschritt | Q^-1*T | 2 |
| Gegenkleinterzschritt | T^-1*Q | 2 |
| Ganztonschritt | Q^2 | 2 |
| Gegenganztonschritt | Q^-2 | 2 |
| Leittonschritt | Q*T | 2 |
| Gegenleittonschritt | Q^-1*T^-1 | 2 |
| Tritonusschritt | Q^3*T | 4 |
| Gegentritonusschritt | Q^-3*T^-1 | 4 |

Wechsel are formed by appending Seitenwechsel to any Schritt: e.g., Quintwechsel = Q*Seitenwechsel.

# Context & Application

Engebretsen notes that Riemann's choice of Q and T as generators reflects his acoustic heritage (fifths and thirds as the primary consonant intervals) and contrasts with the PLR system's choice of voice-leading parsimony as the organizing principle. In the PLR system, the fifth relation (dominant) requires the compound LR, obscuring its acoustic directness. Kopp favors Q and T for analyzing 19th-century music precisely because acoustic relations (not voice-leading efficiency) are the primary organizational logic of that repertoire.

The commutativity of Q and T is reflected in the Tonnetz's structure: the two axes are independent, and path order does not matter. This also means the Schritte subgroup is abelian (Z12), while the full S/W group (including Wechsel) is non-abelian.

# Examples

From C major (Ch. 12):
- Q: C+ to G+ (up perfect fifth)
- T: C+ to E+ (up major third)
- Q^2: C+ to D+ (up whole tone = two fifths)
- Q*T: C+ to B+ (up semitone = fifth then third)
- Q^-1*T: C+ to A+ (down fifth, up third = up minor third/down major sixth)

In dualistic minor (from A minor, with generation directed downward):
- Q: a- to d- (down perfect fifth, in direction of generation)
- T: a- to f#- (down major third, in direction of generation)

# Relationships

## Builds Upon
- harmonieschritte (the system within which Q and T are generators)
- schritte-wechsel (Q and T are Schritte)
- harmonic-dualism (determines schlicht/gegen directionality)

## Enables
- schritt-wechsel-system (Q, T, and Seitenwechsel generate the complete group)
- tonnetz (Q and T generate the two axes)
- verwandtschaftsgrad (distances measured in Q and T steps)

## Related
- over-determined-triad (the reason Q and T produce a system of exactly this structure)

## Contrasts With
- plr-transformations (P, L, R privilege voice-leading parsimony; Q, T privilege acoustic-interval directness)

# Common Errors

- **Error**: Assuming "schlicht" always means "upward"
  **Correction**: Schlicht means "in the direction of chord generation": upward for major (overtone generation), downward for minor (undertone generation in dualistic theory)

# Common Confusions

- **Confusion**: Q is the same as the dominant function
  **Clarification**: Q describes a root-interval relationship (up a fifth); dominant describes a functional role within a key. They often coincide but are conceptually distinct.

- **Confusion**: T is the same as the mediant function
  **Clarification**: Similarly, T describes a root-interval relationship; mediant describes a functional role. They are not equivalent.

# Source Reference

Engebretsen, Nora. "The 'Over-Determined' Triad as a Source of Discord: Nascent Groups and the Individuation of Transformational Systems." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 12.

Riemann, Hugo. *Skizze einer neuen Methode der Harmonielehre*. Leipzig: Breitkopf und Haertel, 1880.

# Verification Notes

Re-extracted from v2 card; preserved: derivation table, commutativity property, order of Q and T, PLR comparison, Kopp's preference argument. Enhanced with dualistic minor examples. High confidence: Q and T are explicitly discussed as generators throughout Ch. 12.
