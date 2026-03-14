---
concept: Identity Element
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An identity element e in a semigroup satisfies ex = xe = x for all elements x - it leaves every element unchanged when combined with it.

# Formal Definition
A left identity for a semigroup is an element l such that for every x, lx = x. A right identity is an element r such that for every x, xr = x. An identity is an element e which is both a left identity and a right identity. If a semigroup has both a left identity and a right identity, they must be equal, so there can be at most one identity element.

# Mathematical Formulation
- e is an identity if ex = xe = x for all x
- Uniqueness: if l is a left identity and r is a right identity, then l = r
- Proof of uniqueness: lr = r (since l is left identity) and lr = l (since r is right identity)
- Hence l = r, and the identity is unique

# Musical Context/Application
The identity element represents "no change" in any transformation system. In pitch-class transposition, T0 is the identity (transposing by 0). In interval groups, the identity interval represents "zero distance" or "same position." In any GIS, int(s, s) = e, meaning the interval from any element to itself is the identity interval.

# Examples
Identity elements in musical groups:
- Integers mod 12 under addition: e = 0 (0 + n = n for all n)
- Positive ratios under multiplication: e = 1 (1 * r = r for all r)
- Transposition group: e = T0

Theorem 1.5.2: If a semigroup has both a left identity l and a right identity r, then l = r. Proof: lr = r (l is left identity) and lr = l (r is right identity), so l = r.

Special case: Some semigroups have infinitely many left identities but no right identity.

# Related Concepts
- Group
- Semigroup
- Inverse Element
- Identity Transformation
- Interval Function int

# Common Confusions
- The identity element is unique IF it exists
- Left identity alone doesn't imply right identity
- Not all semigroups have identity elements
- The notation "e" or "1" for identity depends on context (additive vs multiplicative)

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.5.1-1.5.2
