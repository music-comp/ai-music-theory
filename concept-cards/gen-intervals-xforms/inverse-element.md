---
concept: Inverse Element
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An inverse of element x in a group is an element x' such that xx' = x'x = e (the identity) - it "undoes" the effect of x.

# Formal Definition
Given a semigroup with identity e, a left inverse for x is an element l satisfying lx = e. A right inverse for x is an element r satisfying xr = e. An inverse for x is an element x' which is both a left inverse and a right inverse. If x has both a left inverse and a right inverse, they must be equal, so x can have at most one inverse.

# Mathematical Formulation
- x' (or x^(-1)) is the inverse of x if x'x = xx' = e
- Uniqueness theorem: If lx = e and xr = e, then l = r
- Proof: l = le = l(xr) = (lx)r = er = r
- In a group, every element has a unique inverse
- Notation: x^(-1) in multiplicative groups, -x in additive groups

# Musical Context/Application
Inverses allow us to "reverse" musical transformations. The inverse of transposition by n is transposition by -n (or 12-n mod 12). The inverse of an interval i is the interval -i that returns to the starting point. In GIS theory: int(t, s) = int(s, t)^(-1), meaning the interval from t to s is the inverse of the interval from s to t.

# Examples
Inverse elements in musical contexts:
- In integers mod 12: inverse of 5 is 7 (since 5 + 7 = 12 = 0 mod 12)
- In transpositions: (T5)^(-1) = T7
- In interval ratios: inverse of 3/2 is 2/3 (since (3/2)(2/3) = 1)

Proof from Chapter 1 (Theorem 1.6.2): If l is a left inverse (lx = e) and r is a right inverse (xr = e), then:
l = le = l(xr) = (lx)r = er = r

# Related Concepts
- Group
- Identity Element
- Inverse Function
- GIS Interval Function
- Theorem 2.3.2

# Common Confusions
- Inverses require an identity element to exist first
- Left inverse alone doesn't guarantee right inverse (in semigroups)
- In groups, left and right inverses are always equal
- The inverse of a composition reverses order: (xy)^(-1) = y^(-1)x^(-1)

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.6.1-1.6.3
