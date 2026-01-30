---
concept: Prime Form Algorithm
category: technique
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
unit: null
authors: Joseph N. Straus
---

# Quick Definition
The prime form algorithm determines the standardized set-class identifier by finding the representation starting on 0 that is most packed to the left.

# Formal Definition
The prime form algorithm transforms any pitch-class set into its prime form, the canonical representative of its set class. Prime form always begins with 0 and has the smallest intervals packed toward the left. The algorithm builds on normal form but adds the step of comparing the interval succession in both directions.

# Mathematical Formulation/Recognition
**Step-by-step procedure:**

**Step 1**: Put the set in normal form

**Step 2**: Extract the interval succession
- Calculate intervals between adjacent pitch classes
- Reading left to right

**Step 3**: Compare directions
- Compare intervals reading left-to-right vs. right-to-left
- Choose whichever direction has smaller intervals toward the beginning

**Step 4**: Build prime form
- Start from 0
- Add intervals in the chosen direction

**Quick method (clockface):**
- Find the largest gap between pitch classes
- Assign 0 to the note at the gap's end; read clockwise = one candidate
- Assign 0 to the note at the gap's beginning; read counterclockwise = other candidate
- Choose whichever has smaller numbers toward the left

# Musical Context/Application
Prime form identifies the set class to which any pitch-class set belongs. All members of a set class share the same prime form. This enables cataloging and cross-referencing of set types across different pieces and composers.

# Examples
**Example 2-34**: Three worked examples:

Example 1: [C#, F, F#, G]
- Intervals: 4-1-1
- Compare: 4-1-1 vs 1-1-4
- 1-1-4 more packed left
- Starting from 0: 0, 1, 2, 6
- Prime form: (0126)

Example 2: [Bb, D, F, F#]
- Intervals: 4-3-1
- Compare: 4-3-1 vs 1-3-4
- 1-3-4 more packed left
- Starting from 0: 0, 1, 4, 8
- Prime form: (0148)

Example 3: [F, F#, A]
- Intervals: 1-3
- Compare: 1-3 vs 3-1
- 1-3 more packed left
- Starting from 0: 0, 1, 4
- Prime form: (014)

**Example 2-35**: Clockface method illustrated with four sets.

# Related Concepts
- Prime form
- Normal form algorithm
- Set class
- List of Set Classes
- Interval succession

# Common Confusions
Prime form and normal form differ. Normal form preserves the actual pitch classes; prime form transposes to start on 0 and may invert. The same set class can contain sets with different normal forms but always the same prime form.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.7, pp. 66-68
