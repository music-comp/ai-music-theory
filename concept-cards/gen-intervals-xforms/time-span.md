---
concept: Time Span
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
A time span is an ordered pair (a, x) modeling a musical event that "begins at time a" and "extends x units of time" thereafter. Time spans are the elements of Lewin's non-commutative GIS for rhythm.

# Formal Definition
**Definition 4.1.1:** A time span is an ordered pair (a, x) where:
- a is any real number (the attack time)
- x is any positive real number (the duration)

The family of all time spans is denoted TMSPS.

# Mathematical Formulation
**Time span format:**
(a, x) = (attack time, duration)

**Constraints:**
- a can be any real number (past or future relative to time zero)
- x must be strictly positive (events have non-zero duration)

**As elements of a space:**
TMSPS = {(a, x) : a in R, x in R+}

# Musical Context/Application
Time spans model our sense of location and extension about musical events. Unlike time-points (which have no duration) or pure durations (which have no position), time spans capture both when something happens and how long it lasts.

This is essential for analyzing music where rhythmic relationships depend on both attack times and durations--which is most music with any rhythmic complexity.

# Examples
**Concrete examples:**
- (0, 1): event starting at time 0, lasting 1 unit
- (3.5, 2): event starting at time 3.5, lasting 2 units
- (-1, 0.5): event starting 1 unit before time zero, lasting half a unit

**Musical interpretation:**
In a piece at quarter = 60 (one beat per second):
- (0, 1) = quarter note starting at the downbeat
- (1, 2) = half note starting on beat 2
- (0, 4) = whole note starting at the downbeat

**Time span vs. time point:**
A time point is just a moment; a time span is a moment plus a duration.
Time point 3 vs. time span (3, 1): the latter also specifies "lasting 1 unit."

# Related Concepts
- Time-span GIS
- Time-span Interval
- Time-span Transposition
- Attack Time
- Duration

# Common Confusions
1. **Time spans are not time intervals:** A time span (a, x) is an element of the space S. An interval between time spans is a different object (in IVLS).

2. **The duration x must be positive:** Zero-duration "events" are not time spans. This is a technical requirement.

3. **Attack time a can be negative:** Time spans can begin "before" time zero if that's useful for the analysis.

4. **Units are unspecified:** The definition doesn't fix a time unit. The entire theory is developed so that unit choice doesn't affect intervallic structure (Theorem 4.1.4).

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Definition 4.1.1, pp. 91-92
