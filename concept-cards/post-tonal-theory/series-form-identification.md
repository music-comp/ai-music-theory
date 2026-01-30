---
concept: Series Form Identification
category: analysis
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 310
unit: null
authors: Joseph N. Straus
---

# Quick Definition
Series form identification is the analytical process of determining which of the 48 series forms is being used at a given point in a piece, using matrix lookup or intervallic comparison.

# Formal Definition
Identifying series forms can be done in two ways:
1. **Matrix lookup**: Construct a 12×12 matrix, look at the first few notes of an unknown passage, and find which of the 48 forms begins with those notes
2. **Intervallic analysis**: Apply knowledge of intervallic relationships between series forms—compare the ordered pitch-class intervals of the unknown passage to those of a known form to determine the transformation type (P, R, I, or RI) and calculate the index number

# Mathematical Formulation/Recognition
**Intervallic relationships for identification:**
- Same intervals, same order → transposition of a P-form
- Same intervals, reverse order → an RI-form
- Complementary intervals, same order → an I-form
- Complementary intervals, reverse order → an R-form

**Index number calculation:**
- If intervals match in reverse, calculate index: add first note of unknown + last note of known, second + second-to-last, etc.
- All sums should equal the same index number (mod 12)
- If index = n and known form is P_m, unknown RI-form is RI_x where m + x = n

# Musical Context/Application
- Essential skill for twelve-tone analysis
- Allows the analyst to map the series forms used throughout a piece
- Reveals relationships and invariants between consecutive forms
- Shows how composers create continuity through shared elements
- Necessary first step before examining deeper structural relationships

# Examples
- Webern, "Wie bin ich froh!": Given P7 in the melody, identify the accompaniment
- First five notes of accompaniment: F#-F-D with intervals that match P7's final intervals in reverse
- This indicates an RI-form; calculating index: (6+8), (5+9), etc. all sum to 2 (mod 12)
- Since 7 + 7 = 14 → 2, the accompaniment is RI7
- The song uses only P7, RI7, R7, and I7

# Related Concepts
- Twelve-Counting
- 12×12 Matrix
- Ordered Pitch-Class Intervals
- Index Number
- Intervallic Relationships Between Series Forms

# Common Confusions
- Not recognizing that intervals can identify the transformation type without matrix lookup
- Miscalculating the index number when sums exceed 11
- Forgetting that R_n ends on pc n (not begins), affecting identification
- Assuming the first note heard is the first note of the series form

# Source Reference
Chapter 6: Basic Concepts of Twelve-Tone Music, Section 6.2.9, pages 319-322
