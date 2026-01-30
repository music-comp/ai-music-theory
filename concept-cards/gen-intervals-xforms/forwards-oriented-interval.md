---
concept: Forwards-Oriented Interval
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
A forwards-oriented interval in the time-span GIS is an interval (i, p) where i >= 0, and if i = 0 then p > 1, representing the relationship from an earlier (or shorter) span to a later (or longer) one.

# Formal Definition
An interval (i, p) in the time-span GIS is forwards-oriented if:
1. i >= 0 (the target span begins at or after the source span)
2. If i = 0, then p > 1 (simultaneous spans have target longer than source)

Backwards-oriented intervals satisfy:
1. i <= 0
2. If i = 0, then p < 1

The identity interval (0, 1) is neither forwards nor backwards oriented.

# Mathematical Formulation
The group IVLS of time-span intervals partitions into three categories:
1. Forwards-oriented: {(i, p) : i > 0} union {(0, p) : p > 1}
2. Backwards-oriented: {(i, p) : i < 0} union {(0, p) : 0 < p < 1}
3. Identity: {(0, 1)}

Key relationship:
- If (i, p) is forwards-oriented, then (i, p)^-1 = (-i/p, 1/p) is backwards-oriented
- Inverses swap orientation

Crucial theorem: Given attack-ordered dyads D1 = (s1, t1) and D2 = (s2, t2), D1 and D2 are canonically equivalent if and only if int(s1, t1) = int(s2, t2).

# Musical Context/Application
Forwards-oriented intervals describe how a later event relates to an earlier one in terms of temporal position and duration ratio. They play the same classifying role for time-span dyads that interval classes play for pitch-class dyads in traditional set theory.

# Examples
Forwards-oriented intervals:
- (1, 1): Next event begins 1 duration later, same length
- (2, 0.5): Event begins 2 durations later, half as long
- (0, 2): Simultaneous event, twice as long

Backwards-oriented (inverses):
- (-1, 1): Previous event began 1 duration earlier, same length
- (-4, 2): Previous event began 4 (of its) durations earlier, twice as long

The forwards-oriented intervals label dyad classes, just as interval classes 1-6 label dyad classes in pitch-class theory.

# Related Concepts
- Attack-Ordered Dyad
- Time-Span GIS
- Time-Span Interval Vector
- Interval-Preserving Operations in TMSPS

# Common Confusions
Don't confuse the "orientation" with musical direction. Forwards-oriented means the interval goes from earlier to later (or shorter to longer if simultaneous). The terminology parallels "positive" vs "negative" intervals but accounts for the non-commutativity of the time-span GIS.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section 5.4 and Appendix 5.6
