---
# === CORE IDENTIFICATION ===
concept: Tempo Consonance
slug: tempo-consonance

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: time-span-gis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 99
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: medium

# === VARIANTS ===
aliases:
  - "tempo-consonance metaphor"
  - "tempo as pitch analogy"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span
extends: []
related:
  - interval-function-computation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can tempo ratios be understood by analogy with pitch intervals?"
  - "What makes some polyrhythmic relationships simpler than others?"
  - "How does the pitch representation of tempi reveal their 'harmony'?"
---

# Quick Definition
Tempo consonance refers to the quality of simplicity or complexity in the numerical ratio between two tempi. Tempi in simple ratios (2:1, 3:2) are "consonant" while complex ratios create "dissonance," analogous to frequency ratios in pitch.

# Core Definition
When two local time units have durations in ratio p:q (or equivalently tempi in ratio q:p), the simplicity of this ratio can be characterized as consonant or dissonant by analogy with frequency ratios in pitch. Lewin represents tempi as pitches with corresponding frequency ratios (Figure 4.2b, pp. 99-101), so that the "harmony" of the resulting pitches reveals the "consonance" of the tempo relationships. The approach follows Zarlino's valuation of the variety of consonances rather than Rameau's hierarchical approach with a single referential root.

# Prerequisites
- **Time span** — The temporal objects whose tempi are compared

# Key Properties
1. Tempo ratio = duration ratio inverted (faster tempo = shorter duration)
2. Simple ratios (2:1, 3:2, 4:3) correspond to consonant pitch intervals (octave, fifth, fourth)
3. Complex ratios correspond to dissonant pitch intervals
4. Irrational ratios (e.g., pi:1) would be maximally "dissonant"
5. The analogy is metaphorical, not a claim about perceptual equivalence

# Construction / Recognition
## To Construct:
1. Collect metronome markings from a passage
2. Compute pairwise ratios
3. Represent ratios as pitch intervals (frequency ratios)
4. Assess the resulting "harmony"
## To Recognize:
1. A passage where multiple tempi sound naturally related (simple ratios)
2. A network of tempo relationships that can be described as "consonant" or "dissonant"

# Context & Application
The tempo-consonance metaphor helps musicians and analysts grasp complex tempo relationships intuitively, plan ensemble coordination, understand why some polyrhythms are harder than others, and appreciate the compositional logic of tempo structures. It is a practical aid for performers, not a claim about perceptual mechanism.

# Examples
**Example 1** (Figures 4.2a-b, pp. 99-101): Carter String Quartet No. 1, mm. 22-32. The tempo network:
- MM48:MM96 = 1:2 ("octave" -- maximally consonant)
- MM36:MM48 = 3:4 ("fourth")
- MM120:MM160 = 3:4 ("fourth")
- MM90:MM180 = 1:2 ("octave")

The tempi form a "Db major with added sixth and major seventh" chord when represented as pitches -- a striking consonance.

**Example 2** (p. 101): Representing MM180 as the highest pitch (high C) and computing all other tempi as pitches in frequency ratios to high C. MM120 = 2/3 of MM180, so it maps to a pitch a perfect fifth below high C.

**Example 3** (p. 102): Nancarrow's irrational tempo ratios (e.g., pi:1) would be maximally "dissonant" -- incommensurable, like irrational pitch intervals.

# Relationships
## Builds Upon
- **Time span** — The temporal objects whose tempi are compared
## Enables
- **Understanding of Carter's tempo structures** — The consonance metaphor illuminates Carter's compositional practice
## Related
- **Interval function computation** — Tempo ratios appear as the second component of time-span intervals

# Common Errors
- **Error**: Claiming that tempo consonance is literally perceived like pitch consonance
  **Correction**: The metaphor is an analytical and practical aid; we do not literally hear tempo ratios the way we hear frequency ratios

# Common Confusions
- **Confusion**: The approach implies a single "root" tempo analogous to a tonal root
  **Clarification**: Lewin explicitly follows Zarlino's approach (variety of consonances) rather than Rameau's (hierarchy under a root); the network of ratios matters, not a single referential tempo
- **Confusion**: The pitch representation in Figure 4.2b is a theoretical claim about perception
  **Clarification**: It is a practical device to help musicians intuit the numerical relationships among tempi

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models, Figures 4.2a-b and discussion, pages 99-102.

# Verification Notes
- Definition source: Synthesized from Lewin's analytical discussion
- Confidence rationale: Medium -- concept is metaphorical rather than formally defined; relies on analytical context
- Re-extraction notes: Re-extracted from v2 card; preserved: Carter example details, Zarlino/Rameau distinction, Nancarrow reference, Db major chord observation
