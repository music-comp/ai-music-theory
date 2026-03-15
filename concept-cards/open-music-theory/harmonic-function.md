---
# === CORE IDENTIFICATION ===
concept: Harmonic Function
slug: harmonic-function

# === CLASSIFICATION ===
category: harmony
subcategory: functional-harmony
tier: intermediate

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Introduction to Harmony, Cadences, and Phrase Endings"
chapter_number: 1
pdf_page: null
section: "Introduction to Harmony"

# === CONFIDENCE ===
extraction_confidence: high
# high: Concept explicitly defined in source with three categories and examples

# === VARIANTS (authority control) ===
aliases:
  - "functional harmony"
  - "chord function"
  - "T-PD-D-T"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - triad
  - roman-numeral-analysis
  - scale-degree
extends:
  - chord
related:
  - phrase-model
  - cadence
  - voice-leading
contrasts_with:
  - roman-numeral-analysis

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the three categories of harmonic function?"
  - "Which chords are tonic, predominant, and dominant?"
  - "What is the difference between strong and weak predominants?"
---

# Quick Definition

Harmonic function categorizes chords into three roles based on their tendency toward stability or motion: tonic (stable), predominant (transitional), and dominant (tense, demanding resolution), governing the directional flow of tonal harmony.

# Core Definition

**Harmonic function** refers to three categories of chords in common-practice Western classical music (OMT, Ch. IV.1):

1. **Tonic (T)**: Chords that sound stable, providing a sense of home or center. The only chord belonging to this category is I (in minor: i).
2. **Predominant (PD)**: Chords that transition away from tonic function toward dominant function. This category splits into two groups:
   - **Strong predominants**: IV and ii (in minor: iv and ii-o), which signal that a dominant chord is imminent.
   - **Weak predominants**: iii and vi (in minor: VII, III, and VI), which transition away from tonic, typically moving to a stronger predominant.
3. **Dominant (D)**: Chords that provide a sense of urgency to resolve toward the tonic chord: V and vii-o (the same in minor).

# Prerequisites

- **Triad** — Harmonic function classifies triads by their role, so triad construction must be understood first
- **Roman numeral analysis** — Function labels apply to Roman-numeral chord identities within a key
- **Scale degree** — Each function is associated with specific scale degrees in the bass and upper voices

# Key Properties

1. Three and only three functional categories: Tonic, Predominant, Dominant
2. I is the sole tonic-function chord; V and vii-o are dominant-function chords
3. Predominant divides into strong (IV, ii/ii-o) and weak (vi, iii, III, VI, VII)
4. Strong predominants signal an imminent dominant chord
5. Functions progress left to right in the cycle T-PD-D-T, not backward
6. A phrase needs at minimum tonic and dominant function to exhibit trajectory

# Construction / Recognition

## To Identify Harmonic Function in Analysis:
1. Identify each chord's Roman numeral in the key
2. Assign the functional label: I/i = T; IV/iv, ii/ii-o = PD (strong); vi, iii = PD (weak); V, vii-o = D
3. Verify that the functional flow generally moves T to PD to D to T (left to right)
4. Note any exceptions where functions move backward (retrogression)

## To Compose Using Functional Harmony:
1. Begin with tonic function (I)
2. Optionally move through weak then strong predominant
3. Arrive at dominant function (V or vii-o)
4. Resolve to tonic (I) for authentic cadence, or end on V for half cadence

# Context & Application

- **Typical contexts**: Analysis of common-practice Western classical music phrases; composition in tonal idiom
- **Common applications**: Determining chord selection in composition; analyzing phrase direction; understanding why certain progressions "work"
- **Historical/stylistic notes**: The T-PD-D-T model applies broadly to Baroque through Romantic music. The source emphasizes this as the governing principle behind phrase construction in classical harmony.

# Examples

**Example 1** (OMT Ch. IV.1, Example 1): The phrase model diagram shows (a) the minimum functions needed for forward motion (T-D), (b) the more common layout (T-PD-D-T, with optional final tonic), and (c) that functions do not progress right to left.

**Example 2** (OMT Ch. IV.1): In Joseph Boulogne's "Ballet No. 6" from L'amant anonyme, Act II, the first phrase demonstrates tonic-to-dominant motion ending with a half cadence, and the second phrase adds predominant function before the authentic cadence.

## Worked Example

Functional analysis of a typical phrase:
1. I (T) - vi (weak PD) - ii6 (strong PD) - V7 (D) - I (T)
2. Labels: Tb - PD - PD - D - Te
3. The phrase progresses smoothly left to right through the functional cycle

# Relationships

## Builds Upon
- **Triad** — Harmonic function classifies triads by their contextual role
- **Roman numeral analysis** — Function adds a layer of interpretation above Roman numerals

## Enables
- **Phrase model** — The T-PD-D-T phrase model is built directly on functional categories
- **Cadence** — Cadences are defined by functional progressions (D-T for authentic, x-D for half)
- **Voice leading** — Voice-leading rules are shaped by functional tendencies

## Related
- **Strong predominant** — IV and ii as the most common PD chords
- **Tonic prolongation** — Extending tonic function across multiple chords

## Contrasts With
- **Roman numeral analysis** — Roman numerals identify chords; function describes their role

# Common Errors

- **Error**: Labeling vi as always predominant
  **Correction**: vi can function as tonic substitute (after I) or weak predominant (before ii/IV); context determines function

- **Error**: Moving from dominant back to predominant (V to IV) in a phrase
  **Correction**: Functions normally progress T to PD to D to T; D to PD is retrogression and weakens the phrase

# Common Confusions

- **Confusion**: Believing harmonic function is the same as Roman numeral identity
  **Clarification**: A chord's Roman numeral tells you what it IS; function tells you what it DOES in context

- **Confusion**: Thinking a phrase must include all three functions
  **Clarification**: The source states a phrase needs only tonic and dominant function to create trajectory; predominant is common but not required

- **Confusion**: Assuming iii and vi are tonic substitutes
  **Clarification**: In this source's framework, iii and vi are weak predominants that transition toward strong predominants, not tonic substitutes (though vi can have dual function)

# Source Reference

Open Music Theory, Part IV, Chapter 1: "Introduction to Harmony, Cadences, and Phrase Endings," section "Introduction to Harmony." See especially Example 1 (phrase model diagram).

# Verification Notes

- Definition source: Direct from OMT Ch. IV.1, three-category system explicitly defined
- Confidence rationale: HIGH — source provides explicit definitions with numbered categories
- Uncertainties: The source's treatment of vi as purely weak PD differs from some other theory texts that give vi dual T/PD function; preserved the old card's dual-function discussion in Common Confusions
- Cross-reference status: All slugs verified against existing cards
- Re-extraction notes: Re-extracted from v2 card; preserved: expanded functional cycle examples, functional substitution discussion (merged into Relationships and Common Confusions sections), repertoire examples concept (generalized since old card's examples were generic, not source-specific)
