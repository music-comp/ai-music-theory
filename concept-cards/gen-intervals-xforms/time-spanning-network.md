---
concept: Time-Spanning Network
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network augmented with a TIMESPAN function that assigns a time span to each node, allowing the network to model when musical events occur as well as how they transform.

# Formal Definition
A time-spanning network consists of:
1. A transformation network (S, NODES, ARROW, SGP, TRANSIT, CONTENTS)
2. A function TIMESPAN: NODES -> TimeSpans

TIMESPAN(N) indicates when the musical event CONTENTS(N) occurs.

# Mathematical Formulation
Two approaches to incorporating time spans:

Approach 1: Separate TIMESPAN function
- Keep CONTENTS as musical objects
- Add TIMESPAN as separate mapping to time intervals
- Network structure and temporal structure are formally distinct

Approach 2: Time spans as part of CONTENTS
- CONTENTS(N) = (object, time_span) ordered pair
- SGP must include both object-transformations and time-transformations
- More complex but unifies the structure

# Musical Context/Application
Time-spanning networks address discrepancies between precedence-ordering and musical chronology. When node N precedes node N' formally but CONTENTS(N) is heard after CONTENTS(N'), the time span information makes this explicit. This allows rigorous treatment of temporal relationships alongside transformational relationships.

# Examples
From Figure 9.15 (Beethoven Appassionata):
- Each node has a time span (a, b) indicating onset time a and duration b
- Example: Gb node has TIMESPAN = (3.5, .5)
- The network shows both transformational structure and temporal placement
- Formal input nodes (Gb) are not temporally first

Alternative approach for Figure 9.15:
- Instead of TIMESPAN function, use CONTENTS = (Gb, (3.5, .5))
- The semigroup would include Klang-transformations paired with time-transformations
- More complex but more unified

# Related Concepts
- Transformation Network Definition
- CONTENTS Function
- TIMESPAN Function
- Carriage Return Function
- Precedence Ordering

# Common Confusions
- Time-spanning networks are not standard transformation networks (they add structure)
- The two approaches (separate TIMESPAN vs. complex CONTENTS) are formally distinct
- TIMESPAN can model actual occurrence times or ranges of possible times
- Time-span information does not change the transformational content of the network

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6
