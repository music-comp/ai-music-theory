---
concept: Referential Time-Unit Problem
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The referential time-unit problem asks: what is the absolute conceptual time-unit by which we measure durations? No such unit has privileged status, whether minute, second, beat, or any other. This motivates developing a GIS independent of unit choice.

# Formal Definition
The problem: When we write a time span (a, x), both a and x are measured in some implicit time unit. What is this unit?

Possible answers and their problems:
- Minute/second: derived from arbitrary astronomical periods
- Brevis/whole note: presupposes a notational system
- "The contextual beat": may not exist uniquely in all music

# Mathematical Formulation
**The issue formalized:**
If we change units so that x old units = xu new units, then:
- Time span (a, x) becomes (au, xu)
- In commutative GIS 4.1.2: int changes from (b-a, y/x) to ((b-a)u, y/x)
- In non-commutative GIS 4.1.3: int stays ((b-a)/x, y/x)

GIS 4.1.3 resolves the problem by being unit-independent (Theorem 4.1.4(B)).

# Musical Context/Application
This philosophical issue becomes practical when analyzing music with:
- Multiple simultaneous tempi (Carter, Nancarrow)
- No fixed beat (some electronic music, free improvisation)
- Variable tempo within instruments (Stockhausen's Klavierstuck XI)

For such music, asserting "the" referential time-unit is either impossible or arbitrary.

# Examples
**Carter String Quartet No. 1, mm. 22-32:**
What is "the" beat in this passage?
- First violin: MM36 (changing to MM72, then MM90)
- Second violin: MM96, then through several tempi
- Viola: MM180
- Cello: MM120, then MM48

No single beat governs all instruments. Each has locally referential time-units.

**Stockhausen's Klavierstuck XI:**
Each of 19 groups may be played at any of 6 tempi. Even the same group may occur at different tempi in one performance. There is no fixed "the" tempo.

**Nancarrow's Studies:**
Tempo canons with ratios like 3:4:5 or even irrational ratios like pi. The "referential unit" would need to be incommensurable with ordinary units.

**Ligeti's Poeme symphonique:**
100 metronomes at different tempi. The piece comments ironically on the very idea of a fixed time unit.

# Related Concepts
- Time-Span GIS
- Time-Span Interval Independence
- Referential Zero Time-Point
- Local Time Unit
- Multiple Tempo Layers

# Common Confusions
1. **The problem is philosophical:** It's not that we can't pick a unit, but that any choice is arbitrary/conventional rather than musically privileged.

2. **Practical workaround:** For much music, we CAN identify a contextual unit. The problem arises in music that resists such identification.

3. **GIS 4.1.3 as solution:** The time-span GIS doesn't "solve" the problem by finding the right unit. It dissolves it by being independent of unit choice.

4. **Metronome marks:** We use minutes (via MM) for practical measurement, but this is computational convenience, not musical reality.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Section 4.1 discussion, pp. 92-95
