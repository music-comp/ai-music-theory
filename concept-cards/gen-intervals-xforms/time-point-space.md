---
concept: Time-Point Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Time-point space is a GIS where the musical space consists of regularly pulsing time points, and intervals measure the number of time units between points.

# Formal Definition
In Example 2.2.1, the musical space S is a succession of time points pulsing at regular temporal distances one time unit apart. Given time points s and t, int(s, t) is the number of temporal units by which t is later than s. The interval group IVLS is the integers under addition, with the space extended indefinitely in both temporal directions.

# Mathematical Formulation
- S = time points {..., t-2, t-1, t0, t1, t2, ...} extending indefinitely
- IVLS = (Z, +), the integers under addition
- int(s, t) = number of time units that t is later than s
- Negative intervals: -x later = x earlier
- Identity: int(s, s) = 0

# Musical Context/Application
Time-point space provides a GIS framework for rhythmic analysis. It models discrete, equally-spaced time points like beats, pulses, or attack points. This is the temporal analog of chromatic pitch space. Just as pitch intervals can be transposed, time-point intervals allow temporal transposition (shifting events forward or backward in time).

# Examples
From Example 2.2.1:
- If s is beat 5 and t is beat 9, int(s, t) = 4 (t is 4 units later)
- If s is beat 5 and t is beat 2, int(s, t) = -3 (t is 3 units earlier)
- int(s, s) = 0 for any time point s

The GIS structure:
- S must extend indefinitely backward (past) and forward (future) for Condition (B)
- For any time point s and integer i, there's exactly one time point t with int(s, t) = i

This is structurally identical to chromatic pitch space - both use IVLS = (Z, +).

# Related Concepts
- Beat-Class Space
- Chromatic Pitch Space
- Generalized Interval System
- Duration Space
- Rhythm

# Common Confusions
- Time points are instants (when events occur), not durations (how long they last)
- The space must extend infinitely for mathematical completeness
- Negative intervals represent earlier times, not impossible times
- This differs from duration space (which measures lengths, not positions)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.1, Section 2.4
