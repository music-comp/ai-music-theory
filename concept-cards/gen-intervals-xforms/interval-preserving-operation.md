---
concept: "Interval-Preserving Operation (Pi)"
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
An interval-preserving operation is a transformation on a GIS space that maintains all intervallic relationships: for any two elements s and t, the interval between their images equals the interval between the originals.

# Formal Definition
A transformation X on the space S of a GIS (S, IVLS, int) is called "interval-preserving" if for each s and each t:

int(X(s), X(t)) = int(s, t)

The interval-preserving transformations are precisely the operations Pi defined by:

LABEL(Pi(s)) = i * LABEL(s)

where ref is a fixed reference point.

# Mathematical Formulation
**Definition 3.4.4:** Fix ref in S. Given interval i, Pi is defined by:
LABEL(Pi(s)) = i * LABEL(s)

or equivalently:
int(ref, Pi(s)) = i * int(ref, s)

**Theorem 3.4.5:** The transformations Pi form a group of operations isomorphic to IVLS under f(i) = Pi. In particular:
Pi Pj = Pij (composition preserves order)

**Definition 3.4.6:** X is interval-preserving if int(X(s), X(t)) = int(s, t) for all s, t.

**Theorem 3.4.7:** No matter what ref is chosen, the interval-preserving transformations on S are precisely the Pi.

# Musical Context/Application
Interval-preserving operations generalize isometries--transformations that preserve distance. In music, they maintain all intervallic relationships while potentially repositioning the entire configuration in the space.

Unlike transpositions which right-multiply labels, interval-preserving operations left-multiply labels. In commutative GIS structures, these coincide, but in non-commutative structures they differ fundamentally.

# Examples
**In commutative GIS (pitch classes):**
- Ti = Pi for all i
- T5 is both the "transpose by 5" and the unique interval-preserving operation that maps ref to 5

**In non-commutative time-span GIS:**
- P(h,u)(a, x) = (h + ua, ux)
- This first scales the time span by u, then shifts by h
- T(i,p)(a, x) = (a + ix, px) differs

**Geometric interpretation:**
- Pi(s) = t where int(ref, t) = i * int(ref, s)
- In spatial terms: multiply the "position vector" from origin by i on the left

# Related Concepts
- Transposition Operations (Ti)
- LABEL Function
- Reference Point (ref)
- Inversion Operations
- Commutative vs. Non-commutative GIS

# Common Confusions
1. **Pi vs. Ti:** In commutative GIS these are the same (Ti = Pi). In non-commutative GIS they differ: Ti right-multiplies labels, Pi left-multiplies.

2. **Dependence on ref:** The specific operation labeled "Pi" depends on the choice of ref. However, the *family* of interval-preserving operations is the same regardless of ref (Theorem 3.4.7).

3. **Order of composition:** Pi Pj = Pij (isomorphism), while Ti Tj = Tji (anti-isomorphism). The orders are opposite!

4. **Theorem 3.4.8:** Ti = Pi for some ref if and only if i is central (commutes with all intervals). In a commutative GIS, every interval is central. In a non-commutative GIS, typically only the identity is central.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.4.4-3.4.7, pp. 79-81
