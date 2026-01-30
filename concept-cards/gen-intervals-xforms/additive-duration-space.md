---
concept: Additive Duration Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Additive duration space attempts to measure intervals between durations by their difference (subtraction) rather than their ratio, but fails to form a GIS because negative durations are not meaningful.

# Formal Definition
In Example 2.2.5, the musical space S is a family of durations. int(s, t) is the difference of time units: int(s, t) = (t - s) units. So if r = 3, s = 4, and t = 8 units, then int(r, s) = 1 unit, int(s, t) = 4 units, and int(t, r) = -5 units. However, this does NOT form a GIS because Condition (B) fails.

# Mathematical Formulation
- Attempted: S = positive durations, IVLS = integers under addition
- int(s, t) = t - s (difference in time units)
- Problem: For s = 3 and i = -8, there is no duration t with int(s, t) = i
- This would require t = -5 units, a "negative duration"
- Negative durations are meaningless, unlike negative pitch intervals

# Musical Context/Application
This example illustrates why not every intuitive musical space forms a GIS. While we can meaningfully speak of durations differing by some amount, the asymmetry of time (durations can't be negative) breaks Condition (B). This contrasts with pitch space, where we can conceive of arbitrarily high or low pitches even if we can't hear them. We cannot conceive of a duration lasting "precisely 5 units less than no time at all."

# Examples
From Example 2.2.5:
- r = 3 units (dotted eighth), s = 4 units (quarter), t = 8 units (half)
- int(r, s) = 4 - 3 = 1 unit ("plus a sixteenth")
- int(s, t) = 8 - 4 = 4 units ("plus a quarter")
- int(t, r) = 3 - 8 = -5 units ("minus a quarter-tied-to-a-sixteenth")

Condition (B) failure: Take s = 3 and i = -8. No t exists with t - 3 = -8, since t = -5 is not a duration.

Compare to multiplicative intervals (2.2.3): int(r, s) = 4/3, int(s, t) = 2, int(t, r) = 3/8 - all positive ratios.

# Related Concepts
- Duration Proportion Space
- Duration-Class Space (Additive)
- GIS Condition B
- Time-Point Space

# Common Confusions
- This is NOT a GIS - it fails Condition (B)
- Negative intervals exist but negative durations don't
- Example 2.2.6 rescues this by using modular arithmetic
- The contrast with pitch (where negative intervals lead to valid low pitches) is instructive

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.5, Section 2.4
