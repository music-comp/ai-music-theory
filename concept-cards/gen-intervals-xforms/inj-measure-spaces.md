---
concept: INJ for Measure Spaces
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
INJ can be generalized to infinite sets using measure theory, replacing cardinality with measure to answer questions like "how much of X maps into Y?"

# Formal Definition
Given:
- A family S of objects
- A field FLD of subsets of S (closed under complement, union)
- A measure mes on FLD (assigns non-negative numbers or infinity to sets)
- Sets X, Y of finite measure
- A measurable transformation f

Then: INJ(X, Y)(f) = mes(X intersect f^-1(Y))

This measures "how much of X" (according to mes) maps into Y under f.

# Mathematical Formulation
Measure-theoretic setup:
- FLD is a sigma-field: closed under countable unions
- mes is sigma-finite: mes(union X_n) = sum(mes(X_n)) for disjoint X_n
- f is measurable: f^-1(Y) in FLD whenever Y in FLD

Generalized INJ:
INJ(X, Y)(f) = mes({s in X : f(s) in Y})
             = mes(X intersect f^-1(Y))

When mes is counting measure (finite sets):
INJ(X, Y)(f) = |{s in X : f(s) in Y}| = ordinary INJ

# Musical Context/Application
Measure-theoretic INJ allows questions like:
- "Of the time the violin plays above high C, how much maps to clarinet pianissimo moments 5 seconds later?"
- "What fraction of the red-dot area in a Seurat painting lies below-left of plant areas?"
- "How much of this rhythmic passage's time-span set maps into a target configuration?"

This extends set-theoretic analysis to continuous spaces and statistical questions.

# Examples
Seurat painting model (6.10):
- S = Euclidean plane
- FLD = regions with well-defined area
- mes = area (or: count of red dots, count of yellow dots)
- X = animal regions, Y = plant regions
- f = "move up and right 3cm at 45 degrees"

INJ(X, Y)(f)/areaX = fraction of animal area that lies 3cm below-left of plants

Time-span continuum model:
- S = real numbers (time points)
- FLD = intervals and their combinations
- mes = duration ("amount of time")
- X = times when violin plays above high C
- Y = times when clarinet plays pianissimo
- f = T_5 (shift forward 5 seconds)

INJ(X, Y)(f)/mesX = fraction of "violin high C time" that precedes clarinet pp by 5 seconds

# Related Concepts
- INJ (Injection Function)
- Measure Theory
- Sigma-Field
- P-Invariant Measure
- T-Invariant Measure

# Common Confusions
Measure-theoretic INJ requires careful setup: not all subsets are measurable, and transformations must be measurable. The finite-set theory is a special case (counting measure on all subsets). For most musical applications, finite sets suffice, but continuous models open new analytical possibilities.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.10 (optional)
