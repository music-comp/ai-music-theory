---
concept: SHADOW Function for Time Spans
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
SHADOW(b, y) is the set of all time spans that "happen within" the time of span (b, y) - spans that begin after b and end before b + y.

# Formal Definition
Definition (6.10): Given a time span (b, y), SHADOW(b, y) is the family of all time spans (a, x) satisfying:
- b <= a (the span begins at or after b)
- a + x <= b + y (the span ends at or before b + y)

SHADOW(b, y) forms a triangular region in the half-plane model of time spans.

# Mathematical Formulation
SHADOW(b, y) = {(a, x) : b <= a AND a + x <= b + y}

In the half-plane with coordinates (a, x) where x > 0:
- Constraint b <= a is a vertical half-plane
- Constraint a + x <= b + y is a diagonal half-plane
- Intersection is a triangle with vertices at (b, 0), (b+y, 0), (b, y)

Special case: SHADOW(BEGIN, EXTENT) for a piece = all time spans within the piece

Event containment: (a, x) in SHADOW(b, y) iff event1 (at span (a, x)) happens during event2 (at span (b, y))

# Musical Context/Application
SHADOW models temporal containment. The shadow of a piece's total duration contains all possible event spans within the piece. The shadow of a section contains events within that section. Intersecting SHADOW sets models events occurring during multiple simultaneous contexts.

# Examples
Piece model (6.10):
- Piece begins at BEGIN, lasts EXTENT time units
- SHADOW(BEGIN, EXTENT) = all possible event spans in the piece
- This is a triangle in the (onset, duration) half-plane

Section model:
- Section begins at BEGSEC, lasts DURSEC
- SHADOW(BEGSEC, DURSEC) = all spans within the section
- X = SHADOW(BEGSEC, DURSEC) can be used in INJ calculations

Brass shadow:
- BRASS = set of time spans when brass instruments sound
- Y = SHADOW(BRASS) = union of SHADOW(b, y) over all brass spans
- Time span (a, x) in Y iff event at (a, x) happens during some brass event

Complex query: "How many string events above middle C occur such that doubling their duration places them within a brass shadow?"
- X = {orange dots in section} (strings above middle C)
- Y = SHADOW(BRASS)
- f(a, x) = (a, 2x) (double duration)
- Answer: INJ(X, Y)(f)

# Related Concepts
- Time-Span GIS
- INJ for Measure Spaces
- Temporal Containment
- Event Duration

# Common Confusions
SHADOW is about temporal containment, not pitch or other properties. The "shadow" metaphor: if you shine a light down at event (b, y), its shadow covers all events that happen "during" it. SHADOW sets can overlap when events overlap temporally.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.10 (optional)
