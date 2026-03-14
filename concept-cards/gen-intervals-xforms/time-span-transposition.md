---
concept: Time-Span Transposition
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
Time-span transposition T(i,p) transforms a time span by shifting its attack point i span-lengths later and scaling its duration by factor p. Due to non-commutativity, time-span transposition does not preserve intervals (except for the identity).

# Formal Definition
**Notes 4.1.7(A):** Given interval (i, p) and time span (a, x):

T(i,p)(a, x) = (a + ix, px)

The transposed time span begins i x-lengths later than a, and lasts p times as long as x.

# Mathematical Formulation
**Derivation from Definition 3.4.1:**
T(i,p)(a, x) = (b, y) where int((a, x), (b, y)) = (i, p)
That is: ((b-a)/x, y/x) = (i, p)
So: b = a + ix and y = px
Hence: T(i,p)(a, x) = (a + ix, px)

**LABEL interpretation (Notes 4.1.7(B)-(C)):**
With ref = (0, 1):
LABEL(a, x) = int((0, 1), (a, x)) = (a, x)

The number-pair (a, x) serves as both time span and its LABEL.
T(i,p)(a, x) = (a, x)(i, p) (composition in IVLS)

# Musical Context/Application
Time-span transposition models moving an event later in time and/or changing its duration. Unlike pitch transposition, this operation:
- Does not preserve intervals (Theorem 3.4.8, since (i, p) is not central unless i = 0, p = 1)
- May not even preserve chronological order of events

This counterintuitive behavior is inherent to the non-commutative structure.

# Examples
**Basic calculation:**
T(2,3)(1, 4) = (1 + 2*4, 3*4) = (9, 12)
The event at time 1 lasting 4 units becomes an event at time 9 lasting 12 units.

**Figure 4.4 interpretation:**
If s1 = (a1, x1) and t1 = T(4,2)(s1), then:
t1 = (a1 + 4x1, 2x1)
t1 begins 4 s1-durations after s1, and lasts twice as long.

**Non-preservation of intervals:**
s = (0, 1), t = (1, 1): int(s, t) = (1, 1)
T(1,2)(s) = (1, 2), T(1,2)(t) = (1 + 2, 2) = (3, 2)
int(T(1,2)(s), T(1,2)(t)) = ((3-1)/2, 2/2) = (1, 1)

Wait, that preserved the interval! Let's try another:
s = (0, 1), t = (2, 1): int(s, t) = (2, 1)
T(1,2)(s) = (1, 2), T(1,2)(t) = (3, 2)
int((1, 2), (3, 2)) = ((3-1)/2, 2/2) = (1, 1) =/= (2, 1)

Interval NOT preserved!

**Chronology can reverse:**
On Figure 4.4: s1 precedes s2, but t1 = T(4,2)(s1) may follow t2 = T(4,2)(s2).

# Related Concepts
- Time-Span GIS
- Time-span Interval
- Transposition Operation (Ti)
- Interval-Preserving Operation
- Non-Commutative Groups

# Common Confusions
1. **Transposition doesn't preserve intervals:** This is shocking if you're used to pitch transposition. Only T(0,1) = identity preserves intervals.

2. **The formula T(i,p)(a, x) = (a + ix, px):** The shift is i*x, not just i. It's measured in x-units.

3. **Chronology reversal:** Two events in one temporal order may appear in reversed order after transposition.

4. **Right multiplication:** T(i,p)(a, x) = (a, x)(i, p) in IVLS. This is the anti-isomorphism at work.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(A)-(C), pp. 112-113
