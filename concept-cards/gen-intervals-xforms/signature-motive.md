---
concept: Signature Motive
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
A signature motive is a small partial ordering (melodic fragment) that uniquely identifies a specific row form among some family of row forms, having high "signature value" measured by INJ.

# Formal Definition
Given a row L and its forms under CANON, a partial ordering X has high "signature value" for a specific form f(L) if:
- INJ(L, X)(f) = |X| (X is fully embedded in f(L))
- For other forms g(L), INJ(L, X)(g) < |X| (X is not fully embedded)

X is a signature for f(L) within the family: if you hear X embedded in some form of L, it must be f(L).

# Mathematical Formulation
X is a signature motive for f(L) among forms {g(L) : g in CANON} if:
1. INJ(L, X)(f) = cardX (maximum)
2. INJ(L, X)(g) < cardX for all g != f in CANON

For pitch-class rows with 12-tone operations:
- X can be a signature among transposed forms (12 options)
- X can be a signature among inverted forms (12 options)
- X can be a signature among all 48 forms

Babbitt's terminology: A small motive is "uniquely characteristic" if it identifies one row form within a subarray.

# Musical Context/Application
Signature motives allow economical identification of row forms. Hearing a short melodic fragment can tell us which of 48 possible row forms is in play. Composers may use signature motives for thematic clarity or to mark structural moments.

# Examples
From Moses und Aron analysis (Cherlin):
- Row L1 of the opera
- Motive X1 = E-A-Bb (3 pairs in PROT)

INJ(L1, X1)(J) = 3, where J = I_E (inversion mapping A to E)
For all other inversions I != J: INJ(L1, X1)(I) < 3

X1 is a signature for J(L1) among inverted forms: E-A-Bb identifies the specific inverted row form.

In the opera: Moses steps on stage to a loud trombone playing E-A-Bb. This "signature motive" marks his entrance with a clear row-form identification.

From Babbitt's Reflections:
- Motive X = B-D-A identifies one transposed form
- Tetrachord Y = B-D-A-Db is a signature for the prime row among all 48 forms

# Related Concepts
- Protocol Pairs (PROT)
- Partial Ordering
- INJ (Injection Function)
- Row Forms
- Twelve-Tone Analysis

# Common Confusions
A signature motive identifies a row form, not a set class. Different row forms of the same row class will have different signature motives. The signature property depends on the specific ordering relationships in the row, not just its pitch content.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.2.4 (Moses und Aron and Babbitt discussions)
