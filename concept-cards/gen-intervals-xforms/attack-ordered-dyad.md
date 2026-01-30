---
concept: Attack-Ordered Dyad (AOD)
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
An attack-ordered dyad is a pair of time spans ordered by their temporal relationships: the span that begins first (or is shorter if simultaneous) is listed first.

# Formal Definition
A "dyad" is a set containing two distinct members s and t. An attack-ordered dyad (AOD) orders these according to:
1. If s begins before t, the order is (s, t)
2. If t begins before s, the order is (t, s)
3. If both begin at the same time, the shorter span is listed first

Since s and t are distinct time spans, these criteria suffice to order any dyad.

# Mathematical Formulation
For time spans s = (a, x) and t = (b, y):

AOD ordering:
- If a < b: AOD = (s, t)
- If a > b: AOD = (t, s)
- If a = b and x < y: AOD = (s, t)
- If a = b and x > y: AOD = (t, s)

Given AOD D = (s, t), let int(s, t) = (i, p). Then:
- i >= 0 (forwards-oriented: t begins after or with s)
- If i = 0, then p > 1 (t is longer than s)

The interval (i, p) is called "forwards-oriented."

Inverse: If (i, p) is forwards-oriented, then (i, p)^-1 = (-i/p, 1/p) is "backwards-oriented."

# Musical Context/Application
Attack-ordering respects musical perception: we hear events in temporal sequence. The AOD construction allows dyads to be classified by their forwards-oriented intervals, paralleling how pitch-class dyads are classified by interval classes.

# Examples
Two eighth notes in succession:
- s = (0, 0.5), t = (0.5, 0.5) [quarter = 1]
- AOD = (s, t) since s begins first
- int(s, t) = (1, 1): t begins 1 s-duration later, same length

Half note followed by quarter:
- s = (0, 2), t = (2, 1)
- AOD = (s, t)
- int(s, t) = (1, 0.5): t begins 1 s-duration later, half as long

Simultaneous quarter and half:
- s = (0, 1), t = (0, 2)
- AOD = (s, t) since s is shorter
- int(s, t) = (0, 2): same start time, t twice as long

# Related Concepts
- Time-Span GIS
- Forwards-Oriented Interval
- Time-Span Interval Vector
- Interval-Preserving Operations
- Canonical Group in TMSPS

# Common Confusions
Attack-ordering is not the same as temporal ordering of all events. Two spans may overlap in complex ways; the AOD construction only considers onset times (and durations as tiebreaker). This simplification enables the clean correspondence between dyad classes and forwards-oriented intervals.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, section 5.4 preceding Example 5.4.1
