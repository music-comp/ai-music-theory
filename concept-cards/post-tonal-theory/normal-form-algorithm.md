---
concept: Normal Form Algorithm
category: technique
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
unit: null
authors: Joseph N. Straus
---

# Quick Definition
The normal form algorithm is a step-by-step procedure for arranging a pitch-class set in its most compact ascending form.

# Formal Definition
The normal form algorithm determines the unique most-compact representation of a pitch-class set. This standardized procedure enables consistent comparison between sets. The algorithm presented in this book differs slightly from older formulations but leads more directly to prime form.

# Mathematical Formulation/Recognition
**Step-by-step procedure:**

**Step 1**: Write out all possible orderings
- List pitch classes ascending within an octave
- Create one ordering starting on each pitch class
- Calculate the span (interval from first to last) for each

**Step 2**: Apply Rule 1 (smallest span)
- Choose the ordering with the smallest span from first to last
- If only one ordering has the smallest span, that is the normal form

**Step 3**: Apply Rule 2 (tiebreaker - packed to one end)
- If multiple orderings tie for smallest span
- Choose the one most packed to one end (larger intervals concentrated at top or bottom)
- Either end is acceptable

**Step 4**: Apply Rule 3 (symmetrical sets)
- If still tied (for inversionally symmetrical sets)
- Prefer the ordering packed to the bottom (larger intervals at top)

**Quick method (clockface):**
- Display set on clockface
- Find the largest gap
- Normal form starts after the gap, reading clockwise

# Musical Context/Application
Normal form enables systematic comparison of sets. Transpositionally related sets have identical interval successions in normal form. The first element of one corresponds to the first element of the other.

# Examples
**Example 2-3**: Three worked examples:

Example 1: {A, Bb, F}
- A-Bb-F: span = 8
- Bb-F-A: span = 11
- F-A-Bb: span = 5 (smallest)
- Normal form: [F, A, Bb]

Example 2: {F, Ab, A, C#}
- Tie between spans of 8
- Compare packing: [C#, F, Ab, A] more packed to top
- Normal form: [C#, F, Ab, A]

Example 3: {C, E, G#, A, B} (inversionally symmetrical)
- Tie between [E, G#, A, B, C] and [G#, A, B, C, E]
- Both equally packed; choose packed to bottom
- Normal form: [G#, A, B, C, E]

# Related Concepts
- Normal form
- Prime form algorithm
- Pitch-class set
- Pitch-class clockface
- Set comparison

# Common Confusions
This algorithm differs from Forte's original formulation (which was always packed to the bottom). The current version is simpler and leads more naturally to prime form. When in doubt, verify against the List of Set Classes.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.2.1, pp. 45-46
