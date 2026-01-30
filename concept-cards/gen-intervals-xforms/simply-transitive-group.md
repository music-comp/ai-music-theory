---
concept: Simply Transitive Group
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
unit: null
authors: David Lewin
---

# Quick Definition
A group of operations STRANS on a set S is simply transitive when for any two elements s and t in S, there exists exactly one operation in STRANS that maps s to t.

# Formal Definition
The group STRANS of operations on S is simply transitive when the following condition is satisfied: Given any elements s and t of S, then there exists a unique member OP of STRANS such that OP(s) = t.

This property combines two conditions:
1. Transitivity: For any s and t, some operation maps s to t
2. Simplicity: This operation is unique

# Mathematical Formulation
For a simply transitive group STRANS on set S:
- For all s, t in S: there exists unique OP in STRANS such that OP(s) = t
- |STRANS| = |S| (the group and set have the same cardinality)
- The action is both free and transitive

# Musical Context/Application
Simply transitive groups provide the formal bridge between interval-based thinking and transformation-based thinking. The group of transpositions in any GIS is simply transitive on the space of that GIS. This means that instead of thinking about intervals as measurements, we can think of them as unique transformations.

# Examples
In a standard pitch-class GIS:
- Given any two pitch classes s and t, there is exactly one transposition T_i that maps s to t
- For example, only T_7 maps C to G
- The twelve transposition operations form a simply transitive group on the twelve pitch classes

# Related Concepts
- GIS from Simply Transitive Group
- Intervals as Transpositions
- Transposition Operations
- Group Action
- GIS Structure

# Common Confusions
- Simply transitive does not mean the group itself is simple (in the algebraic sense)
- The uniqueness requirement distinguishes simply transitive from merely transitive actions
- Not all transformation groups are simply transitive (e.g., the full group of pitch-class operations including inversions is not simply transitive)

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.1.1
