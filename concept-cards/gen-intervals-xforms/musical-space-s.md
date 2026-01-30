---
concept: Musical Space S
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
The musical space S in a GIS is the family of musical elements (pitches, pitch classes, time points, durations, etc.) between which intervals are measured.

# Formal Definition
In a GIS (S, IVLS, int), S is the space of the GIS - a family of elements. The space must satisfy Condition (B) of the GIS definition: for every s in S and every interval i in IVLS, there exists a unique t in S such that int(s, t) = i. This means S must be theoretically complete, containing all elements that could conceivably be reached by any interval from any starting point.

# Mathematical Formulation
- S is a non-empty set of musical elements
- Condition (B) implies: for fixed s, the map t -> int(s, t) is a bijection from S to IVLS
- |S| corresponds to |IVLS| in a simply transitive action
- S is the domain of the interval function int

# Musical Context/Application
The space S can represent many different musical dimensions: pitches (chromatic or diatonic), pitch classes, time points, beat classes, durations, chords, or more abstract musical objects. The choice of S determines what musical elements we are analyzing. The space must be complete in a theoretical sense - if we can conceive of moving any interval from any element, the target element must exist in S.

# Examples
From Chapter 2:
- Diatonic pitch space (2.1.1): S = all scale degrees extended indefinitely up and down
- Chromatic pitch space (2.1.2): S = all chromatic pitches extended indefinitely
- Pitch-class space (2.1.3): S = the twelve pitch classes
- Time-point space (2.2.1): S = succession of time points, one unit apart
- Beat-class space (2.2.2): S = N beat classes on an N-hour clock

Practical vs. theoretical: S must contain all theoretical possibilities. For diatonic space, we need "pitches" above and below the audible range to satisfy Condition (B).

# Related Concepts
- Generalized Interval System
- Interval Group IVLS
- Interval Function int
- Pitch Class
- Time Point
- Duration

# Common Confusions
- S is the set of elements, not the intervals (those form IVLS)
- S must be complete: every interval from any point must land in S
- Practical musical contexts may use only a portion of the theoretical space S
- S can be infinite (pitches) or finite (12 pitch classes)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Section 2.4
