---
concept: Developing Spectra
slug: developing-spectra

category: timbral-temporal-systems
subcategory: timbral-gis
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.2"

extraction_confidence: high

aliases:
  - "DVSP"
  - "developing spectral pattern"

prerequisites:
  - timbral-gis
  - direct-product-gis
extends:
  - direct-product-gis
related: []
contrasts_with: []

answers_questions:
  - "What is a developing spectrum (DVSP)?"
  - "How can timbral evolution be modeled using GIS?"
---

# Quick Definition
A Developing Spectrum (DVSP) is a set of spectrum-time pairs representing how a sound's timbral content evolves over time, analyzed using a direct-product GIS combining a timbral GIS (8 partials) with a time-point GIS.

# Core Definition
Let GIS_1 be a timbral GIS (spectra with 8 partials) and GIS_2 be a time-point GIS. The direct product GIS_3 = GIS_1 x GIS_2 has elements (s, a) where s is a spectrum and a is a time-point. A DVSP is an ordered collection {(s_1, a_1), ..., (s_N, a_N)} representing spectral snapshots at successive times. Arranged as an array (Figure 4.5), it approximates a continuous "relief map" characterizing a sound's developing spectral signature (Lewin, pp. 115-117).

# Prerequisites
- **Timbral GIS** — The spectral GIS component
- **Direct-Product GIS** — DVSP lives in a direct product of spectral and temporal GIS

# Key Properties
1. Elements are (spectrum, time-point) pairs in a direct-product GIS
2. Array representation: rows = time points, columns = partial numbers, entries = power values
3. Approximates a continuous "relief map" of spectral evolution
4. The unfolding interval vector of a DVSP tracks spectral-temporal interval accumulation
5. Time points should be dense enough to catch salient spectral features

# Examples
**Example 1** (Figure 4.5): N = 5 snapshots of an 8-partial spectrum, forming a 5 x 8 array of power values that sketches the sound's timbral evolution.

**Example 2** (p. 116): "Lexicon of Analyzed Tones" (Moorer and Grey, Computer Music Journal) uses this representation for violin, clarinet, oboe, and trumpet tones.

# Relationships
## Builds Upon
- **Timbral GIS** — the spectral component
- **Direct-Product GIS** — the construction method

# Common Confusions
- **Confusion**: Thinking DVSP captures all aspects of a sound
  **Clarification**: DVSP captures only the developing spectral profile (partial powers over time), not pitch, loudness, or spatial position

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Figure 4.5 and discussion, pp. 115-117.

# Verification Notes
- Definition source: direct from Section 4.2
- Confidence rationale: high — explicit construction with figure
- Re-extraction notes: Re-extracted from v2 card; preserved: relief map interpretation, Moorer/Grey reference, unfolding interval vector connection
