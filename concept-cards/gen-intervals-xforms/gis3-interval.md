---
concept: GIS3-Interval
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A GIS3-interval is a compound interval in a direct-product GIS combining pitch-class intervals and temporal intervals. Recurrent GIS3-intervals reveal compositional associations between pitch and rhythmic structure.

# Formal Definition
In GIS3 = GIS1 x GIS2 (pitch-class x time-point):
- Elements: (pitch-class, time-point) pairs
- Intervals: (pitch-class interval, temporal interval) pairs

A GIS3-interval (n, t) means:
- Pitch-class interval n between the pitch classes
- Temporal interval t (beats) between the time points

# Mathematical Formulation
**GIS3 structure:**
S3 = Z/12Z x Z (pitch-class, time-point pairs)
IVLS3 = Z/12Z x Z (direct-product group)
int3((p1, t1), (p2, t2)) = (p2 - p1 mod 12, t2 - t1)

**Recurrence:**
A GIS3-interval (n, t) "recurs" when multiple pairs of elements have the same compound interval.

# Musical Context/Application
GIS3-intervals capture how pitch and temporal structure work together in composition. When a specific GIS3-interval recurs, it creates an association:
- Certain pitch intervals become bound to certain temporal intervals
- This reveals thematic relationships across dimensions
- The recurrence itself has compositional significance

# Examples
**Webern Piano Variations analysis (Figures 3.1-3.3):**

Recurrent GIS3-intervals:
- (11, 1): B-Bb, C#-C, A-G# (pitch-class 11, 1 beat apart)
- (11, 5): Eb-D, D-C# (pitch-class 11, 5 beats apart)
- (3, 2): B-D, E-G (pitch-class 3, 2 beats apart)
- (2, 7): connects accompaniment figures

**Interpretation:**
- (11, 1) binds pitch-class 11 to the beat (mensural function)
- (11, 5) associates "5 beats later" with the structurally important interval 11
- This suggests "5/4 meter" hearing

**Contrast with separate intervals:**
Tracking pitch-class intervals alone OR temporal intervals alone would miss these compound relationships.

**Unfolding analysis (Figure 3.3):**
As notes enter chronologically, the interval vector grows. At time-point 5 (first barline), both pitch-class 11 and beat 1 begin to predominate--this marks the first "ictus."

# Related Concepts
- Direct-Product GIS
- Webern Piano Variations Analysis
- Unfolding Interval Vector
- Compound Interval
- Mensural Structure

# Common Confusions
1. **Pair, not product:** GIS3-interval (11, 5) is a pair of numbers, not 11 * 5 = 55.

2. **Recurrence significance:** A GIS3-interval recurring is more specific than either component recurring separately.

3. **The "3" in GIS3:** This just names the third GIS in Lewin's exposition (combining GIS1 and GIS2). It's not a general mathematical notation.

4. **Temporal component:** The temporal interval counts beats between attacks, which may or may not align with notated meter.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Example 3.3.1 and Figures 3.1-3.6, pp. 68-77
