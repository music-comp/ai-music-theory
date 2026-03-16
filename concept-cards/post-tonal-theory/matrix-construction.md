---
concept: Matrix Construction
slug: matrix-construction
category: twelve-tone
subcategory: matrix
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 317
section: "6.2.8 12x12 matrix"
extraction_confidence: high
aliases:
  - "building a matrix"
  - "constructing a row matrix"
prerequisites:
  - twelve-by-twelve-matrix
  - prime-ordering
  - inversion-twelve-tone
extends: []
related:
  - twelve-counting
  - series-class
contrasts_with: []
answers_questions:
  - "How do I construct a 12x12 matrix step by step?"
  - "How do I verify a matrix is correct?"
---

# Quick Definition
Matrix construction is the step-by-step process of building a 12x12 grid: write P0 across the top row, I0 down the first column, then fill each remaining row as a P-form starting on the pitch class in its first column.

# Core Definition
"To construct such a matrix, begin by writing P0 horizontally across the top and I0 vertically down the left side... Then write in the remaining prime orderings in the rows from left to right, beginning on whatever pitch class is in the first column" (Straus, p. 317).

# Prerequisites
- **12x12 matrix** -- understanding what the matrix represents
- **P0 and I0** -- needed as starting material

# Key Properties
1. P0 provides the interval succession for all rows
2. I0 determines the starting pitch class of each row
3. Each cell is determined by transposing the top row to start on the first-column pitch class
4. Each row and column must contain all 12 pcs exactly once (verification check)

# Construction / Recognition
**Step-by-step (Schoenberg, String Quartet No. 4):**
1. Normalize the series to start on 0: P0 = 0-11-7-8-3-1-2-10-6-5-4-9
2. Compute I0 by inverting each pc: I0 = 0-1-5-4-9-11-10-2-6-7-8-3
3. Row 2 starts on 1 (second element of I0), so fill in P1: 1-0-8-9-4-2-3-11-7-6-5-10
4. Row 3 starts on 5 (third element of I0), so fill in P5: 5-4-0-1-8-6-7-3-11-10-9-2
5. Continue for all 12 rows
6. Verify: every row and every column contains each of 0--11 exactly once

# Context & Application
Matrix construction is the essential preparatory step before analyzing any twelve-tone piece. It provides a complete reference for identifying all series forms during twelve-counting. The matrix can also be constructed with letter names for musicians who prefer that notation.

# Examples
**Example 1** (pp. 317--318, Exx. 6-9 and 6-10): Schoenberg, String Quartet No. 4 -- the text shows the construction in two stages: first P0 and I0 alone (Ex. 6-9), then the completed matrix (Ex. 6-10).

# Relationships
## Builds Upon
- **12x12 matrix** -- this is the procedure for building that matrix
- **Inversion** -- I0 is computed by inverting P0

## Enables
- **Twelve-counting** -- the matrix is the primary reference tool
- **Combinatoriality analysis** -- hexachordal relationships visible in the matrix

# Common Errors
- Computing I0 incorrectly: each element x of P0 becomes (12 - x) mod 12
- Starting with the original series form rather than normalizing to P0
- Filling rows by arbitrary transposition rather than using the first column as guide

# Common Confusions
- **Normalization**: The matrix always uses P0 (starting on pc 0), even if the piece begins with a different P-form
- **I0 computation**: I0 has the same first element as P0 (both start on 0); subsequent elements are complements

# Source Reference
Chapter 6, Section 6.2.8, pp. 317--318

# Verification Notes
Preserved from old card: step-by-step procedure, verification check. Added: v3 template, normalization emphasis, two-stage construction from source examples.
