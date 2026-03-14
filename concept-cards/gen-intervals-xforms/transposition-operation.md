---
concept: "Transposition Operation (Ti)"
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The transposition operation Ti in a GIS maps each element s to the unique element t that lies interval i from s, formalizing the intuition that transposing by i moves each element to a position i-distant from its original location.

# Formal Definition
Given a GIS (S, IVLS, int) and an interval i of IVLS, transposition by i, denoted Ti, is defined as a transformation on S by the equation:

int(s, Ti(s)) = i

That is, Ti(s) is the unique member of S which lies the interval i from s.

# Mathematical Formulation
**Definition 3.4.1:**
Ti(s) is well defined by: int(s, Ti(s)) = i

**Theorem 3.4.2:**
- Each Ti is an operation (1-to-1 and onto)
- The transposition operations form a group
- The group is anti-isomorphic to IVLS
- Specifically: Ti Tj = Tji (composition reverses order)
- Te = identity operation (where e is identity interval)
- Ti^(-1) = T(i^(-1))

**Theorem 3.4.3:** For any reference point ref:
LABEL(Ti(s)) = LABEL(s) * i

The label of the transpose equals the label of the original, right-multiplied by i.

# Musical Context/Application
Transposition generalizes the familiar notion of "moving a pitch or set by a fixed interval." In the 12-tone pitch-class GIS, T5 transposes each pitch class up by 5 semitones. The definition works in any GIS: temporal transposition shifts time-points, durational transposition scales durations, etc.

The anti-isomorphism (Ti Tj = Tji) means that composing transpositions reverses the order of intervals. This can be counterintuitive but is algebraically necessary.

# Examples
**Pitch-class transposition:**
- In 12-tone GIS: T5(C) = F (since int(C, F) = 5)
- T5 T3 = T8 (not T15, due to anti-isomorphism in additive notation)

**Time-point transposition:**
- If time-point s is at beat 0, T5(s) is at beat 5
- T5(s) lies 5 beats from s

**Webern Piano Variations (Figure 3.1):**
- Direct-product GIS with pitch-class and time-point components
- Transposition (i, j) moves pitch-class by i and time-point by j

# Related Concepts
- Generalized Interval System (GIS)
- Interval-Preserving Operations (Pi)
- Inversion Operations (I)
- LABEL Function
- Anti-isomorphism

# Common Confusions
1. **Anti-isomorphism:** Ti Tj = Tji, not Tij. The order reverses. This arises because int(s, Ti(s)) = i defines the interval from s TO its image, and interval composition works contravariantly.

2. **Transpositions vs. interval-preserving operations:** In a commutative GIS, these coincide. In a non-commutative GIS, they differ: transposition right-multiplies labels while interval-preserving operations left-multiply.

3. **Interval from element to its transpose:** int(s, Ti(s)) = i. Students sometimes confuse this with int(Ti(s), s), which equals i^(-1).

4. **Transposition does not always preserve intervals:** In non-commutative GIS, Ti may not preserve intervals (Theorem 3.4.8). Only when i is central (commutes with all intervals) does Ti preserve intervals.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.4.1-3.4.3, pp. 77-79
