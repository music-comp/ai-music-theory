---
concept: Addition Table
slug: addition-table
category: operations
subcategory: inversion properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 120
section: "3.3.1 Calculating common tones under inversion"
extraction_confidence: high
aliases:
  - "sum table"
  - "index number table"
prerequisites:
  - index-number
  - common-tones-under-inversion
  - mod-12-arithmetic
extends:
  - common-tones-under-inversion
related:
  - inversional-symmetry
contrasts_with: []
answers_questions:
  - "How do I systematically calculate common tones under inversion?"
  - "What is an addition table in set theory?"
---

# Quick Definition
A tabular method for computing all pairwise sums (mod 12) of pitch classes in a set, used to determine the number and identity of common tones under inversion at any index number.

# Core Definition
An addition table is constructed by writing a pitch-class set along both axes of a grid, then filling each cell with the sum (mod 12) of the row and column values. The table systematically performs all additions: it adds each note twice to every other note and once to itself (on the diagonal). Each occurrence of a number n in the table represents one common tone at In. The table reveals both how many tones are held in common and which specific pitch classes are involved: the sum n at the intersection of x and y means both x and y are common tones at In (Straus, pp. 120-121).

# Prerequisites
- **Index number** -- the sums computed in the table
- **Common tones under inversion** -- the concept the table serves
- **Mod-12 arithmetic** -- all sums are computed modulo 12

# Key Properties
1. The table is symmetric around the main diagonal (cell [i,j] = cell [j,i])
2. Diagonal entries represent notes added to themselves (2x mod 12)
3. Off-diagonal entries represent sums of two different notes
4. Each entry in the table = 1 common tone at that index number
5. The table simultaneously shows count and identity of common tones

# Construction / Recognition
To construct for set {a, b, c, d}:
1. Write {a, b, c, d} across the top row
2. Write {a, b, c, d} down the left column
3. Fill each cell with (row + column) mod 12
4. Count occurrences of each value 0-11 in the completed table

To read results:
- n appears k times -> k common tones at In
- The location shows which notes: sum n at intersection of x and y means x and y map onto each other at In

# Context & Application
The addition table is the standard computational tool for determining common tones under inversion. It is more efficient than calculating each of the 12 inversions separately, especially for larger sets. It also reveals inversional symmetry: if a sum n appears as many times as the set has notes, the set maps entirely onto itself at In.

# Examples
**Example 1** (p. 120, Ex. 3-11): Addition table for [3, 4, 7, 8]:

|   | 3  | 4  | 7  | 8  |
|---|----|----|----|----|
| 3 | 6  | 7  | 10 | 11 |
| 4 | 7  | 8  | 11 | 0  |
| 7 | 10 | 11 | 2  | 3  |
| 8 | 11 | 0  | 3  | 4  |

Results: 11 appears 4 times (4 common tones at I11); 7 appears 2 times (2 common tones at I7, specifically notes 3 and 4); 3 appears 2 times (2 common tones at I3); etc.

The intersection tells us which notes: 10 at intersection of 3 and 7 means both 3 and 7 are common tones at I10.

# Relationships
## Builds Upon
- **Common tones under inversion** -- the table is the computational method

## Enables
- **Inversional symmetry** detection -- when a sum equals the set's cardinality in frequency

## Related
- **Index number** -- the values computed in the table

# Common Errors
- Forgetting to use mod 12 for sums greater than 11
- Miscounting diagonal entries (each counts once, not twice)

# Common Confusions
- Addition tables (for inversion/sums) are different from subtraction or difference tables (for transposition/intervals)
- The table is symmetric, so you only need to fill half and mirror, but counting requires the full table

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.3.1, pp. 120-121

# Verification Notes
Upgraded from old v2 card. Preserved complete worked example with table for [3, 4, 7, 8] and interpretation. Added explicit construction steps and v3 fields.
