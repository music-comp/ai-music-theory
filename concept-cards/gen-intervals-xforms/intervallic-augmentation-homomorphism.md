---
concept: Intervallic Augmentation Homomorphism
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A graph homomorphism where SGMAP multiplies all intervals by a constant factor (such as 2), transforming one intervallic structure into an augmented version while preserving graph structure.

# Formal Definition
An intervallic augmentation homomorphism from graph G to graph G' has:
- NODEMAP = identity on NODES (same nodes)
- SGMAP(i) = k*i for some constant k (intervals multiplied by k)
- If i labels an arrow in G, then k*i labels the corresponding arrow in G'

This is a graph homomorphism because SGMAP preserves the semigroup operation: SGMAP(i + j) = k(i + j) = ki + kj = SGMAP(i) + SGMAP(j).

# Mathematical Formulation
For pitch-class intervals (mod 12):
- SGMAP(i) = 2i maps intervals to their doubles
- Example: 10 -> 20 = 8 (mod 12)
- SGMAP is a homomorphism but not isomorphism (not 1-to-1: SGMAP(0) = SGMAP(6) = 0)

Requirements (9.5.2):
- (A): NODEMAP is identity (node/arrow system isomorphism)
- (B): SGMAP is semigroup homomorphism (i -> 2i)
- (C): Compatibility: If arrow has label i in G, it has label 2i in G'

# Musical Context/Application
Intervallic augmentation captures the relationship between gestures at different intervallic scales. In the Brahms Horn Trio analysis, the "complementary gesture times 2" (Figure 9.6b) is the homomorphic image of the complementary gesture (Figure 9.6a), with all intervals doubled.

# Examples
From Section 9.5.3 (Brahms Horn Trio):
- Graph (a): complementary gesture with intervals 10, 10, 8
- Graph (b): complementary gesture times 2 with intervals 8, 8, 4
- NODEMAP = identity (same three nodes)
- SGMAP(10) = 20 = 8 (mod 12)
- SGMAP(8) = 16 = 4 (mod 12)

The musical context:
- Rhythmic augmentation (values times 2) accompanies intervallic augmentation
- The horn solo (mm. 145-49) realizes the augmented gesture
- The relationship is formal (homomorphism), not just loose analogy

# Related Concepts
- Graph Homomorphism
- SGMAP
- Complementary Gesture
- Brahms Horn Trio Analysis
- Rhythmic Augmentation

# Common Confusions
- SGMAP is a homomorphism but not an isomorphism (not 1-to-1)
- The augmentation factor (2) is fixed for the entire homomorphism
- "Times 2" applies to intervals, not to durations (though both may be augmented)
- The homomorphism is not "onto" unless we redefine the codomain

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.3, Figure 9.6
