---
concept: Matrix Construction
category: technique
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 310
unit: null
authors: Joseph N. Straus
---

# Quick Definition
Matrix construction is the step-by-step process of building a 12×12 grid that displays all forty-eight forms of a twelve-tone series, starting with P0 in the top row and I0 in the first column.

# Formal Definition
To construct a 12×12 matrix:
1. **Step 1**: Write P0 horizontally across the top row
2. **Step 2**: Write I0 vertically down the left column (invert each pitch class of P0)
3. **Step 3**: Fill in the remaining prime orderings in the rows from left to right, beginning on whatever pitch class is in the first column
4. The second row will contain P_x (where x is the second element of I0), the third row P_y, and so on
5. Each cell is determined by transposing the top row to start on the pitch class in the first column of that row

# Mathematical Formulation/Recognition
**Step-by-step example (Schoenberg, String Quartet No. 4):**
1. P0: 0-11-7-8-3-1-2-10-6-5-4-9 (normalize series to start on 0)
2. I0: Invert each pc → 0-1-5-4-9-11-10-2-6-7-8-3 (down first column)
3. Row 2 starts on 1 (second element of I0), so it's P1: 1-0-8-9-4-2-3-11-7-6-5-10
4. Row 3 starts on 5, so it's P5: 5-4-0-1-8-6-7-3-11-10-9-2
5. Continue for all 12 rows

**Verification**: Each row should contain each pc 0-11 exactly once; same for each column.

# Musical Context/Application
- Essential preparatory step before analyzing a twelve-tone piece
- The matrix serves as a reference chart for twelve-counting
- Reveals invariant relationships between series forms
- Shows which forms share common starting/ending pitch classes
- Can be constructed with letter names or integers

# Examples
Complete matrix for Schoenberg's String Quartet No. 4:
```
      I0  I11  I7  I8  I3  I1  I2 I10  I6  I5  I4  I9
P0:    0   11   7   8   3   1   2  10   6   5   4   9  R0
P1:    1    0   8   9   4   2   3  11   7   6   5  10  R1
P5:    5    4   0   1   8   6   7   3  11  10   9   2  R5
P4:    4    3  11   0   7   5   6   2  10   9   8   1  R4
...
     RI0 RI11 RI7 RI8 RI3 RI1 RI2 RI10 RI6 RI5 RI4 RI9
```

# Related Concepts
- 12×12 Matrix
- Forty-Eight Series Forms
- Prime Ordering
- Inversion (Twelve-Tone)
- Twelve-Counting

# Common Confusions
- Filling in rows by transposition without using the first column as guide
- Computing I0 incorrectly—each element should be (0 - x) mod 12 for the corresponding P0 element
- Constructing the matrix with the original series rather than normalizing to P0
- Not verifying that each row and column contains all 12 pitch classes

# Source Reference
Chapter 6: Basic Concepts of Twelve-Tone Music, Section 6.2.8, pages 317-318
