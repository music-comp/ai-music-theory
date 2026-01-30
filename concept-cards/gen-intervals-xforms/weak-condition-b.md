---
concept: Weak Condition B
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Weak Condition B requires that some t exists for each s and i (existence), but not that t is unique; this leads to equivalence classes rather than a true GIS.

# Formal Definition
Consider weakening Condition (B) by replacing "a unique t" with "some t" - call this "(weak B)". Under (weak B), the space S would be partitioned into equivalence classes: s ~ s' iff int(s, s') = e. Given s' ~ s and t' ~ t, int(s', t') = int(s, t). We could replace S by the quotient S/EQUIV and obtain a true GIS on equivalence classes.

# Mathematical Formulation
- Condition (B): exists unique t with int(s, t) = i
- Weak Condition (B): exists some t with int(s, t) = i (not necessarily unique)
- Under (weak B): Define s ~ s' iff int(s, s') = e (identity)
- This is an equivalence relation (reflexive, symmetric, transitive)
- Intervals become well-defined on equivalence classes
- The quotient space S/EQUIV with induced int forms a true GIS

# Musical Context/Application
Weak Condition B arises when a space has "redundant" elements - multiple elements at the same "position" with respect to intervals. Reducing to equivalence classes eliminates this redundancy. In practice, if your space satisfies only (weak B), you should work with the quotient space. This is why Lewin insists on the full Condition (B): it's no loss of generality, and the space is cleaner.

# Examples
Hypothetical example: Suppose S contains both "C4 played on piano" and "C4 played on violin" as distinct elements, but intervals measure only pitch, so int(C4-piano, C4-violin) = e. Then S satisfies only (weak B). The quotient would collapse these to a single pitch class C4.

From the text: "It is hard to see what we could possibly want to do with S that we could not do as well or better with the reduced space S/EQUIV of equivalence classes."

This explains why Condition (B) requires uniqueness: if uniqueness fails, the space contains "redundant" elements that should be identified.

# Related Concepts
- GIS Condition B
- Equivalence Relation
- Quotient Set
- Generalized Interval System
- Simply Transitive Action

# Common Confusions
- (Weak B) is NOT used in the GIS definition - it's a hypothetical weakening
- Under (weak B), the quotient S/EQUIV does satisfy full Condition (B)
- The discussion shows why full Condition (B) is "no loss of generality"
- Uniqueness in Condition (B) prevents redundant elements in S

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, discussion following Definition 2.3.1
