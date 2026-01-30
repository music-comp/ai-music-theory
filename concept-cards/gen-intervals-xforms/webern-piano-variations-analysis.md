---
concept: Webern Piano Variations Analysis
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
Lewin's analysis of Webern's Piano Variations op. 27, third movement, demonstrates how a direct-product GIS combining pitch-class and time-point intervals reveals compositional structure through recurrent compound intervals.

# Formal Definition
The analysis uses GIS3 = GIS1 x GIS2 where:
- GIS1: 12 pitch classes, IVLS1 = Z/12Z
- GIS2: time-points, IVLS2 = integers (beats)
- GIS3: elements are (pitch-class, time-point) pairs
- Intervals are pairs (pitch-class interval, temporal interval)

# Mathematical Formulation
**GIS3 structure:**
- S3 = {(p, t) : p is pitch class, t is time-point}
- IVLS3 = Z/12Z x Z
- int3((p1, t1), (p2, t2)) = (int1(p1, p2), int2(t1, t2))

**Interval calculation example:**
int3((Eb, 0), (D, 5)) = (int(Eb, D), 5 - 0) = (11, 5)

**Unfolding interval vector:** The interval content of the set as it accumulates note-by-note through time.

# Musical Context/Application
The analysis shows how pitch-class structure and mensural rhythmic structure interact in Webern's music. Rather than treating pitch and rhythm as independent parameters, the direct-product GIS reveals their compositional integration.

Key insight: Recurrent GIS3-intervals bind certain pitch-class intervals to certain temporal intervals, creating thematic associations between the two dimensions.

# Examples
**Figure 3.1 (mm. 1-6):**
Recurrent GIS3-intervals:
- (11, 1): B-Bb, C#-C, A-G# (pitch-class 11 with 1 beat)
- (11, 5): Eb-D, D-C# (pitch-class 11 with 5 beats)
- (3, 2): B-D, E-G (pitch-class 3 with 2 beats)
- (2, 7): links accompaniment figure elements

**Mensural interpretation:**
- (11, 1) recurrence establishes pitch-class interval 11 as "beat-defining"
- (11, 5) associates "5 beats later" with this structurally important interval
- This suggests hearing the music "in 5 meter"

**Figure 3.3 (unfolding interval vector):**
As notes enter at time-points 0, 3, 4, 5, 10, 11:
- (Eb, 0), (B, 3), (Bb, 4), (D, 5), (C#, 10), (C, 11)
- Interval vector expands with each note
- Time-point 5 (first barline) marks where pitch-class 11 and beat 1 both predominate

**Ictus analysis:**
Time-point 5 functions as "ictus"--the moment when mensural structure first becomes perceptible. This coincides with the first notated barline.

# Related Concepts
- Direct-Product GIS
- Unfolding Interval Vector
- GIS3-interval
- Mensural Structure
- Compound Interval

# Common Confusions
1. **GIS3-intervals are pairs:** (11, 5) means pitch-class interval 11 AND temporal interval 5, not a product or sum.

2. **Recurrence significance:** A GIS3-interval recurring creates thematic association between its pitch and temporal components.

3. **"In 5 meter":** This is not simply asserting 5/4 time signature, but recognizing the structural significance of 5-beat spans via GIS3-interval analysis.

4. **The analysis is note-by-note:** The unfolding interval vector tracks how our perception develops as we hear successive attacks.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Example 3.3.1 and Figures 3.1-3.6, pp. 69-77
