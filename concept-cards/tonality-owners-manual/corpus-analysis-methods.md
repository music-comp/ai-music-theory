---
# === CORE IDENTIFICATION ===
concept: Corpus Analysis Methods
slug: corpus-analysis-methods

# === CLASSIFICATION ===
category: analysis
subcategory: computational-analysis
tier: foundational

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Implicit Musical Knowledge"
chapter_number: 1
pdf_page: 1
section: "Statistics"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - computational corpus study
  - corpus analysis

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - implicit-musical-knowledge
  - composer-theory-vs-theorist-theory
  - dogmatic-musical-conventions
  - schema-theory
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can corpus analysis test traditional music-theoretical claims?"
  - "What role do computational methods play in modern music theory?"
---

# Quick Definition

Computational study of large collections of musical scores to reveal patterns, tendencies, and regularities that are invisible to conventional analytical methods -- providing a "ground truth" against which traditional theoretical claims can be evaluated (p. 22).

# Core Definition

Corpus analysis uses computers to systematically examine large bodies of musical works. Tymoczko created "machine-readable Roman-numeral analyses of more than one thousand pieces stretching from Dufay to Brahms" (p. 23). This extends music theory "beyond the explicit statements contained in written treatises, to the theories implicitly encoded in musical works themselves" (p. 22). Handmade annotations greatly increase analytical power "but at the cost of introducing subjectivity" (p. 23). All materials are freely available online. The approach was "cultivated by David Huron and brought to the masses by Michael Cuthbert" (p. 3).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Provides a "ground truth" for evaluating theoretical claims
2. Can reveal implicit "composer theory" encoded in musical works
3. Requires hand-annotated data for many purposes (scores are computationally opaque)
4. Introduces subjectivity through annotation decisions
5. The data, code, and methods are freely available (appendix 4)
6. Results are "limited and defeasible" -- they undercut extreme skepticism but also challenge textbook verities

# Construction / Recognition

## To Conduct Corpus Analysis:
1. Assemble a corpus of scores with consistent annotation
2. Create machine-readable analyses (e.g., Roman numeral annotations)
3. Systematically extract patterns using computational tools
4. Compare findings against traditional theoretical claims
5. Acknowledge limitations of annotation and sample size

# Context & Application

Computational corpus studies stand alongside geometry as an important twenty-first-century addition to the music theorist's toolbox (p. 22). They are modeled on linguistic corpus studies, which constructed substantial hand-analyzed data sets serving as both primary objects of study and training sets for automatic parsing algorithms (p. 23). Corpus analysis helps identify the continuities between sixteenth-century modality and eighteenth-century functionality -- a gradual harmonic simplification beginning in the Renaissance (p. 26).

# Examples

**Example 1** (pp. 23-24, Figure 1.4.1): Tymoczko's analysis of Palestrina's mass movements showing leaps disproportionately change melodic direction throughout the range, correcting von Hippel and Huron's limited-sample finding.

**Example 2** (p. 56, Figure 2.2.4): Root progressions between major chords in the Rolling Stone "500 Greatest Songs" list, confirming predictions about rock harmony's retrofunctional norm.

**Example 3** (p. 29, Figure 1.5.4): Percentage of triads above scale-degree 3 that are in first inversion, tracking the gradual emergence of functional harmony across centuries.

# Relationships

## Related
- **Implicit Musical Knowledge** -- Corpus analysis reveals implicit knowledge embedded in compositions
- **Composer Theory vs. Theorist Theory** -- Corpus data reconstruct the dialogue between the two
- **Dogmatic Musical Conventions** -- Corpus analysis tests whether dogmatic rules describe actual practice
- **Schema Theory** -- Statistical generality can help schema-theoretic particularity by identifying all instances

# Common Errors

- **Error**: Assuming simple counting answers all musical questions
  **Correction**: Concepts like "voice" and "step" are difficult to pin down; sophisticated tonal music involves compound melodies, arpeggiation, and embellishment (p. 25)

# Common Confusions

- **Confusion**: Thinking musical scores are computationally transparent
  **Clarification**: "D-F-A can be a tonic in D minor, a supertonic in C major, a mere agglomeration of nonharmonic tones" -- interpretation is required (p. 23)

- **Confusion**: Expecting corpus analysis to definitively settle theoretical disputes
  **Clarification**: Results provide "limited and defeasible evidence" that undercuts extremes but does not yield final answers

# Source Reference

Chapter 1, Section 4: "Statistics," pp. 22-26, Figures 1.4.1-1.4.3. Appendix 4 describes the corpus methodology in detail.

# Verification Notes

- Definition source: Direct from pp. 22-23
- Confidence: HIGH -- explicitly discussed as a key methodological tool
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: note about musical scores being "opaque" for computational purposes
