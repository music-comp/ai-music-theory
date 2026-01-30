---
concept: Time-Span Interval-Preserving Operation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
In the time-span GIS, interval-preserving operations P(h,u) first scale a time span by factor u, then shift it by h units. Unlike transpositions, these operations preserve all intervallic relationships between time spans.

# Formal Definition
**Notes 4.1.7(D):** The interval-preserving operation P(h,u) transforms time span (a, x) to:

P(h,u)(a, x) = (h + ua, ux)

This is computed as (h, u)(a, x) in the IVLS group (left multiplication).

# Mathematical Formulation
**Derivation from Definition 3.4.4:**
P(h,u)(a, x) = (b, y) where LABEL(b, y) = (h, u) * LABEL(a, x)
With LABEL(a, x) = (a, x):
(b, y) = (h, u)(a, x) = (h + ua, ux)

**Verification of interval preservation:**
int(P(h,u)(a, x), P(h,u)(b, y))
= int((h + ua, ux), (h + ub, uy))
= (((h + ub) - (h + ua))/ux, uy/ux)
= ((u(b - a))/ux, y/x)
= ((b - a)/x, y/x)
= int((a, x), (b, y))

# Musical Context/Application
P(h,u) represents uniform temporal transformation:
- Scale all durations by u (changing tempo)
- Shift all events by h (changing absolute position)

Because the operation is uniform, intervallic relationships are preserved. This is like transposing a piece to a different tempo and start time--the internal structure remains the same.

# Examples
**Scaling and shifting:**
P(5,2)(3, 1) = (5 + 2*3, 2*1) = (11, 2)
Original: event at time 3, duration 1
Result: event at time 11, duration 2

**Contrast with transposition:**
T(2,3)(3, 1) = (3 + 2*1, 3*1) = (5, 3)
P(2,3)(3, 1) = (2 + 3*3, 3*1) = (11, 3)

Different results! T uses the span's own duration to measure the shift.

**Interval preservation check:**
s = (0, 1), t = (2, 3)
int(s, t) = (2, 3)

P(10, 2)(s) = (10, 2), P(10, 2)(t) = (10 + 4, 6) = (14, 6)
int((10, 2), (14, 6)) = ((14-10)/2, 6/2) = (2, 3)

Interval preserved!

**Physical interpretation:**
P(h,u) = "play the piece twice as fast (u = 2) and start at time h"
All internal rhythmic relationships are preserved.

# Related Concepts
- Time-Span Transposition
- Interval-Preserving Operation (Pi)
- Time-Span GIS
- LABEL Function
- Left vs. Right Multiplication

# Common Confusions
1. **P vs. T:** P(h,u)(a,x) = (h + ua, ux) uses left multiplication. T(i,p)(a,x) = (a + ix, px) uses right multiplication. They differ in non-commutative groups!

2. **The formula:** Scale first (ua), then shift (h + ua). The duration scales to ux.

3. **Interval preservation:** T(i,p) does NOT preserve intervals (except identity). P(h,u) DOES preserve intervals (always).

4. **Tempo interpretation:** P(h,u) models playing at tempo u (relative to original) starting at time h. This preserves the musical structure.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(D), pp. 113
