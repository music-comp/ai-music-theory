---
concept: Left Orthography
slug: left-orthography

category: mathematical-foundations
subcategory: notation
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.4"

extraction_confidence: high

aliases:
  - left functional orthography

prerequisites:
  - function
  - composition-of-functions
extends: []
related:
  - anti-homomorphism
contrasts_with:
  - right-orthography

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
  - "How do I read Lewin's notation for function composition?"
---

# Quick Definition

Left orthography is the convention of writing the function name to the left of the argument, as in f(s), so that in the composition f'f, the rightmost function f is applied first.

# Core Definition

"Left (functional) orthography" is the convention in which "one writes the function name to the left of the argument in the expression 'f(s)'" and reads it as "the resulting value, when function f is applied to argument s." Accordingly, "f'f(s)" reads as "the result when f' is applied to the result of applying f to s." In right orthography, one instead writes "(s)f" and reverses the order of composition notation (Lewin, Section 1.2.4, p. 32).

# Prerequisites

- **Function** — must understand functions to discuss notation for applying them
- **Composition of Functions** — orthography determines composition notation

# Key Properties

1. Function name appears left of argument: f(s)
2. In compositions, rightmost function is applied first: (f'f)(s) = f'(f(s))
3. Reading order: functions apply right to left in a chain
4. Lewin uses left orthography almost exclusively throughout the book
5. Right orthography would reverse the composition order notation

# Construction / Recognition

## To Construct:
1. Write the function name to the left of its argument: f(s)
2. For compositions, write the last-applied function leftmost: f'f means "apply f first, then f'"

## To Recognize:
1. The function name precedes (appears left of) the argument
2. In a composition chain, operations apply from right to left

# Context & Application

Lewin chose left orthography because of the reader's "presumed familiarity" with it, even though right orthography would be "abstractly more suitable" for his eventual purposes. Right orthography is used only once in the book, at a point where "its intuitive pertinence seems overwhelming." The choice affects how composition equations are written: what Lewin calls f'f in left orthography would be called ff' in right orthography.

# Examples

**Example 1** (p. 32): "IT2 = J" in left orthography means: given any pitch class s, invert about C the 2-transpose of s, obtaining the inversion about B of the given s. T2 is applied first, then I.

**Example 2** (p. 32): In right orthography, one writes "(s)f" for what left orthography writes as "f(s)." The composition "f'f" in left orthography becomes "ff'" in right orthography, preserving the intuitive order: "(s)ff'" reads as "s-transformed-by-f, all transformed by f'."

# Relationships

## Builds Upon
- **Function** — orthography is a notation convention for functions
- **Composition of Functions** — orthography determines composition notation order

## Enables
- **Anti-Homomorphism** — the distinction between homomorphisms and anti-homomorphisms depends on orthographic convention

## Contrasts With
- **Right Orthography** — the alternative convention where function names appear right of arguments

# Common Errors

- **Error**: Reading IT2 as "invert first, then transpose by 2."
  **Correction**: In left orthography, T2 (rightmost) is applied first, then I.

# Common Confusions

- **Confusion**: Believing left and right orthography give different mathematical results.
  **Clarification**: They describe the same operations with different notation. An anti-isomorphism in left orthography becomes an isomorphism in right orthography and vice versa.

# Source Reference

Chapter 1: Mathematical Preliminaries, Section 1.2.4, p. 32.

# Verification Notes

- Definition source: direct from Section 1.2.4
- Confidence rationale: explicit definition with clear discussion of alternatives
- Re-extracted from v2 card; preserved: IT2 = J example, discussion of right orthography alternative
