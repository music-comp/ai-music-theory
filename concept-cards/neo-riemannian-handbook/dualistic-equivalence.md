---
# === CORE IDENTIFICATION ===
concept: Dualistic Equivalence
slug: dualistic-equivalence

# === CLASSIFICATION ===
category: transformations
subcategory: equivalence-relations
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Dmitri Tymoczko"
chapter: "Dualism and the Beholder's Eye"
chapter_number: 8
pdf_page: null
section: "Section 2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "dualistic equivalence relation"
  - "inversional equivalence of progressions"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inversional-symmetry
  - voice-leading
extends: []
related:
  - schritt-wechsel-system
  - contrapuntal-vs-harmonic-dualism
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When are two chord progressions dualistically equivalent?"
  - "How does Tymoczko formalize Riemann's dualistic concepts?"
---

# Quick Definition

Two chord progressions are dualistically equivalent if they are related by uniform transposition or inversion -- that is, if applying the same transposition or inversion operation to every chord in one progression produces the other, preserving the voice-leading distances between all chords.

# Core Definition

Tymoczko defines **dualistic equivalence** as a relation between chord progressions (Ch. 8): two progressions are dualistically equivalent if "related by uniform transposition or inversion." This formalizes what Riemann's Schritte/Wechsel terminology captured informally -- the idea that certain major-mode and minor-mode progressions are "the same" transformation.

Importantly, Tymoczko treats chord progressions as "higher order musical objects related by transposition and inversion," rather than following Lewin's approach of treating Schritte and Wechsel as functions that take a chord as input and return another chord as output (Ch. 8, n. 21). This permits dualistic equivalence between chords with different symmetries.

# Prerequisites

- **Inversional symmetry** -- Dualistic equivalence depends on inversion as a distance-preserving operation
- **Voice leading** -- Equivalent progressions have equivalent voice-leading profiles

# Key Properties

1. Preserves voice-leading distances between all corresponding chords
2. Subsumes transpositional equivalence (transposition is a special case)
3. Extends to progressions involving chords of different types (e.g., augmented to diminished)
4. Two dualistically equivalent progressions will have the same DVLS/AVLS values
5. In Lewin's framework, single transformations cannot capture all dualistic equivalences

# Construction / Recognition

## Testing for Dualistic Equivalence
1. Take two progressions: A→B and C→D
2. Check if there exists a transposition Tn such that Tn(A)=C and Tn(B)=D
3. OR check if there exists an inversion In such that In(A)=C and In(B)=D
4. If either condition holds: the progressions are dualistically equivalent

## Example
- Ab major → C major (German sixth resolution pattern)
- F major → E minor (penultimate progression in *Tristan*)
- These are dualistically equivalent: the inversion transforming Ab to F also transforms C major to E minor

# Context & Application

Dualistic equivalence explains why Riemann's terminology, despite describing symmetries that traditional tonality does not possess, proves useful for analyzing chromatic music. Chromatic passages frequently contain dualistically equivalent progressions because efficient voice leadings come in inversionally related pairs.

The concept also explains the analytical power of neo-Riemannian operations: P, L, R, and compound transformations capture dualistic equivalence classes.

# Examples

**German sixth resolution** (Ch. 8): The resolution of the German augmented sixth chord to the tonic involves a specific voice-leading pattern. Its inversional equivalent appears as the resolution of a related sonority to a different chord -- both are dualistically equivalent.

**Brahms modulations** (Ch. 8, n. 43): The pivot-chord modulation where iv of Bb major becomes iii of Cb major is dualistically equivalent to the modulation in Schubert's "Die junge Nonne" where VI of F minor becomes V of F# minor. "Schubert exploits the former property and Brahms the latter, with their two modulations being dualistically equivalent."

# Relationships

## Builds Upon
- **Inversional symmetry** -- Dualistic equivalence is based on inversion as uniform operation
- **Voice leading** -- Equivalent progressions share voice-leading properties

## Enables
- **Contrapuntal vs. harmonic dualism** -- Dualistic equivalence can be explained harmonically or contrapuntally

## Related
- **Schritt/Wechsel system** -- Historically captured the same equivalences informally

# Common Errors

- **Error**: Applying dualistic equivalence to individual chords rather than progressions
  **Correction**: Dualistic equivalence is a relation between progressions (sequences of chords), not between individual sonorities

# Common Confusions

- **Confusion**: Dualistic equivalence means the two progressions sound the same
  **Clarification**: They share structural properties (voice-leading distances) but may sound very different in musical context

# Source Reference

Chapter 8: Dmitri Tymoczko, "Dualism and the Beholder's Eye," in *The Oxford Handbook of Neo-Riemannian Music Theories*. Section 2, n. 21.

# Verification Notes

- Definition: From Tymoczko's explicit formulation in Ch. 8
- Distinction from Lewin: From n. 21
- Brahms/Schubert example: From n. 43
- Confidence: HIGH -- explicitly defined concept
- New card (no previous v2 card)
