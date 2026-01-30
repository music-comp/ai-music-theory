---
concept: Time-Span GIS
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The time-span GIS is a non-commutative GIS where elements are time spans (a, x) and the interval from one span to another measures how many of the first span's durations fit before the second begins, plus the ratio of their durations.

# Formal Definition
**Theorem 4.1.3.2:** Let int map TMSPS x TMSPS into the group IVLS (from Lemma 4.1.3.1) by:

int((a, x), (b, y)) = ((b - a)/x, y/x)

Then (TMSPS, IVLS, int) is a GIS.

The interval tells us: (b, y) begins (b-a)/x x-lengths after (a, x), and lasts y/x times as long.

# Mathematical Formulation
**IVLS structure (Lemma 4.1.3.1):**
IVLS = {(i, p) : i in R, p in R+}
Composition: (i, p)(j, q) = (i + pj, pq)
Identity: (0, 1)
Inverse: (i, p)^(-1) = (-i/p, 1/p)

**GIS verification:**
Condition (A): int((a,x),(b,y)) int((b,y),(c,z)) = int((a,x),(c,z))
Condition (B): Given (a,x) and (i,p), unique (b,y) exists with int((a,x),(b,y)) = (i,p)

**Key property:** This GIS is non-commutative since IVLS is non-commutative.

# Musical Context/Application
The time-span GIS uses the first time span itself as a measuring unit. This eliminates dependence on arbitrary referential time units and time-point zeros (Theorem 4.1.4).

This is particularly valuable for analyzing music without a fixed beat, such as:
- Elliott Carter's metric modulations
- Stockhausen's Klavierstuck XI
- Music with multiple simultaneous tempi

# Examples
**Basic interval calculation:**
s1 = (0, 1), s2 = (4, 2)
int(s1, s2) = ((4-0)/1, 2/1) = (4, 2)
Meaning: s2 begins 4 s1-lengths after s1, and lasts 2 times as long.

**Non-commutativity example:**
(1, 2)(0, 3) = (1 + 2*0, 2*3) = (1, 6)
(0, 3)(1, 2) = (0 + 3*1, 3*2) = (3, 6)
These differ: the group is non-commutative.

**Figure 4.4 illustration:**
Two pairs of time spans (s1, t1) and (s2, t2) at different tempi.
If int(s1, t1) = int(s2, t2) = (4, 2), both have the same "shape" in their local contexts, even though their absolute positions differ.

# Related Concepts
- Time Span
- Time-span Interval
- Non-Commutative Groups
- Theorem 4.1.4 (Independence of Reference)
- Time-span Transposition

# Common Confusions
1. **The interval uses the first span as unit:** ((b-a)/x, y/x) measures in x-lengths, not absolute units.

2. **Non-commutativity of IVLS:** (i,p)(j,q) =/= (j,q)(i,p) in general. This has profound consequences for transposition behavior.

3. **Contrast with GIS 4.1.2:** The earlier commutative time-span GIS used int((a,x),(b,y)) = (b-a, y/x). The new formula ((b-a)/x, y/x) achieves reference-independence.

4. **Theorem 4.1.5:** This is essentially the ONLY GIS on time spans with the independence properties--a uniqueness result.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Lemma 4.1.3.1, Theorem 4.1.3.2, pp. 106-108
