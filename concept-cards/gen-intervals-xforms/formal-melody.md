---
concept: Formal Melody
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network whose node/arrow system is precedence-ordered and linearly ordered under that ordering, providing a formal model for melody as a directed sequence of transformations between successive elements.

# Formal Definition
A formal "melody" is a transformation network where:
1. The node/arrow system is precedence-ordered
2. PRECEDENCE provides a linear ordering on NODES
3. There is exactly one way to label the J nodes as N_1, N_2, ..., N_J compatible with one-way arrows

# Mathematical Formulation
For a formal melody:
- PRECEDENCE is a linear ordering (every pair of distinct nodes is comparable)
- Nodes can be uniquely ordered: N_1 < N_2 < ... < N_J in precedence
- Each N_i is connected to N_{i+1} by a one-way arrow
- The network has exactly one input (N_1) and one output (N_J)

Different arrow configurations can give the same linear precedence ordering but yield formally different "melodies" (Figure 9.17).

# Musical Context/Application
This concept elaborates the idea of melody as carrying transformational content - not just "a succession of tones" but "the impetus of transition between the tones" (Ernst Kurth). A formal melody encodes not just what pitches occur but how each transforms into the next.

# Examples
From Figure 9.17:
- Network (a): Arrows connect each consecutive pair only
- Network (b): Additional arrows connect non-consecutive nodes
- Both have the same linear precedence ordering
- But (a) and (b) are formally different "melodies"

The difference captures different transition structures:
- (a): Each note transforms only to the next
- (b): Earlier notes have transformational relationships to later non-adjacent notes

# Related Concepts
- Precedence Ordering
- Precedence-Ordered System
- Linear Ordering
- Transformation Network Definition
- Series as Network

# Common Confusions
- "Melody" here is a formal term, not everyday usage
- Different arrow configurations can yield the same precedence ordering
- The definition includes transformation labels, not just sequence of pitches
- Kurth's conception of melody as "transition" is formally captured here

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.7, Figure 9.17
