---
concept: Beat-Class Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Beat-class space is a GIS of N beat classes (metric positions within a measure) arranged on an N-hour clock, with intervals measured as clockwise hours.

# Formal Definition
In Example 2.2.2, the musical space S is time-point space wrapped around the face of an N-hour clock, modeling an N-unit meter. The space has N members called "beat classes," labeled 0 through N-1. Beat-class 0 comprises all pulses at barlines; beat-class k comprises all pulses k units after a barline. If s and t are beat classes, int(s, t) is the number of hours clockwise that t lies from s on the N-hour clock.

# Mathematical Formulation
- S = {0, 1, 2, ..., N-1} (N beat classes)
- IVLS = ZN = integers under addition mod N
- int(s, t) = clockwise distance from s to t on N-hour clock
- int(s, t) is always in {0, 1, ..., N-1}
- int(s, t) + int(t, s) = N = 0 mod N

# Musical Context/Application
Beat-class space models metric position abstractly. In 4/4 time (N=4), the four beat classes represent downbeat, second beat, third beat, and fourth beat - regardless of which measure. This allows analysis of metric patterns and phase relationships. Milton Babbitt developed a system of 12 beat classes that parallels twelve-tone pitch-class theory.

# Examples
From Example 2.2.2:
- In 12/8 meter (N = 12): int(beat-class 10, beat-class 5) = 7
- Dancing master counting waltz: "ONE-two-three, ONE-two-three" assigns beat classes 1, 2, 3 (or 0, 1, 2)
- Conductors associate beat classes with spatial hand positions

From Section 1.9.6.2: Beat classes are equivalence classes of time points under metric equivalence. All first beats of all measures form one beat class.

Babbitt's system: 12 beat classes behave formally like the 12 pitch classes, allowing serial manipulation of rhythm.

# Related Concepts
- Time-Point Space
- Pitch-Class Space
- Generalized Interval System
- Equivalence Class
- Meter

# Common Confusions
- Beat classes collapse all metrically equivalent time points together
- The number N depends on the meter chosen (12 for 12/8, 4 for 4/4, 3 for 3/4)
- Beat class 0 typically represents the downbeat/barline position
- This is the rhythmic analog of pitch-class space

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.2, Section 2.4
