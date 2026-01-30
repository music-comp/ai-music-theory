---
concept: Referential Zero Time-Point
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The referential zero time-point is the implicit moment from which attack times are measured. Like the time-unit, no such moment has privileged status. The time-span GIS is designed to be independent of this choice.

# Formal Definition
When we write time span (a, x), the number a measures units after an implicit "time-point zero." What is this moment?

Possible answers and their problems:
- The Big Bang / Biblical Creation: scientifically/philosophically contentious
- "A very long time ago": arbitrary
- The first barline of the piece: notation-dependent
- The first sound: may not be analytically privileged

# Mathematical Formulation
**The issue formalized:**
If we shift time-point zero back by h units:
- Time span (a, x) becomes (a + h, x)
- In any GIS on time spans: int may or may not change

**GIS 4.1.3 resolution (Theorem 4.1.4(A)):**
int((a+h, x), (b+h, y)) = int((a, x), (b, y))

The interval is independent of where we place time-point zero.

**Less problematic than time-unit:**
Even in commutative GIS 4.1.2:
int((a+h, x), (b+h, y)) = ((b+h) - (a+h), y/x) = (b - a, y/x) = int((a, x), (b, y))

Both GIS are zero-point independent. Only GIS 4.1.3 is also unit-independent.

# Musical Context/Application
Zero-point independence means: you can start the piece whenever you want, and the intervals between time spans are unchanged. This matches our intuition that rhythmic structure is intrinsic to the music, not dependent on clock time.

The zero-point problem is less severe than the unit problem because both familiar time-span GIS structures (4.1.2 and 4.1.3) are zero-point independent.

# Examples
**Practical irrelevance:**
Analyzing mm. 22-32 of Carter: whether time-point 0 is at m. 22, m. 1, or the performer's first breath, the intervals between time spans in the passage are the same.

**Score notation:**
We often place time-point zero at the first barline or first attack. This is notational convenience, not theoretical necessity.

**Performance variation:**
Different performances start at different clock times. The rhythmic structure is the same regardless.

# Related Concepts
- Referential Time-Unit Problem
- Time-Span GIS
- Time-Span Interval Independence
- Attack Time
- Theorem 4.1.4

# Common Confusions
1. **Zero-point vs. unit:** Both GIS 4.1.2 and 4.1.3 are zero-point independent. Only 4.1.3 is unit-independent. These are separate properties.

2. **LABEL depends on zero-point:** Even though intervals don't change, the LABEL function (which measures from a reference) does depend on where ref is placed.

3. **Attack times change:** When we shift zero, (a, x) becomes (a+h, x). The time span "looks different" numerically but represents the same event.

4. **Musical irrelevance:** We don't usually worry about this because zero-point independence is automatic for the GIS structures we use.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Section 4.1 discussion, pp. 95-96
