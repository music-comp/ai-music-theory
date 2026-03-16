---
concept: Row Matrix
slug: row-matrix
category: analysis
subcategory: twelve-tone-theory
tier: advanced
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Basics of Twelve-Tone Theory"
chapter_number: 9
pdf_page: null
section: "IX.1"
extraction_confidence: high
aliases:
  - "twelve-tone matrix"
  - "magic square"
prerequisites:
  - row-class
  - row-operations
extends: []
related:
  - row-naming-conventions
contrasts_with: []
answers_questions:
  - "What is a row matrix?"
  - "How do you read P, R, I, and RI forms from the matrix?"
  - "How do you construct a row matrix?"
---

# Quick Definition
A row matrix is a 12-by-12 grid that compactly displays all 48 forms of a row class. By convention, P forms read left to right across rows, R forms read right to left, I forms read top to bottom down columns, and RI forms read bottom to top.

# Core Definition
The matrix is constructed by placing P0 along the top row (left to right) and I0 along the first column (top to bottom). Each subsequent row is the P form that starts on the pitch class in the first column. The result: every row (left to right) is a P form; every row (right to left) is an R form; every column (top to bottom) is an I form; every column (bottom to top) is an RI form. Row form labels appear at the edges: P labels on the left, R labels on the right, I labels at top, RI labels at bottom.

# Prerequisites
- Row class and row operations (understanding the 48 forms)

# Key Properties
1. 12x12 grid containing all 48 row forms
2. P forms: left to right; R forms: right to left
3. I forms: top to bottom; RI forms: bottom to top
4. P0 occupies the top row; I0 occupies the first column
5. The main diagonal always contains the same integer (0 in fixed-zero convention)
6. Three matrix types exist depending on labeling convention (see row-naming-conventions)

# Context & Application
The matrix is the primary analytical tool for twelve-tone music, allowing analysts to quickly look up any row form. It also reveals invariance relationships and combinatorial properties. Multiple labeling conventions exist (fixed zero, moveable zero, hybrid).

# Examples
**Example 1** (Lutyens, Motet Op. 27): P0 = 0-11-3-7-8-4-2-6-5-1-9-10. I0 runs down the first column: 0-1-9-5-4-8-10-6-7-11-3-2.

**Example 2**: Reading the matrix -- to find R10, locate the row ending in 10 and read right to left. To find I5, locate the column labeled I5 and read top to bottom.

# Relationships
## Builds Upon
- **row-class** -- The matrix displays all forms of a row class
- **row-operations** -- P, R, I, RI reading directions
## Related
- **row-naming-conventions** -- Different conventions affect matrix labeling

# Common Confusions
- **Confusion**: P0 must always start on C
  **Clarification**: Only in fixed-zero convention; moveable-zero lets P0 start anywhere
- **Confusion**: The matrix prescribes the order of row usage
  **Clarification**: It is a reference tool, not a compositional plan

# Source Reference
Open Music Theory, Part IX, Chapters 1-2: "Basics of Twelve-Tone Theory" and "Naming Conventions for Rows."

# Verification Notes
- Definition source: From 09-01 and 09-02 source chapters
- Confidence rationale: High -- multiple matrix examples in source
- Preserved from v2: Lutyens matrix example, reading directions
- Cross-reference status: Verified against three matrix type descriptions
