---
concept: Addition Table
category: technique
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 120
unit: null
authors: Joseph N. Straus
---

# Quick Definition
A tabular method for calculating all possible sums of pitch-class pairs within a set, used to determine common tones under inversion at any index number.

# Formal Definition
An addition table is constructed by writing a pitch-class set along both the vertical and horizontal axes, then filling in all pairwise sums (mod 12) in the resulting grid. This systematically calculates every possible sum within the set:
- Entries on the diagonal represent notes added to themselves (2x mod 12)
- Off-diagonal entries represent sums of two different notes
- The table is symmetric around the diagonal

Each occurrence of a number n in the table represents one common tone at In. The table thus reveals both how many common tones exist at each index and which specific pitch classes are held in common.

# Mathematical Formulation/Recognition
Construction:
1. Write set {a, b, c, d...} along top row and leftmost column
2. Fill each cell with (row value + column value) mod 12
3. The table will be symmetric (cell [i,j] = cell [j,i])

Reading the table:
- Count occurrences of each sum (0-11)
- Number of occurrences of n = number of common tones at In
- Location in table shows which notes are common: sum n at intersection of x and y means both x and y are common tones at In

# Musical Context/Application
The addition table is a practical computational tool for:
- Quickly determining common tones at all 12 inversion levels
- Identifying which specific pitch classes will be retained
- Finding inversional symmetry (when a sum appears as many times as the set has notes)
- Planning voice leading between inversionally related forms

This method is more efficient than calculating each inversion separately, especially for larger sets.

# Examples
From Example 3-11: Addition table for [3, 4, 7, 8]:

|   | 3  | 4  | 7  | 8  |
|---|----|----|----|----|
| 3 | 6  | 7  | 10 | 11 |
| 4 | 7  | 8  | 11 | 0  |
| 7 | 10 | 11 | 2  | 3  |
| 8 | 11 | 0  | 3  | 4  |

Reading the results:
- 11 appears 4 times: 4 common tones at I11
- 3 appears 2 times: 2 common tones at I3
- 7 appears 2 times: 2 common tones at I7 (notes 3 and 4)
- 0 appears 2 times: 2 common tones at I0
- 6, 8, 2, 4, 10 each appear once: 1 common tone each

The number 11 at intersection of 3 and 8 tells us that 3 and 8 map onto each other at I11.

# Related Concepts
- Index number
- Common tones under inversion
- Inversion (In)
- Inversional symmetry
- Multiplication table (for interval calculations)

# Common Confusions
- Forgetting to use mod 12 for sums greater than 11
- Miscounting diagonal entries (each diagonal entry counts once, not twice)
- Confusing addition tables (for inversion/sums) with subtraction tables or interval tables (for transposition/differences)
- Not recognizing that the table is symmetric

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.3.1, pages 120-121
