---
concept: Direct-Product GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A direct-product GIS combines two GIS structures into a compound GIS whose space consists of ordered pairs from the original spaces and whose intervals are pairs of intervals from the original interval groups.

# Formal Definition
Given GIS1 = (S1, IVLS1, int1) and GIS2 = (S2, IVLS2, int2), the direct product GIS3 = GIS1 x GIS2 is constructed as follows:

- S3 = S1 x S2 (Cartesian product: pairs (s1, s2))
- IVLS3 = IVLS1 x IVLS2 (direct-product group: pairs (i1, i2))
- int3((s1, s2), (t1, t2)) = (int1(s1, t1), int2(s2, t2))

The group operation in IVLS3 is componentwise: (i1, i2)(j1, j2) = (i1 j1, i2 j2).

# Mathematical Formulation
**Definition 3.3.3:**

S3 = S1 x S2 = {(s1, s2) : s1 in S1, s2 in S2}

IVLS3 = IVLS1 x IVLS2 with operation (i1, i2)(j1, j2) = (i1 j1, i2 j2)

int3((s1, s2), (t1, t2)) = (int1(s1, t1), int2(s2, t2))

**Verification:** GIS3 satisfies Conditions (A) and (B) of Definition 2.3.1.

# Musical Context/Application
Direct-product GIS structures model conjoint musical dimensions. They allow us to consider multiple aspects of music (pitch, time, duration, timbre) in unified intervallic terms rather than treating them as independent features.

Common applications:
- Pitch-class + time-point = attacks at specific times
- Time-point + duration = time spans (events with attack and length)
- Pitch + register = specific pitches in specific octaves

# Examples
**Example 3.3.1: Pitch-class and time-point GIS:**
- GIS1: 12 pitch classes, IVLS1 = Z/12Z
- GIS2: time-points, IVLS2 = integers
- GIS3: S3 = pairs (pitch-class, time-point)
- Sample element: (C#, 35) = pitch class C# at time 35
- Sample interval: int3((C#, 35), (F, 46)) = (4, 11)

This GIS is applied to analyze Webern's Piano Variations op. 27, third movement.

**Example 3.3.2: Time-point and duration GIS:**
- GIS1: time-points, IVLS1 = integers
- GIS2: durations, IVLS2 = positive rationals under multiplication
- GIS3: elements (s, x) where s = attack time, x = duration
- Interval: int3((s, x), (t, y)) = (t - s, y/x)

**Interval vectors in Webern analysis (Figure 3.3):**
Elements unfold temporally:
- (Eb, 0), (B, 3), (Bb, 4), (D, 5), (C#, 10), (C, 11)
- Recurrent GIS3-intervals: (11, 1), (11, 5), (3, 2), (2, 7)
- These reveal mensural and pitch structures working together

# Related Concepts
- Generalized Interval System (GIS)
- Cartesian Product
- Direct-Product Group
- Quotient GIS
- Time-span GIS

# Common Confusions
1. The direct-product interval is a pair of intervals, not their product or sum. Each component is computed independently.

2. Direct-product GIS should not be confused with quotient GIS. Direct product combines two spaces; quotient reduces one space.

3. In a direct-product GIS, the group operation is componentwise. Students must remember to combine each component according to its own group structure (which may differ--one additive, one multiplicative).

4. The direct product of commutative groups is commutative, but if either factor is non-commutative, the product will be non-commutative.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Examples 3.3.1-3.3.2 and Definition 3.3.3, pp. 68-77
