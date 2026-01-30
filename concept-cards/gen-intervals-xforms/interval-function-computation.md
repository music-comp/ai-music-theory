---
concept: Interval Function Computation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
Computing intervals in the time-span GIS requires applying the formula int((a, x), (b, y)) = ((b-a)/x, y/x), which uses the first span's duration as the measuring unit for temporal distance.

# Formal Definition
**Time-span interval formula (4.1.3.2):**
int((a, x), (b, y)) = ((b - a)/x, y/x)

where:
- (b - a)/x = how many x-durations from a to b
- y/x = ratio of durations

# Mathematical Formulation
**Components:**
1. First component: (b - a)/x
   - b - a = absolute time difference (in whatever units)
   - Divide by x = normalize to x-units

2. Second component: y/x
   - Duration ratio (independent of absolute units)

**Comparison with commutative GIS 4.1.2:**
int_4.1.2((a, x), (b, y)) = (b - a, y/x)

The difference: 4.1.3 divides (b-a) by x; 4.1.2 does not.

# Musical Context/Application
The formula captures: "How many of span-1's beats until span-2 starts, and how do their durations compare?"

This is the musically relevant question when span-1's duration serves as the local time unit.

# Examples
**Basic calculation:**
s = (0, 2), t = (6, 4)
int(s, t) = ((6 - 0)/2, 4/2) = (3, 2)
Meaning: t starts 3 s-durations after s, and lasts twice as long.

**Same interval, different spans (Figure 4.4):**
s1 = (0, 1), t1 = (4, 2): int(s1, t1) = (4, 2)
s2 = (1, 0.5), t2 = (3, 1): int(s2, t2) = ((3-1)/0.5, 1/0.5) = (4, 2)

Same interval (4, 2) despite different absolute positions!

**Why independence matters:**
Change units (multiply all times by 2):
s = (0, 4), t = (12, 8)
int(s, t) = ((12-0)/4, 8/4) = (3, 2)

Same interval as before--unit-independent!

**Inverse calculation:**
int((a, x), (b, y)) = (i, p)
Then: b = a + ix, y = px
(Derived from Definition 3.4.1 for transposition)

# Related Concepts
- Time-Span GIS
- Time-Span Interval Independence
- Time-Span Interval Group
- LABEL Function
- Commutative Time-Span GIS

# Common Confusions
1. **Don't forget to divide:** The formula is (b-a)/x, not (b-a). This is the key difference from GIS 4.1.2.

2. **x is the first span's duration:** The measuring unit comes from the first argument to int.

3. **Second component is same as 4.1.2:** The duration ratio y/x appears in both GIS definitions.

4. **Negative first component:** If b < a (second span starts before first), the first component is negative.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.3.2, pp. 107-108
