---
concept: Commutative Time-Span GIS
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The commutative time-span GIS (Example 4.1.2) uses the interval function int((a,x), (b,y)) = (b-a, y/x). This simpler structure is useful when a fixed time unit can be assumed, but it depends on the choice of unit.

# Formal Definition
**Example 4.1.2:** Take S = TMSPS. Take IVLS to be the direct-product group of:
- Real numbers under addition (for temporal distance)
- Positive reals under multiplication (for duration ratio)

Define: int((a, x), (b, y)) = (b - a, y/x)

Then (TMSPS, IVLS, int) is a commutative GIS.

# Mathematical Formulation
**IVLS structure:**
IVLS = R x R+ with operation (i, p)(j, q) = (i + j, pq)
This is commutative: (i, p)(j, q) = (j, q)(i, p) since addition and multiplication are commutative.

**Interval calculation:**
int((a, x), (b, y)) = (b - a, y/x)
- First component: temporal distance in absolute units
- Second component: duration ratio (unit-independent)

**Contrast with GIS 4.1.3:**
GIS 4.1.2: int = (b - a, y/x) - uses absolute time units
GIS 4.1.3: int = ((b-a)/x, y/x) - uses first span as unit

# Musical Context/Application
This GIS is appropriate when:
- A fixed referential time unit exists and is analytically significant
- The music has a clear global beat or pulse
- We want to measure absolute temporal distances

It is NOT appropriate when:
- No single time unit governs the texture
- Different layers have independent tempi
- We need reference-independence

# Examples
**Basic interval:**
s = (0, 1), t = (5, 2)
int(s, t) = (5 - 0, 2/1) = (5, 2)
Meaning: t begins 5 units after s and lasts twice as long.

**Unit-dependence:**
In beat units: s = (0, 1), t = (5, 2)
In half-beat units: s = (0, 2), t = (10, 4)
int (half-beats) = (10 - 0, 4/2) = (10, 2)

Different first component! The interval depends on the unit choice.

**When it's appropriate:**
Classic tonal music with clear meter: the quarter note (or other fixed value) serves as unambiguous referential unit. GIS 4.1.2 works well.

**When it fails:**
Carter's mm.22-32: no single unit works for all instruments. GIS 4.1.2 would give different intervals depending on which tempo we privilege.

# Related Concepts
- Time-Span GIS (Non-commutative)
- Direct-Product GIS
- Referential Time-Unit Problem
- Time-Span Interval Independence
- Theorem 4.1.4

# Common Confusions
1. **"Simpler" =/= "better":** GIS 4.1.2 is algebraically simpler (commutative) but less powerful (unit-dependent).

2. **Duration ratio is shared:** Both GIS use y/x for duration ratio. Only the temporal distance component differs.

3. **Property (A) but not (B):** GIS 4.1.2 has zero-point independence but not unit independence.

4. **Direct-product structure:** GIS 4.1.2 is literally the direct product of two simpler GIS (time-points and durations). GIS 4.1.3 has a more intricate composition law.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.1.2 and subsequent discussion, pp. 92-93
