---
# === CORE IDENTIFICATION ===
concept: Reductive Analysis
slug: reductive-analysis

# === CLASSIFICATION ===
category: analysis
subcategory: analytical-method
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Nonharmonic Tones"
chapter_number: 5
pdf_page: 210
section: "The standardized second practice"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - nonharmonic reduction
  - harmonic reduction

# === TYPED RELATIONSHIPS ===
prerequisites:
  - nonharmonic-tone
  - snap-system
extends: []
related:
  - palestrina-style-constraints
  - reduction-as-paraphrase
  - standardized-second-practice
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is reductive analysis and what are its three forms?"
  - "How can the nonharmonic system be used computationally to score harmonic interpretations?"
  - "What are the limits of nonharmonic reduction as an analytical method?"
---

# Quick Definition

The practice of eliminating "surface" musical activity to reveal a "background" that clarifies a piece's logic, encompassing three distinct forms: nonharmonic reduction, textural reduction, and summarizing reduction.

# Core Definition

Reductive analysis is the central practice of contemporary music analysis, involving the removal of surface activity to reveal underlying structures. Tymoczko distinguishes three kinds: (1) nonharmonic reduction -- removing nonharmonic tones to reveal a harmonic skeleton; (2) textural reduction -- converting complex textures to chorale-like backgrounds; and (3) summarizing reduction -- finding long-range connections between nonadjacent events. These forms are "poorly understood partly because scholarship has not always distinguished them, partly because earlier composers may not have shared modern reductive instincts, and partly because of difficult philosophical questions" (p. 203). The nonharmonic system imposes a powerful constraint on musical analysis: the theorist must find harmonies such that dissonances belong to recognized nonharmonic species and harmonic successions obey functional norms (p. 239).

# Prerequisites

- **Nonharmonic tone** -- Understanding what constitutes a nonharmonic tone is essential for performing reduction
- **SNAP system** -- The categories of nonharmonic tone provide the vocabulary for reduction

# Key Properties

1. Three distinct forms: nonharmonic, textural, and summarizing reduction
2. Nonharmonic reduction is theoretically feasible in baroque and classical music -- a computer can do it reasonably well
3. In Bach's chorales, more than 95% of chord progressions conform to functional grammar while all but about ten nonharmonic tones behave recognizably (p. 239)
4. Computer analysis of Mozart's first piano sonata correctly identifies chords about 75% of the time using nonharmonic penalty scoring (p. 240, Fig. 5.4.7)
5. Theoretical feasibility does not imply analytical or aesthetic advisability -- syntax may allow a reduction that misrepresents semantics

# Construction / Recognition

## To Perform Nonharmonic Reduction:
1. Identify all nonharmonic tones using SNAP categories (suspensions, neighbors, anticipations, passing tones)
2. Replace each nonharmonic tone with its harmonic counterpart
3. Verify that the resulting harmonic skeleton obeys functional norms
4. Check that notes both leapt-to and leapt-away-from are treated as harmonic
5. Assign penalties to nonharmonic tones (higher for less common varieties) to "score" competing interpretations

# Context & Application

Reductive analysis is most straightforward in strictly controlled styles like Palestrina and Bach. As music becomes more complex (Monteverdi, Mahler), reduction becomes increasingly interpretive. In Mahler's late music, nonharmonic reduction is "no longer something broadly intersubjective or syntactical, but rather autobiographical and interpretive" (p. 245). The nonharmonic system can be put to computational work by scoring harmonic interpretations based on how well-behaved the voices are, applying the procedure recursively at different harmonic rhythms (p. 239).

# Examples

**Example 1** (p. 240, Fig. 5.4.7): Computer analysis of the opening of Mozart's first piano sonata, using nonharmonic penalties to score harmonic interpretations -- correct about 75% of the time.

**Example 2** (p. 239): In Bach's chorales, virtually all nonharmonic tones can be labeled according to traditional counterpoint rules, with only about ten anomalies among more than ten thousand nonharmonic tones.

**Example 3** (p. 245, Fig. 5.5.2): Reduction of the opening of Mahler's Ninth Symphony, Rondo-Burlesque -- where reduction becomes autobiographical and interpretive rather than intersubjective.

# Relationships

## Builds Upon
- **Nonharmonic tone** -- Reduction depends on identifying and removing nonharmonic tones
- **SNAP system** -- Provides the categories for classifying what to reduce

## Enables
- **Harmonic cycle theory** -- Reduction reveals the harmonic skeleton that harmonic cycle analysis operates on

## Related
- **Reduction as paraphrase** -- Tymoczko's alternative view of summarizing reduction as paraphrase rather than deep structure
- **Palestrina style constraints** -- The style where reduction is most straightforwardly applicable
- **Standardized second practice** -- The baroque system that makes systematic reduction possible

## Contrasts With
- (No direct contrasts specified in source)

# Common Errors

- **Error**: Conflating the three types of reduction (nonharmonic, textural, summarizing)
  **Correction**: Each operates differently and has different limitations; conflating them leads to confused analysis

# Common Confusions

- **Confusion**: Thinking that because reduction is syntactically feasible, it is always analytically advisable
  **Clarification**: "The syntax of the music allows for a reduction that sometimes leads us to misunderstand the music's semantics" (p. 240) -- the gap between syntax and semantics is critical

- **Confusion**: Schenkerian "representation" where less important notes represent more important notes at deeper structural levels
  **Clarification**: Tymoczko argues this notion of representation is superfluous -- summarizing reduction is more like paraphrasing a movie plot than revealing hidden structure

# Source Reference

Chapter 5: Nonharmonic Tones, sections 4-5 "The standardized second practice" and "A loophole," pp. 235-246. Also Prelude to Chapter 5, pp. 203-209.

# Verification Notes

- Definition source: Synthesized from multiple sections of Ch. 5
- Confidence rationale: Central concept discussed extensively with computational examples
- Cross-reference status: Verified against reduction-as-paraphrase, SNAP system cards
- Re-extraction notes: Re-extracted from v2 card; preserved: three-part typology of reduction, Mozart computer analysis example, Mahler interpretive reduction quote, Bach chorale statistics
