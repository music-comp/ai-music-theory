---
concept: Time-Span Inversion
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
In the time-span GIS, inversion I_{(c,z)}^{(d,w)} maps time spans using a formula involving both inversion parameters. Unlike commutative GIS, time-span inversions are extremely rigid: two inversions are equal only when their parameters are identical.

# Formal Definition
**Notes 4.1.7(G):** The (c,z)/(d,w) inversion applied to time span (a, x) yields:

I_{(c,z)}^{(d,w)}(a, x) = (d + (c - a)w/x, zw/x)
                        = (d, w)(a, x)^(-1)(c, z)

# Mathematical Formulation
**Derivation:**
From Definition 3.5.1: int((d, w), I_{(c,z)}^{(d,w)}(a, x)) = int((a, x), (c, z))

Let I_{(c,z)}^{(d,w)}(a, x) = (b, y).
Then: int((d, w), (b, y)) = int((a, x), (c, z))
((b-d)/w, y/w) = ((c-a)/x, z/x)

So: b - d = (c - a)w/x, hence b = d + (c - a)w/x
And: y/w = z/x, hence y = zw/x

**Using LABEL notation:**
I_{(c,z)}^{(d,w)}(a, x) = (d, w)(a, x)^(-1)(c, z)

**Rigidity (Notes 4.1.7(H)):**
I_{s'}^{t'} = I_s^t iff s' = s AND t' = t

This follows from Theorem 3.5.3 since only (0, 1) is central in IVLS.

# Musical Context/Application
Time-span inversion is a complex operation that involves:
- Scaling the duration by the ratio of the two parameter durations
- Positioning the result based on both the original and the inversion parameters

The rigidity property means every pair of time spans defines a unique inversion. There is no "axis of symmetry" equivalence class as in pitch-class inversion.

# Examples
**Basic calculation:**
I_{(2,3)}^{(4,5)}(1, 6) = (4 + (2-1)*5/6, 3*5/6)
                        = (4 + 5/6, 15/6)
                        = (29/6, 5/2)

**Verification via LABEL formula:**
(4, 5)(1, 6)^(-1)(2, 3)
= (4, 5)(-1/6, 1/6)(2, 3)
= (4 + 5*(-1/6), 5/6)(2, 3)
= (19/6, 5/6)(2, 3)
= (19/6 + (5/6)*2, (5/6)*3)
= (19/6 + 10/6, 15/6)
= (29/6, 5/2)

Same result!

**Rigidity example:**
I_{(0,1)}^{(2,3)} and I_{(0,1)}^{(2,4)} are different operations.
I_{(0,1)}^{(2,3)} and I_{(1,1)}^{(2,3)} are different operations.
Each pair of time spans gives a distinct inversion.

# Related Concepts
- Inversion Operation (I_u^v)
- Time-Span GIS
- Central Interval
- Inversion Equivalence Conditions
- Non-Commutative GIS

# Common Confusions
1. **Rigidity:** In pitch-class GIS, I_C^E = I_G^B (same operation, different notation). In time-span GIS, every different parameter pair gives a different operation.

2. **The formula:** Both parameters (c, z) and (d, w) contribute to the result. It's not "inversion about a single center."

3. **No interval-reversal:** Time-span inversions do NOT reverse intervals (Theorem 3.6.4). There are no interval-reversing operations in this GIS.

4. **Inverse of inversion:** (I_{(c,z)}^{(d,w)})^(-1) = I_{(d,w)}^{(c,z)}, but these are NOT equal (non-commutative case).

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(G)-(H), pp. 113-114
