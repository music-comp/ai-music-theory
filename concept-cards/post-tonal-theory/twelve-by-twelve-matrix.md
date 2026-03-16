---
concept: "12x12 Matrix"
slug: twelve-by-twelve-matrix
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
  - "twelve-tone matrix"
  - "row matrix"
  - "magic square"
prerequisites:
  - series-class
  - prime-ordering
  - inversion-twelve-tone
extends: []
related:
  - matrix-construction
  - twelve-counting
  - forty-eight-series-forms
contrasts_with: []
answers_questions:
  - "How do I construct a 12x12 matrix?"
  - "How does a twelve-tone matrix organize series forms?"
  - "How do I read P, R, I, and RI forms from a matrix?"
---

# Quick Definition
A 12x12 matrix is a grid displaying all forty-eight forms of a twelve-tone series: P-forms read left-to-right, R-forms right-to-left, I-forms top-to-bottom, and RI-forms bottom-to-top.

# Core Definition
"The simplest way of all... is to construct what is known as a *12 x 12 matrix*" (Straus, p. 317). The matrix contains the entire series class: "an entire small, coherent family of forty-eight closely related series forms." Reading directions: rows left-to-right = P-forms; rows right-to-left = R-forms; columns top-to-bottom = I-forms; columns bottom-to-top = RI-forms. "The same matrix can be written using letter names instead of pitch-class integers" (p. 317).

# Prerequisites
- **Series class** -- the matrix displays the complete series class
- **Prime ordering / Inversion** -- needed to construct the matrix

# Key Properties
1. Contains all 48 series forms in a single 12x12 grid
2. Each row and each column contains all 12 pitch classes exactly once
3. P0 occupies the top row; I0 occupies the first column
4. The first element of each row identifies the P-form number
5. The first element of each column identifies the I-form number
6. Can use pitch-class integers or letter names

# Construction / Recognition
1. Write P0 horizontally across the top row
2. Write I0 vertically down the left column
3. Fill remaining rows as P-forms, each starting on the pitch class in the first column
4. Verify: each row and column contains all 12 pcs exactly once

# Context & Application
The matrix is the essential reference tool for twelve-tone analysis. It makes twelve-counting efficient by allowing quick lookup of any series form. It also reveals relationships between forms (shared starting/ending notes, combinatorial pairings) at a glance. "All of the essential pitch material in a twelve-tone piece is normally drawn from among those forty-eight forms" (p. 318).

# Examples
**Example 1** (pp. 317--318, Ex. 6-10): Schoenberg, String Quartet No. 4 -- complete matrix with P labeled on left, R on right, I on top, RI on bottom:
```
     I
     0  11  7  8  3  1  2  10  6  5  4  9
P  1   0  8  9  4  2  3  11  7  6  5  10  R
   5   4  0  1  8  6  7   3 11 10  9   2
   ...
     RI
```

# Relationships
## Builds Upon
- **Series class** -- the matrix represents the complete class

## Enables
- **Twelve-counting** -- matrix is the reference for identifying forms
- **Combinatoriality analysis** -- matrix reveals hexachordal content relationships

## Related
- **Matrix construction** -- the step-by-step building process

# Common Errors
- Incorrect construction order (must start with P0 across top and I0 down left side)
- Reading R-forms or RI-forms in the wrong direction
- Forgetting that R_n and RI_n end on (not begin on) pitch-class n

# Common Confusions
- **Matrix entries are pitch-class integers**, not order positions
- **P0 vs. the first statement**: P0 is the form starting on pc 0, which may not be the first statement in the piece (e.g., Schoenberg's Quartet begins with P2, not P0)

# Source Reference
Chapter 6, Section 6.2.8, pp. 317--318

# Verification Notes
Preserved from old card: construction steps, reading directions, Schoenberg matrix example. Added: v3 template, direct quotations, emphasis on P0 vs. first statement distinction.
