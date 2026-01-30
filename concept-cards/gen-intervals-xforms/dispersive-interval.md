---
concept: Dispersive Interval
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
unit: null
authors: David Lewin
---

# Quick Definition
An interval i is dispersive for a set X when the transposition T_i maps X to a set with no common elements with X, that is, INJ(X, X)(T_i) = 0.

# Formal Definition
Given a pitch-class set X and an interval i:
- i is dispersive for X if and only if INJ(X, X)(T_i) = 0
- Equivalently: T_i(X) has no common notes with X
- T_i(X) intersection X = empty set

# Mathematical Formulation
For set X and interval i:
- INJ(X, X)(T_i) counts how many elements of X map to elements of X under T_i
- Dispersive: INJ(X, X)(T_i) = 0
- Maximally similar: INJ(X, X)(T_i) = |X| (only possible when i = 0)

# Musical Context/Application
Dispersive intervals are useful for filling chromatic space efficiently. When a motive with a dispersive TCH-interval is RI-chained, the successive forms fill up pitch-class space without repetition until returning to the original transposition level.

# Examples
From Wagner's Parsifal (Figure 7.4):
- The Zauber motive Z has TCH-interval 10
- Interval 10 is dispersive for Z as an unordered set: T_10(Z) shares no notes with Z
- The open noteheads on Figure 7.4 through m. 1140 constitute a non-repeating ten-note series
- F# and B are the only missing pitch classes ("the absent Klingsor")

# Related Concepts
- INJ Function
- TCH Transformation
- RI-Chaining
- Structural Sequencing
- Pitch-Class Set Theory

# Common Confusions
- Dispersive is relative to a specific set; an interval may be dispersive for one set but not another
- Dispersive does not mean "large" - a small interval can be dispersive for certain sets
- The complementary concept (maximally similar intervals) is equally important analytically

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.2
