---
concept: Duration Proportion Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Duration proportion space is a GIS where durations are compared by their ratios (quotients), using a multiplicative interval group.

# Formal Definition
In Example 2.2.3, the musical space S is a family of durations, each measuring a temporal span in time units. The interval int(s, t) is the quotient t/s of the duration measurements. If s spans 4 time units and t spans 3 time units, then int(s, t) = 3/4. The interval group IVLS is a multiplicative group of positive numbers, depending on which proportions we wish to allow.

# Mathematical Formulation
- S = durations (positive real numbers representing time spans)
- IVLS = multiplicative group of positive ratios (specific group depends on context)
- int(s, t) = t/s (ratio of durations)
- If we allow proportions of 2 and 3: IVLS = {2^a * 3^b : a, b in Z}
- If we allow 2, 3, 5, and 7: IVLS = {2^a * 3^b * 5^c * 7^d}
- Identity: int(s, s) = 1
- Inverse: int(t, s) = 1/int(s, t)

# Musical Context/Application
This GIS models rhythmic proportions - how durations relate to each other by ratio. The statement "t is 3/4 the length of s" gives an interval. This framework is used in analyses of proportional notation, metric modulation, and tempo relationships. Durations can be identified with tempi (inverse relationship): if s is a beat duration, the tempo is proportional to 1/s.

# Examples
From Example 2.2.3:
- If s = 4 units and t = 3 units, int(s, t) = 3/4
- "t is 3/4 the length of s"

Different IVLS depending on allowed proportions:
- Basic 2 and 3: IVLS = {2^a * 3^b}, e.g., 1, 2, 3, 4, 6, 8, 9, 3/2, 4/3, ...
- Including 5 and 7: IVLS = {2^a * 3^b * 5^c * 7^d}
- Including sqrt(2) and sqrt(3): IVLS = {2^(a/2) * 3^(b/2)}

Tempo-space interpretation: Durations and tempi are inversely related. A duration of 2 units corresponds to a tempo half as fast as a duration of 1 unit.

# Related Concepts
- Duration-Class Space
- Time-Point Space
- Generalized Interval System
- Just Intonation Pitch Space
- Multiplicative Group

# Common Confusions
- Intervals are ratios (multiplied together), not differences (added)
- This differs from time-point space (which measures positions, not lengths)
- The specific IVLS depends on what proportions the analysis allows
- S must be extended to include "impractical" durations for mathematical completeness

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.3, Section 2.4
