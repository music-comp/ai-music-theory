---
concept: Dispersive Interval
slug: dispersive-interval

category: transformation-theory
subcategory: intervallic-analysis
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
section: "Section 7.2"

extraction_confidence: high

aliases:
  - "dispersive transformation"

prerequisites:
  - inj-function
  - transposition
  - pitch-class-set
extends: []
related:
  - tch-transformation
  - ri-chaining
  - structural-sequencing
contrasts_with:
  - progressive-transformation
  - internal-transformation

answers_questions:
  - "What makes an interval 'dispersive' for a given set?"
  - "How do dispersive intervals relate to RI-chaining and structural sequencing?"
  - "How does a dispersive interval help fill chromatic space efficiently?"
---

# Quick Definition
An interval i is dispersive for a set X when the transposition T_i maps X to a set sharing no common elements with X: INJ(X, X)(T_i) = 0, equivalently T_i(X) intersect X = empty set.

# Core Definition
The concept arises in the broader framework of Section 6.4, where Lewin defines: "a dispersive transformation [is] one that maps X largely outside Y, making the value of INJ(X, Y)(f) minimal or relatively small." In Section 7.2, the specific case where X = Y and f = T_i is applied: "The sequencing-interval of 10 in this particular case is a dispersive interval for Z as an unordered set: INJ(Z, Z)(T_10) = 0; T_10(Z) has no common notes with Z" (Lewin, Section 7.2, p. 188).

# Prerequisites
- **INJ function** — Used to measure how many elements of X map into X under T_i
- **Transposition** — The operation T_i whose dispersive property is being tested
- **Pitch-class set** — The set X with respect to which dispersiveness is evaluated

# Key Properties
1. Dispersive: INJ(X, X)(T_i) = 0, meaning T_i(X) and X share no elements
2. Dispersiveness is relative to a specific set X; an interval may be dispersive for one set but not another
3. A dispersive interval need not be "large" -- small intervals can be dispersive for certain sets
4. When a motive has a dispersive TCH-interval, RI-chaining fills pitch-class space without repetition
5. The complementary concept: maximally similar intervals where INJ(X, X)(T_i) = |X|

# Construction / Recognition
## To Construct:
1. Given a set X and an interval i, compute T_i(X)
2. Check whether T_i(X) intersect X = empty set
3. If so, i is dispersive for X
## To Recognize:
1. Successive transposed forms of a motive filling chromatic space without common tones
2. INJ(X, X)(T_i) = 0 for the transposition interval i

# Context & Application
Dispersive intervals are analytically powerful for understanding how motives fill chromatic space. When an RI-chain uses a motive with a dispersive TCH-interval, successive forms avoid pitch-class repetition, generating maximal pitch-class coverage. Lewin contrasts dispersive transformations with progressive ones (high INJ mapping X into Y) and internal ones (high INJ mapping X into X).

# Examples
**Example 1** (Section 7.2, Wagner Parsifal, Figures 7.2-7.4):
- The Zauber motive Z has TCH-interval 10
- Interval 10 is dispersive for Z as an unordered set: INJ(Z, Z)(T_10) = 0; T_10(Z) shares no notes with Z
- Z_3 = T_10(Z_1) and Z_4 = T_10(Z_2): the RI-chaining produces "structural sequencing"
- "The open noteheads of the figure up through the Ab of measure 1140 constitute a non-repeating ten-note series"
- "F# and B are the only missing pitch classes. (It is amusing, if far-fetched, to imagine them as representing the absent Klingsor.)"

# Relationships
## Builds Upon
- **INJ function** — INJ(X, X)(T_i) = 0 defines dispersiveness
## Enables
- **Structural sequencing** — Dispersive RI-chaining creates structural sequences
- **Chromatic saturation** — Dispersive intervals fill pitch-class space efficiently
## Related
- **TCH transformation** — The interval arising from RI-chaining technique
- **RI-chaining** — The serial technique that exploits dispersive intervals
## Contrasts With
- **Progressive transformation** — High INJ(X, Y)(f); maps X largely into Y
- **Internal transformation** — High INJ(X, X)(f); maps X largely into itself

# Common Errors
- **Error**: Assuming dispersiveness is an intrinsic property of an interval
  **Correction**: Dispersiveness is relative to a specific set X; interval 10 is dispersive for the Zauber motive but not necessarily for other sets

# Common Confusions
- **Confusion**: Thinking dispersive intervals must be large
  **Clarification**: A small interval (e.g., 1 or 2) can be dispersive for sets whose elements are spaced accordingly

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.2, pages 188-189.

# Verification Notes
- Definition source: Direct quotation from Section 7.2 and Section 6.4
- Confidence rationale: Explicitly defined with extended Parsifal analysis
- Re-extraction notes: Re-extracted from v2 card; preserved: Parsifal/Zauber analysis, Klingsor remark, contrast with progressive/internal
