---
concept: Prime Form Algorithm
slug: prime-form-algorithm
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.7 Prime Form"
extraction_confidence: high
aliases:
  - prime form procedure
prerequisites:
  - normal-form
  - normal-form-algorithm
extends:
  - prime-form
related:
  - pitch-class-clockface
  - list-of-set-classes
contrasts_with: []
answers_questions:
  - "How do I determine prime form?"
  - "What is the step-by-step procedure for finding prime form?"
---

# Quick Definition
The prime form algorithm transforms any pitch-class set into its prime form by finding the representation starting on 0 that is most packed to the left.

# Core Definition
The prime form algorithm builds on normal form, adding the step of comparing the interval succession in both directions to determine which is more compactly arranged. The result is the canonical representative of the set's set class: a string of integers beginning on 0 with the smallest intervals packed toward the left.

# Prerequisites
- **Normal form** -- the algorithm starts with normal form
- **Normal form algorithm** -- must be able to compute normal form first

# Key Properties
1. Input: any pitch-class set
2. Output: prime form (parentheses, no commas, starting on 0)
3. The algorithm considers both the original and inverted interval successions
4. This book follows Rahn's formulation for prime form

# Construction / Recognition
**Step-by-step procedure (Example 2-34, p. 83):**

**Step 1**: Put the set in normal form.

**Step 2**: Extract the interval succession (intervals between adjacent elements, reading left to right).

**Step 3**: Compare the interval succession reading left-to-right with the succession reading right-to-left. Choose whichever has the smallest intervals toward the beginning (most packed to the left).

**Step 4**: Starting from 0, build the prime form by adding the intervals in the chosen direction.

**Clockface method (p. 84):**
1. Find the widest gap between pitch classes
2. Assign 0 to the note at the gap's *end*; read clockwise -- candidate A
3. Assign 0 to the note at the gap's *beginning*; read counterclockwise -- candidate B
4. Choose whichever has fewer big integers (more packed to the left)
5. If two gaps of the same size exist, choose the one with another big gap adjacent to it
6. When in doubt, verify against the List of Set Classes

# Context & Application
Prime form identification is essential for set-class analysis. Mastering the algorithm allows analysts to quickly classify any pitch-class collection found in a score. The clockface method makes this nearly instantaneous with practice.

# Examples
**Example 2-34** (p. 83):

*Example 1*: [C#, F, F#, G]
- Intervals: 4-1-1
- Reversed: 1-1-4
- 1-1-4 more packed to the left
- Starting from 0: 0, 1, 2, 6
- Prime form: **(0126)**

*Example 2*: [Bb, D, F, F#]
- Intervals: 4-3-1
- Reversed: 1-3-4
- 1-3-4 more packed to the left
- Starting from 0: 0, 1, 4, 8
- Prime form: **(0148)**

*Example 3*: [F, F#, A]
- Intervals: 1-3
- Reversed: 3-1
- 1-3 more packed to the left (already correct direction)
- Starting from 0: 0, 1, 4
- Prime form: **(014)**

**Example 2-35** (p. 84): Clockface method illustrated with four sets. For inversionally symmetrical sets, both directions yield the same result.

# Relationships
## Builds Upon
- **Normal form algorithm** -- prime form procedure starts with normal form
## Enables
- **Prime form** -- the output of this algorithm
- **Set-class identification** -- the primary purpose
## Related
- **Pitch-class clockface** -- provides the visual shortcut method
- **List of Set Classes** -- reference for verification

# Common Errors
- **Error**: Only checking one direction. **Correction**: Always compare left-to-right and right-to-left interval successions.
- **Error**: Starting from the actual pitch class instead of 0. **Correction**: Prime form always starts on 0.

# Common Confusions
- **Confusion**: Straus/Rahn vs. Forte prime forms. **Clarification**: They disagree on a small number of set classes (5-20, 6-29, 6-31, 7-18, 7-20, 8-26). This book follows Rahn.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.7, pages 83--85.

# Verification Notes
- Definition source: direct from source (Example 2-34 table)
- Confidence rationale: algorithm given explicitly with worked examples
- Re-extraction notes: preserved old card's three worked examples; added clockface method details and Rahn/Forte note; upgraded to v3 template
