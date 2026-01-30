---
concept: Group
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A group is a semigroup with an identity element in which every element has an inverse - the fundamental algebraic structure underlying interval systems.

# Formal Definition
A group is a semigroup with identity in which every element has an inverse. Equivalently, a group of operations on S is a family G of transformations on S satisfying: (A) G is closed (a semigroup), and (B) for any member f of G, there exists a member f' of G satisfying f'f = ff' = 1 (the identity).

# Mathematical Formulation
A group (G, *) satisfies four axioms:
1. Closure: For all a, b in G, a * b is in G
2. Associativity: For all a, b, c in G, (a * b) * c = a * (b * c)
3. Identity: There exists e in G such that e * a = a * e = a for all a
4. Inverses: For each a in G, there exists a^(-1) in G such that a * a^(-1) = a^(-1) * a = e

# Musical Context/Application
Groups are the algebraic foundation of interval systems and transformation theory. The group IVLS in any GIS provides the intervals. Key musical groups include: the integers mod 12 under addition (for pitch-class intervals), the T/I group of 24 transposition and inversion operations, and the group of frequency ratios under multiplication (for just intonation intervals).

# Examples
From Chapter 1: The group of transposition and inversion operations on the twelve pitch classes is non-commutative. IT2 = J (inversion about B), but T2I = K (inversion about C#). Thus T2 and I do not commute.

Mathematical examples:
- (Z, +): integers under addition. Identity = 0, inverse of n is -n
- (Z12, +): integers mod 12 under addition. Identity = 0, inverse of n is 12-n
- (Q+, *): positive rationals under multiplication. Identity = 1, inverse of x is 1/x

# Related Concepts
- Semigroup
- Identity Element
- Inverse Element
- Commutative Group
- Group of Operations
- IVLS (Group of Intervals)

# Common Confusions
- A group requires ALL FOUR properties: closure, associativity, identity, inverses
- Groups can be commutative (abelian) or non-commutative
- The T/I group is non-commutative: order of operations matters
- A semigroup lacks identity and/or inverses

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.7, 1.3.4
