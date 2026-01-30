---
concept: GIS Condition B
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Condition (B) states that for any starting element s and any interval i, there exists a unique element t such that int(s, t) = i - the space is "complete" and intervals act freely.

# Formal Definition
For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s, that is, a unique t which satisfies int(s, t) = i. This guarantees that the space S is large enough to contain all theoretically conceivable elements.

# Mathematical Formulation
- For all s in S and i in IVLS: exists unique t in S with int(s, t) = i
- This defines a simply transitive group action of IVLS on S
- For fixed s, the map i -> t (where int(s, t) = i) is a bijection from IVLS to S
- |S| = |IVLS| (they have the same cardinality)

# Musical Context/Application
Condition (B) ensures the musical space is theoretically complete. If we can conceive of an interval i and a starting point s, we must be able to conceive of a point t lying that interval away. This may require extending practical spaces: chromatic pitch space must include supersonic and subsonic "pitches" to satisfy Condition (B). Figure 2.2 (harmonic space) extends infinitely to accommodate all conceivable dominant/mediant relationships.

# Examples
From Section 2.4: Example 2.2.5 (durations under subtraction) does NOT satisfy Condition (B). If s = 3 units and i = -8 units, there is no duration t with int(s, t) = i, since that would require t = -5 units (negative duration is meaningless).

Pitch-class space satisfies Condition (B): For any pitch class s and any interval i (0-11), there is exactly one pitch class t with int(s, t) = i.

Time-point space requires extension: S must include time points indefinitely far in the past and future.

Harmonic space (Figure 2.2): The map extends infinitely in all directions to satisfy Condition (B) - every dominant/mediant combination must be reachable.

# Related Concepts
- Generalized Interval System
- Interval Function int
- GIS Condition A
- Simply Transitive Action
- Theoretical vs. Practical Space

# Common Confusions
- Condition (B) requires BOTH existence AND uniqueness of t
- "Weak B" (existence without uniqueness) leads to equivalence classes, not a full GIS
- Practical musical contexts often use only a portion of the theoretically complete space S
- Condition (B) is why some intuitive spaces (like durations) don't form GIS directly

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(B), discussion at end of 2.4
