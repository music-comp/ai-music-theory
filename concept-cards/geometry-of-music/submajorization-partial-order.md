---
# === CORE IDENTIFICATION ===
concept: Submajorization Partial Order
slug: submajorization-partial-order

# === CLASSIFICATION ===
category: geometric-theory
subcategory: voice-leading-metrics
tier: advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Appendix A: Measuring Voice-Leading Size"
chapter_number: appendix-a
pdf_page: 415
section: "Appendix A"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "voice-leading partial order"
  - "no-crossings principle"
  - "reasonable metrics"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading-fundamentals
extends: []
related:
  - near-evenness
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How should we measure voice-leading size?"
  - "What makes a voice-leading metric 'reasonable'?"
  - "Can we compare voice leadings without choosing a specific metric?"
---

# Quick Definition
A mathematical framework for comparing voice-leading sizes without committing to any single metric, based on the principle that voice crossings should never make a voice leading smaller, providing a "zone of agreement" among all reasonable measures.

# Core Definition
Multiple reasonable methods exist for measuring voice-leading size (largest-distance metric, taxicab metric, Euclidean metric), with no principled way to choose among them. The submajorization partial order resolves this by identifying what all reasonable metrics must agree about: any metric is acceptable as long as voice crossings do not reduce voice-leading size. Formally, the collection {x1 + c, x2, ..., xn} must be at least as large as {x1, x2 + c, ..., xn} whenever x1 > x2 and c >= 0 -- meaning the metric should not prefer uneven distributions of distances. This determines a partial order that compares many (but not all) voice leadings. Importantly, reasonable metrics all agree that more-even chords have smaller voice leadings to their transpositions, and that inversionally symmetrical voice leadings are particularly small.

# Prerequisites
- Basic understanding of voice leading

# Key Properties
1. No single voice-leading metric is uniquely correct
2. The "no-crossings" principle: voice crossings never make voice leadings smaller
3. Determines a partial order (can compare some but not all voice leadings)
4. All reasonable metrics agree about major-third and minor-third systems
5. Geometrically: bouncing off the mirror boundary never shortens a path
6. Originates in early twentieth-century economics
7. Has an extraordinarily wide range of applications

# Construction / Recognition
## To Construct/Create:
1. Compare two voice leadings by checking whether one's sorted distances are dominated by the other's
2. If the largest distance in A <= largest in B, and sum of two largest in A <= sum of two largest in B, etc., then A is no larger than B
## To Identify/Recognize:
1. Voice leadings that differ dramatically in size can be compared by any reasonable metric
2. Voice leadings of similar size may fall in the "zone of acceptable disagreement"

# Context & Application
This framework justifies making voice-leading comparisons throughout the book without committing to a specific metric. It validates claims about the major-third system, minor-third system, and the relationship between evenness and efficient voice leading.

# Examples
**Example 1** (p. 417, Fig. A2): The Mobius strip showing the geometric meaning of the partial order -- the inner diamond contains voice leadings smaller than (C,G)->(D,F) for any reasonable metric; the outer white space contains those larger.

# Relationships
## Builds Upon
- Basic voice-leading concepts
## Enables
- Metric-independent claims about voice-leading efficiency
## Related
- **near-evenness** -- Nearly even chords have small voice leadings by any reasonable metric

# Common Errors
- **Error**: Assuming one must choose a specific voice-leading metric
  **Correction**: The submajorization partial order allows many claims without choosing

# Source Reference
Appendix A: Measuring Voice-Leading Size, pages 415-418.

# Verification Notes
- Definition source: Formally defined in Appendix A
- Confidence rationale: High -- rigorous mathematical treatment
- Cross-reference status: Supports claims throughout the book
