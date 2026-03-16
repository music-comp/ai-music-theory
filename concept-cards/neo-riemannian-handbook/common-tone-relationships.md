---
# === CORE IDENTIFICATION ===
concept: Common-Tone Relationships
slug: common-tone-relationships

# === CLASSIFICATION ===
category: harmony
subcategory: voice-leading and pitch retention
tier: foundational

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Suzannah Clark"
chapter: "On the Imagination of Tone in Schubert's Liedesend"
chapter_number: 10
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "common tones"
  - "shared pitch classes"
  - "pitch-class retention"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - plr-transformations
  - klangvertretung
  - tonal-imagination
  - hexatonic-systems
  - maximally-smooth-cycles
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are common-tone relationships and why do they matter?"
  - "How do common tones relate to PLR transformations?"
---

# Quick Definition

The pitch classes shared between two chords, serving as a fundamental measure of harmonic proximity in neo-Riemannian theory and the basis for parsimonious voice leading, with each pitch class participating in exactly six triads (three major, three minor).

# Core Definition

**Common tones** are pitch classes that appear in both chords of a harmonic progression. In neo-Riemannian theory, common-tone count is a primary measure of harmonic proximity: P, L, and R each preserve 2 of 3 tones (the maximum possible for distinct triads), making them "maximally parsimonious." Clark (Ch. 10) emphasizes that common tones are not merely shared pitches but are sites of **Klangvertretung transformation**: when a tone is retained across a chord change, its triadic identity (root, third, or fifth) may shift, and this shift is the mechanism of tonal imagination. Each pitch class participates in exactly 6 triads (as root, third, or fifth of 3 major and 3 minor triads), a fact that Cohn (Ch. 11) uses to derive the structure of the Tonnetz and that Kopp used to derive his complete transformation system.

# Prerequisites

Foundational concept with no technical prerequisites.

# Key Properties

1. **Maximum for triads**: Two common tones is the maximum possible between distinct triads (achieved by P, L, R)
2. **Six triads per pitch**: Each pitch class appears in exactly 6 triads (3 major, 3 minor), corresponding to its 3 possible roles (root, third, fifth) in each mode
3. **Inversely related to voice-leading distance**: More common tones = less voice-leading motion required
4. **Site of identity transformation**: Common tones undergo Klangvertretung shifts (change of triadic role) across chord changes
5. **Basis for parsimony**: Neo-Riemannian parsimony is defined by maximizing common tones

# Construction / Recognition

Common-tone count between triads:

| Transformation | Common Tones | Moving Voice |
|----------------|-------------|-------------|
| P (Parallel) | 2 (root, fifth) | Third moves by semitone |
| L (Leittonwechsel) | 2 (third, fifth) | Root moves by semitone |
| R (Relative) | 2 (root, third) | Fifth moves by whole step |
| Fifth relation (LR) | 1 | Two voices move |
| Tritone relation | 0 | All voices move |

The six triads containing pitch class A: A major (A as root), A minor (A as root), F major (A as third), F# minor (A as third), D major (A as fifth), D minor (A as fifth).

# Context & Application

Clark uses common-tone analysis to trace how individual pitches undergo triadic-identity transformation across Schubert's Lieder (Ch. 10). A sustained pitch serves as a common tone across multiple chord changes, but its meaning (as root, third, or fifth) shifts with each new harmonic context. This approach enriches standard neo-Riemannian chord-level analysis by revealing tone-level structural threads.

Cohn (Ch. 11) uses common-tone preservation as a defining feature of PLR transformations and as the basis for constructing the Tonnetz: edges connect pitch classes that form consonant dyads (and hence can be common tones in triadic progressions).

# Examples

Clark's analysis of Schubert Lieder (Ch. 10): A single pitch persists across distant modulations, its triadic identity transforming at each chord change. The analysis traces the pitch through all possible triadic roles, revealing structural coherence invisible at the chord level alone.

Cohn (Ch. 11): On the Tonnetz, P, L, and R correspond to moving between adjacent triangles that share an edge (2 common tones). The shared edge represents the retained dyad; the third vertex changes.

# Relationships

## Builds Upon
(foundational concept)

## Enables
- plr-transformations (defined by maximizing common tones)
- maximally-smooth-cycles (cycles that retain maximum common tones at each step)
- klangvertretung (common tones are the sites of triadic-identity transformation)

## Related
- tonal-imagination (common-tone identity shifts are the mechanism of tonal imagination)
- hexatonic-systems (LP cycles maximize common-tone retention throughout)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Assuming common tones are analytically passive (merely "held over")
  **Correction**: Clark shows that common tones are analytically active — their changing triadic identity is a primary source of musical meaning

# Common Confusions

- **Confusion**: More common tones always means a "closer" or "better" progression
  **Clarification**: Common-tone count measures one dimension of proximity (voice-leading parsimony), not harmonic function or aesthetic quality; V-I has only 1 common tone but is the strongest functional progression

# Source Reference

Clark, Suzannah. "On the Imagination of Tone in Schubert's Liedesend." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 10.

Cohn, Richard. Chapter 11 (common-tone basis for Tonnetz construction and PLR definition).

# Verification Notes

Re-extracted from v2 card; preserved: six-triads-per-pitch enumeration, PLR common-tone table, Clark's Schubert analysis reference, Kopp derivation mention. Chapter attribution corrected: primary treatment is in Clark Ch. 10, with additional discussion in Cohn Ch. 11. High confidence: explicitly discussed in both chapters.
