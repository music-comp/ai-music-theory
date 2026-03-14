---
concept: PROJ Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
Transformations that move between structural levels in a Schenkerian network: PROJ+ increments the level (toward foreground), PROJ- decrements the level (toward background).

# Formal Definition
PROJ+ and PROJ- operate on triples (Klang, degree, level):
- PROJ+(Kng, deg, lev) = (Kng, deg, lev + 1)
- PROJ-(Kng, deg, lev) = (Kng, deg, lev - 1)

PROJ+ and PROJ- are inverses:
- PROJ+ = (PROJ-)^(-1)
- PROJ- = (PROJ+)^(-1)

# Mathematical Formulation
The PROJ operations are context-free:
- They always increment or decrement by exactly 1
- Klang and degree are preserved
- The operations are well-defined even when the new level has no analytic pertinence

This context-freedom ensures PROJ+ and PROJ- are proper operations (invertible, well-defined on all arguments).

# Musical Context/Application
PROJ transformations connect corresponding events across Schenkerian levels. An event at one level "projects" to the next level, representing the elaboration hierarchy of Schenkerian analysis. PROJ arrows typically appear as two-way arrows (since PROJ+ and PROJ- are inverses).

# Examples
From Figure 9.16 (Beethoven Appassionata):
- Level 1: (Db, 5, 1) - deep background
- Level 2: (Db, 5, 2) - middleground
- PROJ+ arrow connects (Db, 5, 1) to (Db, 5, 2)
- PROJ- arrow connects (Db, 5, 2) to (Db, 5, 1)
- Drawn as two-way arrow for convenience

Within-level transformations:
- (DOM, SUST): Klang transforms by DOM, degree sustains
- (SUBD, N+): Klang transforms by SUBD, degree moves by upper neighbor

# Related Concepts
- Schenkerian Network
- Structural Level
- Beethoven Appassionata Analysis
- Klang Representation
- Degree Transformation

# Common Confusions
- PROJ only changes level; Klang and degree are unchanged
- The formal levels are integers; "level 0" or negative levels are formally possible
- PROJ arrows are typically two-way because PROJ+ and PROJ- are inverses
- PROJ connects events across levels, not successive events within a level

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, Figure 9.16
