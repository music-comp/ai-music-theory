---
concept: Timbral GIS
slug: timbral-gis

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
  - "GIS 4.2.1"
  - "spectral GIS"
  - "harmonic-spectrum GIS"

prerequisites:
  - generalized-interval-system
  - direct-product-gis
extends:
  - direct-product-gis
related:
  - rational-spectra-gis
  - developing-spectra
contrasts_with: []

answers_questions:
  - "How can timbral relationships be modeled as a GIS?"
---

# Quick Definition
A timbral GIS models harmonic spectra as elements of a space, with intervals measuring how one spectrum transforms into another via spectral filters or amplitude scaling. The space consists of positive-real tuples representing partial powers, with componentwise multiplicative intervals.

# Core Definition
Example 4.2.1: Let s = (s(1), s(3), s(5)) denote the class of harmonic sounds whose first, third, and fifth partials have respective powers s(1), s(3), s(5). Let i = (i(1), i(3), i(5)) denote a "device" class that multiplies partial powers by factors i(1), i(3), i(5). The GIS has S = all positive-real triples, IVLS = all positive-real triples under componentwise multiplication, int(s, t) = (t(1)/s(1), t(3)/s(3), t(5)/s(5)). This is a commutative GIS (Lewin, Example 4.2.1, pp. 114-115).

# Prerequisites
- **Generalized Interval System** — The GIS framework
- **Direct-Product GIS** — Timbral GIS is a direct product of copies of (R+, *)

# Key Properties
1. Space S: positive-real N-tuples representing partial powers
2. IVLS: positive-real N-tuples under componentwise multiplication
3. int(s, t): componentwise quotient t(n)/s(n)
4. Commutative GIS (componentwise multiplication is commutative)
5. Variations: can consider different partial sets (#1-3-5, #1-2-4, #1-8, etc.)
6. The interval i = int(s, t) means: passing s through device-class i produces t

# Construction / Recognition
## To Construct:
1. Choose which partials to model (e.g., #1, #3, #5)
2. S = all positive-real tuples of that dimension
3. IVLS = same set under componentwise multiplication
4. int = componentwise quotient

# Context & Application
This GIS abstracts timbral relationships by focusing on spectral content. Fundamental frequencies are irrelevant — only partial power ratios matter. The "interval" between timbres is a scaling pattern, and devices (filters, processors) represent intervals. Concatenating devices corresponds to multiplying intervals.

# Examples
**Example 1** (p. 115): int((2, 1, 0.5), (4, 3, 0.5)) = (2, 3, 1) — t has 2x power at partial 1, 3x at partial 3, same at partial 5.

**Example 2** (p. 115): Device concatenation: i = (2, 3, 1) followed by j = (1, 0.5, 2) yields ij = (2, 1.5, 2).

# Relationships
## Builds Upon
- **Direct-Product GIS** — timbral GIS is a direct product of (R+, *) factors

## Enables
- **Developing Spectra** — combining spectral GIS with time-point GIS
- **Rational Spectra GIS** — a more sophisticated timbral GIS using rational functions

# Common Errors
- **Error**: Confusing partial power with amplitude
  **Correction**: Values represent powers (energy), related to amplitude squared

# Common Confusions
- **Confusion**: Thinking fundamental frequency matters
  **Clarification**: The GIS models spectral shape only; two sounds at different pitches can have the same "position" in S

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.2.1, pp. 114-115.

# Verification Notes
- Definition source: direct from Example 4.2.1
- Confidence rationale: high — explicit example
- Re-extraction notes: Re-extracted from v2 card; preserved: device interpretation, concatenation example, power vs amplitude note
