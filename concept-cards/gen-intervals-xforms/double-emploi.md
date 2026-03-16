---
# === CORE IDENTIFICATION ===
concept: Double Emploi
slug: double-emploi

# === CLASSIFICATION ===
category: transformation-theory
subcategory: harmonic-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-network-definition
extends: []
related:
  - rel-transformation
  - beethoven-appassionata-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is double emploi in Lewin's network formalism?"
  - "How is harmonic ambiguity modeled in transformation networks?"
---

# Quick Definition
Rameau's concept of a chord having two simultaneous functional interpretations, modeled in transformation networks as a single sonority represented by two different Klang nodes connected by a one-way transformation arrow (typically REL).

# Core Definition
Double emploi occurs when a single sounding sonority is modeled by two different Klangs in a network. The sonority is connected by a transformation arrow to both interpretations; both are analytically valid. In the Beethoven Appassionata analysis, the fourth sonority is understood both as Gb-major with added sixth and as eb-minor with minor seventh. The arrow goes only one way (from Gb to eb via REL), reflecting functional priority. "This is Rameau's double emploi" (Lewin, Section 9.7.6, p. 245).

# Prerequisites
- **Transformation network** — the formalism in which double emploi is modeled

# Key Properties
1. A single sounding event modeled by two Klang nodes
2. Connected by a transformation (typically REL: relative major/minor)
3. The arrow direction reflects analytical priority
4. Both interpretations are simultaneously valid
5. Originates with Rameau's harmonic theory

# Construction / Recognition
## To Construct:
1. Identify a sonority with dual harmonic interpretation
2. Create two Klang nodes for the two interpretations
3. Connect with the appropriate transformation arrow
4. Assign arrow direction based on functional priority
## To Recognize:
1. Look for a single sonority occupying two node positions in a network
2. Check for a one-way transformation arrow between them

# Context & Application
Double emploi represents moments where harmonic function is ambiguous or dual. In transformational network terms, it manifests as two nodes for one event, creating an asymmetry that can affect the graph's input/output structure and precedence ordering.

# Examples
**Example 1** (Figure 9.14, pp. 244-245): Beethoven Appassionata slow movement. The fourth sonority (Gb-Bb-Db-Eb) is both (Gb,+) with added sixth and (eb,-) with minor seventh inverted. REL(Gb,+) = (eb,-). The arrow goes from Gb to eb only.

# Relationships
## Builds Upon
- **Transformation network** — the formalism
## Related
- **REL transformation** — the typical transformation connecting the two interpretations
- **Beethoven Appassionata analysis** — the primary example

# Common Errors
- **Error**: Treating double emploi as sequential reinterpretation
  **Correction**: Both interpretations are simultaneous, not sequential

# Common Confusions
- **Confusion**: Thinking double emploi is about enharmonic spelling
  **Clarification**: It is about functional interpretation, not notation

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, p. 245. See Figure 9.14.

# Verification Notes
- Definition source: direct from Section 9.7.6 discussion
- Confidence rationale: high -- explicitly named with "This is Rameau's double emploi"
- Re-extracted from v2 card; preserved: Appassionata sonority detail, REL arrow direction, Rameau attribution
