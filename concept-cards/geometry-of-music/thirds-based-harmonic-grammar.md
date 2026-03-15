---
# === CORE IDENTIFICATION ===
concept: Thirds-Based Harmonic Grammar
slug: thirds-based-harmonic-grammar

# === CLASSIFICATION ===
category: harmony
subcategory: functional-harmony
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Functional Harmony"
chapter_number: 7
pdf_page: 244
section: "7.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "descending thirds model"
  - "chain of descending thirds"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - conjunct-melodic-motion
  - efficient-voice-leading
extends: []
related:
  - circle-of-thirds-harmonic
  - strong-vs-weak-progressions
  - harmonic-cycles
  - descending-fifths-as-composite
  - minor-mode-functional-harmony
contrasts_with:
  - traditional-function-theory

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does Tymoczko model chord progressions in functional harmony?"
  - "Why are descending thirds considered more fundamental than descending fifths?"
  - "What single model accounts for nearly all diatonic progressions in Bach and Mozart?"
---

# Quick Definition
A model of functional harmony in which diatonic chords (excluding iii) are arranged as a chain of descending thirds from I to V, with rightward motion always permitted and leftward motion restricted to specific arrows.

# Core Definition
Tymoczko's thirds-based harmonic grammar arranges the diatonic triads as a chain of descending thirds: I - vi - IV - ii - vii° - V. Chords may move rightward by any number of steps along this chain (from tonic toward dominant), but may move leftward only along specifically labeled arrows (V to I, vii° to I, and IV to I being the most important). A sequence of rightward motions from the tonic followed by a leftward return constitutes a "harmonic cycle." Each Roman numeral can represent either a triad or seventh chord, with sevenths on ii and V being particularly common. The V chord can be preceded by a cadential I-six-four, and any chord other than vii° can be preceded by its own applied dominant. This model privileges descending thirds and fifths as more often permissible than ascending thirds and fifths, while ascending steps are more permissible than descending steps.

# Prerequisites
- Basic understanding of diatonic triads and Roman numeral analysis
- Knowledge of efficient voice leading between chords (see efficient-voice-leading)

# Key Properties
1. Descending thirds are the fundamental motion; descending fifths are composite (factored into pairs of thirds)
2. 95% of roughly 3000 two-chord diatonic progressions in Bach chorales conform to the model
3. 97-99% of roughly 10,000 diatonic progressions in Mozart piano sonatas conform
4. The iii chord is omitted because it is rare in nonsequential harmony
5. One geometrical picture, explainable in a single hour, accounts for the vast majority of common-practice chord progressions

# Construction / Recognition
## To Construct/Create:
1. Arrange diatonic triads as: I - vi - IV - ii - vii° - V
2. Allow free rightward motion (toward dominant)
3. Add leftward arrows for V->I, vii°->I, IV->I, and vi->I
4. Optionally add applied dominants before any chord except vii°
5. Add cadential I-six-four before V

## To Identify/Recognize:
1. Check if a progression moves rightward along the chain (strong motion)
2. Check if leftward motions use one of the permitted arrows
3. Progressions violating both directions are likely sequences, chromatic chords, or other special devices

# Context & Application
The thirds-based grammar is Tymoczko's alternative to traditional descriptions of functional harmony. Where traditional theory privileges root motion by fifth (the circle of fifths), Tymoczko argues that descending thirds are more fundamental because fifths can be factored into pairs of thirds (I-IV can become I-vi-IV) but thirds cannot generally be factored into fifths. The model unifies ideas from Rameau, Kostka and Payne, Agmon, Meeus, and Quinn into a single geometric picture. It applies primarily to Western classical music of the baroque and classical periods, where harmonic constraints are unusually strict compared to Renaissance, popular, or folk music.

# Examples
**Example 1** (Fig. 7.1.7, p. 248): The most common harmonic cycles in Bach chorales all conform to the model: I-V-I (90 instances), I-ii-V-I (30), I-IV-V-I (26), I-IV-I (22), I-vii°-I (21).

**Example 2** (Fig. 7.1.3, p. 246): Bach's chorale "Auf, auf, mein Herz" traverses an extended segment of the descending thirds cycle.

**Example 3** (Fig. 7.1.4, p. 246): Bach's duet BWV 803, mm. 22-26, interposes iii between V and I, where the engine of falling thirds temporarily overcomes the V-I paradigm.

# Relationships
## Builds Upon
- **efficient-voice-leading** — Third-related chords share common tones and connect by efficient voice leading
## Enables
- **harmonic-cycles** — Cycles are defined as rightward motion followed by leftward return
- **strong-vs-weak-progressions** — The asymmetry of the graph defines strong vs weak
- **third-substitution** — Adjacent chords on the chain can substitute for each other
## Related
- **circle-of-thirds-harmonic** — The underlying geometric structure
- **descending-fifths-as-composite** — Fifths decompose into pairs of thirds
## Contrasts With
- Traditional function theory (Riemann) which groups chords into three functions

# Common Errors
- **Error**: Assuming the model claims descending fifths are unimportant
  **Correction**: Fifths may be more *common* than thirds, but thirds are more *fundamental* because they cannot be factored further

# Common Confusions
- **Confusion**: Conflating what is *permissible* with what is *probable*
  **Clarification**: Root progressions by descending fifth are more common than by third, but when describing permissible progressions, falling thirds are the basic unit
- **Confusion**: Thinking the model is a complete description of tonal harmony
  **Clarification**: The model is a first approximation; it does not incorporate chromatic chords, sequences, parallel first-inversion triads, or three-chord idioms like vi-I6-(IV/ii6)

# Source Reference
Chapter 7: Functional Harmony, Section 7.1, pages 244-248, Figures 7.1.1-7.1.7.

# Verification Notes
- Definition source: Direct from Section 7.1, with Figure 7.1.1 as the central diagram
- Confidence rationale: High — this is the chapter's central theoretical contribution, extensively developed with statistical evidence
- Cross-reference status: Verified against discussion of sequences (7.3), voice leading (7.2), and the Schenkerian critique (7.6)
