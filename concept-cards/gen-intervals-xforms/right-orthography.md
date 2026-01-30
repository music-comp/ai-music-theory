---
concept: Right Orthography
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
Right orthography is the notational convention of writing function names to the right of arguments, as in sf or (s)f, where the composition ff' means "apply f first, then f'."

# Formal Definition
In right orthography, one writes "sf" or "(s)f" for "the operand s, transformed by the function f." The composition ff' (in right notation) means "s transformed by f, then all transformed by f'," so that (s)ff' = ((s)f)f'. The function written leftmost in a composition is applied first.

# Mathematical Formulation
- (s)f denotes: apply f to s (same as f(s) in left orthography)
- (s)ff' = ((s)f)f': in the composition ff', f is applied first (then f')
- Reading order: functions apply from left to right in a composition
- The composition "ff'" in right orthography equals "f'f" in left orthography

# Musical Context/Application
Right orthography is preferred by some mathematicians and in certain music-theoretic contexts because the order of symbols matches the order of application. When a transformation network shows s -> t -> u via transformations f then g, right orthography writes this naturally as s(fg) = u. However, Lewin primarily uses left orthography throughout his book.

# Examples
In right orthography: (C)T2I means "C transformed by T2, then transformed by I."
- (C)T2 = D
- (D)I = Bb (inverting D about C)
- So (C)T2I = Bb

The same result in left orthography: IT2(C) = I(T2(C)) = I(D) = Bb.

Note: The composition "T2I" in right orthography equals "IT2" in left orthography.

# Related Concepts
- Left Orthography
- Composition of Functions
- Function
- Transformation
- Anti-homomorphism

# Common Confusions
- Right orthography reverses the appearance of composition relative to left orthography
- "fg" in right orthography = "gf" in left orthography
- Lewin uses right orthography only once in the book "when its intuitive pertinence seems overwhelming"
- Mixing conventions leads to serious errors in calculation

# Source Reference
Chapter 1: Mathematical Preliminaries, Section 1.2.4
