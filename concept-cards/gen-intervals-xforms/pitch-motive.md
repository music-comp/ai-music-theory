---
# === CORE IDENTIFICATION ===
concept: Pitch Motive
slug: pitch-motive

# === CLASSIFICATION ===
category: analytical-applications
subcategory: motivic-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (4): Some Further Analyses"
chapter_number: 10
pdf_page: 251
section: "10.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "PM"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - rich-transformation
  - tch-transformation
extends: []
related:
  - durational-motive
  - mozart-k550-development-analysis
  - ri-chaining
contrasts_with:
  - durational-motive

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a pitch motive in Lewin's transformational analysis?"
  - "How does RICH operate on pitch motives?"
---

# Quick Definition
A pitch motive (PM) is a specific recurring pitch pattern -- an operand of transformational operations such as RICH and TCH -- that serves as a fundamental unit of motivic analysis within transformation networks.

# Core Definition
In the context of Lewin's transformational analysis, a pitch motive is a fixed sequence of pitches subjected to serial transformations including transposition, inversion, retrograde, and retrograde-inversion (RICH). The PM maintains its intervallic identity under transformation while appearing in different pitch-class contexts. RICH chains PM-forms in sequence; TCH measures the transposition interval between successive forms (Lewin, Section 10.1, pp. 251-253).

# Prerequisites
- **RICH transformation** — the primary chaining operation on PM
- **TCH transformation** — measures distance between successive PM forms

# Key Properties
1. PM is an operand for RICH, TCH, and other serial transformations
2. PM maintains intervallic identity across transformations
3. RICH-chaining produces a sequence of PM-forms
4. TCH interval measures the transposition between adjacent forms
5. PM is paired with DM (durational motive) in the Mozart analysis

# Construction / Recognition
## To Construct:
1. Identify a recurring pitch pattern in the music
2. Describe it by its intervallic content
3. Trace its transformations through RICH, TCH, etc.
## To Recognize:
1. Look for recurring intervallic patterns subjected to serial transformations
2. Identify RICH-chaining: the end of one form overlaps the beginning of the next

# Context & Application
Pitch motives are the primary melodic building blocks in Lewin's transformational analyses. They allow tracing how a composer develops a basic idea throughout a composition, particularly in development sections where motivic transformation is paramount.

# Examples
**Example 1** (Figure 10.1, pp. 251-253): In Mozart K.550 (last movement, measures 127-133), PM comprises "a diminished fourth up followed by a diminished seventh down." The PM-forms chain via RICH with TCH interval of falling fourth: (E-Ab-B), (Ab-B-Eb), (B-Eb-F#), (Eb-F#-Bb), (F#-Bb-C#), (Bb-C#-F), (C#-F-G#).

# Relationships
## Builds Upon
- **RICH transformation** — chains PM-forms
- **TCH transformation** — measures inter-form transposition
## Related
- **Durational motive** — the rhythmic counterpart to PM
- **Mozart K.550 development analysis** — the primary analytical context
- **RI-chaining** — the general technique PM exemplifies
## Contrasts With
- **Durational motive** — PM operates on pitch; DM operates on durations

# Common Errors
- **Error**: Treating PM as merely a memorable melodic fragment
  **Correction**: PM is specifically a transformational operand, analyzed for its behavior under RICH, TCH, and other operations

# Common Confusions
- **Confusion**: Confusing PM with a theme or subject
  **Clarification**: PM is a precise intervallic pattern, not necessarily a complete musical phrase; it is defined by its transformation-theoretic role

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Section 10.1, pp. 251-253. See Figure 10.1.

# Verification Notes
- Definition source: direct from Section 10.1 discussion
- Confidence rationale: high -- explicitly named and analyzed in detail
- Re-extracted from v2 card; preserved: PM chain forms, TCH interval specification
