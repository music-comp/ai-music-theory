---
# === CORE IDENTIFICATION ===
concept: Rational Spectra GIS
slug: rational-spectra-gis

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: timbral-gis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "GIS 4.2.2"
  - "linear filter GIS"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - timbral-gis
extends:
  - timbral-gis
related: []
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can linear filter classes be modeled as a GIS?"
---

# Quick Definition
The rational spectra GIS models linear filter classes as elements, where each "rational spectrum" is a positive rational function of frequency, and intervals represent the filter needed to transform one spectrum into another.

# Core Definition
Example 4.2.2: A "rational spectrum" is a function s(x) = p(x)/q(x) (ratio of polynomials) that is strictly positive for all x in [LO, HI]. S = all rational spectra; IVLS = same family under pointwise multiplication; int(s, t) = t/s (pointwise quotient). This is a commutative GIS. Each rational spectrum corresponds to a class of linear filters (built from all-zero and all-pole components) that multiply the power at frequency x by s(x) (Lewin, Example 4.2.2, pp. 117-118).

# Prerequisites
- **Generalized Interval System** — The GIS framework
- **Timbral GIS** — The simpler discrete-partial timbral GIS of Example 4.2.1

# Key Properties
1. Elements are positive rational functions of frequency
2. IVLS is the same set under pointwise multiplication
3. int(s, t) = t/s (pointwise quotient)
4. Commutative GIS
5. Models linear filter classes (FIR and IIR filters)
6. The frequency range [LO, HI] can be varied

# Examples
**Example 1** (p. 117): If s is a current spectrum and t is a desired spectrum, int(s, t) = t/s is the filter needed to transform s into t.

**Example 2** (p. 118): Filter concatenation: filter s followed by filter t = filter st (pointwise product).

# Relationships
## Builds Upon
- **Timbral GIS** — extends from discrete partials to continuous frequency spectra

## Related
- Computer music techniques described in Cann's "Analysis/Synthesis Tutorial"

# Common Confusions
- **Confusion**: Thinking elements are numbers
  **Clarification**: Each element is an entire function of frequency — a frequency-response curve

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.2.2, pp. 117-118.

# Verification Notes
- Definition source: direct from Example 4.2.2
- Confidence rationale: high — explicit example
- Re-extraction notes: Re-extracted from v2 card; preserved: filter interpretation, concatenation example, frequency range variability
