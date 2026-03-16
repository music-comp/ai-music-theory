---
concept: Series Form Identification
slug: series-form-identification
category: analysis
subcategory: null
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 319
section: "6.2.9 Twelve-count"
extraction_confidence: high
aliases:
  - "identifying series forms"
  - "row identification"
prerequisites:
  - twelve-counting
  - twelve-by-twelve-matrix
  - index-number-twelve-tone
extends: []
related:
  - series-class
  - ordered-pitch-class-intervals
contrasts_with: []
answers_questions:
  - "How do I identify which series form is being used in a passage?"
  - "How can intervallic analysis identify a series form without a matrix?"
---

# Quick Definition
Series form identification is the process of determining which of the 48 forms is being used at a given point, using either matrix lookup or intervallic comparison with a known form.

# Core Definition
Two methods exist for identifying series forms: (1) construct a 12x12 matrix and look up the first few notes; (2) "more directly by applying our knowledge of the intervallic relationships between series forms" (Straus, p. 320). The intervallic method compares the unknown passage's ordered pc intervals to those of a known form, then calculates the index number to determine the specific transposition level.

# Prerequisites
- **Twelve-counting** -- the overall analytical process
- **12x12 matrix** -- reference tool for lookup method
- **Index number** -- needed for intervallic method

# Key Properties
1. Same intervals, same order = prime-related (another P or same-type form)
2. Complementary intervals, same order = I-related
3. Same intervals, reversed = RI-related
4. Complementary intervals, reversed = R-related
5. Index number: sum of corresponding pitch classes between related forms

# Construction / Recognition
**Intervallic method (demonstrated for Webern, "Wie bin ich froh!"):**
1. Known: P7 has certain ordered pc intervals
2. Accompaniment intervals match P7's final intervals in reverse = RI-form
3. Calculate index: first note of unknown + last note of known, etc.
4. All sums = 2 (mod 12), so index number = 2
5. Since P7 + RI_x must sum to 2, and 7 + 7 = 14 = 2, the form is RI7

# Context & Application
Series form identification is the first step in twelve-tone analysis. The intervallic method is more efficient than matrix lookup for experienced analysts. It reveals how composers create relationships between successive series forms.

# Examples
**Example 1** (pp. 319--322): Webern, "Wie bin ich froh!" -- accompaniment identified as RI7 through intervallic comparison with P7. Index number 2 confirms: 6+8 = 5+9 = 2 (mod 12).

# Relationships
## Builds Upon
- **Twelve-counting** -- identification is part of the counting process
- **Index number** -- used to calculate specific transposition

## Enables
- **Invariant analysis** -- requires knowing which forms are in use
- **Area analysis** -- requires identifying all series forms in a passage

# Common Errors
- Miscalculating the index number when sums exceed 11
- Forgetting that R_n ends on pc n, not begins
- Assuming the first note heard is the first note of the series form

# Common Confusions
- **Matrix lookup vs. intervallic method**: Both yield the same result; the intervallic method is faster but requires understanding of interval relationships

# Source Reference
Chapter 6, Section 6.2.9, pp. 319--322

# Verification Notes
Preserved from old card: both identification methods, Webern index calculation. Added: v3 template, step-by-step intervallic method example.
