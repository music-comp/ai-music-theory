---
concept: Time-Span Interval Independence
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
In the time-span GIS, intervals are independent of the choice of time-point zero and time unit. Moving the zero point or changing the unit of measurement does not affect the interval between any two time spans.

# Formal Definition
**Theorem 4.1.4:** GIS 4.1.3 has properties (A) and (B):

(A) For any real h: int((a+h, x), (b+h, y)) = int((a, x), (b, y))
    (Shifting time-point zero doesn't change intervals)

(B) For any positive real u: int((au, xu), (bu, yu)) = int((a, x), (b, y))
    (Changing time unit doesn't change intervals)

# Mathematical Formulation
**Proof of (A):**
int((a+h, x), (b+h, y)) = (((b+h)-(a+h))/x, y/x)
                        = ((b-a)/x, y/x)
                        = int((a, x), (b, y))

**Proof of (B):**
int((au, xu), (bu, yu)) = ((bu-au)/xu, yu/xu)
                        = ((b-a)u/xu, y/x)
                        = ((b-a)/x, y/x)
                        = int((a, x), (b, y))

**Contrast with GIS 4.1.2:**
int_4.1.2((a, x), (b, y)) = (b - a, y/x)
int_4.1.2((au, xu), (bu, yu)) = (bu - au, y/x) = ((b-a)u, y/x) =/= (b - a, y/x)

GIS 4.1.2 does NOT have property (B).

# Musical Context/Application
This independence is crucial for analyzing music without fixed reference structures. The interval between two time spans remains the same regardless of:
- When we start our clock (time-point zero)
- What unit we use to measure time (second, beat, measure, etc.)

This allows analysis of music with multiple tempi, metric modulation, or no fixed beat.

# Examples
**Property (A) example:**
Events at (3, 1) and (7, 2) have interval ((7-3)/1, 2/1) = (4, 2).
If we shift zero back by 3 units: (0, 1) and (4, 2).
Interval: ((4-0)/1, 2/1) = (4, 2). Same!

**Property (B) example:**
Events at (3, 1) and (7, 2) in "beat units."
In "half-beat units": (6, 2) and (14, 4).
Interval: ((14-6)/2, 4/2) = (4, 2). Same!

**Musical interpretation:**
Play the piece at any tempo, start anywhere--the time-span intervals between events remain invariant. This is exactly what we want for analyzing rhythmic structure independent of performance choices.

**Contrast with absolute time:**
In GIS 4.1.2, changing tempo DOES change intervals. That GIS is appropriate only when a fixed time unit is assumed.

# Related Concepts
- Time-Span GIS
- Time-span Interval
- Reference Point (ref)
- Commutative GIS 4.1.2
- Uniqueness Theorem 4.1.5

# Common Confusions
1. **Both properties matter:** (A) handles time-point zero choice; (B) handles time-unit choice. GIS 4.1.3 has both; GIS 4.1.2 has only (A).

2. **The numerical values change:** The time spans (a, x) become (a+h, x) or (au, xu). But the INTERVAL stays the same.

3. **This motivates the GIS:** The independence properties are WHY we want this particular non-commutative GIS, despite its complexity.

4. **Theorem 4.1.5:** GIS 4.1.3 is essentially the ONLY time-span GIS with both independence properties.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.4, pp. 108-110
