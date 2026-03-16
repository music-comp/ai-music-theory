---
concept: Modular Clock
slug: modular-clock

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Modular clock"

extraction_confidence: high

aliases:
  - n-hour clock
  - chromatic clock

prerequisites:
  - modular-integers
extends:
  - wrapping-real-line-around-circle
related:
  - modular-arithmetic
  - modular-chromatic-intervals
  - n-tone-row-chart
contrasts_with: []

answers_questions:
  - "How does the modular clock visualize Z_n?"
  - "How is addition in Z_n computed using the clock?"
---

# Quick Definition

A circular diagram with n equally spaced positions representing the elements of Z_n, where addition corresponds to clockwise rotation.

# Core Definition

The group Z_n is realized as the group of rotations of a regular n-gon, or a clock with n positions dividing a circle into n equal arcs. The top position is labeled [0], with positions [1], [2], ..., [n-1] proceeding clockwise. Addition [k] + [l] is computed by rotating clockwise by k positions then by l positions; the sum is where the top position lands after both rotations (counterclockwise if k or l is negative) (Wright, pp. 90-91).

# Prerequisites

- **Modular integers** — The clock visualizes the group Z_n

# Key Properties

1. n equally spaced positions correspond to elements of Z_n
2. [0] is at the top
3. Addition = rotation: [k] + [l] is the position reached after rotating k then l steps clockwise
4. The clock establishes a bijection between Z_n elements and positions
5. Each position can be labeled with both its modular integer and the corresponding note class

# Construction / Recognition

## To Build a Modular Clock
1. Draw a circle and place n equally spaced points
2. Label the top point [0]
3. Label successive points clockwise: [1], [2], ..., [n-1]
4. Optionally, add note class labels (e.g., for Z_12: [0]=C, [1]=C#, etc.)

## To Compute [k] + [l] on the Clock
1. Start at the top position
2. Rotate clockwise k positions
3. From that position, rotate clockwise l more positions
4. The landing position is [k] + [l]

# Context & Application

The 12-position modular clock is the "chromatic clock" used extensively in twelve-tone theory. Labeling each position with both its modular integer and the corresponding note class allows rapid conversion between numerical and musical representations. This is essential for constructing and reading row charts.

# Examples

**Example 1** (p. 90): Z_4 is the group of rotations of a square, with four clock positions.

**Example 2** (pp. 92-93): Z_12 labeled from E: [0]=E, [1]=F, [2]=F#, ..., [11]=D#, used to translate row chart entries to note names.

**Example 3** (p. 93): Z_7 labeled from C: [0]=C, [1]=D, [2]=E, [3]=F, [4]=G, [5]=A, [6]=B, for 7-tone composition.

**Example 4**: Adding [5] + [7] in Z_12: rotate 5 then 7 positions clockwise, landing on [0] (the top).

# Relationships

## Builds Upon
- **Modular integers** — The clock visualizes Z_n
- **Wrapping real line around circle** — The clock is the discrete restriction of the wrapping function

## Enables
- **N-tone row chart** — The clock aids in converting between modular integers and note classes

## Related
- **Modular arithmetic** — The clock provides a geometric computation method
- **Modular chromatic intervals** — The clock represents chromatic interval classes

# Common Errors

- **Error**: Placing [1] at the top instead of [0]
  **Correction**: By convention, [0] is at the top; [1] is the first position clockwise from [0]

# Common Confusions

- **Confusion**: Thinking the modular clock always has 12 positions
  **Clarification**: The clock has n positions for any positive integer n; it is not limited to 12

- **Confusion**: Confusing clockwise with counterclockwise for negative values
  **Clarification**: Positive values rotate clockwise; negative values rotate counterclockwise

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 90-91 (Modular clock section). See the Z_4 and Z_12 clock diagrams.

# Verification Notes

- Definition source: Direct from Wright, pp. 90-91
- Confidence rationale: High — explicit definition with multiple diagrams
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_4, Z_12, Z_7 examples, rotation interpretation
